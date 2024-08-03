use crate::{
    game::{main_character::Player, PlayerAction},
    screen::inventory::Inventory,
    ui::widgets::{Containers, UiRoot, Widgets},
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use leafwing_input_manager::prelude::ActionState;

use super::voxels::{Block, Blocks}; // Adjust this path as needed

#[derive(Component)]
pub struct FullInventoryUI;

pub fn setup_inventory_ui(
    mut commands: Commands,
    player_query: Query<(&Inventory, &Player)>,
    voxels: Res<Blocks>,
    voxel_data: Res<Assets<Block>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    if let Ok(player_inventory) = player_query.get_single() {
        commands
            .ui_root() // Assuming you have this method from the Containers trait
            .with_children(|parent| {
                parent.hotbar(player_inventory.0, &voxels, &voxel_data, &materials);
                parent.full_inventory(player_inventory.0, &voxels, &voxel_data, &materials);
            });
    }
}

pub fn cleanup_inventory_ui(mut commands: Commands, ui_query: Query<Entity, With<UiRoot>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn toggle_full_inventory(
    mut inventory_ui_query: Query<(Entity, &mut Visibility), With<FullInventoryUI>>,
) {
    if let Ok((_, mut visibility)) = inventory_ui_query.get_single_mut() {
        // Toggle visibility of existing inventory UI
        *visibility = match *visibility {
            Visibility::Visible => Visibility::Hidden,
            _ => Visibility::Visible,
        };
    }
}

pub fn update_inventory_ui(
    mut commands: Commands,
    player_query: Query<&Inventory, (With<Player>, Changed<Inventory>)>,
    ui_root_query: Query<Entity, With<UiRoot>>,
    voxels: Res<Blocks>,
    voxel_data: Res<Assets<Block>>,
    materials: Res<Assets<StandardMaterial>>,
) {
    if let Ok(inventory) = player_query.get_single() {
        if let Ok(ui_root) = ui_root_query.get_single() {
            // Remove the old inventory UI
            commands.entity(ui_root).despawn_descendants();

            // Spawn the new inventory UI
            commands.entity(ui_root).with_children(|parent| {
                parent.hotbar(inventory, &voxels, &voxel_data, &materials);
                parent.full_inventory(inventory, &voxels, &voxel_data, &materials);
            });
        }
    }
}

pub fn handle_slot_selection(
    mut inventory_query: Query<&mut Inventory, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    input: Query<&ActionState<PlayerAction>>,
) {
    if let Ok(mut inventory) = inventory_query.get_single_mut() {
        for (i, key) in [
            KeyCode::Digit1,
            KeyCode::Digit2,
            KeyCode::Digit3,
            KeyCode::Digit4,
            KeyCode::Digit5,
            KeyCode::Digit6,
            KeyCode::Digit7,
            KeyCode::Digit8,
            KeyCode::Digit9,
            KeyCode::Digit0,
        ]
        .iter()
        .enumerate()
        {
            if keyboard_input.just_pressed(*key) {
                inventory.select_slot(i);
                break;
            }
        }
        let mut delta = 0;
        if let Ok(input) = input.get_single() {
            delta += input.just_pressed(&PlayerAction::ItemInc) as isize;
            delta -= input.just_pressed(&PlayerAction::ItemDec) as isize;
        }
        let new = (inventory.selected_slot as isize + delta) % 10;
        inventory.select_slot(new as usize);
    }
}
