use std::{
    collections::HashMap,
    env,
    io::{Read, Write},
    sync::mpsc::{self, Receiver},
    thread,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use portable_pty::{Child, CommandBuilder, MasterPty, PtySize, native_pty_system};
use vt100::{Parser, Screen};

use crate::window_manager::{Geometry, Window};

type PtyChild = Box<dyn Child + Send + Sync>;
type PtyMaster = Box<dyn MasterPty + Send>;

pub struct PtySession {
    parser: Parser,
    writer: Box<dyn Write + Send>,
    master: PtyMaster,
    child: PtyChild,
    output: Receiver<Vec<u8>>,
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

        let shell = env::var("SHELL").unwrap_or_else(|_| String::from("/bin/sh"));
        let mut command = CommandBuilder::new(shell);
        command.env("TERM", "xterm-256color");
        command.env("COLORTERM", "truecolor");

        let child = pair.slave.spawn_command(command)?;
        drop(pair.slave);

        let mut reader = pair.master.try_clone_reader()?;
        let writer = pair.master.take_writer()?;
        let (sender, output) = mpsc::channel();

        thread::spawn(move || {
            let mut buffer = [0_u8; 4096];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(length) => {
                        if sender.send(buffer[..length].to_vec()).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Ok(Self {
            parser: Parser::new(rows, cols, 10_000),
            writer,
            master: pair.master,
            child,
            output,
            exited: false,
        })
    }

    fn poll_output(&mut self) {
        for bytes in self.output.try_iter() {
            self.parser.process(&bytes);
        }

        if !self.exited {
            match self.child.try_wait() {
                Ok(Some(_)) => self.exited = true,
                Ok(None) => {}
                Err(_) => self.exited = true,
            }
        }
    }

    fn write(&mut self, bytes: &[u8]) -> std::io::Result<()> {
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

    fn stop(&mut self) {
        if !self.exited {
            let _ = self.child.kill();
            let _ = self.child.try_wait();
            self.exited = true;
        }
    }
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

    pub fn poll(&mut self) {
        for session in self.sessions.values_mut() {
            session.poll_output();
        }
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
            .filter_map(|(&id, session)| session.exited.then_some(id))
            .collect()
    }

    pub fn write_key(&mut self, window_id: u64, key: KeyEvent) -> std::io::Result<()> {
        let Some(session) = self.sessions.get_mut(&window_id) else {
            return Ok(());
        };
        let bytes = key_to_bytes(key);
        if !bytes.is_empty() {
            session.write(&bytes)?;
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
        for window in windows {
            let geometry = window.geometry.fit(area_width, area_height);
            self.resize_window(window.id, geometry)?;
        }
        Ok(())
    }

    pub fn start_for_windows(
        &mut self,
        windows: &[Window],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for window in windows {
            self.spawn_for_window(window.id, window.geometry)?;
        }
        Ok(())
    }

    pub fn close_processes(&mut self) {
        for session in self.sessions.values_mut() {
            session.stop();
        }
    }
}

pub fn key_to_bytes(key: KeyEvent) -> Vec<u8> {
    let modifiers = key.modifiers;
    if modifiers.contains(KeyModifiers::CONTROL)
        && let KeyCode::Char(ch) = key.code
    {
        let ch = ch.to_ascii_lowercase();

        if ch.is_ascii_lowercase() {
            return vec![ch as u8 - b'a' + 1];
        }

        return match ch {
            '@' | ' ' => vec![0],
            '[' => vec![0x1b],
            '\\' => vec![0x1c],
            ']' => vec![0x1d],
            '^' => vec![0x1e],
            '_' => vec![0x1f],
            _ => ch.to_string().into_bytes(),
        };
    }

    let mut bytes = match key.code {
        KeyCode::Char(ch) => ch.to_string().into_bytes(),
        KeyCode::Enter => b"\r".to_vec(),
        KeyCode::Backspace => vec![0x7f],
        KeyCode::Tab => b"\t".to_vec(),
        KeyCode::BackTab => b"\x1b[Z".to_vec(),
        KeyCode::Esc => vec![0x1b],
        KeyCode::Up => b"\x1b[A".to_vec(),
        KeyCode::Down => b"\x1b[B".to_vec(),
        KeyCode::Right => b"\x1b[C".to_vec(),
        KeyCode::Left => b"\x1b[D".to_vec(),
        KeyCode::Home => b"\x1b[H".to_vec(),
        KeyCode::End => b"\x1b[F".to_vec(),
        KeyCode::PageUp => b"\x1b[5~".to_vec(),
        KeyCode::PageDown => b"\x1b[6~".to_vec(),
        KeyCode::Delete => b"\x1b[3~".to_vec(),
        KeyCode::Insert => b"\x1b[2~".to_vec(),
        KeyCode::F(1) => b"\x1bOP".to_vec(),
        KeyCode::F(2) => b"\x1bOQ".to_vec(),
        KeyCode::F(3) => b"\x1bOR".to_vec(),
        KeyCode::F(4) => b"\x1bOS".to_vec(),
        KeyCode::F(n @ 5..=12) => format!("\x1b[{}~", 15 + (n - 5) * 2).into_bytes(),
        _ => Vec::new(),
    };

    if modifiers.contains(KeyModifiers::ALT) && !bytes.is_empty() {
        bytes.insert(0, 0x1b);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::key_to_bytes;

    #[test]
    fn encodes_regular_and_control_keys() {
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE)),
            b"a".to_vec()
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            vec![3]
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
            b"\r".to_vec()
        );
        assert_eq!(
            key_to_bytes(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
            b"\x1b[A".to_vec()
        );
    }
}
