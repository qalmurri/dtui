use std::{
    collections::HashMap,
    env,
    io::{self, Read, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, TryRecvError, TrySendError},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use vt100::{Parser, Screen};

use crate::application::ports::{TerminalBackend, TerminalCell, TerminalColor, TerminalScreen};
use crate::domain::window_manager::{Geometry, Window};

type PtyChild = Box<dyn Child + Send + Sync>;
type PtyMaster = Box<dyn MasterPty + Send>;

const READER_POLL_TIMEOUT_MS: i32 = 100;
const OUTPUT_QUEUE_CHUNKS: usize = 32;
pub const MANAGED_SHELL_ENV: &str = "TUICLI_MANAGED_SHELL";

pub struct PtySession {
    parser: Parser,
    writer: Box<dyn Write + Send>,
    master: PtyMaster,
    child: PtyChild,
    output: Receiver<Vec<u8>>,
    reader_thread: Option<JoinHandle<()>>,
    reader_stop: Arc<AtomicBool>,
    output_closed: bool,
    exited: bool,
}

impl PtySession {
    fn spawn(cols: u16, rows: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let cols = cols.max(1);
        let rows = rows.max(1);

        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let shell = resolve_shell();

        let mut command = CommandBuilder::new(shell);
        command.env(MANAGED_SHELL_ENV, "1");
        command.env("TERM", "xterm-256color");
        // The emulated child terminal promises xterm's 256-color profile. Do not
        // advertise truecolor separately: the outer Linux console may not support it.
        command.env_remove("COLORTERM");

        let child = pair.slave.spawn_command(command)?;
        drop(pair.slave);

        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;

        #[cfg(unix)]
        let poll_fd = pair
            .master
            .as_raw_fd()
            .ok_or_else(|| io::Error::other("PTY master tidak menyediakan file descriptor"))?;

        let (sender, output) = mpsc::sync_channel::<Vec<u8>>(OUTPUT_QUEUE_CHUNKS);
        let reader_stop = Arc::new(AtomicBool::new(false));
        let stop_signal = Arc::clone(&reader_stop);

        let reader_thread = thread::spawn(move || {
            let mut buffer = [0_u8; 4096];
            let mut pending = None;

            loop {
                if stop_signal.load(Ordering::Acquire) {
                    break;
                }

                if let Some(bytes) = pending.take() {
                    match sender.try_send(bytes) {
                        Ok(()) => {}
                        Err(TrySendError::Full(bytes)) => {
                            pending = Some(bytes);
                            thread::sleep(Duration::from_millis(2));
                            continue;
                        }
                        Err(TrySendError::Disconnected(_)) => break,
                    }
                }

                #[cfg(unix)]
                {
                    let mut poll_fd = libc::pollfd {
                        fd: poll_fd,
                        events: libc::POLLIN,
                        revents: 0,
                    };

                    // Periodic polling allows the reader to notice shutdown
                    // even when no PTY output arrives.
                    let result = unsafe { libc::poll(&mut poll_fd, 1, READER_POLL_TIMEOUT_MS) };

                    if result == 0 {
                        continue;
                    }

                    if result < 0 {
                        let error = io::Error::last_os_error();

                        if error.kind() == io::ErrorKind::Interrupted {
                            continue;
                        }

                        break;
                    }

                    if poll_fd.revents & libc::POLLNVAL != 0 {
                        break;
                    }

                    if poll_fd.revents & (libc::POLLIN | libc::POLLHUP | libc::POLLERR) == 0 {
                        continue;
                    }
                }

                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(length) => pending = Some(buffer[..length].to_vec()),
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            parser: Parser::new(rows, cols, 0),
            writer,
            master: pair.master,
            child,
            output,
            reader_thread: Some(reader_thread),
            reader_stop,
            output_closed: false,
            exited: false,
        })
    }

    fn drain_output(&mut self) -> bool {
        let mut changed = false;
        loop {
            match self.output.try_recv() {
                Ok(bytes) => {
                    self.parser.process(&bytes);
                    changed = true;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    self.output_closed = true;
                    break;
                }
            }
        }
        changed
    }

    fn poll_output(&mut self) -> bool {
        let mut changed = self.drain_output();

        if !self.exited {
            match self.child.try_wait() {
                Ok(Some(_status)) => {
                    self.exited = true;
                    changed = true;
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("TUICLI: gagal memeriksa status shell: {error}");
                }
            }
        }

        if self.output_closed
            && let Some(reader_thread) = self.reader_thread.take()
            && reader_thread.join().is_err()
        {
            eprintln!("TUICLI: reader thread terminal mengalami panic");
        }
        changed
    }

    fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.exited {
            return Ok(());
        }

        self.writer.write_all(bytes)?;
        self.writer.flush()
    }

    fn resize(&mut self, cols: u16, rows: u16) -> Result<(), Box<dyn std::error::Error>> {
        let cols = cols.max(1);
        let rows = rows.max(1);

        self.master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        self.parser.screen_mut().set_size(rows, cols);

        Ok(())
    }

    fn stop_reader(&mut self) {
        self.reader_stop.store(true, Ordering::Release);

        if let Some(reader_thread) = self.reader_thread.take()
            && reader_thread.join().is_err()
        {
            eprintln!("TUICLI: reader thread terminal mengalami panic");
        }

        self.drain_output();
        self.output_closed = true;
    }

    fn stop(&mut self) {
        if !self.exited {
            #[cfg(unix)]
            self.terminate_foreground_group();

            // portable-pty's Unix Child::kill sends SIGHUP and may escalate
            // for the direct child. It does not guarantee killing every
            // background process in the session.
            if let Err(error) = self.child.kill() {
                eprintln!("TUICLI: gagal menghentikan shell: {error}");
            }

            match self.child.wait() {
                Ok(_status) => self.exited = true,
                Err(error) => {
                    eprintln!("TUICLI: gagal menunggu shell berhenti: {error}");
                }
            }
        }

        // Keep the master alive while the reader polls its descriptor.
        // The stop flag lets the reader leave its polling loop.
        self.stop_reader();
    }

    #[cfg(unix)]
    fn terminate_foreground_group(&self) {
        let Some(group_id) = self.master.process_group_leader() else {
            return;
        };

        let Some(session_id) = self.child.process_id() else {
            return;
        };

        let Ok(session_id) = i32::try_from(session_id) else {
            return;
        };

        if group_id <= 0 || session_id <= 0 {
            return;
        }

        // Verify that the foreground process group still belongs to this
        // PTY session before sending a signal.
        let actual_session = unsafe { libc::getsid(group_id) };

        if actual_session != session_id {
            return;
        }

        // Signal the foreground job's process group, not an arbitrary PID.
        // The direct shell is handled separately by Child::kill().
        unsafe {
            libc::kill(-group_id, libc::SIGTERM);
        }
    }
}

fn resolve_shell() -> String {
    let candidate = env::var("SHELL")
        .ok()
        .filter(|shell| !shell.trim().is_empty());
    let Some(candidate) = candidate else {
        return String::from("/bin/sh");
    };

    let path = std::path::Path::new(&candidate);
    if path.is_absolute() || candidate.contains(std::path::MAIN_SEPARATOR) {
        return if is_executable(path) {
            candidate
        } else {
            String::from("/bin/sh")
        };
    }

    if let Some(search_path) = env::var_os("PATH") {
        for directory in env::split_paths(&search_path) {
            let path = directory.join(&candidate);
            if is_executable(&path) {
                return path.to_string_lossy().into_owned();
            }
        }
    }

    String::from("/bin/sh")
}

#[cfg(unix)]
fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.metadata()
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_executable(path: &std::path::Path) -> bool {
    path.is_file()
}

impl Drop for PtySession {
    fn drop(&mut self) {
        self.stop();
    }
}

#[derive(Default)]
pub struct TerminalManager {
    sessions: HashMap<u64, PtySession>,
}

impl TerminalScreen for Screen {
    fn cell(&self, row: u16, column: u16) -> Option<TerminalCell<'_>> {
        let cell = Screen::cell(self, row, column)?;
        let convert = |color| match color {
            vt100::Color::Default => TerminalColor::Default,
            vt100::Color::Idx(index) => TerminalColor::Indexed(index),
            vt100::Color::Rgb(red, green, blue) => TerminalColor::Rgb(red, green, blue),
        };

        Some(TerminalCell {
            contents: cell.contents(),
            foreground: convert(cell.fgcolor()),
            background: convert(cell.bgcolor()),
            bold: cell.bold(),
            dim: cell.dim(),
            italic: cell.italic(),
            underline: cell.underline(),
            inverse: cell.inverse(),
            has_contents: cell.has_contents(),
            wide_continuation: cell.is_wide_continuation(),
        })
    }

    fn cursor_position(&self) -> (u16, u16) {
        Screen::cursor_position(self)
    }

    fn hide_cursor(&self) -> bool {
        Screen::hide_cursor(self)
    }
}

