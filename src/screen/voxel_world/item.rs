use super::{voxel_util::Blocks, BlockType};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Item;

pub fn spawn_item(block: BlockType, voxels: &Blocks, offset: Vec3, commands: &mut Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        PbrBundle {
            mesh: voxels.mesh(&block),
            material: voxels.texture(&block),
            transform: Transform::from_translation(offset).with_scale(Vec3::ONE * 0.5),
            ..Default::default()
        },
    ));
}
