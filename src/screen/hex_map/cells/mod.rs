use bevy::{
    asset::{AssetServer, Handle},
    prelude::{Changed, FromWorld, Query, Resource},
    render::texture::Image,
    transform::components::Transform,
    utils::HashMap,
};
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
pub struct CellIcons(HashMap<WorldType, Handle<Image>>);

impl CellIcons {
    pub fn get(&self, hex: WorldType) -> Handle<Image> {
        self.0.get(&hex).cloned().unwrap_or_default()
    }
}

impl FromWorld for CellIcons {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut icons = CellIcons(HashMap::default());
        let asset_server = world.resource::<AssetServer>();
        for hex in WorldType::iter() {
            icons.0.insert(
                hex,
                asset_server.load(format!("images/hexes/{:?}.png", hex).to_lowercase()),
            );
        }
        icons
    }
}
