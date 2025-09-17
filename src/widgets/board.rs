//! Implementation of [`Board`] and its [associated state](BoardState)

use crate::{
    Command, CommandManaged,
    game::{CellState, Index, Player},
};

use ratatui::{layout::Flex, prelude::*};
use ratatui_macros::{constraint, constraints};
use std::iter::once;

/// State of the [`Board`] widget
///
/// This state can be used to move through cells and select the departure and destination positions used for a [jump](crate::game::Board::jump).
/// The underlying type managed by this state is [`Board`](crate::game::Board).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardState {
    board: crate::game::Board,
    current_player: Player,
    selected: Index,
    from: Option<Index>,
    to: Option<Index>,
}

impl BoardState {
    /// Create a new [`BoardState`]
    ///
    /// # Parameters
    ///
    /// - `board` - The underlying [`Board`](crate::game::Board) used
    /// - `current_player` - The first player to play
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::{
    ///     game::{Board, CellState::*, Player},
    ///     widgets::board::BoardState,
    /// };
    ///
    /// #[rustfmt::skip]
    /// let board = vec![
    ///     Red,  Free, Free,       Free, Free,
    ///     Free, Free, Free,       Free, Free,
    ///     Free, Free, Restricted, Free, Free,
    ///     Free, Free, Free,       Free, Free,
    ///     Free, Free, Free,       Free, Blue,
    /// ];
    ///
    /// let board = Board::new(5, 5, board);
    /// let _state = BoardState::new(board, Player::Blue);
    /// ```
    pub fn new(board: crate::game::Board, current_player: Player) -> Self {
        Self {
            board,
            current_player,
            selected: Index::default(),
            from: None,
            to: None,
        }
    }

    /// Get the height of the current board
    ///
    /// This method is just a wrapper around [`Board::height()`](crate::game::Board::height()).
    pub fn height(&self) -> usize {
        self.board.height()
    }

    /// Get the width of the current board
    ///
    /// This method is just a wrapper around [`Board::width()`](crate::game::Board::width()).
    pub fn width(&self) -> usize {
        self.board.width()
    }

    fn jump<F>(&mut self, jump: F)
    where
        F: FnOnce(Index) -> Index,
    {
        let selected @ (i, j) = jump(self.selected);

        if self.board.contains(i, j) {
            self.selected = selected;
        }
    }

    /// Move the selector to the left
    ///
    /// # Example
    ///
    /// ```rust
    /// # use blobwars::{
    /// #     game::{Board, CellState::*, Player},
    /// #     widgets::board::BoardState,
    /// # };
    /// #
    /// # #[rustfmt::skip]
    /// # let board = vec![
    /// #     Red,  Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Restricted, Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Blue,
    /// # ];
    /// #
    /// # let board = Board::new(5, 5, board);
    /// let mut state = BoardState::new(board, Player::Blue); // selected = (0, 0)
    /// state.right(); // selected = (1, 0)
    /// state.left(); // Selected = (0, 0)
    /// state.left(); // Selected = (0, 0)
    /// ```
    #[inline]
    pub fn left(&mut self) {
        self.jump(|(i, j)| (i, j.saturating_sub(1)));
    }

    /// Move the selector to the right
    ///
    /// # Example
    ///
    /// ```rust
    /// # use blobwars::{
    /// #     game::{Board, CellState::*, Player},
    /// #     widgets::board::BoardState,
    /// # };
    /// #
    /// # #[rustfmt::skip]
    /// # let board = vec![
    /// #     Red,  Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Restricted, Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Blue,
    /// # ];
    /// #
    /// # let board = Board::new(5, 5, board);
    /// let mut state = BoardState::new(board, Player::Blue); // selected = (0, 0)
    /// state.right(); // selected = (1, 0)
    /// ```
    #[inline]
    pub fn right(&mut self) {
        self.jump(|(i, j)| (i, j + 1));
    }

    /// Move the selector up
    /// Move the selector to the left
    ///
    /// # Example
    ///
    /// ```rust
    /// # use blobwars::{
    /// #     game::{Board, CellState::*, Player},
    /// #     widgets::board::BoardState,
    /// # };
    /// #
    /// # #[rustfmt::skip]
    /// # let board = vec![
    /// #     Red,  Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Restricted, Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Blue,
    /// # ];
    /// #
    /// # let board = Board::new(5, 5, board);
    /// let mut state = BoardState::new(board, Player::Blue); // selected = (0, 0)
    /// state.down(); // selected = (0, 1)
    /// state.up(); // Selected = (0, 0)
    /// state.up(); // Selected = (0, 0)
    /// ```
    #[inline]
    pub fn up(&mut self) {
        self.jump(|(i, j)| (i.saturating_sub(1), j));
    }

    /// Move the selector down
    ///
    /// # Example
    ///
    /// ```rust
    /// # use blobwars::{
    /// #     game::{Board, CellState::*, Player},
    /// #     widgets::board::BoardState,
    /// # };
    /// #
    /// # #[rustfmt::skip]
    /// # let board = vec![
    /// #     Red,  Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Restricted, Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Blue,
    /// # ];
    /// #
    /// # let board = Board::new(5, 5, board);
    /// let mut state = BoardState::new(board, Player::Blue); // selected = (0, 0)
    /// state.down(); // selected = (0, 1)
    /// ```
    #[inline]
    pub fn down(&mut self) {
        self.jump(|(i, j)| (i + 1, j));
    }

    /// Remove last selected position
    ///
    /// # Example
    ///
    /// ```rust
    /// # use blobwars::{
    /// #     game::{Board, CellState::*, Player},
    /// #     widgets::board::BoardState,
    /// # };
    /// #
    /// # #[rustfmt::skip]
    /// # let board = vec![
    /// #     Red,  Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Restricted, Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Blue,
    /// # ];
    /// #
    /// # let board = Board::new(5, 5, board);
    /// let mut state = BoardState::new(board, Player::Blue); // 0 selected & selected = (0, 0)
    /// state.select(); // 1 selected & selected = (0, 0)
    /// state.down(); // 1 selected & selected = (0, 1)
    /// state.select(); // 2 selected & selected = (0, 1)
    /// state.reset(); // 1 selected & selected = (0, 1)
    /// state.reset(); // 0 selected & selected = (0, 1)
    /// state.reset(); // 0 selected & selected = (0, 1)
    pub fn reset(&mut self) {
        if self.to.is_some() {
            self.to = None;
        } else {
            self.from = None;
        }
    }

    /// Select the cell just below the selector
    ///
    /// The first selection corresponds to the departure point; the second one corresponds to the destination point.
    /// The departure point must be the color of the current player and the destination point must be [free](CellState::Free); if it is not respected, [`Self::select()`] has no effects.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use blobwars::{
    /// #     game::{Board, CellState::*, Player},
    /// #     widgets::board::BoardState,
    /// # };
    /// #
    /// # #[rustfmt::skip]
    /// # let board = vec![
    /// #     Red,  Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Restricted, Free, Free,
    /// #     Free, Free, Free,       Free, Free,
    /// #     Free, Free, Free,       Free, Blue,
    /// # ];
    /// #
    /// # let board = Board::new(5, 5, board);
    /// let mut state = BoardState::new(board, Player::Blue); // 0 selected & selected = (0, 0)
    /// state.select(); // 1 selected & selected = (0, 0)
    /// state.down(); // 1 selected & selected = (0, 1)
    /// state.select(); // 2 selected & selected = (0, 1)
    /// state.select(); // jump & 0 selected & selected = (0, 1)
    pub fn select(&mut self) {
        let current_cell = self.board.get(self.selected.0, self.selected.1);

        if let Some(from) = self.from {
            if let Some(to) = self.to {
                self.board.jump(from, to);

                self.from = None;
                self.to = None;
            } else if current_cell.map(CellState::is_free).unwrap_or_default() {
                self.to = Some(self.selected);
            }
        } else if current_cell
            .and_then(|selected| selected.try_into().ok())
            .map(|selected: Player| selected == self.current_player)
            .unwrap_or_default()
        {
            self.from = Some(self.selected);
        }
    }

    /// Pass to the next player
    ///
    /// The next player is the opponent.
    pub fn pass_to_next_player(&mut self) {
        self.current_player = -self.current_player;
    }
}

impl CommandManaged for BoardState {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Reset => self.reset(),

