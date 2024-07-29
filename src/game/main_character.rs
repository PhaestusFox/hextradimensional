use bevy::{
    core::Name,
    ecs::query::QueryData,
    prelude::{
        Commands, Component, GlobalTransform, InheritedVisibility, ReflectComponent,
        ReflectResource, Resource,
    },
    reflect::Reflect,
};

use crate::screen::inventory::Inventory;

/// This should be all facets of the main character that we want to store in save data

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, QueryData)]
#[reflect(Component)]
pub struct Player;

pub fn spawn_main_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Saved Player"),
        Player,
        Inventory::new(60),
        InheritedVisibility::VISIBLE,
        GlobalTransform::IDENTITY,
    ));
}
