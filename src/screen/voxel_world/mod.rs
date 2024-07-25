//! The screen state for the voxel world game loop.
mod player_controller;
mod voxel_util;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};

use voxel_util::{spawn_voxel_map, Blocks};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::VoxelWorld), enter_playing);
    app.add_systems(OnEnter(Screen::VoxelWorld), spawn_voxel_map);
    app.add_systems(OnExit(Screen::VoxelWorld), exit_playing);

    app.add_systems(
        Update,
        return_to_hex_map
            .run_if(in_state(Screen::VoxelWorld).and_then(input_just_pressed(KeyCode::Escape))),
    );
    app.init_resource::<Blocks>();
    app.add_plugins(player_controller::VoxelCamera);
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_hex_map(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::HexMap);
}
