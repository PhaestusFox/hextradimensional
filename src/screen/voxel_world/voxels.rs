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
    Serialize, Deserialize, Reflect, Clone, Copy, PartialEq, Eq, Hash, strum_macros::EnumIter,
)]
pub enum BlockType {
    Air,
    Stone,
    Coal,
    OreIron,
}

impl BlockType {
    pub fn path(&self) -> &'static str {
        match self {
            BlockType::Air => "blocks/air.block",
            BlockType::Stone => "blocks/stone.block",
            BlockType::Coal => "blocks/coal.block",
            BlockType::OreIron => "blocks/ore_iron.block",
        }
    }
}

#[derive(Serialize, Deserialize, Reflect, PartialEq, Eq)]
enum BlockFlags {
    CanMelt(BlockType),
    Fuel,
}

#[test]
fn output() {
    let block = BlockAsset {
        id: BlockType::Stone,
        flags: Vec::new(),
        mesh: None,
        texture: "images/voxels/stone.png".to_string(),
        color: LinearRgba::gray(0.75).into(),
        solid: true,
        components: Vec::new(),
    };

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
