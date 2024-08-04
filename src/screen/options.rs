use bevy::prelude::*;
use leafwing_input_manager::prelude::{InputMap, UserInput};

use crate::{
    game::{
        assets::{HandleMap, ImageKey},
        main_character::Player,
        PlayerAction,
    },
    ui::{
        icons::KeyIcons,
        prelude::InteractionQuery,
        widgets::{Containers, UiIcon, Widgets},
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
    .add_systems(
        Update,
        (handle_option_action, run_rebind_actions).run_if(in_state(Menu)),
    )
    .add_systems(
        Update,
        (update_new_binding).run_if(resource_changed::<RebindingState>),
    )
    .insert_resource(RebindingState {
        action: PlayerAction::Hit,
        old: BindingKey(None),
        new: BindingKey(None),
    });
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
    Rebind,
}

impl OptionMenus {
    fn prev(&self) -> Screen {
        match self {
            OptionMenus::Select => Screen::Title,
            OptionMenus::KeyBinding => Screen::Options(OptionMenus::Select),
            OptionMenus::Dev => Screen::Options(OptionMenus::Select),
            OptionMenus::Rebind => Screen::Options(OptionMenus::KeyBinding),
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

#[derive(Component)]
pub enum RebindAction {
    New,
    Delete,
    Apply,
    Open,
}

fn spawn_rebind_menu(
    commands: &mut Commands,
    action: PlayerAction,
    state: &RebindingState,
    icons: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Options(OptionMenus::Rebind)))
        .with_children(|p| {
            p.label(format!("Rebinding {:?}", action));
            p.horizontal().with_children(|p| {
                if let Some(icon) = state.old.0.clone() {
                    p.label("Current Binding: ");
                    p.icon(layout.clone(), icons.clone(), KeyIcons::from(icon));
                } else {
                    p.label("Unbound");
                }
            });
            p.horizontal().with_children(|p| {
                if let Some(icon) = state.new.0.clone() {
                    p.label("New Binding: ");
                    p.icon(layout.clone(), icons.clone(), KeyIcons::from(icon))
                        .insert(NewBinding);
                } else {
                    p.label("Unbound");
                }
            });
            p.button("Apply").insert(RebindAction::Apply);
            p.button("Clear").insert(RebindAction::Delete);
            p.button("Close").insert(OptionAction::Back);
        });
}

#[derive(Component)]
struct NewBinding;

fn update_new_binding(
    state: Res<RebindingState>,
    mut new: Query<(&mut TextureAtlas, &mut Visibility), With<NewBinding>>,
) {
    for (mut atlas, mut vis) in &mut new {
        println!("Info");
        if let Some(new) = &state.new.0 {
            atlas.index = KeyIcons::from(new.clone()).index();
        } else {
            atlas.index = KeyIcons::NotSupported.index();
            *vis = Visibility::Hidden;
        }
    }
}

fn run_rebind_actions(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<(Entity, &RebindAction)>,
    mut button_data: Query<(&BindingKey, &PlayerAction)>,
    mut commands: Commands,
    mut state: ResMut<RebindingState>,
    layout: Res<crate::game::assets::ButtonLayout>,
    icons: Res<HandleMap<ImageKey>>,
    mut bindings: Query<&mut InputMap<PlayerAction>>,
) {
    let Some(icons) = icons.get(&ImageKey::ButtonIcons) else {
        error!("Button Icons Not loaded");
        return;
    };
    for (interaction, (button, action)) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                RebindAction::Open => {
                    let Ok((binding, player)) = button_data.get(button) else {
                        error!("Open button needs key and action componets");
                        continue;
                    };
                    next_screen.set(Screen::Options(OptionMenus::Rebind));
                    state.action = *player;
                    state.old = binding.clone();
                    state.new = binding.clone();
                    spawn_rebind_menu(&mut commands, *player, &state, icons, &layout.0);
                }
                RebindAction::Delete => state.new = BindingKey(None),
                RebindAction::Apply => {
                    next_screen.set(Screen::Options(OptionMenus::Rebind));
                    for mut bindings in &mut bindings {
                        if let Some(old) = state.old.0.clone() {
                            bindings.remove(&state.action, old);
                        }
                        if let Some(new) = state.new.0.clone() {
                            bindings.insert(state.action, new);
                        }
                    }
                }
                RebindAction::New => {
                    let Ok((_, player)) = button_data.get(button) else {
                        error!("New button needs key and action componets");
                        continue;
                    };
                    state.action = *player;
                    state.old = BindingKey(None);
                    state.new = BindingKey(None);
                    spawn_rebind_menu(&mut commands, *player, &state, icons, &layout.0);
                }
            }
        }
    }
}

#[derive(Resource)]
struct RebindingState {
    action: PlayerAction,
    old: BindingKey,
    new: BindingKey,
}

#[derive(Component, Clone)]
pub struct BindingKey(pub Option<UserInput>);
