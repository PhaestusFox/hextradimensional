use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::screen::voxel_world::{inventory::Inventory, voxel_util::VoxelPlayer, BlockType};

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct BlockBreakDebugSettings {
    show_player_ray: bool,
    show_target_voxel: bool,
}

#[derive(Component)]
struct Breaking(bool);

pub(crate) fn block_breaking_plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    {
        app.init_resource::<BlockBreakDebugSettings>();
        app.register_type::<BlockBreakDebugSettings>();
        app.add_systems(Update, draw_debug)
        .add_systems(Update, (break_block, scail_breaking_block, pickup_block).chain());
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
            gizmo.ray(player.translation(), player.translation() + forward, LinearRgba::GREEN);
        }
    }
}

fn break_block(
    mut commands: Commands,
    input: Res<ButtonInput<MouseButton>>,
    physics: Res<RapierContext>,
    player: Query<&GlobalTransform, With<VoxelPlayer>>,
    mut voxels: Query<Option<&mut Breaking>, With<BlockType>>
) {
    if !input.just_pressed(MouseButton::Left) {return;}
    for player in &player {
        if let Some((hit, _)) = physics.cast_ray(player.translation(), player.forward().as_vec3(), 3., false, QueryFilter::only_fixed()) {
            match voxels.get_mut(hit) {
                Ok(None) => {
                    commands.entity(hit).insert(Breaking(false));
                },
                Ok(Some(mut breaking)) => {
                    if breaking.0 {
                        continue;
                    } else {    
                        breaking.0 = true;
                    }
                },
                Err(_) => {
                    error!("rays should only hit voxels");
                },
            }
        }
    }
}

fn pickup_block(
    mut commands: Commands,
    mut player: Query<&mut Inventory, With<VoxelPlayer>>,
    blocks: Query<(Entity, &BlockType, &Breaking), Changed<Breaking>>,
) {
    for mut inventory in &mut player {
        for (entity, block, state) in &blocks {
            if state.0 {
                inventory.add_resource(block.clone(), 1);
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn scail_breaking_block(
    mut blocks: Query<(&mut Transform, &Breaking), Changed<Breaking>>,
) {
    for (mut transform, breaking) in &mut blocks {
        if breaking.0 {
            transform.scale = Vec3::ONE * 0.01;
        } else {
            transform.scale = Vec3::ONE * 0.5;
        }
    }
}