// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_quickstart::AppPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins(AppPlugin)
        .insert_resource(Time::from_hz(1.))
        .run()
}
