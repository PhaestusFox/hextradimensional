use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        main_character::Player,
        PlayerAction,
    },
    ui::{
        prelude::InteractionQuery,
        widgets::{Containers, Widgets},
    },
};

use super::{inventory::Inventory, Menu, Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Options(OptionMenus::Select)),
        spawn_select_menu,
    )
    .add_systems(
        OnEnter(Screen::Options(OptionMenus::KeyBinding)),
        spawn_keybind_menu,
    )
    .add_systems(OnEnter(Screen::Options(OptionMenus::Dev)), spawn_dev_menu)
    .add_systems(Update, handle_option_action.run_if(in_state(Menu)));
}

fn spawn_select_menu(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Options(OptionMenus::Select)))
        .with_children(|p| {
            p.button("Dev Tools").insert(OptionAction::OpenDev);
            p.button("Key Bindings").insert(OptionAction::OpenKeyBind);
            p.button("Back").insert(OptionAction::Back);
        });
}

fn spawn_dev_menu(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Options(OptionMenus::Dev)))
        .with_children(|p| {
            p.button("Clear Inventory")
                .insert(OptionAction::ClearInventory);
            p.button("Back").insert(OptionAction::Back);
        });
}

fn spawn_keybind_menu(
    mut commands: Commands,
    layout: Res<crate::game::assets::ButtonLayout>,
    player: Query<&InputMap<PlayerAction>>,
    icons: Res<HandleMap<ImageKey>>,
) {
    let Ok(binding) = player.get_single() else {
        warn!("Player not loaded");
        return;
    };
    commands
        .ui_root()
        .insert(StateScoped(Screen::Options(OptionMenus::KeyBinding)))
        .with_children(|p| {
            if let Some(icons) = icons.get(&ImageKey::ButtonIcons) {
                p.key_bindings(&layout.0, binding, icons);
            } else {
                error!("Failed to get image");
            }
            p.button("Back").insert(OptionAction::Back);
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptionMenus {
    Select,
    KeyBinding,
    Dev,
}

impl OptionMenus {
    fn prev(&self) -> Screen {
        match self {
            OptionMenus::Select => Screen::Title,
            OptionMenus::KeyBinding => Screen::Options(OptionMenus::Select),
            OptionMenus::Dev => Screen::Options(OptionMenus::Select),
        }
    }
}

#[derive(Component)]
enum OptionAction {
    OpenDev,
    OpenKeyBind,
    Back,
    ClearInventory,
}

fn handle_option_action(
    current_screen: Res<State<Screen>>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&OptionAction>,
    mut commands: Commands,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                OptionAction::OpenDev => next_screen.set(Screen::Options(OptionMenus::Dev)),
                OptionAction::OpenKeyBind => {
                    next_screen.set(Screen::Options(OptionMenus::KeyBinding))
                }
                OptionAction::Back => {
                    next_screen.set(if let Screen::Options(open) = current_screen.get() {
                        open.prev()
                    } else {
                        warn!("Options Button Not in Options Menu");
                        Screen::Title
                    });
                }
                OptionAction::ClearInventory => commands.add(|world: &mut World| {
                    let mut query =
                        QueryBuilder::<&mut Inventory, With<Player>>::new(world).build();
                    for mut inventory in query.iter_mut(world) {
                        inventory.clear();
                    }
                }),
            }
        }
    }
}
