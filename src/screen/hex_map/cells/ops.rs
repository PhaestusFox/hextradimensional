use super::{MapDirection, HexId};
use core::ops::{Add, AddAssign};

impl Add<MapDirection> for HexId {
    type Output = HexId;
    fn add(self, rhs: MapDirection) -> Self::Output {
        self + rhs.direction()
    }
}

impl Add for HexId {
    type Output = HexId;
    fn add(self, rhs: Self) -> Self::Output {
        HexId::new(self.q() + rhs.q(), self.r() + rhs.r())
    }
}

impl AddAssign for HexId {
    fn add_assign(&mut self, rhs: Self) {
        self.0.y += rhs.0.y;
        self.0.x += rhs.0.x;
    }
}

impl AddAssign<MapDirection> for HexId {
    fn add_assign(&mut self, rhs: MapDirection) {
        *self += rhs.direction();
    }
}
