use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{Collider, QueryFilter, ShapeCastOptions},
};

use crate::{
    game::HexSelect,
    screen::{
        voxel_world::{
            item::{spawn_item, Item},
            voxel_util::Blocks,
            BlockType,
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
            (drill_logic, melter_logic, score_logic).run_if(in_state(Screen::VoxelWorld)),
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
    chunks: Res<Assets<VoxelChunk>>,
) {
    let Some(chunk) = chunks.get(selected.chunk.id()) else {
        return;
    };
    for extractor in &extractors {
        let below = chunk.get(extractor.0 - IVec3::Y);
        if !below.can_mine() {
            return;
        }
        spawn_item(
            below,
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
        if !fule.fuel() {
            continue;
        };
        if let Some(melt) = up.melt() {
            *up = melt;
            commands.entity(down).despawn();
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
            target.0 = next_target.next();
            score.0 += 1;
        }

        commands.entity(up).despawn();
    }
}
