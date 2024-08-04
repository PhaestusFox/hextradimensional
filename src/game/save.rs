use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::{
    app::{App, PostStartup, Startup},
    asset::Assets,
    log::{error, info, warn},
    prelude::{Commands, Entity, OnEnter, OnExit, Query, ReflectResource, Res, Resource, With},
    reflect::Reflect,
};
use leafwing_input_manager::prelude::InputMap;
use serde::{Deserialize, Serialize};

use crate::screen::{
    inventory::Inventory,
    voxel_world::world::{VoxelChunk, VoxelStore},
    Screen,
};

use super::{main_character::Player, HexSelect, PlayerAction};

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct Seed(pub u64);

impl Seed {
    pub fn from_string(input: String) -> Self {
        let mut hasher = DefaultHasher::new();
        input.hash(&mut hasher);
        Seed(hasher.finish())
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct SeedString(pub String);

/// This setup should be used to get the seed from save data if the player has played, or generate a new seed if they haven't
fn seed_load_and_save(
    mut commands: Commands,
    pkv: Res<VoxelStore>,
    input_seed: Option<Res<SeedString>>,
) {
    let seed = match input_seed {
        Some(seed_string) => {
            let seed = Seed::from_string(seed_string.to_owned().0);
            if let Some(mut pkv) = pkv.write() {
                pkv.set("seed", &seed).expect("failed to store seed");
            } else {
                error!("failed to get pkv store")
            }
            commands.remove_resource::<SeedString>();
            seed
        }
        None => {
            if let Some(mut pkv) = pkv.write() {
                if let Ok(seed) = pkv.get::<Seed>("seed") {
                    info!("Seed is {}", seed.0);
                    seed
                } else {
                    let seed = Seed::from_string("Bevy Jam 5 Hextradimensional".to_string());
                    pkv.set("seed", &seed).expect("failed to store seed");
                    seed
                }
            } else {
                error!("failed to get pkv store");
                Seed::from_string("Bevy Jam 5 Hextradimensional".to_string())
            }
        }
    };

    commands.insert_resource(seed);
}

/// This setup should be used to get the seed from save data if the player has played, or generate a new seed if they haven't
pub fn inventory_load(
    pkv: Res<VoxelStore>,
    mut player_inventory: Query<&mut Inventory, With<Player>>,
) {
    if let Some(pkv) = pkv.write() {
        let inventory = if let Ok(inventory) = pkv.get::<Inventory>("inventory") {
            info!("Inventory is {:?}", inventory);
            inventory
        } else {
            return;
        };

        // Check if a player entity with an Inventory component exists
        if let Ok(mut player_inventory) = player_inventory.get_single_mut() {
            // If it exists, update its value
            *player_inventory = inventory;
        } else {
            panic!("No player");
        }
    } else {
        error!("failed to get pkv store")
    }
}

/// This setup should be used to get the seed from save data if the player has played, or generate a new seed if they haven't
pub fn inventory_save(pkv: Res<VoxelStore>, player_inventory: Query<&Inventory, With<Player>>) {
    if let Some(mut pkv) = pkv.write() {
        let inventory = player_inventory.get_single().unwrap();
        pkv.set("inventory", inventory)
            .expect("failed to store seed");
    } else {
        error!("failed to get pkv store")
    }
}

pub fn save_chunk_data(
    store: Res<VoxelStore>,
    chunks: Res<Assets<VoxelChunk>>,
    selected: Res<HexSelect>,
) {
    let Some(chunk) = chunks.get(selected.chunk.id()) else {
        warn!("Chunk not loaded");
        return;
    };
    if let Some(mut store) = store.write() {
        info!("Saved chunk as {}", selected.hex_id.to_string());
        if let Err(e) = store.set(selected.hex_id.to_string(), chunk) {
            error!("Chunk not saved {e}");
        };
    } else {
        warn!("Failed to write chunk to store");
    }
}

pub(super) fn plugin(app: &mut App) {
    // This initializes as Company, Game to set store locations
    app.add_systems(Startup, seed_load_and_save)
        .add_systems(PostStartup, keybind_load)
        // save the keybind every time you exit the menu
        .add_systems(
            OnExit(Screen::Options(
                crate::screen::options::OptionMenus::KeyBinding,
            )),
            keybind_save,
        );
}

pub fn keybind_save(player: Query<&InputMap<PlayerAction>>, store: Res<VoxelStore>) {
    if let Some(mut store) = store.write() {
        for map in &player {
            if store.set("Keybinds", map).is_err() {
                error!("Failed to save keybingings");
            };
        }
    }
}

pub fn keybind_load(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    store: Res<VoxelStore>,
) {
    if let Some(store) = store.read() {
        if let Ok(bindings) = store.get::<InputMap<PlayerAction>>("Keybinds") {
            if let Ok(player) = player.get_single() {
                commands.entity(player).insert(bindings);
            } else {
                error!("Failed to get Player");
            }
        } else {
            error!("Failed to save keybingings");
        };
    }
}
