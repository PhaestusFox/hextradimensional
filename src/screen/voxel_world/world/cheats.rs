use bevy::prelude::*;

use crate::screen::voxel_world::{inventory::Inventory, voxel_util::VoxelPlayer, BlockType};

pub fn give_player_block(
    mut player: Query<&mut Inventory, With<VoxelPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for key in input.get_just_pressed() {
        let give = match key {
            KeyCode::Numpad0 => BlockType::Stone,
            KeyCode::Numpad1 => BlockType::Coal,
            KeyCode::Numpad2 => BlockType::IronOre,
            _ => continue,
        };
        for mut inventory in &mut player {
            warn!("cheated {:?} in", give);
            inventory.add_resource(give.clone(), 200);
        }
    }
}
