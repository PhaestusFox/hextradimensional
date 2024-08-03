use bevy::{
    input::ButtonInput,
    log::warn,
    prelude::{info, Component, KeyCode, Query, Res, With},
    reflect::Reflect,
};
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};

use crate::{
    game::{main_character::Player, PlayerAction},
    voxel_world::voxels::BlockType,
};

/// Define a struct for inventory slots
/// Fields are public to allow direct access from UI. This can be changed to getter in the future
#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct InventorySlot {
    pub resource_type: Option<BlockType>,
    pub quantity: u32,
}

/// This is the inventory component, meant to be used in conjunction with Player
/// Fields are public to allow direct access from UI. This can be changed to getter in the future
#[derive(Component, Reflect, Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub slots: Vec<InventorySlot>,
    pub selected_slot: usize,
    pub selected_row: usize,
}

impl Inventory {
    pub fn new(size: usize) -> Self {
        Inventory {
            slots: vec![
                InventorySlot {
                    resource_type: None,
                    quantity: 0
                };
                size
            ],
            selected_slot: 0,
            selected_row: 0,
        }
    }

    pub fn clear(&mut self) {
        self.selected_row = 0;
        self.selected_slot = 0;
        for item in self.slots.iter_mut() {
            item.quantity = 0;
            item.resource_type = None;
        }
    }

    /// returns true if added and false if Inventory full
    pub fn add_resource(&mut self, resource_type: BlockType, quantity: u32) -> bool {
        // First, try to find a matching slot and add to it
        for slot in &mut self.slots {
            if let Some(rt) = &slot.resource_type {
                if *rt == resource_type {
                    slot.quantity += quantity;
                    return true;
                }
            }
        }

        // If no matching slot found, find the first empty slot
        if let Some(empty_slot) = self
            .slots
            .iter_mut()
            .find(|slot| slot.resource_type.is_none())
        {
            empty_slot.resource_type = Some(resource_type);
            empty_slot.quantity = quantity;
            return true;
        }

        // If no empty slot found, the inventory is full
        info!("Inventory full, couldn't add resource");
        false
    }

    pub fn get_total_resource(&self, resource_type: BlockType) -> u32 {
        self.slots
            .iter()
            .filter(|slot| matches!(&slot.resource_type, Some(rt) if *rt == resource_type))
            .map(|slot| slot.quantity)
            .sum()
    }

    // This method first checks to see if one has the resources for crafting.
    // If the inventory has those resources it then deducts those resources and returns true.
    // If the inventory does not it returns false
    pub fn check_and_deduct_resources(&mut self, requirements: &[(BlockType, u32)]) -> bool {
        // First, check if we have enough of each resource
        for (resource_type, required_amount) in requirements {
            if self.get_total_resource((*resource_type).clone()) < *required_amount {
                return false;
            }
        }

        // If we have enough, proceed with deduction
        for (resource_type, required_amount) in requirements {
            let mut remaining = *required_amount;
            for slot in &mut self.slots {
                if slot.resource_type == Some((*resource_type).clone()) {
                    if slot.quantity >= remaining {
                        slot.quantity -= remaining;
                        if slot.quantity == 0 {
                            slot.resource_type = None;
                        }
                        break;
                    } else {
                        remaining -= slot.quantity;
                        slot.quantity = 0;
                        slot.resource_type = None;
                    }
                }
            }
        }

        true
    }

    pub fn select_slot(&mut self, slot_index: usize) {
        if slot_index < 10 {
            self.selected_slot = slot_index + (10 * self.selected_row);
        }
    }

    pub fn get_selected_block(&self) -> Option<BlockType> {
        self.slots[self.selected_slot].resource_type.clone()
    }
}

pub fn clear_inventory(
    input: Res<ButtonInput<KeyCode>>,
    mut player_inventory: Query<&mut Inventory, With<Player>>,
) {
    if input.just_pressed(KeyCode::NumpadSubtract) {
        for mut inventory in &mut player_inventory {
            inventory.clear()
        }
    }
}

pub fn change_row_inventory(
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&ActionState<PlayerAction>>,
    mut player_inventory: Query<&mut Inventory, With<Player>>,
) {
    if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        for key in input.get_just_pressed() {
            match key {
                KeyCode::Digit1 => {
                    if let Ok(mut inventory) = player_inventory.get_single_mut() {
                        inventory.selected_row = 0;
                        inventory.selected_slot = 0;
                    }
                }
                KeyCode::Digit2 => {
                    if let Ok(mut inventory) = player_inventory.get_single_mut() {
                        inventory.selected_row = 1;
                        inventory.selected_slot = 10;
                    }
                }
                KeyCode::Digit3 => {
                    if let Ok(mut inventory) = player_inventory.get_single_mut() {
                        inventory.selected_row = 2;
                        inventory.selected_slot = 20;
                    }
                }
                KeyCode::Digit4 => {
                    if let Ok(mut inventory) = player_inventory.get_single_mut() {
                        inventory.selected_row = 3;
                        inventory.selected_slot = 30;
                    }
                }
                KeyCode::Digit5 => {
                    if let Ok(mut inventory) = player_inventory.get_single_mut() {
                        inventory.selected_row = 4;
                        inventory.selected_slot = 40;
                    }
                }
                KeyCode::Digit6 => {
                    if let Ok(mut inventory) = player_inventory.get_single_mut() {
                        inventory.selected_row = 5;
                        inventory.selected_slot = 50;
                    }
                }
                _ => {}
            }
        }
    }
    let Ok(input) = player.get_single() else {
        warn!("Player not loaded");
        return;
    };

    if input.just_pressed(&PlayerAction::ToolbarNext) {
        inc(&mut player_inventory)
    }

    if input.just_pressed(&PlayerAction::ToolbarPrev) {
        dec(&mut player_inventory)
    }
}

fn inc(player_inventory: &mut Query<&mut Inventory, With<Player>>) {
    if let Ok(mut inventory) = player_inventory.get_single_mut() {
        inventory.selected_row = (inventory.selected_row + 5) % 6;
        inventory.selected_slot = inventory.selected_row * 10;
    }
}

fn dec(player_inventory: &mut Query<&mut Inventory, With<Player>>) {
    if let Ok(mut inventory) = player_inventory.get_single_mut() {
        inventory.selected_row = (inventory.selected_row + 1) % 6;
        inventory.selected_slot = inventory.selected_row * 10;
    }
}

fn inc_dec(
    input: &ButtonInput<KeyCode>,
    player_inventory: &mut Query<&mut Inventory, With<Player>>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        if let Ok(mut inventory) = player_inventory.get_single_mut() {
            inventory.selected_row = (inventory.selected_row + 5) % 6;
            inventory.selected_slot = inventory.selected_row * 10;
        }
    }

    #[cfg(not(feature = "dev"))]
    let increment_key = KeyCode::KeyE;

    #[cfg(feature = "dev")]
    let increment_key = KeyCode::KeyZ;

    if input.just_pressed(increment_key) {
        if let Ok(mut inventory) = player_inventory.get_single_mut() {
            inventory.selected_row = (inventory.selected_row + 1) % 6;
            inventory.selected_slot = inventory.selected_row * 10;
        }
    }
}
