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
    ui_root_query: Query<Entity, With<UiRoot>>,
    mut inventory_ui_query: Query<(Entity, &mut Visibility), With<FullInventoryUI>>,
) {
    if let Ok(ui_root) = ui_root_query.get_single() {
        match inventory_ui_query.get_single_mut() {
            Ok((_, mut visibility)) => {
                // Toggle visibility of existing inventory UI
                *visibility = match *visibility {
                    Visibility::Visible => Visibility::Hidden,
                    _ => Visibility::Visible,
                };
            }
            Err(_) => {}
        }
    }
}
