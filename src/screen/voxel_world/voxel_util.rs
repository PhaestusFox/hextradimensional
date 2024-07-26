use crate::screen::{voxel_world::player_controller::VoxelCamera, HexSelect, MapDirection, Screen};
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::*;
use rand::{Rng, SeedableRng};
use strum::IntoEnumIterator;

use super::{inventory::Inventory, BlockType};

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Blocks>().add_plugins(VoxelCamera);
    }
}

/// This describes the main player in the voxel world
#[derive(Component)]
pub struct VoxelPlayer;

pub fn spawn_voxel_map(mut commands: Commands, blocks: Res<Blocks>, hex_select: Res<HexSelect>) {
    commands
        .spawn((
            StateScoped(Screen::VoxelWorld),
            SpatialBundle {
                transform: Transform::from_translation(pos_from_enter(&hex_select.direction)),
                ..Default::default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule_y(0.5, 0.45),
            KinematicCharacterControllerOutput::default(),
            bevy_rapier3d::control::KinematicCharacterController {
                ..Default::default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                VoxelPlayer,
                Inventory::new(10),
                Camera3dBundle {
                    camera: Camera {
                        order: 1,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::Y * 0.5),
                    ..Default::default()
                },
            ));
        });

    fill_world(
        commands,
        hex_select.hex_id,
        WorldType::from_u8(3), // ! FIX THIS AS WORLD TYPE SELECTION. CURRENTLY FORCES STONE. USE SEED AND SAVE DATA
        blocks.as_ref(),
    );
}

fn pos_from_enter(direction: &MapDirection) -> Vec3 {
    match direction {
        MapDirection::Down => Vec3::new(8., 0., 8.),
        MapDirection::North => Vec3::new(16., 8., 8.),
        MapDirection::East => Vec3::new(8., 8., 16.),
        MapDirection::Up => Vec3::new(8., 16., 8.),
        MapDirection::South => Vec3::new(0., 8., 8.),
        MapDirection::West => Vec3::new(8., 8., 0.),
    }
}

#[derive(PartialEq, Eq)]
enum WorldType {
    Empty,
    Stone,
    Coal,
    Flat,
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
            3 => WorldType::Flat,
            _ => unreachable!(),
        }
    }

    fn sample(&self, mut rng: impl Rng, pos: IVec3) -> BlockType {
        match self {
            WorldType::Flat => {
                if pos.y == 0 {
                    BlockType::Stone
                } else if pos.y == 1 && rand::thread_rng().gen_bool(0.1) {
                    BlockType::Coal
                } else if pos.y == 2 && rand::thread_rng().gen_bool(0.1) {
                    BlockType::Coal
                } else {
                    BlockType::Air
                }
            }
            WorldType::Empty => BlockType::Air,
            WorldType::Stone => {
                if rng.gen_bool(0.6) || pos.y == 0 {
                    BlockType::Stone
                } else {
                    BlockType::Air
                }
            }
            WorldType::Coal => {
                if rng.gen_bool(0.3) && pos.y != 0 {
                    BlockType::Air
                } else if rng.gen_bool(0.25) {
                    BlockType::Coal
                } else {
                    BlockType::Stone
                }
            }
        }
    }
}

fn fill_world(mut commands: Commands, id: Vec2, world_type: WorldType, blocks: &Blocks) {
    if world_type == WorldType::Empty {
        return;
    }
    let mut rng = rand::rngs::StdRng::seed_from_u64((id.x as u64) << 32 | id.y as u64);
    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let block = world_type.sample(&mut rng, IVec3::new(x, y, z));
                let solidity = block.is_solid();
                let mut entity = commands.spawn((
                    StateScoped(Screen::VoxelWorld),
                    PbrBundle {
                        mesh: blocks.mesh(),
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
        }
    }

    const fn is_solid(&self) -> bool {
        match self {
            BlockType::Air => false,
            BlockType::Stone => true,
            BlockType::Coal => true,
            BlockType::Voxel(_) => false,
            BlockType::MultiVoxel(_) => false,
        }
    }
}

#[derive(Resource)]
pub struct Blocks {
    mesh: Handle<Mesh>,
    textures: HashMap<BlockType, Handle<StandardMaterial>>,
}

impl Blocks {
    pub fn texture(&self, block: BlockType) -> Handle<StandardMaterial> {
        self.textures.get(&block).cloned().unwrap_or_default()
    }
    pub fn mesh(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }
}

impl FromWorld for Blocks {
    fn from_world(world: &mut World) -> Self {
        let mut blocks = Blocks {
            mesh: world
                .resource_mut::<Assets<Mesh>>()
                .add(Cuboid::new(1., 1., 1.)),
            textures: HashMap::default(),
        };
        let asset_server = world.resource::<AssetServer>().clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        for block in BlockType::iter() {
            let texture_path = block.texture_path();
            blocks.textures.insert(
                block,
                materials.add(StandardMaterial {
                    base_color_texture: Some(asset_server.load(texture_path)),
                    ..Default::default()
                }),
            );
        }

        blocks
    }
}
