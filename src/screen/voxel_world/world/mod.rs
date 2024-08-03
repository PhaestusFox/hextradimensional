use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use crate::{
    game::HexSelect,
    screen::{hex_vox_util::HexId, Screen},
};
use bevy::{
    asset::{
        io::{AssetReader, AssetReaderError, Reader},
        AssetLoader,
    },
    prelude::*,
    tasks::futures_lite::{AsyncRead, AsyncSeek},
};
use bevy_pkv::{GetError, PkvStore};
use block_breaking::block_breaking_plugin;
use rand::SeedableRng;
use serde_big_array::Array;

use super::{
    voxel_util::WorldType,
    voxels::{Block, BlockType, Blocks},
};

pub mod voxel_logic;

#[derive(Component, Clone, Copy, Debug)]
pub struct VoxelId(pub IVec3);

impl VoxelId {
    pub fn in_chunk(&self) -> bool {
        !(self.x() < 0
            || self.x() >= CHUNK_SIZE as i32
            || self.y() < 0
            || self.y() >= CHUNK_SIZE as i32
            || self.z() < 0
            || self.z() >= CHUNK_SIZE as i32)
    }

    pub fn x(&self) -> i32 {
        self.0.x
    }
    pub fn y(&self) -> i32 {
        self.0.y
    }
    pub fn z(&self) -> i32 {
        self.0.z
    }
}

impl std::ops::Add for VoxelId {
    type Output = VoxelId;
    fn add(self, rhs: Self) -> Self::Output {
        VoxelId(IVec3::new(
            self.0.x + rhs.0.x,
            self.0.y + rhs.0.y,
            self.0.z + rhs.0.z,
        ))
    }
}

pub mod block_breaking;
pub mod cheats;
pub mod multi_block;

pub const CHUNK_SIZE: usize = 16;
pub const BLOCKS_IN_CHUNK: usize = CHUNK_SIZE.pow(3);

