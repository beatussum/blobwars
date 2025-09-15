//! The implementation of [`Board`] and its associated types

use ratatui::style::Color;
use std::ops::Neg;

/// An enumeration reprensenting the state of a cell
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum CellState {
    /// The cell is occupied by the blue player
    Blue,

    /// The cell is occupied by the red player
    Red,

    /// The cell is free
    #[default]
    Free,

    /// The cell is restricted
    ///
    /// A restricted cell cannot be occupied by any player.
    Restricted,
}

impl CellState {
    /// Check if the current cell is occupied by the [`Self::Blue`] player
    pub fn is_blue(self) -> bool {
        self == Self::Blue
    }

    /// Check if the current cell is occupied by the [`Self::Red`] player
    pub fn is_red(self) -> bool {
        self == Self::Red
    }

    /// Check if the current cell is [`Self::Free`]
    pub fn is_free(self) -> bool {
        self == Self::Free
    }

    /// Check if the current cell is [`Self::Restricted`]
    pub fn is_restricted(self) -> bool {
        self == Self::Restricted
    }

    /// Check if the current cell is occupied
    ///
    /// An occupied cell is not [free](Self::Free).
    pub fn is_occupied(self) -> bool {
        !self.is_free()
    }

    /// Check if the current cell is occupied by a player
    pub fn is_playable(self) -> bool {
        matches!(self, Self::Blue | Self::Red)
    }

    /// Checks if the current cell and `other` are opposite colors
    ///
    /// # Parameter
    ///
    /// - `other` - The other cell
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::CellState::*;
    ///
    /// assert!(!Free.is_opponent_of(Free));
    /// assert!(!Free.is_opponent_of(Blue));
    /// assert!(Red.is_opponent_of(Blue));
    /// ```
    pub fn is_opponent_of(self, other: Self) -> bool {
        match self {
            Self::Blue => other == Self::Red,
            Self::Red => other == Self::Blue,
            _ => false,
        }
    }
}

impl From<Player> for CellState {
    fn from(value: Player) -> Self {
        match value {
            Player::Blue => Self::Blue,
            Player::Red => Self::Red,
        }
    }
}

impl From<CellState> for Color {
    fn from(value: CellState) -> Self {
        match value {
            CellState::Blue => Self::Blue,
            CellState::Red => Self::Red,
            CellState::Free => Self::default(),
            CellState::Restricted => Self::Rgb(0xff, 0xa5, 0x00),
        }
    }
}

/// A player
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    /// The blue player
    Blue,

    /// The red player
    Red,
}

impl Neg for Player {
    type Output = Self;

    /// Get the opposing player
    fn neg(self) -> Self::Output {
        match self {
            Self::Blue => Self::Red,
            Self::Red => Self::Blue,
        }
    }
}

impl TryFrom<CellState> for Player {
    type Error = &'static str;

    fn try_from(value: CellState) -> Result<Self, Self::Error> {
        match value {
            CellState::Blue => Ok(Self::Blue),
            CellState::Red => Ok(Self::Red),
            _ => Err("The cell is not playable"),
        }
    }
}

/// An index of the [`Board`]
pub type Index = (usize, usize);

/// A game board
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Board {
    board: Vec<CellState>,
    height: usize,
    width: usize,
}

impl Board {
    /// Create a free [`Board`]
    ///
    /// # Parameters
    ///
    /// - `height` - The height of the grid
    /// - `width` - The width of the grid
    pub fn free(height: usize, width: usize) -> Self {
        let board = vec![CellState::Free; height * width];

        Self {
            board,
            height,
            width,
        }
    }

    /// Create a new [`Board`]
    ///
    /// # Parameters
    ///
    /// - `height` - The height of the grid
    /// - `width` - The width of the grid
    /// - `board` - The underlying board (its size must be consistent)
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::{Board, CellState};
    /// use std::panic::catch_unwind;
    ///
    /// Board::new(5, 5, vec![CellState::Free; 25]);
    /// let is_panicking = catch_unwind(|| Board::new(5, 5, Vec::default())).is_err();
    /// assert!(is_panicking);
    /// ```
    pub fn new(height: usize, width: usize, board: Vec<CellState>) -> Self {
        assert_eq!(board.len(), height * width);

        Self {
            board,
            height,
            width,
        }
    }

    /// Get the height of the underlying grid
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the width of the underlying grid
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the length of the underlying grid
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::Board;
    ///
    /// let board = Board::free(5, 5);
    /// assert_eq!(board.len(), 25);
    /// ```
    pub fn len(&self) -> usize {
        self.board.len()
    }