            Command::Select => {
                self.select();
                self.pass_to_next_player();
            }

            Command::Left => self.left(),
            Command::Right => self.right(),
            Command::Up => self.up(),
            Command::Down => self.down(),
            _ => (),
        }
    }
}

/// A board allowing to show a [`Board`](crate::game::Board)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board<'a> {
    /// The selected symbol
    ///
    /// This symbol is used to show the selected cell.
    pub selected_symbol: &'a str,

    /// The unselected symbol
    ///
    /// This symbol is used to show unselected cells.
    pub unselected_symbol: &'a str,
}

impl<'a> Board<'a> {
    /// Set the [selected symbol](Self::selected_symbol)
    pub fn selected_symbol(self, selected_symbol: &'a str) -> Self {
        Self {
            selected_symbol,
            ..self
        }
    }

    /// Set the [unselected symbol](Self::unselected_symbol)
    pub fn unselected_symbol(self, unselected_symbol: &'a str) -> Self {
        Self {
            unselected_symbol,
            ..self
        }
    }
}

impl Default for Board<'static> {
    fn default() -> Self {
        Self {
            selected_symbol: "V",
            unselected_symbol: "O",
        }
    }
}

impl StatefulWidget for Board<'_> {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [area] = Layout::vertical(constraints![==100%]).areas(area);
        let [area] = Layout::horizontal(constraints![==100%]).areas(area);

        let constraints = once(constraint!(==1)).cycle().take(state.height());
        let mut state_iterator = state.board.iter();
        let rows = Layout::vertical(constraints).flex(Flex::Center).split(area);

        for (i, row) in rows.iter().copied().enumerate() {
            let constraints = once(constraint!(==1)).cycle().take(state.width());

            let column = Layout::horizontal(constraints)
                .flex(Flex::Center)
                .spacing(1)
                .split(row);

            for (j, area) in column.iter().copied().enumerate() {
                if let Some(current) = state_iterator.next() {
                    let content = if (i, j) == state.selected {
                        self.selected_symbol
                    } else {
                        self.unselected_symbol
                    };

                    Text::raw(content).fg(current).render(area, buf);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::CellState::*;

    fn tested(pre: impl FnOnce(&mut BoardState)) -> Buffer {
        let area = Rect::new(0, 0, 9, 5);
        let mut tested = Buffer::empty(area);

        #[rustfmt::skip]
        let board = vec![
            Red,  Free, Free,       Free, Free,
            Free, Free, Free,       Free, Free,
            Free, Free, Restricted, Free, Free,
            Free, Free, Free,       Free, Free,
            Free, Free, Free,       Free, Blue,
        ];

        let board = crate::game::Board::new(5, 5, board);
        let mut state = BoardState::new(board, Player::Red);
        let widget = Board::default();

        pre(&mut state);
        widget.render(area, &mut tested, &mut state);

        tested
    }

    fn expected<'a, Iter>(lines: Iter) -> Buffer
    where
        Iter: IntoIterator,
        Iter::Item: Into<Line<'a>>,
    {
        let mut expected = Buffer::with_lines(lines);
        expected.set_style(Rect::new(0, 0, 1, 1), Color::Red);
        expected.set_style(Rect::new(4, 2, 1, 1), Color::Rgb(0xff, 0xa5, 0x00));
        expected.set_style(Rect::new(8, 4, 1, 1), Color::Blue);

        expected
    }

    #[test]
    fn render() {
        let lines = [
            "V O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
        ];

        let expected = expected(lines);
        let tested = tested(|_| {});

        pretty_assertions::assert_eq!(tested, expected);
    }

    #[test]
    fn left() {
        let lines = [
            "V O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
        ];

        let expected = expected(lines);

        let tested = tested(|state| {
            state.right();
            state.left();
        });

        pretty_assertions::assert_eq!(tested, expected);
    }

    #[test]
    fn right() {
        let lines = [
            "O V O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
        ];

        let expected = expected(lines);
        let tested = tested(|state| state.right());

        pretty_assertions::assert_eq!(tested, expected);
    }

    #[test]
    fn up() {
        let lines = [
            "V O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
        ];

        let expected = expected(lines);

        let tested = tested(|state| {
            state.down();
            state.up();
        });

        pretty_assertions::assert_eq!(tested, expected);
    }

    #[test]
    fn down() {
        let lines = [
            "O O O O O",
            "V O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
        ];

        let expected = expected(lines);
        let tested = tested(|state| state.down());

        pretty_assertions::assert_eq!(tested, expected);
    }

    #[test]
    fn blob() {
        let lines = [
            "O O O O O",
            "O O O O O",
            "O O O O O",
            "O O O V O",
            "O O O O O",
        ];

        let mut expected = expected(lines);
        expected.set_style(Rect::new(6, 3, 1, 1), Color::Red);
        expected.set_style(Rect::new(8, 4, 1, 1), Color::Red);

        let tested = tested(|state| {
            state.select();

            state.down();
            state.right();

            state.select();
            state.select();
            state.select();

            state.down();
            state.right();
            state.down();
            state.right();

            state.select();
            state.select();
        });

        pretty_assertions::assert_eq!(tested, expected);
    }

    #[test]
    fn jump() {
        let lines = [
            "O O O O O",
            "O O O O O",
            "V O O O O",
            "O O O O O",
            "O O O O O",
        ];

        let mut expected = expected(lines);
        expected.set_style(Rect::new(0, 0, 1, 1), Color::default());
        expected.set_style(Rect::new(0, 2, 1, 1), Color::Red);

        let tested = tested(|state| {
            state.select();
            state.down();
            state.down();
            state.select();
            state.select();
        });

        pretty_assertions::assert_eq!(tested, expected);
    }

    #[test]
    fn spread() {
        let lines = [
            "O O O O O",
            "V O O O O",
            "O O O O O",
            "O O O O O",
            "O O O O O",
        ];

        let mut expected = expected(lines);
        expected.set_style(Rect::new(0, 1, 1, 1), Color::Red);

        let tested = tested(|state| {
            state.select();
            state.down();
            state.select();
            state.select();
        });

        pretty_assertions::assert_eq!(tested, expected);
    }
}
