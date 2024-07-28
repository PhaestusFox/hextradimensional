use bevy::prelude::*;

use crate::{
    game::main_character::Player,
    screen::{
        inventory::Inventory,
        voxel_world::{voxel_util::VoxelPlayer, voxels::BlockType},
    },
};

pub fn give_player_block(
    mut player: Query<&mut Inventory, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for key in input.get_just_pressed() {
        let give = match key {
            KeyCode::Numpad0 => BlockType::Stone,
            KeyCode::Numpad1 => BlockType::Coal,
            KeyCode::Numpad2 => BlockType::IronOre,
            KeyCode::Numpad3 => BlockType::Drill,
            KeyCode::Numpad4 => BlockType::Score,
            KeyCode::Numpad5 => BlockType::Piston(crate::screen::hex_vox_util::MapDirection::Up),
            _ => continue,
        };
        for mut inventory in &mut player {
            warn!("cheated {:?} in", give);
            inventory.add_resource(give, 200);
        }
    }
}
