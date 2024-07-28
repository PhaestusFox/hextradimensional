use bevy::prelude::*;

use crate::screen::voxel_world::{
    inventory::Inventory, voxel_util::VoxelPlayer, BasicBlock, BlockType, ComplexBlock,
};

pub fn give_player_block(
    mut player: Query<&mut Inventory, With<VoxelPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for key in input.get_just_pressed() {
        let give = match key {
            KeyCode::Numpad0 => BlockType::Basic(BasicBlock::Stone),
            KeyCode::Numpad1 => BlockType::Basic(BasicBlock::Coal),
            KeyCode::Numpad2 => BlockType::Basic(BasicBlock::IronOre),
            KeyCode::Numpad3 => BlockType::Complex(ComplexBlock::Drill),
            KeyCode::Numpad4 => BlockType::Basic(BasicBlock::Score),
            _ => continue,
        };
        for mut inventory in &mut player {
            warn!("cheated {:?} in", give);
            inventory.add_resource(give, 200);
        }
    }
}
