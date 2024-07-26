use super::voxel_util::VoxelPlayer;
use crate::screen::voxel_world::inventory::Inventory;
use crate::ui::widgets::{Containers, UiRoot, Widgets};
use bevy::prelude::*; // Adjust this path as needed

pub fn setup_inventory_ui(mut commands: Commands, player_query: Query<(&Inventory, &VoxelPlayer)>) {
    if let Ok(player_inventory) = player_query.get_single() {
        commands
            .ui_root() // Assuming you have this method from the Containers trait
            .with_children(|parent| {
                parent.inventory(player_inventory.0);
            });
    }
}

pub fn cleanup_inventory_ui(mut commands: Commands, ui_query: Query<Entity, With<UiRoot>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/*pub fn update_inventory_ui(
    mut commands: Commands,
    player_query: Query<&Inventory, Changed<Inventory>>,
    ui_root_query: Query<Entity, With<UiRoot>>, // Assuming you have a UiRoot component
) {
    if let Ok(player_inventory) = player_query.get_single() {
        if let Ok(ui_root) = ui_root_query.get_single() {
            // Remove the old inventory UI
            commands.entity(ui_root).despawn_descendants();

            // Spawn the new inventory UI
            commands.entity(ui_root).with_children(|parent| {
                parent.inventory(player_inventory);
            });
        }
    }
}
*/
