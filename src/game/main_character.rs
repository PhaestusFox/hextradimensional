use bevy::{ecs::query::QueryData, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::screen::inventory::Inventory;

use super::PlayerAction;

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
        leafwing_input_manager::InputManagerBundle::<super::PlayerAction> {
            action_state: ActionState::default(),
            input_map: default_player_inputs(),
        },
    ));
}

fn default_player_inputs() -> InputMap<PlayerAction> {
    let mut map = InputMap::default();

    map.insert(
        PlayerAction::Hit,
        UserInput::Single(InputKind::Mouse(MouseButton::Left)),
    );

    map.insert(
        PlayerAction::Place,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger2)),
    );

    map.insert(
        PlayerAction::Place,
        UserInput::Single(InputKind::Mouse(MouseButton::Right)),
    );

    map.insert(
        PlayerAction::Hit,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger2)),
    );

    map.insert(
        PlayerAction::Jump,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
    );

    map.insert(
        PlayerAction::Jump,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::West)),
    );

    map.insert(
        PlayerAction::MoveUp,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyW)),
    );

    map.insert(
        PlayerAction::MoveUp,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::ArrowUp)),
    );

    map.insert(
        PlayerAction::MoveDown,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyS)),
    );

    map.insert(
        PlayerAction::MoveDown,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::ArrowDown)),
    );

    map.insert(
        PlayerAction::EnterHex,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::Enter)),
    );

    map.insert(
        PlayerAction::EnterHex,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South)),
    );

    map.insert(
        PlayerAction::ExitChunk,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::Escape)),
    );

    map.insert(
        PlayerAction::ExitChunk,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::East)),
    );

    map.insert(
        PlayerAction::MoveLeft,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyA)),
    );

    map.insert(
        PlayerAction::MoveLeft,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::ArrowLeft)),
    );

    map.insert(
        PlayerAction::MoveRight,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyD)),
    );

    map.insert(
        PlayerAction::MoveRight,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::ArrowDown)),
    );

    map.insert(
        PlayerAction::Move,
        UserInput::Single(InputKind::DualAxis(DualAxis::left_stick())),
    );

    map.insert(
        PlayerAction::Look,
        UserInput::Single(InputKind::DualAxis(
            DualAxis::right_stick()
                .with_sensitivity(15., 15.)
                .inverted_y()
                .with_deadzone(DeadZoneShape::Ellipse {
                    radius_x: 0.2,
                    radius_y: 0.2,
                }),
        )),
    );

    map.insert(
        PlayerAction::Look,
        UserInput::Single(InputKind::DualAxis(DualAxis::mouse_motion())),
    );

    map.insert(
        PlayerAction::ToolbarNext,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyQ)),
    );

    map.insert(
        PlayerAction::ToolbarNext,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadUp)),
    );

    map.insert(
        PlayerAction::ToolbarPrev,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::DPadDown)),
    );

    map.insert(
        PlayerAction::ItemInc,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger)),
    );

    map.insert(
        PlayerAction::ItemInc,
        UserInput::Single(InputKind::MouseWheel(MouseWheelDirection::Up)),
    );

    map.insert(
        PlayerAction::ItemDec,
        UserInput::Single(InputKind::MouseWheel(MouseWheelDirection::Down)),
    );

    map.insert(
        PlayerAction::ItemDec,
        UserInput::Single(InputKind::GamepadButton(GamepadButtonType::LeftTrigger)),
    );

    #[cfg(feature = "dev")]
    map.insert(
        PlayerAction::ToolbarPrev,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyZ)),
    );

    #[cfg(not(feature = "dev"))]
    map.insert(
        PlayerAction::ToolbarPrev,
        UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyE)),
    );
    map
}
