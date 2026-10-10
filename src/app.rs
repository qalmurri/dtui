use std::{error::Error, time::Duration};

use crossterm::event::{Event, KeyEventKind};

use crate::{
    Tui,
    config::Config,
    event,
    identity::Identity,
    keybindings::{self, Action},
    ui,
};

pub fn run(terminal: &mut Tui, config: &Config, identity: &Identity) -> Result<(), Box<dyn Error>> {
    loop {
        terminal
            .draw(|frame| ui::layout::render_dashboard(frame, &config.keybindings, identity))?;

        if let Some(Event::Key(key)) = event::poll_event(Duration::from_millis(100))? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match keybindings::resolve(key.code, &config.keybindings) {
                Some(Action::Quit) | Some(Action::Back) => break,
                Some(Action::NavigateUp)
                | Some(Action::NavigateDown)
                | Some(Action::Select)
                | None => {}
            }
        }
    }

    Ok(())
}
