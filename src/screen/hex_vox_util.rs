use std::{
    default,
    f32::consts::PI,
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

use bevy::{
    math::{IVec2, Quat, Vec3},
    prelude::Component,
    reflect::Reflect,
};
use serde::{Deserialize, Serialize};

pub const SQR_3: f32 = 1.732050807568877;
pub const SQR_3_DIV_TWO: f32 = 0.8660254037844386;
pub const SQR_3_DIV_THREE: f32 = 0.5773502691896258;
pub const HEX_SIZE: f32 = 100.;
pub const HEX_SPACING: f32 = HEX_SIZE / 2.;

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy, Default)]
pub struct HexId(IVec2);

impl Display for HexId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}:{})", self.q(), self.r()))
    }
}

impl FromStr for HexId {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('(') {
            return Err("need to start with '('");
        }
        if !s.ends_with(')') {
            return Err("need to end with ')'");
        }
        let mut segs = s[1..s.len() - 1].split(':');
        let q = segs.next().ok_or("No q between ()")?;
        let r = segs.next().ok_or("No r between ()")?;
        let q = q.parse().or(Err("Failed to parse q"))?;
        let r = r.parse().or(Err("Failed to parse r"))?;
        Ok(HexId::new(q, r))
    }

    type Err = &'static str;
}

impl HexId {
    pub const fn new(q: i32, r: i32) -> HexId {
        Self(IVec2 { x: q, y: r })
    }

    #[inline]
    pub fn q(&self) -> i32 {
        self.0.x
    }

    #[inline]
    pub fn r(&self) -> i32 {
        self.0.y
    }

    #[inline]
    pub fn s(&self) -> i32 {
        -self.q() - self.r()
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.q() as f32 * 1.5
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.q() as f32 * SQR_3_DIV_TWO + SQR_3 * self.r() as f32
    }

    #[inline]
    pub fn xyz(&self) -> Vec3 {
        Vec3 {
            x: self.x(),
            y: self.y(),
            z: 0.,
        } * HEX_SIZE
            / 2.0
    }

    pub fn ivec2(&self) -> IVec2 {
        self.0
    }

    pub fn round(q: f32, r: f32) -> HexId {
        let s = -q - r;
        let round_q = q.round();
        let round_r = r.round();
        let round_s = s.round();

        let s_dif = (s - round_s).abs();
        let r_dif = (r - round_r).abs();
        let q_dif = (q - round_q).abs();

        if s_dif > r_dif {
            if s_dif > q_dif {
                HexId::new(round_q as i32, round_r as i32)
            } else {
                HexId::new((-round_s - round_r) as i32, round_r as i32)
            }
        } else {
            if r_dif > q_dif {
                let r = (-round_s - round_q) as i32;
                HexId::new(round_q as i32, r)
            } else {
                HexId::new((-round_s - round_r) as i32, round_r as i32)
            }
        }
    }

    pub fn from_xyz(pos: Vec3) -> HexId {
        let x = pos.x / HEX_SPACING;
        let y = pos.y / HEX_SPACING;
        let q = x * 2. / 3.;
        let r = y * SQR_3_DIV_THREE - 1. / 3. * x;
        HexId::round(q, r)
    }
}

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

/// This represents the edges of the hexagon mapping to the voxel world.
/// The Direction with reference to the hexagon is in clockwise order for the enum, starting from the top edge.
#[derive(
    Clone,
    Copy,
    PartialEq,
    strum_macros::EnumIter,
    Debug,
    Component,
    Eq,
    Hash,
    Reflect,
    Serialize,
    Deserialize,
    Default,
)]
pub enum MapDirection {
    #[default]
    Up,
    North,
    East,
    Down,
    South,
    West,
}

impl MapDirection {
    pub fn to_rotation(&self) -> Quat {
        match self {
            MapDirection::Up => Quat::IDENTITY,
            MapDirection::South => Quat::from_rotation_x(PI / 2.),
            MapDirection::West => Quat::from_rotation_z(-PI / 2.),
            MapDirection::Down => Quat::from_rotation_x(PI),
            MapDirection::North => Quat::from_rotation_x(-PI / 2.),
            MapDirection::East => Quat::from_rotation_z(PI / 2.),
        }
    }
}

const TWO_THIRDS_PI: f32 = PI * 2.0 / 3.0;
const ONE_THIRD_PI: f32 = PI / 3.0;

impl MapDirection {
    pub const fn direction(&self) -> HexId {
        match self {
            MapDirection::Down => HexId::new(0, -1),
            MapDirection::East => HexId::new(1, -1),
            MapDirection::North => HexId::new(1, 0),
            MapDirection::Up => HexId::new(0, 1),
            MapDirection::West => HexId::new(-1, 1),
            MapDirection::South => HexId::new(-1, 0),
        }
    }

    pub const fn next(&self) -> MapDirection {
        match self {
            MapDirection::Down => MapDirection::East,
            MapDirection::East => MapDirection::North,
            MapDirection::North => MapDirection::Up,
            MapDirection::Up => MapDirection::West,
            MapDirection::West => MapDirection::South,
            MapDirection::South => MapDirection::Down,
        }
    }

    pub fn angle(&self) -> f32 {
        match self {
            MapDirection::Down => -PI,
            MapDirection::East => -TWO_THIRDS_PI,
            MapDirection::North => -ONE_THIRD_PI,
            MapDirection::Up => 0.0,
            MapDirection::West => ONE_THIRD_PI,
            MapDirection::South => TWO_THIRDS_PI,
        }
    }
}
