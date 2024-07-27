use bevy::utils::hashbrown::HashSet;
pub use fill::FastFill;
pub use offset::{Offset, WithOffset};
pub use ring::{RingIter, SpiralIter};

use super::HexId;

mod fill;
mod offset;
mod ring;

pub trait Ranged: Iterator<Item = HexId> + WithOffset {
    fn in_range(range: u32) -> Self;
}

#[test]
fn range_test() {
    let range_0 = FastFill::new(0).count();
    assert_eq!(range_0, 1);
    let range_1 = FastFill::new(1).count();
    assert_eq!(range_1, 7);
    let range_2 = FastFill::new(2).count();
    assert_eq!(range_2, 19);
    let range_3 = FastFill::new(3).count();
    assert_eq!(range_3, 37);

    let ring_1 = RingIter::new(1).count();
    assert_eq!(ring_1, 6);
    let ring_2 = RingIter::new(2).count();
    assert_eq!(ring_2, 12);
    let ring_3 = RingIter::new(3).count();
    assert_eq!(ring_3, 18);

    let all_in_3 = SpiralIter::new(3).collect::<HashSet<_>>();
    assert_eq!(all_in_3, FastFill::new(3).collect())
}

// for dq in range(-N, N + 1):
//    for dr in range(max(-N, -dq - N), min(N, -dq + N) + 1):
//        q = center_q + dq
//        r = center_r + dr
//        hexagons_in_range.append((q, r))
