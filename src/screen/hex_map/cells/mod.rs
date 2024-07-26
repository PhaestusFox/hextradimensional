use bevy::{
    asset::{AssetServer, Handle},
    math::{IVec2, Vec3},
    prelude::{Changed, Component, FromWorld, Query, Resource},
    render::texture::Image,
    transform::components::Transform,
    utils::HashMap,
};
use std::f32::consts::PI;
mod iterators;
mod ops;

pub use iterators::*;
use strum::IntoEnumIterator;

use crate::screen::MapDirection;

use super::hex_util::{HEX_SIZE, HEX_SPACING, SQR_3, SQR_3_DIV_THREE, SQR_3_DIV_TWO};

#[derive(Component, PartialEq, Eq, Hash, Debug, Clone, Copy, Default)]
pub struct HexId(IVec2);

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

pub(crate) fn update_transforms(mut hexagons: Query<(&mut Transform, &HexId), Changed<HexId>>) {
    for (mut pos, hex) in &mut hexagons {
        pos.translation.x = hex.x() * HEX_SPACING;
        pos.translation.y = hex.y() * HEX_SPACING;
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

#[derive(Resource)]
pub struct CellIcons(HashMap<HexagonType, Handle<Image>>);

impl CellIcons {
    pub fn get(&self, hex: HexagonType) -> Handle<Image> {
        self.0.get(&hex).cloned().unwrap_or_default()
    }
}

impl FromWorld for CellIcons {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut icons = CellIcons(HashMap::default());
        let asset_server = world.resource::<AssetServer>();
        for hex in HexagonType::iter() {
            icons.0.insert(
                hex,
                asset_server.load(format!("images/hexes/{:?}.png", hex).to_lowercase()),
            );
        }
        icons
    }
}

#[derive(Component, PartialEq, Eq, Debug, strum_macros::EnumIter, Hash, Clone, Copy)]
pub enum HexagonType {
    Empty,
    Stone,
    Coal,
}
