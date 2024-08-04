use bevy::{
    log::warn,
    prelude::{GamepadAxisType, GamepadButtonType, KeyCode, MouseButton},
};
use leafwing_input_manager::{axislike::AxisType, prelude::DualAxis};

pub enum KeyIcons {
    Keyboard(KeyCode),
    GamepadButton(GamepadButtonType),
    GamepadAxis(GamepadAxisType),
    MouseButton(MouseButton),
    MouseMotion,
    MouseScroll,
    MouseScrollUp,
    MouseScrollDown,
    LeftStick,
    RightStick,
    UntypedStick,
    NotDone,
    Add,
    NotSupported,
}

const NOT_DONE_INDEX: usize = 17 * 34;

impl super::widgets::UiIcon for KeyIcons {
    fn index(&self) -> usize {
        match self {
            KeyIcons::GamepadButton(buttons) => match buttons {
                GamepadButtonType::South => 8,
                GamepadButtonType::East => 9,
                GamepadButtonType::North => 11,
                GamepadButtonType::West => 10,
                GamepadButtonType::LeftTrigger => 16 * 34 + 9,
                GamepadButtonType::LeftTrigger2 => 16 * 34 + 7,
                GamepadButtonType::RightTrigger => 16 * 34 + 10,
                GamepadButtonType::RightTrigger2 => 16 * 34 + 8,
                GamepadButtonType::LeftThumb => 12 * 34 + 18,
                GamepadButtonType::RightThumb => 14 * 34 + 18,
                GamepadButtonType::DPadUp => 34 + 1,
                GamepadButtonType::DPadDown => 34 + 3,
                GamepadButtonType::DPadLeft => 34 + 4,
                GamepadButtonType::DPadRight => 34 + 2,
                _ => NOT_DONE_INDEX,
            },
            KeyIcons::MouseButton(button) => match button {
                MouseButton::Left => 2 * 34 + 9,
                MouseButton::Right => 2 * 34 + 10,
                MouseButton::Middle => 2 * 34 + 11,
                _ => NOT_DONE_INDEX,
            },
            // KeyIcons::Keyboard(_) => todo!(),
            KeyIcons::GamepadAxis(axis) => match axis {
                GamepadAxisType::LeftStickX => 12 * 34 + 13,
                GamepadAxisType::LeftStickY => 12 * 34 + 14,
                GamepadAxisType::RightStickX => 14 * 34 + 13,
                GamepadAxisType::RightStickY => 14 * 34 + 14,
                _ => NOT_DONE_INDEX,
            },
            KeyIcons::Keyboard(key) => match key {
                KeyCode::Escape => 17,
                KeyCode::F1 => 18,
                KeyCode::F2 => 19,
                KeyCode::F3 => 20,
                KeyCode::F4 => 21,
                KeyCode::F5 => 22,
                KeyCode::F6 => 23,
                KeyCode::F7 => 24,
                KeyCode::F8 => 25,
                KeyCode::F9 => 26,
                KeyCode::F10 => 27,
                KeyCode::F11 => 28,
                KeyCode::F12 => 29,
                KeyCode::Backquote => 30,
                KeyCode::Digit1 => 34 + 17,
                KeyCode::Digit2 => 34 + 18,
                KeyCode::Digit3 => 34 + 19,
                KeyCode::Digit4 => 34 + 20,
                KeyCode::Digit5 => 34 + 21,
                KeyCode::Digit6 => 34 + 22,
                KeyCode::Digit7 => 34 + 23,
                KeyCode::Digit8 => 34 + 24,
                KeyCode::Digit9 => 34 + 25,
                KeyCode::Digit0 => 34 + 26,
                KeyCode::Minus => 34 + 27,
                KeyCode::Equal => 34 + 28,
                KeyCode::KeyQ => 2 * 34 + 17,
                KeyCode::KeyW => 2 * 34 + 18,
                KeyCode::KeyE => 2 * 34 + 19,
                KeyCode::KeyR => 2 * 34 + 20,
                KeyCode::KeyT => 2 * 34 + 21,
                KeyCode::KeyY => 2 * 34 + 22,
                KeyCode::KeyU => 2 * 34 + 23,
                KeyCode::KeyI => 2 * 34 + 24,
                KeyCode::KeyO => 2 * 34 + 25,
                KeyCode::KeyP => 2 * 34 + 26,
                KeyCode::BracketLeft => 2 * 34 + 27,
                KeyCode::BracketRight => 2 * 34 + 28,
                KeyCode::ShiftLeft => 3 * 34 + 17,
                KeyCode::ShiftRight => 3 * 34 + 17,
                KeyCode::KeyA => 3 * 34 + 18,
                KeyCode::KeyS => 3 * 34 + 19,
                KeyCode::KeyD => 3 * 34 + 20,
                KeyCode::KeyF => 3 * 34 + 21,
                KeyCode::KeyG => 3 * 34 + 22,
                KeyCode::KeyH => 3 * 34 + 23,
                KeyCode::KeyJ => 3 * 34 + 24,
                KeyCode::KeyK => 3 * 34 + 25,
                KeyCode::KeyL => 3 * 34 + 26,
                KeyCode::Quote => 3 * 34 + 27,
                KeyCode::Semicolon => 3 * 34 + 28,
                KeyCode::Space => 4 * 34 + 17,
                KeyCode::SuperLeft => 4 * 34 + 18,
                KeyCode::SuperRight => 4 * 34 + 18,
                KeyCode::KeyZ => 4 * 34 + 19,
                KeyCode::KeyX => 4 * 34 + 20,
                KeyCode::KeyC => 4 * 34 + 21,
                KeyCode::KeyV => 4 * 34 + 22,
                KeyCode::KeyB => 4 * 34 + 23,
                KeyCode::KeyN => 4 * 34 + 24,
                KeyCode::KeyM => 4 * 34 + 25,
                KeyCode::Comma => 4 * 34 + 26,
                KeyCode::Period => 4 * 34 + 27,
                KeyCode::Slash => 4 * 34 + 28,
                KeyCode::ArrowUp => 4 * 34 + 30,
                KeyCode::ArrowRight => 4 * 34 + 31,
                KeyCode::ArrowDown => 4 * 34 + 32,
                KeyCode::ArrowLeft => 4 * 34 + 33,
                KeyCode::Enter => 3 * 34 + 32,
                _ => NOT_DONE_INDEX,
            },
            KeyIcons::LeftStick => 12 * 34 + 15,
            KeyIcons::RightStick => 14 * 34 + 15,
            KeyIcons::UntypedStick => 10 * 34 + 15,
            KeyIcons::MouseMotion => 2 * 34 + 8,
            KeyIcons::MouseScroll => 2 * 34 + 14,
            KeyIcons::MouseScrollUp => 2 * 34 + 12,
            KeyIcons::MouseScrollDown => 2 * 34 + 13,
            KeyIcons::NotSupported => 12 * 34 + 28,
            KeyIcons::NotDone => NOT_DONE_INDEX,
            KeyIcons::Add => 20 * 34 + 14,
        }
    }
}

