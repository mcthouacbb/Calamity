use crate::util::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Connect4Square(Square<7, 7>);

impl Connect4Square {
    pub const fn from_row_column(row: u8, column: u8) -> Self {
        // swapped intentionally
        Self(Square::from_rank_file(column, row))
    }

    pub const fn from_util_sq(sq: Square<7, 7>) -> Self {
        Self(sq)
    }

    pub const fn from_raw(sq: u16) -> Self {
        Self(Square::from_raw(sq))
    }

    pub const fn to_util_sq(self) -> Square<7, 7> {
        self.0
    }

    pub const fn value(self) -> u16 {
        self.0.value()
    }

    pub const fn row(self) -> u8 {
        self.0.file()
    }

    pub const fn column(self) -> u8 {
        self.0.rank()
    }
}