impl TerminalManager {
    pub fn spawn_for_window(
        &mut self,
        window_id: u64,
        geometry: Geometry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.sessions.contains_key(&window_id) {
            return Ok(());
        }

        let cols = geometry.width.saturating_sub(2).max(1);
        let rows = geometry.height.saturating_sub(2).max(1);

        let session = PtySession::spawn(cols, rows)?;
        self.sessions.insert(window_id, session);

        Ok(())
    }

    pub fn remove(&mut self, window_id: u64) {
        self.sessions.remove(&window_id);
    }

    pub fn poll(&mut self) -> bool {
        let mut changed = false;
        for session in self.sessions.values_mut() {
            changed |= session.poll_output();
        }
        changed
    }

    pub fn screen(&self, window_id: u64) -> Option<&Screen> {
        self.sessions
            .get(&window_id)
            .map(|session| session.parser.screen())
    }

    pub fn is_exited(&self, window_id: u64) -> bool {
        self.sessions
            .get(&window_id)
            .is_some_and(|session| session.exited)
    }

    pub fn exited_window_ids(&self) -> Vec<u64> {
        self.sessions
            .iter()
            .filter_map(|(&id, session)| (session.exited && session.output_closed).then_some(id))
            .collect()
    }

    pub fn write_input(&mut self, window_id: u64, bytes: &[u8]) -> io::Result<()> {
        let Some(session) = self.sessions.get_mut(&window_id) else {
            return Ok(());
        };

        if !bytes.is_empty() {
            session.write(bytes)?;
        }

        Ok(())
    }

