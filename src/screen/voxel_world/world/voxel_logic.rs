use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{Collider, ExternalImpulse, QueryFilter, ShapeCastOptions},
};

use crate::{
    game::{assets::SfxKey, audio::sfx::PlaySfx, HexSelect},
    screen::{
        voxel_world::{
            item::{spawn_item, Item},
            voxel_util::VoxelPlayer,
            voxels::{Block, BlockType, Blocks},
        },
        NextTarget, Score, Screen, Target,
    },
};

use super::{VoxelChunk, VoxelId};

pub struct VoxelLogic;

impl Plugin for VoxelLogic {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (drill_logic, melter_logic, score_logic, piston_logic)
                .run_if(in_state(Screen::VoxelWorld)),
        );
    }
}

#[derive(Component)]
pub struct Extractor;

#[derive(Component)]
pub struct Melter;

fn drill_logic(
    selected: Res<HexSelect>,
    extractors: Query<&VoxelId, With<Extractor>>,
    mut commands: Commands,
    voxels: Res<Blocks>,
    data: Res<Assets<Block>>,
    chunks: Res<Assets<VoxelChunk>>,
) {
    let Some(chunk) = chunks.get(selected.chunk.id()) else {
        return;
    };
    for extractor in &extractors {
        let below = chunk.get(extractor.0 - IVec3::Y);
        let block = voxels.get(below);
        let block = data.get(block.id()).expect("all blocks loaded");
        if !block.can_mine() {
            return;
        }
        spawn_item(
            below,
            &data,
            &voxels,
            (extractor.0 + IVec3::Y).as_vec3(),
            &mut commands,
        )
    }
}

fn melter_logic(
    context: Res<RapierContext>,
    melters: Query<&Transform, With<Melter>>,
    mut items: Query<&mut BlockType, With<Item>>,
    mut commands: Commands,
    data: Res<Assets<Block>>,
    voxels: Res<Blocks>,
) {
    for pos in &melters {
        let Some((up, _)) = context.cast_shape(
            pos.translation,
            Quat::IDENTITY,
            Vec3::Y,
            &Collider::cuboid(0.5, 0.5, 0.5),
            ShapeCastOptions {
                max_time_of_impact: 1.,
                target_distance: 0.5,
                stop_at_penetration: false,
                compute_impact_geometry_on_penetration: false,
            },
            QueryFilter::only_dynamic(),
        ) else {
            continue;
        };

        let Some((down, _)) = context.cast_shape(
            pos.translation,
            Quat::IDENTITY,
            Vec3::NEG_Y,
            &Collider::cuboid(0.5, 0.5, 0.5),
            ShapeCastOptions {
                max_time_of_impact: 1.,
                target_distance: 0.5,
                stop_at_penetration: false,
                compute_impact_geometry_on_penetration: false,
            },
            QueryFilter::only_dynamic(),
        ) else {
            continue;
        };

        let Ok([mut up, fule]) = items.get_many_mut([up, down]) else {
            continue;
        };
        let fule = voxels.get(*fule);
        let fule = data.get(fule.id()).expect("All Blocks loaded");
        if !fule.is_fuel() {
            continue;
        };
        let melt = voxels.get(*up.as_ref());
        let melt = data.get(melt.id()).expect("All Blocks loaded");
        if let Some(melt) = melt.melt() {
            *up = melt;
            commands.entity(down).despawn();
            commands.trigger(PlaySfx::Key(SfxKey::Melt));
        }
    }
}

#[derive(Component)]
pub struct ScoreGive;

fn score_logic(
    score_giver: Query<&Transform, With<ScoreGive>>,
    context: Res<RapierContext>,
    items: Query<&BlockType, With<Item>>,
    mut commands: Commands,
    mut target: ResMut<Target>,
    mut next_target: ResMut<NextTarget>,
    mut score: ResMut<Score>,
) {
    for pos in &score_giver {
        let Some((up, _)) = context.cast_shape(
            pos.translation,
            Quat::IDENTITY,
            Vec3::Y,
            &Collider::cuboid(0.5, 0.5, 0.5),
            ShapeCastOptions {
                max_time_of_impact: 1.,
                target_distance: 0.5,
                stop_at_penetration: false,
                compute_impact_geometry_on_penetration: false,
            },
            QueryFilter::only_dynamic(),
        ) else {
            continue;
        };

        let Ok(block) = items.get(up) else {
            return;
        };

        if block == &target.0 {
            commands.trigger(PlaySfx::Key(SfxKey::Progress));
            target.0 = next_target.next();
            score.0 += 1;
        } else {
            commands.trigger(PlaySfx::Key(SfxKey::NoProgress));
        }
        commands.entity(up).despawn();
    }
}

#[derive(Component)]
pub struct Piston;

fn piston_logic(
    pistons: Query<&Transform, With<Piston>>,
    context: Res<RapierContext>,
    items: Query<Entity, With<Item>>,
    player: Query<&Parent, With<VoxelPlayer>>,
    mut commands: Commands,
) {
    for pos in &pistons {
        let Some((hit, _)) = context.cast_shape(
            pos.translation,
            Quat::IDENTITY,
            pos.up().as_vec3(),
            &Collider::cuboid(0.25, 0.25, 0.25),
            ShapeCastOptions {
                max_time_of_impact: 1.,
                target_distance: 1.,
                stop_at_penetration: false,
                compute_impact_geometry_on_penetration: false,
            },
            QueryFilter::only_dynamic(),
        ) else {
            continue;
        };

        for player in &player {
            if hit == player.get() {
                commands.entity(player.get()).insert(ExternalImpulse {
                    torque_impulse: Vec3::ZERO,
                    impulse: pos.up() * 25.,
                });
            }
        }

        if let Ok(block) = items.get(hit) {
            commands.entity(block).insert(ExternalImpulse {
                torque_impulse: Vec3::ZERO,
                impulse: pos.up() * 5.,
            });
        }
    }
}