impl From<leafwing_input_manager::user_input::UserInput> for KeyIcons {
    fn from(value: leafwing_input_manager::user_input::UserInput) -> KeyIcons {
        match value {
            leafwing_input_manager::prelude::UserInput::Single(input) => match input {
                leafwing_input_manager::prelude::InputKind::GamepadButton(button) => {
                    KeyIcons::GamepadButton(button)
                }
                leafwing_input_manager::prelude::InputKind::SingleAxis(_) => KeyIcons::NotDone,
                leafwing_input_manager::prelude::InputKind::DualAxis(DualAxis {
                    x,
                    y,
                    deadzone: _,
                }) => match (x.axis_type, y.axis_type) {
                    (AxisType::Gamepad(stick), AxisType::Gamepad(_)) => match stick {
                        GamepadAxisType::LeftStickX | GamepadAxisType::LeftStickY => {
                            KeyIcons::LeftStick
                        }
                        GamepadAxisType::RightStickX | GamepadAxisType::RightStickY => {
                            KeyIcons::RightStick
                        }
                        _ => KeyIcons::UntypedStick,
                    },
                    (AxisType::MouseWheel(_), AxisType::MouseWheel(_)) => KeyIcons::MouseScroll,
                    (AxisType::MouseMotion(_), AxisType::MouseMotion(_)) => KeyIcons::MouseMotion,
                    (_, _) => {
                        warn!("None Matching Axis on DualAxis not supported");
                        KeyIcons::NotSupported
                    }
                },
                leafwing_input_manager::prelude::InputKind::PhysicalKey(key) => {
                    KeyIcons::Keyboard(key)
                }
                leafwing_input_manager::prelude::InputKind::Modifier(_) => KeyIcons::NotDone,
                leafwing_input_manager::prelude::InputKind::Mouse(button) => {
                    KeyIcons::MouseButton(button)
                }
                leafwing_input_manager::prelude::InputKind::MouseWheel(direction) => {
                    match direction {
                        leafwing_input_manager::prelude::MouseWheelDirection::Up => {
                            KeyIcons::MouseScrollUp
                        }
                        leafwing_input_manager::prelude::MouseWheelDirection::Down => {
                            KeyIcons::MouseScrollDown
                        }
                        _ => KeyIcons::NotDone,
                    }
                }
                leafwing_input_manager::prelude::InputKind::MouseMotion(_) => KeyIcons::NotDone,
                _ => KeyIcons::NotDone,
            },
            leafwing_input_manager::prelude::UserInput::Chord(_) => KeyIcons::NotDone,
            leafwing_input_manager::prelude::UserInput::VirtualDPad(_) => KeyIcons::NotDone,
            leafwing_input_manager::prelude::UserInput::VirtualAxis(_) => KeyIcons::NotDone,
        }
    }
}