    pub fn resize_window(
        &mut self,
        window_id: u64,
        geometry: Geometry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = self.sessions.get_mut(&window_id) {
            session.resize(
                geometry.width.saturating_sub(2).max(1),
                geometry.height.saturating_sub(2).max(1),
            )?;
        }

        Ok(())
    }

    pub fn resize_all(
        &mut self,
        windows: &[Window],
        area_width: u16,
        area_height: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut failures = Vec::new();

        for window in windows {
            let geometry = window.geometry.fit(area_width, area_height);
            if let Err(error) = self.resize_window(window.id, geometry) {
                failures.push(format!("jendela {}: {error}", window.id));
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(format!("gagal resize terminal ({})", failures.join("; ")).into())
        }
    }

    pub fn start_for_windows(&mut self, windows: &[Window]) -> Vec<(u64, String)> {
        windows
            .iter()
            .filter_map(|window| {
                self.spawn_for_window(window.id, window.geometry)
                    .err()
                    .map(|error| (window.id, error.to_string()))
            })
            .collect()
    }

    pub fn close_processes(&mut self) {
        for session in self.sessions.values_mut() {
            session.stop();
        }
    }
}

impl TerminalBackend for TerminalManager {
    fn spawn_for_window(
        &mut self,
        window_id: u64,
        geometry: Geometry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        TerminalManager::spawn_for_window(self, window_id, geometry)
    }

    fn remove(&mut self, window_id: u64) {
        TerminalManager::remove(self, window_id)
    }

    fn poll(&mut self) -> bool {
        TerminalManager::poll(self)
    }

    fn screen(&self, window_id: u64) -> Option<&dyn TerminalScreen> {
        TerminalManager::screen(self, window_id).map(|screen| screen as &dyn TerminalScreen)
    }

    fn is_exited(&self, window_id: u64) -> bool {
        TerminalManager::is_exited(self, window_id)
    }

    fn exited_window_ids(&self) -> Vec<u64> {
        TerminalManager::exited_window_ids(self)
    }

    fn write_input(&mut self, window_id: u64, bytes: &[u8]) -> io::Result<()> {
        TerminalManager::write_input(self, window_id, bytes)
    }

    fn resize_window(
        &mut self,
        window_id: u64,
        geometry: Geometry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        TerminalManager::resize_window(self, window_id, geometry)
    }

    fn resize_all(
        &mut self,
        windows: &[Window],
        area_width: u16,
        area_height: u16,
    ) -> Result<(), Box<dyn std::error::Error>> {
        TerminalManager::resize_all(self, windows, area_width, area_height)
    }

    fn start_for_windows(&mut self, windows: &[Window]) -> Vec<(u64, String)> {
        TerminalManager::start_for_windows(self, windows)
    }

    fn close_processes(&mut self) {
        TerminalManager::close_processes(self)
    }
}
