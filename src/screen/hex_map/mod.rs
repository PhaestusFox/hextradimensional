//! The screen state for the main hex map game loop.
mod bundle;
pub(crate) mod cells;
mod cursor;
mod hex_util;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use cells::CellIcons;
use hex_util::{go_to_voxel, spawn_test_grid};

use super::Screen;
use crate::game::{
    assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack, spawn::player::SpawnPlayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::HexMap), enter_playing);
    app.add_systems(OnExit(Screen::HexMap), exit_playing);
    app.add_systems(PreUpdate, cells::update_transforms);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::HexMap).and_then(input_just_pressed(KeyCode::Escape))),
    );

    app.add_plugins(cursor::CursorPlugin)
        .init_resource::<CellIcons>();

    #[cfg(debug_assertions)]
    // todo Remove from game
    app.add_systems(OnEnter(Screen::HexMap), spawn_test_grid)
        .add_systems(Update, go_to_voxel.run_if(in_state(Screen::HexMap)));
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(SpawnPlayer);
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
