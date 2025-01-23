use std::ops;

// Square represents a location on a rectangular 2d board
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Square<const WIDTH: u8, const HEIGHT: u8>(u16);

impl<const WIDTH: u8, const HEIGHT: u8> Square<WIDTH, HEIGHT> {
    pub const fn from_rank_file(rank: u8, file: u8) -> Self {
        assert!(rank < HEIGHT && file < WIDTH);
        Self(rank as u16 * WIDTH as u16 + file as u16)
    }

    pub const fn from_raw(sq: u16) -> Self {
        assert!(sq < WIDTH as u16 * HEIGHT as u16);
        Self(sq)
    }

    pub const fn value(self) -> u16 {
        self.0
    }

    pub const fn rank(self) -> u8 {
        (self.0 / WIDTH as u16) as u8
    }

    pub const fn file(self) -> u8 {
        (self.0 % WIDTH as u16) as u8
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::Add<i32> for Square<WIDTH, HEIGHT> {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self::from_raw((self.0 as i32 + rhs) as u16)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::AddAssign<i32> for Square<WIDTH, HEIGHT> {
    fn add_assign(&mut self, rhs: i32) {
        *self = *self + rhs;
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::Sub<i32> for Square<WIDTH, HEIGHT> {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self::from_raw((self.0 as i32 - rhs) as u16)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::SubAssign<i32> for Square<WIDTH, HEIGHT> {
    fn sub_assign(&mut self, rhs: i32) {
        *self = *self - rhs;
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> ops::Sub<Self> for Square<WIDTH, HEIGHT> {
    type Output = i32;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 as i32 - rhs.0 as i32
    }
}
