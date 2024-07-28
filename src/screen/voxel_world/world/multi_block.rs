use bevy::{prelude::*, utils::HashMap};

use crate::{
    game::HexSelect,
    screen::voxel_world::{voxel_util::Blocks, BasicBlock, BlockType, ComplexBlock},
};

use super::{VoxelChunk, VoxelId, CHUNK_SIZE};

#[derive(Resource)]
pub struct MultiBlocks {
    recipes: HashMap<MultiBlockType, MultiBlockRecipe>,
}

impl FromWorld for MultiBlocks {
    fn from_world(world: &mut World) -> Self {
        let mut map: bevy::utils::hashbrown::HashMap<MultiBlockType, MultiBlockRecipe> =
            HashMap::new();
        map.insert(
            MultiBlockType::Furnace,
            MultiBlockRecipe {
                size: IVec3::new(3, 3, 3),
                rules: vec![
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::Stone)),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::Stone)),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::Stone)),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::Stone)),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::Stone)),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::Stone)),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Solid,
                ],
                output_clear: vec![ClearType::All],
                output_block: MultiOutput::Specific(BlockType::Complex(ComplexBlock::Furnace)),
                output_offset: IVec3::new(1, 1, 1),
            },
        );
        map.insert(
            MultiBlockType::Smelt,
            MultiBlockRecipe {
                size: IVec3::new(1, 3, 1),
                rules: vec![
                    MultiBlockRule::Fuel,
                    MultiBlockRule::Specific(BlockType::Complex(ComplexBlock::Furnace)),
                    MultiBlockRule::CanMelt,
                ],
                output_block: MultiOutput::Melt(IVec3::new(0, 2, 0)),
                output_offset: IVec3::new(0, 2, 0),
                output_clear: vec![
                    ClearType::Offset(IVec3::ZERO),
                    ClearType::Offset(IVec3::new(0, 2, 0)),
                ],
            },
        );
        map.insert(
            MultiBlockType::Drill,
            MultiBlockRecipe {
                size: IVec3::splat(3),
                rules: vec![
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::IronBlock)),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::IronBlock)),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::IronBlock)),
                    MultiBlockRule::Solid,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::IronBlock)),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Specific(BlockType::Basic(BasicBlock::IronBlock)),
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Solid,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                    MultiBlockRule::Empty,
                ],
                output_block: MultiOutput::Specific(BlockType::Complex(ComplexBlock::Drill)),
                output_offset: IVec3 { x: 1, y: 1, z: 1 },
                output_clear: vec![ClearType::All],
            },
        );
        MultiBlocks { recipes: map }
    }
}

#[derive(Component)]
struct MultiBlock {
    size: IVec3,
    origin: IVec3,
    building_blocks: Vec<BlockType>,
}

enum MultiBlockRule {
    Solid,
    Specific(BlockType),
    Empty,
    CanMelt,
    Fuel,
}

enum MultiOutput {
    Specific(BlockType),
    Melt(IVec3),
}

impl MultiBlockRule {
    fn applies_to(&self, block: &BlockType) -> bool {
        match self {
            MultiBlockRule::Solid => block.is_solid(),
            MultiBlockRule::Specific(cmp) => block == cmp,
            MultiBlockRule::Empty => block == &BlockType::Basic(BasicBlock::Air),
            MultiBlockRule::CanMelt => block.melt().is_some(),
            MultiBlockRule::Fuel => block.fuel(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum MultiBlockType {
    Furnace,
    Smelt,
    Drill,
}

struct MultiBlockRecipe {
    size: IVec3,
    rules: Vec<MultiBlockRule>,
    output_block: MultiOutput,
    output_offset: IVec3,
    output_clear: Vec<ClearType>,
}

impl MultiBlockRecipe {
    pub fn clear(
        &self,
        pos: IVec3,
        chunk: &mut VoxelChunk,
        commands: &mut Commands,
        blocks: &Query<(Entity, &VoxelId)>,
    ) {
        for clear in self.output_clear.iter() {
            clear.apply(self, pos, chunk, commands, blocks);
        }
    }

    pub fn output(&self, pos: IVec3, chunk: &VoxelChunk) -> Vec<(IVec3, BlockType)> {
        let mut out = Vec::new();
        match &self.output_block {
            MultiOutput::Specific(block) => out.push((pos + self.output_offset, block.clone())),
            MultiOutput::Melt(offset) => {
                let block = chunk.get(pos + *offset);
                if let Some(melt) = block.melt() {
                    out.push((pos + *offset, melt));
                }
            }
        };
        out
    }
}

impl ClearType {
    fn apply(
        &self,
        recipe: &MultiBlockRecipe,
        pos: IVec3,
        chunk: &mut VoxelChunk,
        commands: &mut Commands,
        blocks: &Query<(Entity, &VoxelId)>,
    ) {
        match self {
            ClearType::All => {
                for rx in 0..recipe.size.x {
                    for ry in 0..recipe.size.y {
                        for rz in 0..recipe.size.z {
                            let pos = pos + IVec3::new(rx, ry, rz);
                            chunk.set(pos, BlockType::Basic(BasicBlock::Air));
                            for (entity, id) in blocks {
                                if id.0 == pos {
                                    commands.entity(entity).despawn_recursive();
                                }
                            }
                        }
                    }
                }
            }
            ClearType::Offset(offset) => {
                let pos = pos + *offset;
                chunk.set(pos, BlockType::Basic(BasicBlock::Air));
                for (entity, id) in blocks {
                    if id.0 == pos {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

enum ClearType {
    All,
    Offset(IVec3),
}

pub fn check_for_multi_blocks(
    mut chunk_data: ResMut<Assets<VoxelChunk>>,
    blocks: Query<(Entity, &VoxelId)>,
    recipes: Res<MultiBlocks>,
    selected: Res<HexSelect>,
    voxels: Res<Blocks>,
    mut commands: Commands,
) {
    let Some(chunk) = chunk_data.get_mut(selected.chunk.id()) else {
        error!("chunk not loaded");
        return;
    };
    for (_, recipe) in recipes.recipes.iter() {
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_SIZE as i32 {
                'failed: for z in 0..CHUNK_SIZE as i32 {
                    for rx in 0..recipe.size.x {
                        for ry in 0..recipe.size.y {
                            for rz in 0..recipe.size.z {
                                let pos = IVec3::new(x, y, z) + IVec3::new(rx, ry, rz);
                                let rule_index =
                                    rx + rz * recipe.size.x + ry * recipe.size.x * recipe.size.z;
                                let block = chunk.get(pos);
                                if !recipe.rules[rule_index as usize].applies_to(&block) {
                                    continue 'failed;
                                }
                            }
                        }
                    }
                    let out = recipe.output(IVec3::new(x, y, z), chunk);
                    recipe.clear(IVec3::new(x, y, z), chunk, &mut commands, &blocks);
                    for (pos, block) in out {
                        let mut entity = commands.spawn((
                            Name::new("Multi Block"),
                            VoxelId(pos),
                            PbrBundle {
                                mesh: voxels.mesh(&block),
                                material: voxels.texture(&block),
                                transform: Transform::from_translation(pos.as_vec3()),
                                ..Default::default()
                            },
                        ));
                        if block.is_solid() {
                            entity.insert(bevy_rapier3d::prelude::Collider::cuboid(0.5, 0.5, 0.5));
                        }
                        chunk.set(pos, block);
                    }
                    println!("Found multi block");
                }
            }
        }
    }
}
