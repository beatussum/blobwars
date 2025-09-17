//! TODO

#![warn(missing_docs)]

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    prelude::*,
};

use ratatui_macros::constraints;

use widgets::{
    Credits, Logo, Theme,
    board::{BoardState, Score},
};

pub mod game;
pub mod widgets;

/// Commands used to perform actions based on user inputs
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Command {
    /// Go back
    Back,

    /// Exit
    Exit,

    /// Reset current progress
    Reset,

    /// Select current selected item
    Select,

    /// Go left
    Left,

    /// Go right
    Right,

    /// Go up
    Up,

    /// Go down
    Down,
}

impl TryFrom<Event> for Command {
    type Error = &'static str;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        if let Event::Key(value) = value
            && value.kind == KeyEventKind::Press
        {
            match value.code {
                KeyCode::Backspace => Ok(Self::Reset),
                KeyCode::Enter => Ok(Self::Select),
                KeyCode::Left => Ok(Self::Left),
                KeyCode::Right => Ok(Self::Right),
                KeyCode::Up => Ok(Self::Up),
                KeyCode::Down => Ok(Self::Down),
                KeyCode::Char('q') => Ok(Self::Exit),
                KeyCode::Esc => Ok(Self::Back),
                _ => Err("The key is not recognized as a valid command"),
            }
        } else {
            Err("Only key press events are valid commands")
        }
    }
}

/// Trait allowing to manage states with [commands](Command)
pub trait CommandManaged {
    /// Handle given [command](Command)
    fn handle_command(&mut self, command: Command);
}

/// Main state of the application
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ApplicationState {
    /// The main state of the application with the [`Board` widget](widgets::board::Board)
    Board(BoardState),

    /// The application has exited
    Exit,

    /// Default state showing the application logo
    #[default]
    Logo,
}

impl ApplicationState {
    /// Check if the application has exited
    pub fn has_exited(&self) -> bool {
        *self == Self::Exit
    }
}

impl CommandManaged for ApplicationState {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Back => match self {
                Self::Board(_) => *self = Self::Logo,
                Self::Exit => (),
                Self::Logo => *self = Self::Exit,
            },

            Command::Exit => *self = Self::Exit,

            _ => match self {
                Self::Board(board_state) => board_state.handle_command(command),
                Self::Exit => (),

                Self::Logo => {
                    if matches!(command, Command::Select) {
                        use game::CellState::*;
                        use game::Player::*;

                        #[rustfmt::skip]
                        let board = vec![
                            Player(Red),  Free, Free, Free, Free, Free, Free, Free,
                            Free,         Free, Free, Free, Free, Free, Free, Free,
                            Free,         Free, Free, Free, Free, Free, Free, Free,
                            Free,         Free, Free, Free, Free, Free, Free, Free,
                            Free,         Free, Free, Free, Free, Free, Free, Free,
                            Free,         Free, Free, Free, Free, Free, Free, Free,
                            Free,         Free, Free, Free, Free, Free, Free, Free,
                            Free,         Free, Free, Free, Free, Free, Free, Player(Blue),
                        ];

                        let board = game::Board::new(8, 8, board);
                        let board_state = BoardState::new(board, Blue);
                        *self = Self::Board(board_state);
                    }
                }
            },
        }
    }
}

/// Main widget of the application
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Application<'a> {
    /// The selected symbol
    ///
    /// This symbol is used to show the selected cell.
    pub selected_symbol: &'a str,

    /// The unselected symbol
    ///
    /// This symbol is used to show unselected cells.
    pub unselected_symbol: &'a str,

    /// The [theme](Theme) used to colorize text
    pub theme: Theme,
}

impl Default for Application<'static> {
    fn default() -> Self {
        Self {
            selected_symbol: "V",
            unselected_symbol: "O",
            theme: Theme::default(),
        }
    }
}

impl StatefulWidget for Application<'_> {
    type State = ApplicationState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        match state {
            Self::State::Board(state) => {
                let [top, bottom] = Layout::vertical(constraints![==60%, ==40%]).areas(area);
                let [left, right] = Layout::horizontal(constraints![==80%, ==20%]).areas(top);

                widgets::board::Board::default()
                    .selected_symbol(self.selected_symbol)
                    .unselected_symbol(self.unselected_symbol)
                    .render(left, buf, state);

                Score { theme: self.theme }.render(right, buf, state);
                Credits { theme: self.theme }.render(bottom, buf);
            }

            Self::State::Exit => (),
            Self::State::Logo => Logo { theme: self.theme }.render(area, buf),
        }
    }
}
