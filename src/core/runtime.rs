use std::{io, time::Duration};

use crossterm::event::Event;

pub fn poll(timeout: Duration) -> io::Result<Option<Event>> {
    crate::event::poll_event(timeout)
}
