use bevy::prelude::Component;

use super::BlockType;

/// Define a struct for inventory slots
/// Fields are public to allow direct access from UI. This can be changed to getter in the future
#[derive(Debug, Clone)]
pub struct InventorySlot {
    pub resource_type: Option<BlockType>,
    pub quantity: u32,
}

/// This is the inventory component, meant to be used in conjunction with VoxelPlayer
/// Fields are public to allow direct access from UI. This can be changed to getter in the future
#[derive(Component)]
pub struct Inventory {
    pub slots: Vec<InventorySlot>,
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
        }
    }

    pub fn add_resource(&mut self, resource_type: BlockType, quantity: u32) {
        match self.slots.iter_mut().find(|slot| {
            slot.resource_type == Some(resource_type.clone()) || slot.resource_type.is_none()
        }) {
            Some(slot) => {
                slot.resource_type = Some(resource_type);
                slot.quantity += quantity;
            }
            None => println!("Inventory full, couldn't add resource"),
        }
    }

    pub fn get_total_resource(&self, resource_type: BlockType) -> u32 {
        self.slots
            .iter()
            .filter(|slot| slot.resource_type == Some(resource_type.clone()))
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
}
