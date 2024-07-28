use crate::{
    game::main_character::Player,
    screen::{inventory::Inventory, Screen},
};

use super::{
    voxel_util::VoxelPlayer,
    voxels::{Block, BlockType, Blocks},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, kill_layer)
            .add_systems(Update, (pickup_item, update_icon));
    }
}

#[derive(Component)]
pub struct Item;

pub fn spawn_item(
    block_type: BlockType,
    blocks: &Assets<Block>,
    voxels: &Blocks,
    offset: Vec3,
    commands: &mut Commands,
) {
    let block = voxels.get(block_type);
    let block = blocks.get(block.id()).expect("All Blocks loaded");
    commands.spawn((
        Item,
        StateScoped(Screen::VoxelWorld),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        block_type,
        PbrBundle {
            mesh: block.mesh(),
            material: block.material(),
            transform: Transform::from_translation(offset).with_scale(Vec3::ONE * 0.5),
            ..Default::default()
        },
    ));
}

pub fn pickup_item(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    physics: Res<RapierContext>,
    mut vox_player: Query<(&Parent, &GlobalTransform), With<VoxelPlayer>>,
    mut player_inventory: Query<&mut Inventory, With<Player>>,
    mut voxels: Query<&BlockType, With<Item>>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }
    for (ignore, player) in &mut vox_player {
        if let Some((hit, _)) = physics.cast_ray(
            player.translation(),
            player.forward().as_vec3(),
            6.,
            false,
            QueryFilter::new().exclude_rigid_body(ignore.get()),
        ) {
            if let Ok(block) = voxels.get_mut(hit) {
                commands.entity(hit).despawn();
                player_inventory.single_mut().add_resource(*block, 1);
            }
        }
    }
}

pub fn kill_layer(items: Query<(Entity, &Transform), With<Item>>, mut commands: Commands) {
    for (entity, item) in &items {
        if item.translation.y < -10. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_icon(
    mut items: Query<
        (&mut Handle<Mesh>, &mut Handle<StandardMaterial>, &BlockType),
        (With<Item>, Changed<BlockType>),
    >,
    voxels: Res<Blocks>,
    data: Res<Assets<Block>>,
) {
    for (mut mesh, mut material, block) in &mut items {
        let block = voxels.get(*block);
        let block = data.get(block.id()).expect("All Blocks Loaded");
        *mesh = block.mesh();
        *material = block.material();
    }
}
