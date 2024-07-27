use crate::screen::{hex_map::cells::HexId, voxel_world::player_controller::VoxelCamera, Screen};
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::*;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::BlockType;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Blocks>().add_plugins(VoxelCamera);
    }
}

/// This describes the main player in the voxel world
#[derive(Component)]
pub struct VoxelPlayer;

#[derive(PartialEq, Eq, Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub enum WorldType {
    #[default]
    Empty,
    Stone,
    Coal,
    Iron,
}

#[derive(Resource)]
pub struct Solid([bool; 16 * 16 * 16]);

impl Default for Solid {
    fn default() -> Self {
        Self([false; 16 * 16 * 16])
    }
}

impl Solid {
    fn set(&mut self, x: i32, y: i32, z: i32, val: bool) {
        self.0[(x + z * 16 + y * 16 * 16) as usize] = val;
    }
    fn clear(&mut self) {
        self.0 = [false; 16 * 16 * 16];
    }
    pub fn get(&self, x: i32, y: i32, z: i32) -> bool {
        self.0
            .get((x + z * 16 + y * 16 * 16) as usize)
            .cloned()
            .unwrap_or(false)
    }
}

impl WorldType {
    fn from_u8(id: u8) -> WorldType {
        match id {
            0 => WorldType::Empty,
            1 => WorldType::Stone,
            2 => WorldType::Coal,
            3 => WorldType::Iron,
            _ => unreachable!(),
        }
    }

    pub fn sample(&self, rng: &mut impl Rng, pos: IVec3) -> BlockType {
        match self {
            WorldType::Empty => BlockType::Air,
            WorldType::Stone => {
                if rng.gen_bool(0.6) || pos.y == 0 {
                    BlockType::Stone
                } else {
                    BlockType::Air
                }
            },
            WorldType::Coal => {
                if rng.gen_bool(0.3) && pos.y != 0 {
                    BlockType::Air
                } else if rng.gen_bool(0.25) {
                    BlockType::Coal
                } else {
                    BlockType::Stone
                }
            },
            WorldType::Iron => {
                if rng.gen_bool(0.3) && pos.y != 0 {
                    BlockType::Air
                } else if rng.gen_bool(0.25) {
                    BlockType::IronOre
                } else {
                    BlockType::Stone
                }
            }
        }
    }
}

fn fill_world(mut commands: Commands, id: HexId, world_type: WorldType, blocks: &Blocks) {
    if world_type == WorldType::Empty {
        return;
    }
    let mut rng = rand::rngs::StdRng::seed_from_u64((id.q() as u64) << 32 | id.r() as u64);
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let block = &world_type.sample(&mut rng, IVec3::new(x, y, z));
                let solidity = block.is_solid();
                let mut entity = commands.spawn((
                    StateScoped(Screen::VoxelWorld),
                    PbrBundle {
                        mesh: blocks.mesh(block),
                        material: blocks.texture(block),
                        transform: Transform::from_translation(Vec3::new(
                            x as f32, y as f32, z as f32,
                        )),
                        ..Default::default()
                    },
                ));
                if solidity {
                    entity.insert(Collider::cuboid(0.5, 0.5, 0.5));
                }
            }
        }
    }
}

impl BlockType {
    const fn texture_path(&self) -> &'static str {
        match self {
            BlockType::Air => "", // ! To fix
            BlockType::Stone => "images/voxels/stone.png",
            BlockType::Coal => "images/voxels/coal.png",
            BlockType::Voxel(_) => "",
            BlockType::MultiVoxel(_) => "",
            BlockType::IronOre => "images/voxels/ore_iron.png",
            BlockType::IronBlock => "images/voxels/refined_iron.png",
            BlockType::Furnace => "images/multi_blocks/furnace.png",
        }
    }

    const fn mesh_path(&self) -> Option<&'static str> {
        match self {
            BlockType::Air => None,
            BlockType::Stone => None,
            BlockType::Coal => None,
            BlockType::Voxel(_) => None,
            BlockType::MultiVoxel(_) => None,
            BlockType::IronOre => None,
            BlockType::IronBlock => None,
            BlockType::Furnace => Some("images/multi_blocks/furnace.glb#Mesh0/Primitive0")
        }
    }

    pub const fn is_solid(&self) -> bool {
        match self {
            BlockType::Air => false,
            BlockType::Stone => true,
            BlockType::Coal => true,
            BlockType::Voxel(_) => false,
            BlockType::MultiVoxel(_) => false,
            BlockType::IronBlock => true,
            BlockType::IronOre => true,
            BlockType::Furnace => true,
        }
    }
}

#[derive(Resource)]
pub struct Blocks {
    meshs: HashMap<BlockType, Handle<Mesh>>,
    textures: HashMap<BlockType, Handle<StandardMaterial>>,
}

impl Blocks {
    pub fn texture(&self, block: &BlockType) -> Handle<StandardMaterial> {
        self.textures.get(block).cloned().unwrap_or_default()
    }
    pub fn mesh(&self, block: &BlockType) -> Handle<Mesh> {
        self.meshs.get(block).cloned().unwrap_or_default()
    }
}

impl FromWorld for Blocks {
    fn from_world(world: &mut World) -> Self {
        let mut blocks = Blocks {
            meshs: HashMap::default(),
            textures: HashMap::default(),
        };
        let asset_server = world.resource::<AssetServer>().clone();
        let mut materials = world.resource_scope::<Assets<StandardMaterial>, ()>(|world, mut materials| {
            let mut meshes = world.resource_mut::<Assets<Mesh>>();
            let default = meshes.add(Cuboid::from_length(1.));
            for block in BlockType::iter() {
                let texture_path = block.texture_path();
                let mesh_path = block.mesh_path();
                blocks.textures.insert(
                    block.clone(),
                    materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load(texture_path)),
                        ..Default::default()
                    }),
                );
                if let Some(mesh) = mesh_path {
                    blocks.meshs.insert(block, asset_server.load(mesh));
                } else {
                    blocks.meshs.insert(block, default.clone());
                }
            }
        });
            
        blocks
    }
}
