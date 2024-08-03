use crate::{
    game::{HexSelect, PlayerAction},
    screen::{hex_vox_util::MapDirection, voxel_world::voxel_util::VoxelPlayer, Screen},
};

use leafwing_input_manager::prelude::*;

use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::{
    Collider, KinematicCharacterController, KinematicCharacterControllerOutput, LockedAxes,
    RigidBody,
};

pub struct VoxelCamera;

impl Plugin for VoxelCamera {
    fn build(&self, app: &mut App) {
        app.init_resource::<VoxelSettings>()
            .init_resource::<InputState>()
            .add_systems(Update, cursor_toggle)
            .add_systems(
                Update,
                (player_look, player_move, apply_jump, player_jump).chain(),
            )
            .add_systems(OnEnter(Screen::VoxelWorld), cursor_grab)
            .add_systems(OnExit(Screen::VoxelWorld), cursor_release);
    }
}

fn player_move(
    mut player: Query<&mut bevy_rapier3d::prelude::KinematicCharacterController>,
    camera: Query<(&Transform, &Parent), With<VoxelPlayer>>,
    input: Query<&ActionState<PlayerAction>>,
    time: Res<Time>,
) {
    let Ok(input) = input.get_single() else {
        return;
    };
    for (camera, body) in &camera {
        let mut delta = Vec3::ZERO;

        if let Some(input) = input.axis_pair(&PlayerAction::Move) {
            let offset = input.xy().round();
            delta.x += offset.x;
            delta.z += offset.y;
        };

        if input.pressed(&PlayerAction::MoveUp) {
            delta.z += 1.;
        }
        if input.pressed(&PlayerAction::MoveDown) {
            delta.z -= 1.;
        }
        if input.pressed(&PlayerAction::MoveLeft) {
            delta.x -= 1.;
        }
        if input.pressed(&PlayerAction::MoveRight) {
            delta.x += 1.;
        }
        let mut forward = camera.forward().as_vec3();
        forward.y = 0.;
        forward = forward.normalize();
        let mut right = camera.right().as_vec3();
        right.y = 0.;
        right = right.normalize();
        let next = (forward * delta.z + right * delta.x) * time.delta_seconds() * 10.;
        if let Ok(mut controller) = player.get_mut(body.get()) {
            // need to add small amount of down movement or the player is never grounded standing still
            controller.translation = Some(next + Vec3::NEG_Y * 0.01);
        } else {
            warn!("Voxel Player should be child of player controller");
        }
    }
}

// don't know why this is here maybe legacy from the flycam im copying
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

fn player_look(
    settings: Res<VoxelSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<VoxelPlayer>>,
    input: Query<&ActionState<PlayerAction>>,
) {
    let Ok(input) = input.get_single() else {
        warn!("Player not found");
        return;
    };
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            if let Some(data) = input.axis_pair(&PlayerAction::Look) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                let window_scale = window.height().min(window.width());
                pitch -= (settings.mouse_sensitivity * data.y() * window_scale).to_radians();
                yaw -= (settings.mouse_sensitivity * data.x() * window_scale).to_radians();
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_release(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn cursor_grab(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn cursor_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<VoxelSettings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            match window.cursor.grab_mode {
                CursorGrabMode::None => {
                    window.cursor.grab_mode = CursorGrabMode::Confined;
                    window.cursor.visible = false;
                }
                _ => {
                    window.cursor.grab_mode = CursorGrabMode::None;
                    window.cursor.visible = true;
                }
            }
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

#[derive(Resource)]
pub struct VoxelSettings {
    pub toggle_grab_cursor: KeyCode,
    pub mouse_sensitivity: f32,
}

impl Default for VoxelSettings {
    fn default() -> Self {
        Self {
            toggle_grab_cursor: KeyCode::Backquote,
            mouse_sensitivity: 0.00012,
        }
    }
}

#[derive(Component)]
struct Jump {
    left: f32,
}

// jumping does not disable gravity so a value less then 9.8 will not make you move up
const JUMP_POWER: f32 = 9.8 * 3.;

fn apply_jump(
    mut commands: Commands,
    mut jumping: Query<(Entity, &mut Jump, &mut KinematicCharacterController)>,
    time: Res<Time>,
) {
    // the max jump for this frame
    let max_jump = JUMP_POWER * time.delta_seconds();
    for (player, mut jump, mut controller) in &mut jumping {
        let jump_power = max_jump.min(jump.left);
        let to_move = if let Some(other) = controller.translation {
            other + Vec3::Y * jump_power
        } else {
            Vec3::Y * jump_power
        };
        controller.translation = Some(to_move);
        jump.left -= jump_power;
        if jump.left <= 0. {
            commands.entity(player).remove::<Jump>();
        }
    }
}

fn player_jump(
    input: Query<&ActionState<PlayerAction>>,
    mut commands: Commands,
    players: Query<(Entity, &KinematicCharacterControllerOutput)>,
) {
    let Ok(input) = input.get_single() else {
        return;
    };

    if input.just_pressed(&PlayerAction::Jump) {
        for (entity, output) in &players {
            if output.grounded {
                commands.entity(entity).insert(Jump { left: 3. });
            }
        }
    }
}

fn pos_from_enter(direction: &MapDirection) -> Vec3 {
    match direction {
        MapDirection::Down => Vec3::new(8., 0., 8.),
        MapDirection::North => Vec3::new(16., 8., 8.),
        MapDirection::East => Vec3::new(8., 8., 16.),
        MapDirection::Up => Vec3::new(8., 16., 8.),
        MapDirection::South => Vec3::new(0., 8., 8.),
        MapDirection::West => Vec3::new(8., 8., 0.),
    }
}

pub fn spawn_player(mut commands: Commands, hex_select: Res<HexSelect>) {
    commands
        .spawn((
            StateScoped(Screen::VoxelWorld),
            SpatialBundle {
                transform: Transform::from_translation(pos_from_enter(&hex_select.direction)),
                ..Default::default()
            },
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::capsule_y(0.5, 0.45),
            KinematicCharacterControllerOutput::default(),
            bevy_rapier3d::control::KinematicCharacterController {
                ..Default::default()
            },
        ))
        // This is the child camera
        .with_children(|p| {
            p.spawn((
                Camera3dBundle {
                    camera: Camera {
                        order: 1,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::Y * 0.5),
                    ..Default::default()
                },
                VoxelPlayer,
            ))
            .with_children(|p| {
                p.spawn(SpotLightBundle {
                    ..Default::default()
                });
            });
        });
}
