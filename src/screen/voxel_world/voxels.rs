use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    ecs::system::EntityCommands,
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    utils::HashMap,
};
use block_mesh::{
    greedy_quads, ndshape::ConstShape3u32, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility,
    RIGHT_HANDED_Y_UP_CONFIG,
};
use serde::{Deserialize, Serialize};
use serde_big_array::Array;
use std::{array, hash::Hash, sync::Arc};
use strum::IntoEnumIterator;

use crate::screen::hex_vox_util::MapDirection;

use super::voxel_block_generation::generate_voxel_mesh;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Block>()
            .init_asset_loader::<BlockLoader>()
            .init_resource::<Blocks>()
            .add_systems(Startup, test_load);
    }
}

#[derive(Resource)]
pub struct Blocks {
    type_to_asset: HashMap<BlockType, Handle<Block>>,
}

impl Blocks {
    pub fn get(&self, block: BlockType) -> Handle<Block> {
        self.type_to_asset
            .get(&block)
            .cloned()
            .expect("All blocks be loaded")
    }
}

impl FromWorld for Blocks {
    fn from_world(world: &mut World) -> Self {
        let mut map = HashMap::new();
        let asset_server = world.resource::<AssetServer>();
        for block in BlockType::iter() {
            map.insert(block.clone(), asset_server.load(block.path()));
        }
        Blocks { type_to_asset: map }
    }
}

#[derive(Serialize, Deserialize)]
struct BlockAsset {
    id: BlockType,
    flags: Vec<BlockFlags>,
    mesh: Option<String>,
    texture: String,
    color: Color,
    #[serde(default = "default_solid")]
    solid: bool,
    #[serde(default)]
    components: Vec<BlockLogic>,
}

fn default_solid() -> bool {
    true
}

#[derive(Asset, Reflect)]
pub struct Block {
    id: BlockType,
    flags: Vec<BlockFlags>,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    color: Color,
    solid: bool,
    components: Vec<BlockLogic>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Reflect)]
enum BlockLogic {
    Extractor,
    Melter,
    ScoreGive,
    Piston,
}

impl Block {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn can_mine(&self) -> bool {
        !self.flags.contains(&BlockFlags::NoMine)
    }

    pub fn get_type(&self) -> BlockType {
        self.id.clone()
    }

    pub fn is_solid(&self) -> bool {
        self.solid
    }

    pub fn is_fuel(&self) -> bool {
        self.flags.contains(&BlockFlags::Fuel)
    }

    pub fn melt(&self) -> Option<BlockType> {
        for flag in self.flags.iter() {
            if let BlockFlags::CanMelt(into) = flag {
                return Some(into.clone());
            };
        }
        None
    }

    pub fn can_melt(&self) -> bool {
        self.melt().is_some()
    }

    pub fn mesh(&self) -> Handle<Mesh> {
        self.mesh.clone()
    }

    pub fn material(&self) -> Handle<StandardMaterial> {
        self.material.clone()
    }

    pub fn add_components(&self, entity: &mut EntityCommands) {
        use super::world::voxel_logic::*;
        for logic in self.components.iter() {
            match logic {
                BlockLogic::Extractor => entity.insert(Extractor),
                BlockLogic::Melter => entity.insert(Melter),
                BlockLogic::ScoreGive => entity.insert(ScoreGive),
                BlockLogic::Piston => entity.insert(Piston),
            };
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Reflect,
    Clone,
    strum_macros::EnumIter,
    strum_macros::EnumDiscriminants,
    Debug,
    Component,
    Eq,
    Default,
)]
#[strum_discriminants(derive(Hash))]
pub enum BlockType {
    #[default]
    Air,
    Stone,
    Coal,
    IronOre,
    IronBlock,
    BedRock,
    Score,
    Furnace,
    Drill(MapDirection),
    Sand,
    Glass,
    CobaltOre,
    CobaltBlock,
    CopperOre,
    CopperBlock,
    Sodium,
    Potassium,
    Magnesium,
    Piston(MapDirection),
    Voxel(VoxelBlock),
}

const VOXEL_DIVISION_FACTOR: usize = 16usize.pow(3);

#[derive(Serialize, Deserialize, Reflect, Clone, Debug, Component, PartialEq, Eq)]
pub struct VoxelBlock(#[reflect(ignore)] pub Arc<Array<BlockType, VOXEL_DIVISION_FACTOR>>);

impl Default for VoxelBlock {
    fn default() -> Self {
        Self(Arc::new(Array(array::from_fn(|_| BlockType::default()))))
    }
}

#[test]
fn hash_test() {
    use std::hash::Hasher;
    let mut hasher_two = std::hash::DefaultHasher::new();
    let mut hasher_one = std::hash::DefaultHasher::new();
    BlockType::Piston(MapDirection::Up).hash(&mut hasher_one);
    BlockType::Piston(MapDirection::Down).hash(&mut hasher_two);
    assert!(BlockType::Piston(MapDirection::Up) == BlockType::Piston(MapDirection::Down));
    assert_eq!(hasher_one.finish(), hasher_two.finish())
}

impl Hash for BlockType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        BlockTypeDiscriminants::from(self).hash(state)
    }
}

impl PartialEq for BlockType {
    fn eq(&self, other: &Self) -> bool {
        BlockTypeDiscriminants::from(self) == BlockTypeDiscriminants::from(other)
    }
}

impl BlockType {
    pub fn direction(&self) -> MapDirection {
        match self {
            BlockType::Piston(direction) => *direction,
            _ => MapDirection::Up,
        }
    }

