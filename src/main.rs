use blobwars::{Application, ApplicationState, Command, CommandManaged};
use ratatui::{DefaultTerminal, crossterm::event};
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
    let mut application_state = ApplicationState::default();

    while !application_state.has_exited() {
        terminal.draw(|frame| {
            frame.render_stateful_widget(
                Application::default(),
                frame.area(),
                &mut application_state,
            )
        })?;

        if event::poll(Duration::from_millis(500))?
            && let Ok(command) = Command::try_from(event::read()?)
        {
            application_state.handle_command(command);
        }
    }

    Ok(())
}
