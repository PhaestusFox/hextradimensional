use crate::screen::hex_map::cells::HexId as CellId;

use super::{super::MapDirection, Ranged};

pub struct RingIter {
    cell: CellId,
    len: u32,
    direction: MapDirection,
    i: u32,
    done: bool,
}

impl RingIter {
    pub fn new(ring: u32) -> RingIter {
        RingIter {
            cell: CellId::new(-(ring as i32), ring as i32),
            len: ring,
            direction: MapDirection::Down,
            i: 0,
            done: false,
        }
    }
    pub fn ring(&self) -> u32 {
        self.len
    }
}

impl Iterator for RingIter {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let out = Some(self.cell);
        self.cell += self.direction.direction();
        self.i += 1;
        if self.len == 0 {
            self.done = true;
        }
        if self.i == self.len {
            if self.direction == MapDirection::South {
                self.done = true
            };
            self.direction = self.direction.next();
            self.i = 0;
        }
        out
    }
}

pub struct SpiralIter {
    target: u32,
    ring: RingIter,
}

impl SpiralIter {
    pub fn new(size: u32) -> SpiralIter {
        SpiralIter {
            target: size,
            ring: RingIter::new(0),
        }
    }
}

impl Iterator for SpiralIter {
    type Item = CellId;
    fn next(&mut self) -> Option<Self::Item> {
        self.ring.next().or_else(|| {
            if self.ring.ring() == self.target {
                None
            } else {
                self.ring = RingIter::new(self.ring.ring() + 1);
                self.ring.next()
            }
        })
    }
}

impl Ranged for RingIter {
    fn in_range(range: u32) -> Self {
        Self::new(range)
    }
}

impl Ranged for SpiralIter {
    fn in_range(range: u32) -> Self {
        Self::new(range)
    }
}