#[derive(Asset, Reflect, serde::Serialize, serde::Deserialize)]
pub struct VoxelChunk(#[reflect(ignore)] pub Array<BlockType, BLOCKS_IN_CHUNK>);

impl VoxelChunk {
    pub fn new() -> VoxelChunk {
        VoxelChunk(Array(std::array::from_fn(|_| BlockType::default())))
    }

    fn from_hex(hex: &WorldType, rng: &mut impl rand::Rng) -> VoxelChunk {
        let mut chunk = VoxelChunk::new();
        if hex == &WorldType::Empty {
            return chunk;
        }
        for x in 0..CHUNK_SIZE as i32 {
            for y in 0..CHUNK_SIZE as i32 {
                for z in 0..CHUNK_SIZE as i32 {
                    let pos = IVec3::new(x, y, z);
                    chunk.set(pos, hex.sample(rng, pos));
                }
            }
        }
        chunk
    }

    pub fn set(&mut self, pos: IVec3, block: BlockType) -> BlockType {
        if pos.x >= CHUNK_SIZE as i32
            || pos.x < 0
            || pos.y >= CHUNK_SIZE as i32
            || pos.y < 0
            || pos.z >= CHUNK_SIZE as i32
            || pos.z < 0
        {
            return BlockType::Air;
        }
        let index =
            pos.x as usize + pos.z as usize * CHUNK_SIZE + pos.y as usize * CHUNK_SIZE.pow(2);
        let old = self.0[index].clone();
        self.0[index] = block;
        old
    }

    pub fn get(&self, pos: IVec3) -> BlockType {
        if pos.y == -1 {
            return BlockType::BedRock;
        }
        if pos.z == -1 {
            return BlockType::Air;
        }
        if pos.x == -1 {
            return BlockType::Air;
        }
        let index =
            pos.x as usize + pos.z as usize * CHUNK_SIZE + pos.y as usize * CHUNK_SIZE.pow(2);
        if index >= CHUNK_SIZE.pow(3) {
            return BlockType::Air;
        }
        self.0[index].clone()
    }
}

pub(crate) fn voxel_world(app: &mut App) {
    block_breaking_plugin(app);
    app.init_resource::<VoxelStore>();
    app.init_asset::<VoxelChunk>()
        .init_resource::<multi_block::MultiBlocks>();
    app.init_asset_loader::<VoxelChunkLoader>();
    app.add_systems(
        FixedUpdate,
        multi_block::check_for_multi_blocks.run_if(in_state(Screen::VoxelWorld)),
    );
    app.add_systems(
        Update,
        fill_world_after_load.run_if(in_state(Screen::VoxelWorld)),
    );
    app.add_systems(
        OnEnter(Screen::VoxelWorld),
        (load_chunk, open_loaded_world).run_if(resource_changed::<HexSelect>),
    );
    app.add_plugins(voxel_logic::VoxelLogic);
    #[cfg(feature = "dev")]
    app.add_systems(Update, cheats::give_player_block);
}

fn load_chunk(mut hex: ResMut<HexSelect>, asset_server: Res<AssetServer>) {
    let world = hex.world;
    let id = hex.hex_id;
    hex.chunk = asset_server.load_with_settings(format!("chunk://{}", id), move |w| *w = world)
}

fn open_loaded_world(
    mut commands: Commands,
    change: Res<HexSelect>,
    chunks: Res<Assets<VoxelChunk>>,
    blocks: Res<Blocks>,
    data: Res<Assets<Block>>,
) {
    if let Some(chunk) = chunks.get(change.chunk.id()) {
        fill_world(chunk, &mut commands, &blocks, &data);
    }
}

fn fill_world(chunk: &VoxelChunk, commands: &mut Commands, blocks: &Blocks, data: &Assets<Block>) {
    commands
        .spawn((
            SpatialBundle::default(),
            Name::new("Chunk"),
            StateScoped(crate::screen::Screen::VoxelWorld),
        ))
        .with_children(|commands| {
            for x in 0..CHUNK_SIZE as i32 {
                for y in -1..CHUNK_SIZE as i32 {
                    for z in 0..CHUNK_SIZE as i32 {
                        let id = IVec3::new(x, y, z);
                        let block = chunk.get(id);
                        spawn_voxel(block, blocks, id, commands, data);
                    }
                }
            }
        });
}

fn fill_world_after_load(
    mut commands: Commands,
    mut event: EventReader<AssetEvent<VoxelChunk>>,
    chunks: Res<Assets<VoxelChunk>>,
    blocks: Res<Blocks>,
    voxels: Res<Assets<super::voxels::Block>>,
) {
    for event in event.read() {
        match event {
            AssetEvent::Added { id } => {
                let chunk = chunks.get(*id).expect("just loaded");
                fill_world(chunk, &mut commands, &blocks, &voxels);
            }
            AssetEvent::Modified { id: _ } => {}
            _ => {}
        }
    }
}

fn spawn_voxel(
    block: BlockType,
    voxels: &Blocks,
    offset: IVec3,
    commands: &mut ChildBuilder,
    voxel_data: &Assets<Block>,
) {
    if block == BlockType::Air {
        return;
    };
    let direction = block.direction();
    let data = voxels.get(block);
    let data = voxel_data.get(data.id()).expect("all blocks loaded");
    let mut entity = commands.spawn((
        Name::new("Voxel Block"),
        VoxelId(offset),
        PbrBundle {
            mesh: data.mesh(),
            material: data.material(),
            transform: Transform::from_translation(offset.as_vec3())
                .with_rotation(direction.to_rotation()),
            ..Default::default()
        },
    ));
    data.add_components(&mut entity);
    if data.is_solid() {
        entity.insert(bevy_rapier3d::prelude::Collider::cuboid(0.5, 0.5, 0.5));
    }
}

struct VoxelChunkLoader(VoxelStore);

#[derive(Resource, Clone)]
pub struct VoxelStore(Arc<std::sync::RwLock<PkvStore>>);

impl VoxelStore {
    pub fn write(&self) -> Option<std::sync::RwLockWriteGuard<'_, PkvStore>> {
        self.0.write().ok()
    }
}

