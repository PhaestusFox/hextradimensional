use bevy::asset::{Asset, AssetId, AssetIndex, AssetServer, Assets, Handle};
use bevy::color::{Color, Srgba};
use bevy::input::ButtonInput;
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy::prelude::{AlphaMode, KeyCode, Query, Res, ResMut, Resource, With};
use bevy::reflect::{Reflect, TypePath};
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::utils::{HashMap, HashSet};
use bevy_rapier3d::prelude::Collider;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{
    greedy_quads, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::game::main_character::Player;
use crate::game::HexSelect;
use crate::screen::hex_vox_util::HexId;
use crate::screen::inventory::Inventory;
use crate::screen::voxel_world::world::VoxelChunk;

use super::voxel_util::WorldType;
use super::voxels::{Block, BlockType, Blocks, VoxelBlock};
use super::world::VoxelStore;

#[derive(TypePath, Asset)]
pub struct VoxelMesh {
    pub mesh: Mesh,
    pub material: StandardMaterial,
}

type VoxelShape = ConstShape3u32<16, 16, 16>;

impl Voxel for BlockType {
    fn get_visibility(&self) -> VoxelVisibility {
        match self {
            BlockType::Air => VoxelVisibility::Empty,
            _ => VoxelVisibility::Opaque,
        }
    }
}

impl MergeVoxel for BlockType {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        self.clone()
    }
}

pub fn generate_voxel_mesh(voxel_block: &VoxelChunk) -> VoxelMesh {
    let voxels: Vec<BlockType> = voxel_block.0.iter().cloned().collect();

    let mut buffer = GreedyQuadsBuffer::new(voxels.len());
    greedy_quads(
        &voxels,
        &VoxelShape {},
        [0; 3],
        [15; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );

    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(num_indices);
    let mut colors: Vec<[f32; 4]> = Vec::with_capacity(num_vertices);

    for (group, face) in buffer
        .quads
        .groups
        .iter()
        .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.iter())
    {
        for quad in group.iter() {
            let normal = face.signed_normal();
            let [x, y, z] = quad.minimum;
            let corners = face.quad_corners(quad);
            let vertices =
                corners.map(|corner| [corner.x as f32, corner.y as f32, corner.z as f32]);

            let block_type = &voxels[VoxelShape::linearize([x, y, z]) as usize];
            let color = block_type.color();
            let srgba: Srgba = color.into();

            for vertex in vertices.iter() {
                positions.push(*vertex);
                normals.push([normal[0] as f32, normal[1] as f32, normal[2] as f32]);
                colors.push([srgba.red, srgba.green, srgba.blue, srgba.alpha]);
            }
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32 - 4));
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    // Create a default material
    let material = StandardMaterial {
        base_color: Color::WHITE,
        alpha_mode: AlphaMode::Opaque,
        unlit: true,
        ..Default::default()
    };

    VoxelMesh { mesh, material }
}

pub fn handle_to_voxelid(handle: &Handle<super::world::VoxelChunk>) -> Option<u64> {
    match handle.id() {
        bevy::asset::AssetId::Index { index, marker: _ } => Some(index.to_bits()),
        bevy::asset::AssetId::Uuid { uuid: _ } => None,
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect, Default)]
pub(crate) struct VoxelBlockId(u32);

#[derive(Debug, Resource, Default)]
pub struct VoxelDataMap {
    id_to_data: HashMap<VoxelBlockId, Handle<VoxelChunk>>,
    id_to_block: HashMap<VoxelBlockId, Handle<Block>>,
    to_generate: HashSet<VoxelBlockId>,
    next_id: u32,
}

impl VoxelDataMap {
    fn next(&mut self) -> VoxelBlockId {
        let out = self.next_id;
        self.next_id += 1;
        VoxelBlockId(out)
    }

    fn add_voxel(&mut self, handle: Handle<VoxelChunk>) -> VoxelBlockId {
        let id = self.next();
        self.id_to_data.insert(id, handle);
        self.to_generate.insert(id);
        id
    }
}

pub fn compress(
    input: Res<ButtonInput<KeyCode>>,
    cursor: Query<&HexId, With<crate::screen::hex_map::cursor::Cursor>>,
    hexes: Query<(&HexId, &WorldType)>,
    asset_server: Res<AssetServer>,
    mut voxels: ResMut<VoxelDataMap>,
    mut inventory: Query<&mut Inventory, With<Player>>,
) {
    if input.just_pressed(KeyCode::KeyC) {
        let mut world = WorldType::Empty;
        let cursor = cursor.single();
        for (id, new_world) in hexes.iter() {
            if id == cursor {
                world = *new_world;
            }
        }
        let handle: Handle<VoxelChunk> =
            asset_server.load_with_settings(format!("chunk://{}", cursor), move |w| *w = world);
        for mut inventory in &mut inventory {
            let id = voxels.add_voxel(handle.clone());
            inventory.add_resource(BlockType::Voxel(id), 1);
        }
    }
}

pub fn generate_dynamic_voxels(
    mut voxel_mapping: ResMut<VoxelDataMap>,
    chunks: Res<Assets<VoxelChunk>>,
    mut blocks: ResMut<Assets<Block>>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut voxels: ResMut<Blocks>,
) {
    let to_gen = std::mem::take(&mut voxel_mapping.to_generate);
    for id in to_gen {
        let Some(handle) = voxel_mapping.id_to_data.get(&id) else {
            error!("Id should have associated data");
            continue;
        };
        let Some(data) = chunks.get(handle) else {
            error!("blocks can only be made using loaded chunks");
            continue;
        };
        let mesh = generate_voxel_mesh(data);
        let block = blocks.add(Block {
            id: BlockType::Voxel(id),
            flags: Vec::new(),
            mesh: meshs.add(mesh.mesh),
            material: materials.add(mesh.material),
            color: Color::linear_rgb(1., 0., 1.),
            solid: true,
            components: Vec::new(),
        });
        voxel_mapping.id_to_block.insert(id, block.clone());
        voxels.set(BlockType::Voxel(id), block);
    }
}

pub fn test_if_voxel_genrate(
    mut blocks: Query<(&BlockType, &mut Collider), Added<BlockType>>,
    meshs: Res<Assets<Mesh>>,
    voxels: Res<Assets<Block>>,
    voxel_lookup: Res<Blocks>,
) {
    for (block, mut collider) in &mut blocks {
        let BlockType::Voxel(block) = block else {
            continue;
        };
        let block = voxel_lookup.get(BlockType::Voxel(*block));
        let Some(block_data) = voxels.get(block.id()) else {
            warn!("no block data");
            continue;
        };
        let Some(mesh) = meshs.get(block_data.mesh.id()) else {
            warn!("no mesh data");
            continue;
        };
        *collider = Collider::from_bevy_mesh(
            mesh,
            &bevy_rapier3d::prelude::ComputedColliderShape::TriMesh,
        )
        .expect("To Work");
    }
}
