use super::voxel_util::VoxelPlayer;
use crate::screen::voxel_world::inventory::Inventory;
use crate::ui::widgets::{Containers, UiRoot, Widgets};
use bevy::prelude::*; // Adjust this path as needed

#[derive(Component)]
pub struct FullInventoryUI;

pub fn setup_inventory_ui(
    mut commands: Commands,
    player_query: Query<(&Inventory, &VoxelPlayer)>,
    server: Res<AssetServer>,
) {
    if let Ok(player_inventory) = player_query.get_single() {
        commands
            .ui_root() // Assuming you have this method from the Containers trait
            .with_children(|parent| {
                parent.hotbar(player_inventory.0, &server);
                parent.full_inventory(player_inventory.0, &server);
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
    player_query: Query<&Inventory, (With<VoxelPlayer>, Changed<Inventory>)>,
    ui_root_query: Query<Entity, With<UiRoot>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(inventory) = player_query.get_single() {
        if let Ok(ui_root) = ui_root_query.get_single() {
            // Remove the old inventory UI
            commands.entity(ui_root).despawn_descendants();

            // Spawn the new inventory UI
            commands.entity(ui_root).with_children(|parent| {
                parent.hotbar(inventory, &asset_server);
                parent.full_inventory(inventory, &asset_server);
            });
        }
    }
}

pub fn handle_slot_selection(
    mut inventory_query: Query<&mut Inventory, With<VoxelPlayer>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
    }
}