impl FromWorld for VoxelStore {
    fn from_world(_world: &mut World) -> Self {
        VoxelStore(Arc::new(std::sync::RwLock::new(PkvStore::new(
            "Bevy Jam 5",
            "Hextradimensional",
        ))))
    }
}

impl FromWorld for VoxelChunkLoader {
    fn from_world(world: &mut World) -> Self {
        VoxelChunkLoader(world.resource::<VoxelStore>().clone())
    }
}

impl AssetLoader for VoxelChunkLoader {
    type Asset = VoxelChunk;

    type Settings = WorldType;

    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        _reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async {
            let Some(path) = load_context.path().to_str() else {
                return Err(std::io::Error::new(
                    ErrorKind::InvalidInput,
                    "path could not be converted to str",
                ));
            };
            let Ok(lock) = self.0 .0.read() else {
                return Err(std::io::Error::new(
                    ErrorKind::PermissionDenied,
                    "Rwlock failed to read",
                ));
            };
            match lock.get::<VoxelChunk>(path) {
                Ok(chunk) => Ok(chunk),
                Err(GetError::NotFound) => {
                    let file = load_context
                        .path()
                        .file_name()
                        .ok_or(Error::new(ErrorKind::NotFound, "failed get file name"))?
                        .to_string_lossy();
                    let hex = file.trim().parse::<HexId>().map_err(|e| {
                        println!("{e}");
                        std::io::Error::new(ErrorKind::NotFound, "failed to parse id")
                    })?;
                    let mut rng = rand::rngs::StdRng::seed_from_u64(
                        ((hex.q() as u64) << 32) | hex.r() as u64,
                    );
                    let chunk = VoxelChunk::from_hex(settings, &mut rng);
                    Ok(chunk)
                }
                Err(e) => {
                    error!("{}", e);
                    Err(std::io::Error::new(ErrorKind::Unsupported, "pkv error"))
                }
            }
        }
    }
}

pub struct ChunkReader;

struct NotFoundReader;
impl AsyncSeek for NotFoundReader {
    fn poll_seek(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        _: std::io::SeekFrom,
    ) -> std::task::Poll<std::io::Result<u64>> {
        std::task::Poll::Ready(Ok(0))
    }
}

impl AsyncRead for NotFoundReader {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        _buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        std::task::Poll::Ready(Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Not Found",
        )))
    }
}

impl AssetReader for ChunkReader {
    async fn read<'a>(
        &'a self,
        _path: &'a std::path::Path,
    ) -> Result<Box<Reader<'a>>, AssetReaderError> {
        Ok(Box::new(NotFoundReader))
    }

    fn read_meta<'a>(
        &'a self,
        _: &'a std::path::Path,
    ) -> impl bevy::utils::ConditionalSendFuture<
        Output = Result<Box<bevy::asset::io::Reader<'a>>, bevy::asset::io::AssetReaderError>,
    > {
        async { Err(bevy::asset::io::AssetReaderError::HttpError(404)) }
    }

    fn read_directory<'a>(
        &'a self,
        _: &'a std::path::Path,
    ) -> impl bevy::utils::ConditionalSendFuture<
        Output = Result<Box<bevy::asset::io::PathStream>, bevy::asset::io::AssetReaderError>,
    > {
        async { Err(bevy::asset::io::AssetReaderError::HttpError(404)) }
    }

    fn is_directory<'a>(
        &'a self,
        _: &'a std::path::Path,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<bool, bevy::asset::io::AssetReaderError>>
    {
        async { Err(bevy::asset::io::AssetReaderError::HttpError(404)) }
    }
}