    /// Check if the underlying grid is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::Board;
    ///
    /// let board = Board::default();
    /// assert!(board.is_empty());
    /// assert_eq!(board.len(), 0);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.board.is_empty()
    }

    /// Get the distance between two positions
    ///
    /// # Parameters
    ///
    /// - `from` - The departure [position](Index)
    /// - `to` - The destination [position](Index)
    ///
    /// # Return
    ///
    /// This function returns the jump distance if the positions and the jump distance are valid; otherwise, `None` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::Board;
    ///
    /// let board = Board::free(5, 5);
    ///
    /// assert_eq!(board.jump_distance((0, 0), (0, 1)), Some(1));
    /// assert_eq!(board.jump_distance((0, 0), (0, 2)), Some(2));
    /// assert_eq!(board.jump_distance((0, 0), (0, 3)), None);
    /// assert_eq!(board.jump_distance((4, 4), (4, 5)), None);
    /// ```
    pub fn jump_distance(&self, from: Index, to: Index) -> Option<usize> {
        (self.contains(from.0, from.1) && self.contains(to.0, to.1))
            .then(|| from.0.abs_diff(to.0).max(from.1.abs_diff(to.1)))
            .filter(|distance| (1..=2).contains(distance))
    }

    /// Check if the given index is valid
    ///
    /// # Parameters
    ///
    /// - `row` - The row index
    /// - `column` - The column index
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::Board;
    ///
    /// let board = Board::free(5, 5);
    ///
    /// assert!(board.contains(0, 0));
    /// assert!(!board.contains(5, 0));
    /// ```
    pub fn contains(&self, row: usize, column: usize) -> bool {
        (0..self.height).contains(&row) && (0..self.width).contains(&column)
    }

    /// Get the state of the corresponding cell
    ///
    /// # Parameters
    ///
    /// - `row` - The row index
    /// - `column` - The column index
    ///
    /// # Return
    ///
    /// If the given indexes are not valid, `None` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::{Board, CellState};
    ///
    /// let board = Board::free(5, 5);
    ///
    /// assert_eq!(board.get(0, 0), Some(CellState::Free));
    /// assert_eq!(board.get(5, 0), None);
    /// ```
    pub fn get(&self, row: usize, column: usize) -> Option<CellState> {
        if self.contains(row, column) {
            self.board.get(row * self.width + column).copied()
        } else {
            None
        }
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut CellState> {
        if self.contains(row, column) {
            self.board.get_mut(row * self.width + column)
        } else {
            None
        }
    }

    /// Get an [iterator](Iterator) over the cells
    ///
    /// # Example
    ///
    /// ```
    /// use blobwars::game::{Board, CellState};
    ///
    /// let board = Board::free(2, 2);
    /// let mut iterator = board.iter();
    ///
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = CellState> {
        self.board.iter().copied()
    }

    fn neighbors_mut(
        &mut self,
        row: usize,
        column: usize,
        radius: usize,
    ) -> impl Iterator<Item = &mut CellState> {
        self.contains(row, column)
            .then(|| {
                self.board
                    .chunks_mut(self.width)
                    .skip(row.saturating_sub(radius))
                    .take(1 + radius + row.saturating_sub(radius).min(radius))
                    .flat_map(move |row| {
                        row.iter_mut()
                            .skip(column.saturating_sub(radius))
                            .take(1 + radius + column.saturating_sub(radius).min(radius))
                    })
            })
            .into_iter()
            .flatten()
    }

    fn blob(&mut self, row: usize, column: usize) -> bool {
        if let Some(new) = self.get(row, column) {
            for state in self
                .neighbors_mut(row, column, 1)
                .filter(|&&mut current| current.is_opponent_of(new))
            {
                *state = new;
            }

            true
        } else {
            false
        }
    }

    /// Jump a _blob_ from one point to another
    ///
    /// # Parameter
    ///
    /// - `from` - The departure [position](Index)
    /// - `to` - The destination [position](Index)
    ///
    /// # Return
    ///
    /// If the operation succeeds, `true` is returned; otherwise, `false` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use blobwars::game::{Board, CellState::*};
    ///
    /// #[rustfmt::skip]
    /// let tested = vec![
    ///     Red,  Red,  Free,       Free,       Free,
    ///     Red,  Red,  Restricted, Free,       Free,
    ///     Free, Blue, Free,       Restricted, Free,
    ///     Free, Free, Free,       Free,       Blue,
    ///     Free, Free, Free,       Free,       Blue,
    /// ];
    ///
    /// let mut tested = Board::new(5, 5, tested);
    /// let status = tested.jump((1, 1), (3, 3));
    ///
    /// #[rustfmt::skip]
    /// let expected = vec![
    ///     Red,  Red,  Free,       Free,       Free,
    ///     Red,  Free, Restricted, Free,       Free,
    ///     Free, Blue, Free,       Restricted, Free,
    ///     Free, Free, Free,       Red,        Red,
    ///     Free, Free, Free,       Free,       Red,
    /// ];
    ///
    /// let expected = Board::new(5, 5, expected);
    ///
    /// assert!(status);
    /// assert_eq!(tested, expected);
    /// ```
    pub fn jump(&mut self, from: Index, to: Index) -> bool {
        if let Some(distance) = self.jump_distance(from, to) {
            let new_state = self
                .get(from.0, from.1)
                .filter(|new_state| new_state.is_playable());

            let to_update = self
                .get_mut(to.0, to.1)
                .filter(|to_update| to_update.is_free());

            if let Some((new_state, to_update)) = new_state.zip(to_update) {
                *to_update = new_state;

                if distance == 2 {
                    // It cannot panic because it has been checked just before
                    // that `from` is a valid index.
                    *self.get_mut(from.0, from.1).unwrap() = CellState::Free;
                }

                self.blob(to.0, to.1)
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl IntoIterator for Board {
    type Item = CellState;
    type IntoIter = <Vec<CellState> as IntoIterator>::IntoIter;

    /// Get an [iterator](Iterator) over the cells
    ///
    /// # Example
    ///
    /// ```
    /// use blobwars::game::{Board, CellState};
    ///
    /// let board = Board::free(2, 2);
    /// let mut iterator = board.into_iter();
    ///
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), Some(CellState::Free));
    /// assert_eq!(iterator.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.board.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use CellState::*;

    mod neighbors {
        use super::*;

        fn check(row: usize, column: usize, expected: Vec<CellState>) {
            let board = vec![Free; 25];
            let mut tested = Board::new(5, 5, board);

            for state in tested.neighbors_mut(row, column, 1) {
                *state = Restricted;
            }

            let expected = Board::new(5, 5, expected);
            pretty_assertions::assert_eq!(tested, expected);
        }

        #[test]
        fn mid() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Free,       Free,       Free,       Free,
                Free, Restricted, Restricted, Restricted, Free,
                Free, Restricted, Restricted, Restricted, Free,
                Free, Restricted, Restricted, Restricted, Free,
                Free, Free,       Free,       Free,       Free,
            ];

            check(2, 2, expected);
        }

        #[test]
        fn top() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Restricted, Restricted, Restricted, Free,
                Free, Restricted, Restricted, Restricted, Free,
                Free, Free,       Free,       Free,       Free,
                Free, Free,       Free,       Free,       Free,
                Free, Free,       Free,       Free,       Free,
            ];

            check(0, 2, expected);
        }

        #[test]
        fn bottom() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Free,       Free,       Free,       Free,
                Free, Free,       Free,       Free,       Free,
                Free, Free,       Free,       Free,       Free,
                Free, Restricted, Restricted, Restricted, Free,
                Free, Restricted, Restricted, Restricted, Free,
            ];

            check(4, 2, expected);
        }

        #[test]
        fn left() {
            #[rustfmt::skip]
            let expected = vec![
                Free,       Free,       Free, Free, Free,
                Restricted, Restricted, Free, Free, Free,
                Restricted, Restricted, Free, Free, Free,
                Restricted, Restricted, Free, Free, Free,
                Free,       Free,       Free, Free, Free,
            ];

            check(2, 0, expected);
        }

        #[test]
        fn right() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Free, Free, Free,       Free,
                Free, Free, Free, Restricted, Restricted,
                Free, Free, Free, Restricted, Restricted,
                Free, Free, Free, Restricted, Restricted,
                Free, Free, Free, Free,       Free,
            ];

            check(2, 4, expected);
        }

        #[test]
        fn top_left() {
            #[rustfmt::skip]
            let expected = vec![
                Restricted, Restricted, Free, Free, Free,
                Restricted, Restricted, Free, Free, Free,
                Free,       Free,       Free, Free, Free,
                Free,       Free,       Free, Free, Free,
                Free,       Free,       Free, Free, Free,
            ];

            check(0, 0, expected);
        }

        #[test]
        fn bottom_left() {
            #[rustfmt::skip]
            let expected = vec![
                Free,       Free,       Free, Free, Free,
                Free,       Free,       Free, Free, Free,
                Free,       Free,       Free, Free, Free,
                Restricted, Restricted, Free, Free, Free,
                Restricted, Restricted, Free, Free, Free,
            ];

            check(4, 0, expected);
        }

        #[test]
        fn top_right() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Free, Free, Restricted, Restricted,
                Free, Free, Free, Restricted, Restricted,
                Free, Free, Free, Free,       Free,
                Free, Free, Free, Free,       Free,
                Free, Free, Free, Free,       Free,
            ];

            check(0, 4, expected);
        }

        #[test]
        fn bottom_right() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Free, Free, Free,       Free,
                Free, Free, Free, Free,       Free,
                Free, Free, Free, Free,       Free,
                Free, Free, Free, Restricted, Restricted,
                Free, Free, Free, Restricted, Restricted,
            ];

            check(4, 4, expected);
        }

        #[test]
        fn out_of_bound_column() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
            ];

            check(0, 5, expected);
        }

        #[test]
        fn out_of_bound_row() {
            #[rustfmt::skip]
            let expected = vec![
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
                Free, Free, Free, Free, Free,
            ];

            check(5, 0, expected);
        }
    }

    mod jump {
        use super::*;

        fn check(
            from: Index,
            to: Index,
            tested: Vec<CellState>,
            expected: Vec<CellState>,
            expected_status: bool,
        ) {
            let mut tested = Board::new(5, 5, tested);
            let tested_status = tested.jump(from, to);
            pretty_assertions::assert_eq!(tested_status, expected_status);
            let expected = Board::new(5, 5, expected);
            pretty_assertions::assert_eq!(tested, expected);
        }

        mod legal {
            use super::*;

            #[inline]
            fn check(from: Index, to: Index, tested: Vec<CellState>, expected: Vec<CellState>) {
                super::check(from, to, tested, expected, true);
            }

            #[test]
            fn jump() {
                #[rustfmt::skip]
                let tested = vec![
                    Red,  Red,  Free,       Free,       Free,
                    Red,  Red,  Restricted, Free,       Free,
                    Free, Blue, Free,       Restricted, Free,
                    Free, Free, Free,       Free,       Blue,
                    Free, Free, Free,       Free,       Blue,
                ];

                #[rustfmt::skip]
                let expected = vec![
                    Red,  Red,  Free,       Free,       Free,
                    Red,  Free, Restricted, Free,       Free,
                    Free, Blue, Free,       Restricted, Free,
                    Free, Free, Free,       Red,        Red,
                    Free, Free, Free,       Free,       Red,
                ];

                check((1, 1), (3, 3), tested, expected);
            }

            #[test]
            fn spread() {
                #[rustfmt::skip]
                let tested = vec![
                    Red,  Red,  Free,       Free,       Free,
                    Red,  Red,  Restricted, Free,       Free,
                    Free, Blue, Free,       Restricted, Free,
                    Free, Free, Free,       Free,       Blue,
                    Free, Free, Free,       Free,       Blue,
                ];

                #[rustfmt::skip]
                let expected = vec![
                    Red,  Red,  Free,       Free,       Free,
                    Red,  Red,  Restricted, Free,       Free,
                    Free, Red,  Red,        Restricted, Free,
                    Free, Free, Free,       Free,       Blue,
                    Free, Free, Free,       Free,       Blue,
                ];

                check((1, 1), (2, 2), tested, expected);
            }
        }

        mod illegal {
            use super::*;

            #[inline]
            fn check(from: Index, to: Index) {
                #[rustfmt::skip]
                let tested = vec![
                    Red,  Red,  Free,       Free,       Free,
                    Red,  Red,  Restricted, Free,       Free,
                    Free, Blue, Free,       Restricted, Free,
                    Free, Free, Free,       Free,       Blue,
                    Free, Free, Free,       Free,       Blue,
                ];

                let expected = tested.clone();

                super::check(from, to, tested, expected, false);
            }

            #[test]
            #[inline]
            fn bad_distance() {
                check((1, 1), (4, 4));
            }

            #[test]
            #[inline]
            fn no_move() {
                check((1, 1), (1, 1));
            }

            #[test]
            #[inline]
            fn not_playable() {
                check((2, 2), (2, 3));
            }

            #[test]
            #[inline]
            fn occupied() {
                check((1, 1), (0, 1));
            }

            #[test]
            #[inline]
            fn outside() {
                check((4, 4), (5, 5));
                check((5, 5), (4, 4));
                check((5, 5), (5, 5));
            }

            #[test]
            #[inline]
            fn restricted() {
                check((1, 1), (2, 1));
            }
        }
    }
}
