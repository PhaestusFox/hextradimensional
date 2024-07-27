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
        app.add_systems(Update, draw_debug);
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
    physics: Res<RapierContext>,
    mut player: Query<(&GlobalTransform, &mut Inventory), With<VoxelPlayer>>,
    mut voxels: Query<(Option<&mut Breaking>, &BlockType)>
) {
    for (player, mut inventory) in &mut player {
        if let Some((hit, _)) = physics.cast_ray(player.translation(), player.forward().as_vec3(), 3., false, QueryFilter::only_fixed()) {
            match voxels.get_mut(hit) {
                Ok((None, _)) => {
                    commands.entity(hit).insert(Breaking(false));
                },
                Ok((Some(mut breaking), voxel)) => {
                    if breaking.0 {
                        inventory.add_resource(voxel.clone(), 1);
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