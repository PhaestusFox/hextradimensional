#[derive(Component)]
pub struct Cursor;

use super::spawn::player::HexPlayer;
use crate::screen::{
    hex_vox_util::{HexId, MapDirection, HEX_SIZE},
    Screen,
};
use bevy::prelude::*;
use strum::IntoEnumIterator;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::HexMap), spawn_cursor);
        app.add_systems(OnExit(Screen::HexMap), clear_cursor);
        app.add_systems(Update, move_cursor.run_if(in_state(Screen::HexMap)));
    }
}

fn spawn_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MapDirection::Down,
        Name::new("Cursor"),
        HexId::new(0, 0),
        Cursor,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(HEX_SIZE)),
                ..Default::default()
            },
            texture: asset_server.load("images/hexes/outline.png"),
            transform: Transform::from_translation(Vec3::NEG_Z * -9.),
            ..Default::default()
        },
    ));
}

fn clear_cursor(mut commands: Commands, cursor: Query<Entity, With<Cursor>>) {
    for cursor in &cursor {
        commands.entity(cursor).despawn_recursive();
    }
}

fn move_cursor(
    player: Query<&Transform, With<HexPlayer>>,
    mut cursors: Query<
        (&mut HexId, &mut Transform, &mut MapDirection),
        (With<Cursor>, Without<HexPlayer>),
    >,
) {
    let player = player.single();
    let id = HexId::from_xyz(player.translation);
    for (mut cursor, mut pos, mut n) in &mut cursors {
        if &id != cursor.as_ref() {
            *cursor = id;
        }
        let mut direction = MapDirection::Down;
        let mut distance = (id + MapDirection::Down)
            .xyz()
            .distance_squared(player.translation);
        for neighbor in MapDirection::iter() {
            let next = ((id + neighbor).xyz() / 2.).distance_squared(player.translation);
            if next < distance {
                distance = next;
                direction = neighbor;
            }
        }
        pos.rotation = Quat::from_rotation_z(direction.angle());
        if &direction != n.as_ref() {
            *n = direction;
        }
    }
}
