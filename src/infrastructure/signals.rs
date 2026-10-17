use std::sync::atomic::{AtomicBool, Ordering};

static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

extern "C" fn request_shutdown(_: libc::c_int) {
    SHUTDOWN_REQUESTED.store(true, Ordering::Relaxed);
}

/// Install minimal signal handlers so the main loop can close PTYs and let
/// the terminal guard restore the user's TTY on normal process termination.
pub fn install() -> std::io::Result<()> {
    #[cfg(unix)]
    unsafe {
        for signal in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
            if libc::signal(signal, request_shutdown as *const () as libc::sighandler_t)
                == libc::SIG_ERR
            {
                return Err(std::io::Error::last_os_error());
            }
        }
    }

    Ok(())
}

pub fn shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemShutdown;

impl crate::application::ports::ShutdownStatus for SystemShutdown {
    fn is_requested(&self) -> bool {
        shutdown_requested()
    }
}
