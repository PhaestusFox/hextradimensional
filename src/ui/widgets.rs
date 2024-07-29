//! Helper traits for creating common widgets.

use bevy::{
    ecs::system::EntityCommands, prelude::*, render::texture::TRANSPARENT_IMAGE_HANDLE, ui::Val::*,
};

use super::{interaction::InteractionPalette, palette::*};
use crate::screen::inventory::{Inventory, InventorySlot};
use crate::screen::voxel_world::ui::FullInventoryUI;
use crate::screen::voxel_world::voxels::{Block, Blocks};

// Define the UiRoot component
#[derive(Component)]
pub struct UiRoot;

/// An extension trait for spawning UI widgets.
pub trait Widgets {
    /// Spawn a simple button with text.
    fn button(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple header label. Bigger than [`Widgets::label`].
    fn header(&mut self, text: impl Into<String>) -> EntityCommands;

    /// Spawn a simple text label.
    fn label(&mut self, text: impl Into<String>) -> EntityCommands;
    /// Spawn an inventory slot UI element
    fn inventory_slot(
        &mut self,
        slot: &InventorySlot,
        voxel_data: &Assets<Block>,
        voxels: &Blocks,
        materials: &Assets<StandardMaterial>,
    ) -> EntityCommands;

    /// Spawn a hotbar inventory UI
    fn hotbar(
        &mut self,
        inventory: &Inventory,
        voxels: &Blocks,
        voxel_data: &Assets<Block>,
        materials: &Assets<StandardMaterial>,
    ) -> EntityCommands;

    /// Spawn a complete inventory UI
    fn full_inventory(
        &mut self,
        inventory: &Inventory,
        voxels: &Blocks,
        voxel_data: &Assets<Block>,
        materials: &Assets<StandardMaterial>,
    ) -> EntityCommands;
}

impl<T: Spawn> Widgets for T {
    fn button(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Button"),
            ButtonBundle {
                style: Style {
                    width: Px(200.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
            InteractionPalette {
                none: NODE_BACKGROUND,
                hovered: BUTTON_HOVERED_BACKGROUND,
                pressed: BUTTON_PRESSED_BACKGROUND,
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Button Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 40.0,
                        color: BUTTON_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn header(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Header"),
            NodeBundle {
                style: Style {
                    width: Px(500.0),
                    height: Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(NODE_BACKGROUND),
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Header Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 40.0,
                        color: HEADER_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    fn label(&mut self, text: impl Into<String>) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Label"),
            NodeBundle {
                style: Style {
                    width: Px(500.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
        ));
        entity.with_children(|children| {
            children.spawn((
                Name::new("Label Text"),
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 24.0,
                        color: LABEL_TEXT,
                        ..default()
                    },
                ),
            ));
        });
        entity
    }

    /// This controls the styling for individual inventory slots
    fn inventory_slot(
        &mut self,
        slot: &InventorySlot,
        blocks: &Assets<Block>,
        voxels: &Blocks,
        materials: &Assets<StandardMaterial>,
    ) -> EntityCommands {
        let (image_handle, image_color) = match &slot.resource_type {
            Some(block_type) => {
                let block = voxels.get(block_type.clone());
                let block = blocks.get(block.id()).expect("Block Struct Failed to Load");
                let material = materials
                    .get(block.material().id())
                    .expect("block material to exist");
                if let Some(texture) = &material.base_color_texture {
                    (texture.clone(), block.color())
                } else {
                    (TRANSPARENT_IMAGE_HANDLE, Color::srgb(0.8, 0.8, 0.8))
                }
            }
            None => (TRANSPARENT_IMAGE_HANDLE, Color::srgb(0.8, 0.8, 0.8)),
        };
        let mut entity = self.spawn((
            Name::new("Inventory Slot"),
            ImageBundle {
                style: Style {
                    width: Percent(80.0),
                    height: Percent(80.0),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                background_color: BackgroundColor(image_color),
                image: UiImage {
                    texture: image_handle,
                    ..default()
                },
                ..default()
            },
        ));

        entity.with_children(|children| {
            if let Some(resource_type) = &slot.resource_type {
                children.spawn((
                    Name::new("Resource Type"),
                    TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            bottom: Val::Percent(5.0),
                            left: Val::Percent(5.0),
                            ..default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("{:?}", resource_type),
                                style: TextStyle {
                                    font_size: 12.0,
                                    color: Color::srgb(0.0, 1.0, 1.0), // ! Have this change depending on resource type
                                    ..default()
                                },
                            }],
                            justify: JustifyText::Left,
                            linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                        },
                        ..default()
                    },
                ));
            }
            children.spawn((
                Name::new("Quantity"),
                TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Percent(5.0),
                        right: Val::Percent(5.0),
                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: slot.quantity.to_string(),
                            style: TextStyle {
                                font_size: 16.0,
                                color: Color::srgb(1.0, 1.0, 0.0), // ! Have this change depending on resource type
                                ..default()
                            },
                        }],
                        justify: JustifyText::Right,
                        linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                    },
                    ..default()
                },
            ));
        });

