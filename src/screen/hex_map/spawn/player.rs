//! Spawn the player.

use bevy::prelude::*;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        HexSelect,
    },
    screen::{
        hex_map::{
            animation::PlayerAnimation,
            movement::{Movement, MovementController, WrapWithinWindow},
        },
        Screen,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_player);
    app.register_type::<HexPlayer>();
}

#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct HexPlayer;

pub fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    hex_select: Res<HexSelect>,
) {
    // A texture atlas is a way to split one image with a grid into multiple sprites.
    // By attaching it to a [`SpriteBundle`] and providing an index, we can specify which section of the image we want to see.
    // We will use this to animate our player character. You can learn more about texture atlases in this example:
    // https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    commands.spawn((
        Name::new("Player"),
        HexPlayer,
        SpriteBundle {
            texture: image_handles[&ImageKey::Ducky].clone_weak(),
            transform: Transform {
                translation: (hex_select.hex_id).xyz(),
                scale: Vec2::splat(2.0).extend(1.0),
                rotation: Quat::IDENTITY,
            },
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player_animation.get_atlas_index(),
        },
        MovementController::default(),
        Movement { speed: 420.0 },
        WrapWithinWindow,
        player_animation,
        StateScoped(Screen::HexMap),
    ));
}
