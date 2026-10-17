use std::{
    error::Error,
    io::{self, IsTerminal, Write},
    path::Path,
    process::ExitCode,
};

use crossterm::{
    cursor::Show,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
        supports_keyboard_enhancement,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tuicli::{
    application::ports::{CpuFrequencyUpdates, SessionRepository},
    infrastructure::{
        config,
        pty::{MANAGED_SHELL_ENV, TerminalManager},
        session,
    },
    presentation,
};

const USAGE: &str = "Usage: tuicli [OPTION]\n\nOptions:\n  -h, --help         Tampilkan bantuan\n  -V, --version      Tampilkan versi\n      --check        Periksa kesiapan lingkungan tanpa mengubah file\n      --no-restore   Abaikan sesi tersimpan untuk peluncuran ini\n      --reset-session Hapus sesi tersimpan setelah konfirmasi";

enum Command {
    Run { no_restore: bool },
    Check,
    ResetSession,
    Help,
    Version,
}

struct TerminalGuard {
    keyboard_enhancement: bool,
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut stdout = io::stdout();

        if self.keyboard_enhancement {
            let _ = execute!(stdout, PopKeyboardEnhancementFlags);
        }

        let _ = disable_raw_mode();
        let _ = execute!(stdout, Show, LeaveAlternateScreen);
        let _ = stdout.flush();
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("tuicli: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn Error>> {
    match parse_command()? {
        Command::Help => {
            println!("{USAGE}");
            Ok(ExitCode::SUCCESS)
        }
        Command::Version => {
            println!("tuicli {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        Command::Check => check_environment(),
        Command::ResetSession => reset_session(),
        Command::Run { no_restore } => run_interactive(no_restore),
    }
}

fn parse_command() -> Result<Command, Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let Some(argument) = args.next() else {
        return Ok(Command::Run { no_restore: false });
    };

    if args.next().is_some() {
        return Err(format!("hanya satu opsi yang dapat digunakan\n\n{USAGE}").into());
    }

    match argument.as_str() {
        "-h" | "--help" => Ok(Command::Help),
        "-V" | "--version" => Ok(Command::Version),
        "--check" => Ok(Command::Check),
        "--no-restore" => Ok(Command::Run { no_restore: true }),
        "--reset-session" => Ok(Command::ResetSession),
        _ => Err(format!("opsi tidak dikenal: {argument}\n\n{USAGE}").into()),
    }
}

fn check_environment() -> Result<ExitCode, Box<dyn Error>> {
    let mut failed = false;
    let config_path = config::config_path()?;
    let config_exists = config_path.exists();
    let loaded_config = match config::load() {
        Ok(config) => {
            println!(
                "OK   konfigurasi {} ({})",
                config_path.display(),
                if config_exists {
                    "valid"
                } else {
                    "default; file belum dibuat"
                }
            );
            Some(config)
        }
        Err(error) => {
            println!("FAIL konfigurasi {}: {error}", config_path.display());
            failed = true;
            None
        }
    };

    if let Some(config) = &loaded_config {
        match tuicli::presentation::controller::keybindings::Keymap::compile(&config.keybindings) {
            Ok(_) => println!("OK   keybinding valid"),
            Err(error) => {
                println!("FAIL keybinding: {error}");
                failed = true;
            }
        }
    }

    let session_path = session::session_path()?;
    let repository = session::FileSessionRepository;
    match repository.load() {
        Ok(Some(_)) => println!("OK   sesi {} valid", session_path.display()),
        Ok(None) => println!("INFO belum ada sesi tersimpan"),
        Err(error) => {
            println!("FAIL sesi {}: {error}", session_path.display());
            println!(
                "     Gunakan --no-restore untuk melewati sesi atau --reset-session untuk menghapusnya."
            );
            failed = true;
        }
    }

    let stdin_tty = io::stdin().is_terminal();
    let stdout_tty = io::stdout().is_terminal();
    if stdin_tty && stdout_tty {
        println!("OK   stdin dan stdout terhubung ke TTY");
    } else {
        println!("FAIL tuicli memerlukan stdin dan stdout yang terhubung ke TTY interaktif");
        failed = true;
    }

    match std::env::var("TERM") {
        Ok(term) if !term.is_empty() && term != "dumb" => {
            println!("OK   TERM={term}; input dasar tersedia dengan fallback terminal")
        }
        Ok(term) if term == "dumb" => {
            println!("WARN TERM=dumb; tampilan atau tombol khusus mungkin terbatas")
        }
        _ => println!("WARN TERM tidak diatur; tuicli tetap mencoba fallback terminal"),
    }
    println!(
        "INFO input dasar tersedia; dukungan tombol tambahan diperiksa saat startup interaktif"
    );

    let shell = tuicli::infrastructure::pty::resolve_shell();
    if Path::new(&shell).is_file() {
        println!("OK   shell tersedia: {shell}");
    } else {
        println!("FAIL shell tidak tersedia: {shell}");
        failed = true;
    }

    if std::env::var_os(MANAGED_SHELL_ENV).as_deref() == Some(std::ffi::OsStr::new("1")) {
        println!("FAIL tuicli sedang berjalan di dalam shell PTY yang dikelolanya");
        failed = true;
    }

    if failed {
        println!("\nPemeriksaan selesai: ada masalah yang perlu ditangani.");
        Ok(ExitCode::FAILURE)
    } else {
        println!("\nPemeriksaan selesai.");
        Ok(ExitCode::SUCCESS)
    }
}

fn reset_session() -> Result<ExitCode, Box<dyn Error>> {
    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("--reset-session memerlukan terminal interaktif untuk konfirmasi".into());
    }

    let path = session::session_path()?;
    print!("Hapus sesi tersimpan di {}? [y/N] ", path.display());
    io::stdout().flush()?;

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)?;
    if !matches!(answer.trim(), "y" | "Y" | "yes" | "YES" | "Yes") {
        println!("Sesi tidak diubah.");
        return Ok(ExitCode::SUCCESS);
    }

    if session::remove_default()? {
        println!("Sesi tersimpan dihapus.");
    } else {
        println!("Tidak ada sesi tersimpan.");
    }

    Ok(ExitCode::SUCCESS)
}

fn run_interactive(no_restore: bool) -> Result<ExitCode, Box<dyn Error>> {
    if std::env::var_os(MANAGED_SHELL_ENV).as_deref() == Some(std::ffi::OsStr::new("1")) {
        return Err("tuicli sudah berjalan; keluar dari shell/window tuicli sebelum menjalankan instance baru".into());
    }

    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        return Err("tuicli harus dijalankan dari terminal interaktif (TTY), bukan melalui pipe atau redirect".into());
    }

    let config = config::load_or_create()?;
    let keymap =
        tuicli::presentation::controller::keybindings::Keymap::compile(&config.keybindings)?;

    let session_repository = session::FileSessionRepository;
    let (saved_session, persist_session) = if no_restore {
        (None, true)
    } else {
        match session_repository.load() {
            Ok(session) => (session, true),
            Err(error) => {
                eprintln!("TUICLI: sesi tidak dapat dipulihkan: {error}");
                (None, false)
            }
        }
    };

    enable_raw_mode()?;

    let mut guard = TerminalGuard {
        keyboard_enhancement: false,
    };

    execute!(io::stdout(), EnterAlternateScreen)?;

    if supports_keyboard_enhancement().unwrap_or(false) {
        execute!(
            io::stdout(),
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
            )
        )?;

        guard.keyboard_enhancement = true;
    }

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;
    tuicli::infrastructure::signals::install()?;

    let cpu_frequency = config.desktop.show_cpu_frequency.then(|| {
        Box::new(tuicli::infrastructure::cpu_frequency::spawn_sampler())
            as Box<dyn CpuFrequencyUpdates>
    });
    let shutdown = tuicli::infrastructure::signals::SystemShutdown;

    presentation::runtime::run(
        &mut terminal,
        &config,
        &keymap,
        saved_session,
        persist_session,
        presentation::runtime::RuntimeServices {
            terminal_backend: Box::new(TerminalManager::default()),
            session_repository: Box::new(session_repository),
            cpu_frequency,
            shutdown: Box::new(shutdown),
        },
    )?;

    Ok(ExitCode::SUCCESS)
}
