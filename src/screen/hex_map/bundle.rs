use bevy::prelude::*;

use crate::screen::hex_vox_util::{HexId, HEX_SIZE};

#[derive(Bundle)]
pub struct HexCellBundle {
    pub id: HexId,
    /// The visibility of the entity.
    pub visibility: Visibility,
    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,
    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,
    /// The transform of the entity.
    pub transform: Transform,
    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
    /// Specifies the rendering properties of the sprite, such as color tint and flip.
    pub sprite: Sprite,
    /// A reference-counted handle to the image asset to be drawn.
    pub texture: Handle<Image>,
}

impl Default for HexCellBundle {
    fn default() -> Self {
        HexCellBundle {
            id: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(HEX_SIZE)),
                ..Default::default()
            },
            texture: Default::default(),
        }
    }
}
