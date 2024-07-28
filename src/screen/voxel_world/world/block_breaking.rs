use crate::{
    game::HexSelect,
    screen::{
        voxel_world::{
            inventory::Inventory,
            voxel_util::{Blocks, VoxelPlayer},
            BasicBlock, BlockType,
        },
        Screen,
    },
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{VoxelChunk, VoxelId};

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct BlockBreakDebugSettings {
    show_player_ray: bool,
    show_target_voxel: bool,
}

#[derive(Component)]
struct Breaking(f32);

pub(crate) fn block_breaking_plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    {
        app.init_resource::<BlockBreakDebugSettings>();
        app.register_type::<BlockBreakDebugSettings>();
        app.add_systems(Update, draw_debug)
            .add_systems(Update, block_placing.run_if(in_state(Screen::VoxelWorld)))
            .add_systems(
                Update,
                (
                    break_block,
                    scail_breaking_block,
                    unbreak_block,
                    pickup_block,
                )
                    .chain(),
            );
    }
}

fn draw_debug(
    settings: Res<BlockBreakDebugSettings>,
    mut gizmo: Gizmos,
    player: Query<&GlobalTransform, With<VoxelPlayer>>,
) {
    if settings.show_player_ray {
        for player in &player {
            let forward = player.forward() * 3.;
            gizmo.ray(
                player.translation(),
                player.translation() + forward,
                LinearRgba::GREEN,
            );
        }
    }
}

fn unbreak_block(
    time: Res<Time>,
    mut commands: Commands,
    mut voxels: Query<(Entity, &mut Breaking)>,
) {
    for (entity, mut breaking) in &mut voxels {
        breaking.0 -= time.delta_seconds() * 0.3;
        if breaking.0 <= 0. {
            commands.entity(entity).remove::<Breaking>();
        }
    }
}

fn break_block(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    physics: Res<RapierContext>,
    player: Query<(&Parent, &GlobalTransform), With<VoxelPlayer>>,
    mut voxels: Query<Option<&mut Breaking>, With<VoxelId>>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }
    for (ignore, player) in &player {
        if let Some((hit, _)) = physics.cast_ray(
            player.translation(),
            player.forward().as_vec3(),
            6.,
            false,
            QueryFilter::new().exclude_rigid_body(ignore.get()),
        ) {
            match voxels.get_mut(hit) {
                Ok(None) => {
                    commands.entity(hit).insert(Breaking(0.5));
                }
                Ok(Some(mut breaking)) => {
                    breaking.0 += 0.5;
                }
                Err(_) => {
                    error!("rays should only hit voxels");
                }
            }
        }
    }
}

fn pickup_block(
    mut commands: Commands,
    mut player: Query<&mut Inventory, With<VoxelPlayer>>,
    blocks: Query<(Entity, &Breaking, &VoxelId), Changed<Breaking>>,
    selected: Res<HexSelect>,
    mut chunk_data: ResMut<Assets<VoxelChunk>>,
) {
    for mut inventory in &mut player {
        for (entity, state, id) in &blocks {
            if state.0 >= 0.55 {
                let Some(chunk) = chunk_data.get_mut(selected.chunk.id()) else {
                    continue;
                };
                let out = chunk.set(id.0, BlockType::Basic(BasicBlock::Air));
                if out == BlockType::Basic(BasicBlock::Air) {
                    warn!("Removed Air");
                    continue;
                };
                inventory.add_resource(out, 1);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn scail_breaking_block(
    mut blocks: Query<&mut Transform>,
    mut changed: Query<(Entity, &Breaking), Changed<Breaking>>,
    mut removed: RemovedComponents<Breaking>,
) {
    for (block, breaking) in &mut changed {
        let Ok(mut transform) = blocks.get_mut(block) else {
            warn!("Breaking block has no transform");
            continue;
        };
        // max is so 0 scale is not possible;
        transform.scale = Vec3::splat(1. - breaking.0).max(Vec3::splat(0.01));
    }
    for block in removed.read() {
        let Ok(mut transform) = blocks.get_mut(block) else {
            continue;
        };
        transform.scale = Vec3::ONE;
    }
}

fn block_placing(
    mut commands: Commands,
    voxel_blocks: Res<Blocks>,
    mut player: Query<(&GlobalTransform, &mut Inventory), With<VoxelPlayer>>,
    input: Res<ButtonInput<MouseButton>>,
    physics: Res<RapierContext>,
    blocks: Query<&VoxelId>,
    selected: Res<HexSelect>,
    mut chunk_data: ResMut<Assets<VoxelChunk>>,
) {
    if !input.just_pressed(MouseButton::Right) {
        return;
    }
    for (transform, mut inventory) in &mut player {
        let Some((hit, normal)) = physics.cast_ray_and_get_normal(
            transform.translation(),
            transform.forward().as_vec3(),
            6.,
            false,
            QueryFilter::only_fixed(),
        ) else {
            continue;
        };
        let id = vec3_to_voxelId(normal.normal);
        let Ok(old) = blocks.get(hit) else {
            error!("hit block is not voxel");
            continue;
        };
        let id = id + *old;
        if !id.in_chunk() {
            continue;
        }
        let Some(chunk) = chunk_data.get_mut(selected.chunk.id()) else {
            continue;
        };
        let Some(block) = inventory.get_selected_block() else {
            continue;
        };
        let mut entity = commands.spawn((
            Name::new("Voxel Block Placed"),
            id,
            PbrBundle {
                mesh: voxel_blocks.mesh(&block),
                material: voxel_blocks.texture(&block),
                transform: Transform::from_translation(id.0.as_vec3()),
                ..Default::default()
            },
        ));
        block.add_components(&mut entity);
        if block.is_solid() {
            entity.insert(bevy_rapier3d::prelude::Collider::cuboid(0.5, 0.5, 0.5));
        }
        chunk.set(id.0, block.clone());
        inventory.check_and_deduct_resources(&[(block, 1)]);
    }
}

fn vec3_to_voxelId(vec: Vec3) -> VoxelId {
    let abs = vec.abs();

    // x is biggest
    if abs.x > abs.y && abs.x > abs.z {
        if vec.x > 0. {
            VoxelId(IVec3::X)
        } else {
            VoxelId(IVec3::NEG_X)
        }
    }
    // y is biggest
    else if abs.y > abs.x && abs.y > abs.z {
        if vec.y > 0. {
            VoxelId(IVec3::Y)
        } else {
            VoxelId(IVec3::NEG_Y)
        }
    }
    // z is biggest
    else if abs.z > abs.x && abs.z > abs.y {
        if vec.z > 0. {
            VoxelId(IVec3::Z)
        } else {
            VoxelId(IVec3::NEG_Z)
        }
    } else {
        panic!("I dont know if this works")
    }
}
