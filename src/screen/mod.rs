//! The game's main screen states and transitions between them.

mod credits;
mod hex_map;
pub mod hex_vox_util;
mod loading;
mod splash;
mod title;
pub mod voxel_world;

use crate::game;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        hex_map::plugin,
        voxel_world::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    HexMap,
    VoxelWorld,
    //Multiplayer,
}
