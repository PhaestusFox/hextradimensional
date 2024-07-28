use crate::screen::Screen;

use super::{
    inventory::Inventory,
    voxel_util::{Blocks, VoxelPlayer},
    BlockType,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Item;

pub fn spawn_item(block: BlockType, voxels: &Blocks, offset: Vec3, commands: &mut Commands) {
    commands.spawn((
        Item,
        StateScoped(Screen::VoxelWorld),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        block,
        PbrBundle {
            mesh: voxels.mesh(&block),
            material: voxels.texture(&block),
            transform: Transform::from_translation(offset).with_scale(Vec3::ONE * 0.5),
            ..Default::default()
        },
    ));
}

pub fn pickup_item(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    physics: Res<RapierContext>,
    mut player: Query<(&Parent, &GlobalTransform, &mut Inventory), With<VoxelPlayer>>,
    mut voxels: Query<&BlockType, With<Item>>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }
    for (ignore, player, mut inventory) in &mut player {
        if let Some((hit, _)) = physics.cast_ray(
            player.translation(),
            player.forward().as_vec3(),
            6.,
            false,
            QueryFilter::new().exclude_rigid_body(ignore.get()),
        ) {
            if let Ok(block) = voxels.get_mut(hit) {
                commands.entity(hit).despawn();
                inventory.add_resource(*block, 1);
            }
        }
    }
}
