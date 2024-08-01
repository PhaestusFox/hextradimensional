//! The screen state for the voxel world game loop.

mod item;
mod player_controller;
pub mod ui;
mod voxel_block_generation;
pub mod voxel_util;
pub mod world;

use super::{
    inventory::{change_row_inventory, Inventory},
    Screen,
};
use crate::game::{
    assets::SoundtrackKey,
    audio::soundtrack::PlaySoundtrack,
    save::{inventory_save, save_chunk_data},
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use player_controller::spawn_player;
use ui::{
    cleanup_inventory_ui, handle_slot_selection, setup_inventory_ui, toggle_full_inventory,
    update_inventory_ui,
};

pub mod voxels;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(voxels::VoxelPlugin);

    app.add_systems(
        OnEnter(Screen::VoxelWorld),
        (
            enter_playing,
            spawn_player,
            setup_inventory_ui.after(spawn_player),
        ),
    );
    app.add_systems(
        Update,
        (update_inventory_ui, handle_slot_selection).run_if(in_state(Screen::VoxelWorld)),
    );
    app.add_systems(
        OnExit(Screen::VoxelWorld),
        (
            exit_playing,
            cleanup_inventory_ui,
            inventory_save,
            save_chunk_data,
        ),
    );

    app.add_systems(
        Update,
        return_to_hex_map
            .run_if(in_state(Screen::VoxelWorld).and_then(input_just_pressed(KeyCode::Escape))),
    );
    app.add_systems(
        Update,
        toggle_full_inventory
            .run_if(in_state(Screen::VoxelWorld).and_then(input_just_pressed(KeyCode::KeyT))),
    );
    app.add_plugins(player_controller::VoxelCamera);
    app.register_type::<Inventory>();
    app.add_plugins(item::ItemPlugin);
    app.add_systems(Update, change_row_inventory);

    app.add_systems(
        Update,
        (
            voxel_block_generation::compress,
            voxel_block_generation::generate_dynamic_voxels,
        )
            .run_if(in_state(Screen::HexMap)),
    );

    app.add_systems(Update, voxel_block_generation::test_if_voxel_genrate);

    app.init_resource::<voxel_block_generation::VoxelDataMap>();

    world::voxel_world(app);
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
