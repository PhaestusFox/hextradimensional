//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), enter_title);

    app.register_type::<TitleAction>();
    app.add_systems(
        Update,
        (handle_title_action, handle_keyboard_action).run_if(in_state(Screen::Title)),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum TitleAction {
    Play,
    Credits,
    Options,
    /// Exit doesn't work well with embedded applications.
    #[cfg(not(target_family = "wasm"))]
    Exit,
}

fn enter_title(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.button("Play").insert(TitleAction::Play);
            children.button("Credits").insert(TitleAction::Credits);
            children.button("Options").insert(TitleAction::Options);
            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").insert(TitleAction::Exit);
        });
}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&TitleAction>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::HexMap),
                TitleAction::Credits => next_screen.set(Screen::Credits),
                TitleAction::Options => {
                    next_screen.set(Screen::Options(super::options::OptionMenus::Select))
                }
                #[cfg(not(target_family = "wasm"))]
                TitleAction::Exit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn handle_keyboard_action(
    mut next_screen: ResMut<NextState<Screen>>,
    input: Res<ButtonInput<KeyCode>>,
    #[cfg(not(target_family = "wasm"))] mut app_exit: EventWriter<AppExit>,
) {
    for key in (*input).get_just_pressed() {
        match key {
            KeyCode::Escape => {
                #[cfg(not(target_family = "wasm"))]
                app_exit.send(AppExit::Success);
            }
            KeyCode::KeyP => {
                next_screen.set(Screen::HexMap);
            }
            KeyCode::KeyC => {
                next_screen.set(Screen::Credits);
            }
            _ => {}
        }
    }
}
