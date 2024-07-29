use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::{
    app::{App, Startup},
    asset::Assets,
    log::{error, info, warn},
    prelude::{Commands, Event, EventReader, Query, ReflectResource, Res, ResMut, Resource, With},
    reflect::{self, Reflect},
};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::screen::{
    inventory::Inventory,
    voxel_world::world::{VoxelChunk, VoxelStore},
};

use super::{main_character::Player, HexSelect};

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
    mut pkv: ResMut<PkvStore>,
    input_seed: Option<Res<SeedString>>,
) {
    let seed = match input_seed {
        Some(seed_string) => {
            let seed = Seed::from_string(seed_string.to_owned().0);
            pkv.set("seed", &seed).expect("failed to store seed");
            commands.remove_resource::<SeedString>();
            seed
        }
        None => {
            if let Ok(seed) = pkv.get::<Seed>("seed") {
                info!("Seed is {}", seed.0);
                seed
            } else {
                let seed = Seed::from_string("Bevy Jam 5 Hextradimensional".to_string());
                pkv.set("seed", &seed).expect("failed to store seed");
                seed
            }
        }
    };

    commands.insert_resource(seed);
}

/// This setup should be used to get the seed from save data if the player has played, or generate a new seed if they haven't
pub fn inventory_load(
    pkv: ResMut<PkvStore>,
    mut player_inventory: Query<&mut Inventory, With<Player>>,
) {
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
}

/// This setup should be used to get the seed from save data if the player has played, or generate a new seed if they haven't
pub fn inventory_save(
    mut pkv: ResMut<PkvStore>,
    player_inventory: Query<&Inventory, With<Player>>,
) {
    let inventory = player_inventory.get_single().unwrap();
    pkv.set("inventory", inventory)
        .expect("failed to store seed");
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
    app.insert_resource(PkvStore::new("Bevy Jam 5", "Hextradimensional"));
    app.add_systems(Startup, seed_load_and_save);
}
