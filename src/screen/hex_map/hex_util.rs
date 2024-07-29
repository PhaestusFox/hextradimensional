use bevy::prelude::*;

pub struct HexPlugin;

const SEED: [u8; 32] = [
    0b01000010, 0b01100101, 0b01110110, 0b01111001, 0b01001010, 0b01100001, 0b01101101, 0b00110101,
    0b01101000, 0b01100101, 0b01111000, 0b01110100, 0b01110010, 0b01100001, 0b01100100, 0b01101001,
    0b01101101, 0b01100101, 0b01101110, 0b01110011, 0b01101001, 0b01101111, 0b01101110, 0b01100001,
    0b01101100, 0, 0, 0, 0, 0, 0, 0,
];

use rand::{seq::IteratorRandom, Rng, SeedableRng};
use strum::IntoEnumIterator;
// ! Fix test module
use crate::{
    game::{save::Seed, HexSelect},
    screen::{
        hex_map::{
            bundle::HexCellBundle,
            cells::{self, CellIcons},
            cursor,
        },
        hex_vox_util::{HexId, MapDirection, HEX_SIZE},
        voxel_world::voxel_util::WorldType,
        Screen,
    },
};

#[derive(Component, Reflect)]
pub struct HexCellContainer;

pub fn spawn_hex_grid(
    mut commands: Commands,
    icons: Res<CellIcons>,
    container: Query<Entity, With<HexCellContainer>>,
    seed: Res<Seed>,
) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed.0);
    let container_entity = if container.is_empty() {
        commands
            .spawn((
                Name::new("Hex Cell Container"),
                HexCellContainer,
                VisibilityBundle::default(),
                GlobalTransform::default(),
            ))
            .id()
    } else {
        container.single()
    };

    commands.entity(container_entity).with_children(|parent| {
        for hex_coord in cells::SpiralIter::new(10) {
            let hex_type = if rng.gen_bool(0.3) {
                WorldType::iter().choose(&mut rng).expect("Iter not Empty")
            } else {
                WorldType::Empty
            };

            // Get the base position from HexId
            let mut position = hex_coord.xyz();
            position.z = -10.0;

            parent.spawn((
                Name::new("Hex Cell"),
                StateScoped(Screen::HexMap),
                hex_type,
                HexCellBundle {
                    id: hex_coord,
                    transform: Transform::from_translation(position),
                    global_transform: GlobalTransform::from_translation(position),
                    texture: icons.get(hex_type),
                    ..Default::default()
                },
            ));
        }
    });
}

pub fn go_to_voxel(
    input: Res<ButtonInput<KeyCode>>,
    cursor: Query<(&HexId, &MapDirection), With<cursor::Cursor>>,
    hexes: Query<(&HexId, &WorldType)>,
    mut hex_select: ResMut<HexSelect>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        let cursor = cursor.single();
        let mut hex_type = WorldType::Empty;
        for (id, hex) in &hexes {
            if id == cursor.0 {
                hex_type = *hex;
                break;
            }
        }
        let a = *cursor.1;

        *hex_select = HexSelect {
            hex_id: *cursor.0,
            direction: *cursor.1,
            world: hex_type.into(),
            chunk: Handle::default(),
        };
        // ! Fix type later
        //hex_type: hex_type as u8,
        next_screen.set(Screen::VoxelWorld);
    }
}