        entity
    }

    /// This controls the styling for the inventory hotbar. The hotbar holds 10 items
    fn hotbar(
        &mut self,
        inventory: &Inventory,
        voxels: &Blocks,
        voxel_data: &Assets<Block>,
        materials: &Assets<StandardMaterial>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Hotbar"),
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: vec![RepeatedGridTrack::flex(10, 1.0)], // 3 equal-width columns
                    grid_template_rows: vec![RepeatedGridTrack::flex(1, 1.0)], // 2 equal-height rows
                    justify_content: JustifyContent::SpaceAround,
                    position_type: PositionType::Absolute,
                    bottom: Percent(0.0),
                    left: Percent(10.0),
                    right: Percent(10.0),
                    height: Percent(10.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                ..default()
            },
        ));

        entity.with_children(|children| {
            for (index, slot) in inventory
                .slots
                .iter()
                .skip(inventory.selected_row * 10usize)
                .take(10)
                .enumerate()
            {
                let global_index = inventory.selected_row * 10 + index;
                let mut slot_entity = children.spawn((
                    Name::new(format!("Hotbar Slot {}", global_index)),
                    NodeBundle {
                        style: Style {
                            width: Percent(100.0),
                            height: Percent(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: if global_index == inventory.selected_slot {
                            BackgroundColor(Color::srgb(1.0, 0.0, 0.0))
                        } else {
                            BackgroundColor(Color::NONE)
                        },
                        ..default()
                    },
                ));

                slot_entity.with_children(|slot_children| {
                    slot_children.inventory_slot(slot, voxel_data, voxels, materials);
                });
            }
        });

        entity
    }

    fn full_inventory(
        &mut self,
        inventory: &Inventory,
        voxels: &Blocks,
        voxel_data: &Assets<Block>,
        materials: &Assets<StandardMaterial>,
    ) -> EntityCommands {
        let mut entity = self.spawn((
            Name::new("Full Inventory"),
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::flex(10, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(6, 1.0),
                    justify_content: JustifyContent::SpaceAround,
                    position_type: PositionType::Absolute,
                    bottom: Percent(25.0),
                    left: Percent(10.0),
                    right: Percent(10.0),
                    height: Percent(60.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                visibility: Visibility::Hidden,
                ..default()
            },
            FullInventoryUI,
        ));

        entity.with_children(|children| {
            for slot in inventory.slots.iter().take(60) {
                children.inventory_slot(slot, voxel_data, voxels, materials);
            }
        });

        entity
    }
}

/// An extension trait for spawning UI containers.
pub trait Containers {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl Containers for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Px(10.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            UiRoot,
        ))
    }
}

/// An internal trait for types that can spawn entities.
/// This is here so that [`Widgets`] can be implemented on all types that
/// are able to spawn entities.
/// Ideally, this trait should be [part of Bevy itself](https://github.com/bevyengine/bevy/issues/14231).
trait Spawn {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands;
}

impl Spawn for Commands<'_, '_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}

impl Spawn for ChildBuilder<'_> {
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        self.spawn(bundle)
    }
}
