//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::screen::{inventory::clear_inventory, Screen};

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
    // Added Editor in dev builds
    // app.add_plugins(bevy_editor_pls::EditorPlugin::default());
    // Add Debug Rendering for rapier
    app.add_plugins(bevy_rapier3d::prelude::RapierDebugRenderPlugin::default());
    app.add_systems(Update, toggle_rapier_debug);
    app.add_systems(Update, clear_inventory);
}

fn toggle_rapier_debug(
    mut context: ResMut<bevy_rapier3d::prelude::DebugRenderContext>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F12) {
        context.enabled = !context.enabled;
    }
}
