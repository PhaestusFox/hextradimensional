use bevy::{
    asset::{AssetLoader, AssetPath, AsyncReadExt},
    ecs::system::EntityCommands,
    prelude::*,
    reflect::TypeRegistry,
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

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
            map.insert(block, asset_server.load(block.path()));
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
}

impl Block {
    pub fn color(&self) -> Color {
        self.color
    }

    pub fn can_mine(&self) -> bool {
        !self.flags.contains(&BlockFlags::NoMine)
    }

    pub fn get_type(&self) -> BlockType {
        self.id
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
                return Some(*into);
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
            };
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    Reflect,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    strum_macros::EnumIter,
    Debug,
    Component,
)]
pub enum BlockType {
    Air,
    Stone,
    Coal,
    IronOre,
    IronBlock,
    BedRock,
    Score,
    Furnace,
    Drill,
    Sand,
    Glass,
    CobaltOre,
    CobaltBlock,
    CopperOre,
    CopperBlock,
    Sodium,
    Potassium,
    Magnesium,
}

impl BlockType {
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
            BlockType::Drill => "blocks/drill.block",
            BlockType::Sand => "blocks/sand.block",
            BlockType::Glass => "blocks/glass.block",
            BlockType::Sodium => "blocks/sodium.block",
            BlockType::Potassium => "blocks/potassium.block",
            BlockType::Magnesium => "blocks/magnesium.block",
        }
    }
}

#[derive(Serialize, Deserialize, Reflect, PartialEq, Eq)]
enum BlockFlags {
    NoMine,
    CanMelt(BlockType),
    Fuel,
}

#[test]
fn output() {
    let block = BlockAsset {
        id: BlockType::IronBlock,
        flags: vec![],
        mesh: None,
        texture: "images/voxels/refined_iron.png".to_string(),
        color: LinearRgba::gray(0.5).into(),
        solid: true,
        components: vec![],
    };

    //Some("images/multi_blocks/furnace.glb#Mesh0/Primitive0")
    // }
    // BlockType::Complex(ComplexBlock::Drill) => {
    //     Some("images/multi_blocks/drill.glb#Mesh1/Primitive0")

    println!(
        "{}",
        ron::ser::to_string_pretty(&block, ron::ser::PrettyConfig::default()).unwrap()
    );
}

struct BlockLoader {
    default_mesh: Handle<Mesh>,
    type_registry: AppTypeRegistry,
}

impl FromWorld for BlockLoader {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        BlockLoader {
            default_mesh: meshes.add(Cuboid::from_length(1.)),
            type_registry: world.resource::<AppTypeRegistry>().clone(),
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
            let block = ron::from_str::<BlockAsset>(&str)?;

            Ok(Block {
                id: block.id,
                flags: block.flags,
                mesh: if let Some(path) = block.mesh {
                    load_context.load(path)
                } else {
                    self.default_mesh.clone()
                },
                material: {
                    let material = StandardMaterial {
                        base_color: block.color,
                        base_color_texture: Some(load_context.load(block.texture)),
                        alpha_mode: AlphaMode::Mask(0.1),
                        unlit: true,
                        ..Default::default()
                    };
                    load_context.add_labeled_asset(String::from("material"), material)
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
