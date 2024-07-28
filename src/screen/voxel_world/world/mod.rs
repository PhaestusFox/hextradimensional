use std::io::{Error, ErrorKind};

use crate::{
    game::HexSelect,
    screen::{hex_vox_util::HexId, Screen},
};
use bevy::{
    asset::{
        io::{AssetReader, AssetReaderError, ErasedAssetReader, Reader},
        AssetLoader, AsyncReadExt,
    },
    prelude::*,
    tasks::futures_lite::{AsyncRead, AsyncSeek},
};
use block_breaking::block_breaking_plugin;
use rand::SeedableRng;

use super::{
    voxel_util::{Blocks, WorldType},
    BasicBlock, BlockType, ComplexBlock,
};

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

#[derive(Asset, Reflect)]
pub struct VoxelChunk([BlockType; CHUNK_SIZE.pow(3)]);

impl VoxelChunk {
    fn new() -> VoxelChunk {
        VoxelChunk(std::array::from_fn(|_| BlockType::Basic(BasicBlock::Air)))
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
            return BlockType::Basic(BasicBlock::Air);
        }
        let index =
            pos.x as usize + pos.z as usize * CHUNK_SIZE + pos.y as usize * CHUNK_SIZE.pow(2);
        let old = self.0[index].clone();
        self.0[index] = block;
        old
    }

    pub fn get(&self, pos: IVec3) -> BlockType {
        if pos.y == -1 {
            return BlockType::Complex(ComplexBlock::BedRock);
        }
        let index =
            pos.x as usize + pos.z as usize * CHUNK_SIZE + pos.y as usize * CHUNK_SIZE.pow(2);
        if index >= CHUNK_SIZE.pow(3) {
            return BlockType::Basic(BasicBlock::Air);
        }
        self.0[index].clone()
    }
}

pub(crate) fn voxel_world(app: &mut App) {
    block_breaking_plugin(app);
    app.init_asset::<VoxelChunk>()
        .init_resource::<multi_block::MultiBlocks>();
    app.init_asset_loader::<VoxelChunkLoader>();
    app.add_systems(FixedUpdate, multi_block::check_for_multi_blocks);
    app.add_systems(
        Update,
        fill_world_after_load.run_if(in_state(Screen::VoxelWorld)),
    );
    app.add_systems(
        OnEnter(Screen::VoxelWorld),
        (load_chunk, open_loaded_world).run_if(resource_changed::<HexSelect>),
    );
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
) {
    if let Some(chunk) = chunks.get(change.chunk.id()) {
        fill_world(chunk, &mut commands, &blocks);
    }
}

fn fill_world(chunk: &VoxelChunk, commands: &mut Commands, blocks: &Blocks) {
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
                        let block = &chunk.get(id);
                        let solidity = block.is_solid();
                        let mut entity = commands.spawn((
                            Name::new("Voxel Block"),
                            VoxelId(id),
                            PbrBundle {
                                mesh: blocks.mesh(block),
                                material: blocks.texture(block),
                                transform: Transform::from_translation(Vec3::new(
                                    x as f32, y as f32, z as f32,
                                )),
                                ..Default::default()
                            },
                        ));
                        if solidity {
                            entity.insert(bevy_rapier3d::prelude::Collider::cuboid(0.5, 0.5, 0.5));
                        }
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
) {
    for event in event.read() {
        match event {
            AssetEvent::Added { id } => {
                let chunk = chunks.get(id.clone()).expect("just loaded");
                fill_world(chunk, &mut commands, &blocks);
            }
            AssetEvent::Modified { id } => {
                println!("Update changed to world");
            }
            _ => {}
        }
    }
}

#[derive(Default)]
struct VoxelChunkLoader;

impl AssetLoader for VoxelChunkLoader {
    type Asset = VoxelChunk;

    type Settings = WorldType;

    type Error = std::io::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        async {
            let mut data = Vec::new();
            match reader.read(&mut data).await {
                Ok(_) => Err(std::io::Error::new(
                    ErrorKind::Unsupported,
                    "Loading not implemented get out of (1-1)",
                )),
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
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
                    } else {
                        Err(e)
                    }
                }
            }
        }
    }
}

pub struct ChunkReader(pub Box<dyn ErasedAssetReader>);

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
        path: &'a std::path::Path,
    ) -> Result<Box<Reader<'a>>, AssetReaderError> {
        match self.0.read(path).await {
            Ok(reader) => Ok(reader),
            Err(e) => match e {
                AssetReaderError::NotFound(_) => Ok(Box::new(NotFoundReader)),
                e => Err(e),
            },
        }
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
