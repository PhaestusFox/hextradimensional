use crate::screen::{voxel_world::voxel_util::VoxelPlayer, Screen};
use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

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
    settings: Res<VoxelSettings>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (camera, body) in &camera {
        let mut delta = Vec3::ZERO;
        if input.pressed(settings.move_forward) {
            delta.z += 1.;
        }
        if input.pressed(settings.move_backward) {
            delta.z -= 1.;
        }
        if input.pressed(settings.move_left) {
            delta.x -= 1.;
        }
        if input.pressed(settings.move_right) {
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
            controller.translation = Some(next);
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
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -=
                            (settings.mouse_sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -=
                            (settings.mouse_sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
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
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub toggle_grab_cursor: KeyCode,
    pub move_speed: f32,
    pub mouse_sensitivity: f32,
}

impl Default for VoxelSettings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            jump: KeyCode::Space,
            toggle_grab_cursor: KeyCode::Backquote,
            mouse_sensitivity: 0.00012,
            move_speed: 12.,
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
        info!("jump to use {}", jump_power);
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
    input: Res<ButtonInput<KeyCode>>,
    settings: Res<VoxelSettings>,
    mut commands: Commands,
    players: Query<(Entity, &KinematicCharacterControllerOutput), With<VoxelPlayer>>,
) {
    if input.just_pressed(settings.jump) {
        for (entity, output) in &players {
            if output.grounded {
                commands.entity(entity).insert(Jump { left: 3. });
            }
        }
    }
}
