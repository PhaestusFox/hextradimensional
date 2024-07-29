use bevy::asset::{Asset, Handle};
use bevy::color::{Color, Srgba};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{AlphaMode, Res};
use bevy::reflect::TypePath;
use bevy::render::mesh::{Indices, Mesh};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{
    greedy_quads, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility, RIGHT_HANDED_Y_UP_CONFIG,
};

use super::voxels::{BlockType, VoxelBlock};

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

pub fn generate_voxel_mesh(voxel_block: &VoxelBlock) -> VoxelMesh {
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
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    };

    VoxelMesh { mesh, material }
}
