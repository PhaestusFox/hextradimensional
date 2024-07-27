use bevy::{prelude::*, utils::HashMap};

use crate::screen::voxel_world::BlockType;

#[derive(Resource)]
pub struct MultiBlocks {
    recipes: HashMap<MultiBlockType, MultiBlockRecipe>,
}

impl FromWorld for MultiBlocks {
    fn from_world(world: &mut World) -> Self {
        let mut map = HashMap::new();
        map.insert(MultiBlockType::Furnace, MultiBlockRecipe {
            size: IVec3::new(3, 4, 3),
            rules: vec![
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Empty, MultiBlockRule::Solid,
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Empty, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid,
                MultiBlockRule::Solid, MultiBlockRule::Solid, MultiBlockRule::Solid, 
            ],
            output_block: BlockType::Furnace,
            output_offset: IVec3::new(1, 2, 1),
        });
        MultiBlocks {
            recipes: map,
        }
    }
}

#[derive(Component)]
struct MultiBlock {
    size: IVec3,
    origin: IVec3,
    building_blocks: Vec<BlockType>,
}

enum MultiBlockRule {
    Solid,
    Empty,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum MultiBlockType {
    Furnace,
}

struct MultiBlockRecipe {
    size: IVec3,
    rules: Vec<MultiBlockRule>,
    output_block: BlockType,
    output_offset: IVec3,
}