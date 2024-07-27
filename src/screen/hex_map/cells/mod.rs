use bevy::{
    asset::{AssetServer, Handle},
    math::{IVec2, Vec3},
    prelude::{Changed, Component, FromWorld, Query, Resource},
    render::texture::Image,
    transform::components::Transform,
    utils::HashMap,
};
use std::{f32::consts::PI, fmt::Display, str::FromStr};
mod iterators;

pub use iterators::*;
use strum::IntoEnumIterator;

use crate::screen::{
    hex_vox_util::{HexId, HEX_SPACING},
    voxel_world::voxel_util::WorldType,
};

pub(crate) fn update_transforms(mut hexagons: Query<(&mut Transform, &HexId), Changed<HexId>>) {
    for (mut pos, hex) in &mut hexagons {
        pos.translation.x = hex.x() * HEX_SPACING;
        pos.translation.y = hex.y() * HEX_SPACING;
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

#[derive(
    Default,
    Component,
    PartialEq,
    Eq,
    Debug,
    strum_macros::EnumIter,
    Hash,
    Clone,
    Copy,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum HexagonType {
    #[default]
    Empty,
    Stone,
    Coal,
    Iron,
}

impl Into<WorldType> for HexagonType {
    fn into(self) -> WorldType {
        match self {
            HexagonType::Empty => WorldType::Empty,
            HexagonType::Stone => WorldType::Stone,
            HexagonType::Coal => WorldType::Coal,
            HexagonType::Iron => WorldType::Iron,
        }
    }
}