    pub fn set_direction(&mut self, direction: MapDirection) {
        match self {
            BlockType::Piston(to) => {
                *to = direction;
            }
            _ => {}
        }
    }

    pub fn path(&self) -> &'static str {
        match self {
            BlockType::Air => "blocks/air.block",
            BlockType::Stone => "blocks/stone.block",
            BlockType::Coal => "blocks/coal.block",
            BlockType::IronOre => "blocks/ore_iron.block",
            BlockType::CobaltOre => "blocks/ore_cobalt.block",
            BlockType::CopperOre => "blocks/ore_copper.block",
            BlockType::IronBlock => "blocks/solid_iron.block",
            BlockType::CobaltBlock => "blocks/solid_cobalt.block",
            BlockType::CopperBlock => "blocks/solid_copper.block",
            BlockType::BedRock => "blocks/bedrock.block",
            BlockType::Score => "blocks/score.block",
            BlockType::Furnace => "blocks/furnace.block",
            BlockType::Drill(_) => "blocks/drill.block",
            BlockType::Sand => "blocks/sand.block",
            BlockType::Glass => "blocks/glass.block",
            BlockType::Sodium => "blocks/sodium.block",
            BlockType::Potassium => "blocks/potassium.block",
            BlockType::Magnesium => "blocks/magnesium.block",
            BlockType::Piston(_) => "blocks/piston.block",
            BlockType::Voxel(_) => "blocks/voxel.block",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            BlockType::Air => Color::srgba(0.0, 0.0, 0.0, 0.0), // Transparent
            BlockType::Stone => Color::srgba(0.5, 0.5, 0.5, 1.0), // Gray
            BlockType::Coal => Color::srgba(0.1, 0.1, 0.1, 1.0), // Black
            BlockType::IronOre => Color::srgba(0.8, 0.2, 0.2, 1.0), // Red
            BlockType::IronBlock => Color::srgba(0.7, 0.7, 0.7, 1.0), // Silver
            BlockType::BedRock => Color::srgba(0.3, 0.3, 0.3, 1.0), // Dark Gray
            BlockType::Score => Color::srgba(1.0, 0.84, 0.0, 1.0), // Gold
            BlockType::Furnace => Color::srgba(0.8, 0.4, 0.0, 1.0), // Orange-Red
            BlockType::Drill(_) => Color::srgba(0.5, 0.0, 0.5, 1.0), // Purple
            BlockType::Sand => Color::srgba(0.94, 0.87, 0.69, 1.0), // Yellow
            BlockType::Glass => Color::srgba(0.8, 1.0, 1.0, 1.0), // Cyan
            BlockType::CobaltOre => Color::srgba(0.0, 0.0, 0.8, 1.0), // Blue
            BlockType::CobaltBlock => Color::srgba(0.0, 0.0, 0.5, 1.0), // Navy
            BlockType::CopperOre => Color::srgba(0.0, 0.8, 0.0, 1.0), // Green
            BlockType::CopperBlock => Color::srgba(1.0, 0.65, 0.0, 1.0), // Orange
            BlockType::Sodium => Color::srgba(0.6, 0.8, 0.2, 1.0), // Yellow-Green
            BlockType::Potassium => Color::srgba(1.0, 0.0, 1.0, 1.0), // Magenta
            BlockType::Magnesium => Color::srgba(1.0, 0.75, 0.8, 1.0), // Pink
            BlockType::Piston(_) => Color::srgba(0.5, 0.5, 0.0, 1.0), // Olive
            BlockType::Voxel(_) => Color::srgba(1.0, 1.0, 1.0, 1.0), // White
        }
    }
}

#[derive(Serialize, Deserialize, Reflect, PartialEq, Eq)]
enum BlockFlags {
    NoMine,
    CanMelt(BlockType),
    Fuel,
}

struct BlockLoader {
    default_mesh: Handle<Mesh>,
}

impl FromWorld for BlockLoader {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        BlockLoader {
            default_mesh: meshes.add(Cuboid::from_length(1.)),
        }
    }
}

impl AssetLoader for BlockLoader {
    type Error = LoadError;
    type Asset = Block;
    type Settings = ();
    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async {
            let mut str = String::default();
            reader.read_to_string(&mut str).await?;
            println!("{}", str);
            let block = ron::from_str::<BlockAsset>(&str)?;

            let mesh = match &block.id {
                BlockType::Voxel(voxel_block) => {
                    let mesh = generate_voxel_mesh(voxel_block);
                    load_context.add_labeled_asset("mesh".to_string(), mesh.mesh)
                }
                _ => {
                    if let Some(path) = block.mesh {
                        load_context.load(path)
                    } else {
                        self.default_mesh.clone()
                    }
                }
            };

            Ok(Block {
                id: block.id,
                flags: block.flags,
                mesh,
                material: {
                    let material = StandardMaterial {
                        base_color: block.color,
                        base_color_texture: Some(load_context.load(block.texture)),
                        alpha_mode: AlphaMode::Mask(0.1),
                        unlit: true,
                        ..Default::default()
                    };
                    load_context.add_labeled_asset("material".to_string(), material)
                },
                color: block.color,
                solid: block.solid,
                components: block.components,
            })
        }
    }
    fn extensions(&self) -> &[&str] {
        &["block"]
    }
}

#[derive(thiserror::Error, Debug)]
enum LoadError {
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ron Error: {0}")]
    Ron(#[from] ron::de::SpannedError),
}

fn test_load(asset_server: Res<AssetServer>) {
    asset_server.load::<Block>("blocks/stone.block");
}
