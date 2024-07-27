use bevy::{
    prelude::{Component, ReflectComponent, ReflectResource, Resource},
    reflect::Reflect,
};

/// This should be all facets of the main character that we want to store in save data

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Resource)]
pub struct Seed(pub u64);
