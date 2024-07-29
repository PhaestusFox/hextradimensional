//! The screen state for the main hex map game loop.
mod animation;
mod bundle;
pub(crate) mod cells;
mod cursor;
mod hex_util;
pub mod movement;
pub mod spawn;

use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};

use super::Screen;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use cells::CellIcons;
use hex_util::{go_to_voxel, spawn_hex_grid};
use spawn::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, movement::plugin, spawn::plugin));
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

    // #[cfg(debug_assertions)]
    app.add_systems(OnEnter(Screen::HexMap), spawn_hex_grid)
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
