// use bevy::gltf::{Gltf, GltfMesh, GltfPrimitive};
// use bevy::prelude::*;
// use bevy::render::mesh::{VertexAttributeDescriptor, VertexAttributeValues};
// use bevy::utils::HashMap;
// use strum::IntoEnumIterator;
// use strum_macros::EnumIter;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
// pub enum Tile {
//     Dbg,

//     Air,
//     Solid,
//     Ground,
//     CliffLow,
//     CliffLowCorner,
//     CliffUpper,
//     CliffUpperCorner,
// }

// #[derive(States, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
// pub enum TileLoadState {
//     #[default]
//     Loading,
//     Finished,
// }

// #[derive(Resource)]
// pub struct TileAssets(Vec<Handle<Gltf>>);

// impl TileAssets {
//     pub fn get(&self, index: Tile) -> Option<&Handle<Gltf>> {
//         match index {
//             Tile::Dbg => None,
//             Tile::Air => None,
//             Tile::Solid => None,
//             Tile::Ground => Some(&self.0[0]),
//             Tile::CliffLow => Some(&self.0[1]),
//             Tile::CliffLowCorner => Some(&self.0[2]),
//             Tile::CliffUpper => Some(&self.0[3]),
//             Tile::CliffUpperCorner => todo!(),
//         }
//     }
// }

// pub fn check_tiles_loaded(
//     ass: Res<AssetServer>,
//     tiles: Res<TileAssets>,
//     mut next_state: ResMut<NextState<TileLoadState>>,
// ) {
//     for tile in &tiles.0 {
//         if ass.get_load_state(tile).is_none() {
//             debug!("not all tiles loaded!");
//             return;
//         }
//     }
//     debug!("Tiles loaded!");
//     next_state.set(TileLoadState::Finished);
// }

// pub fn load_tiles(mut cmds: Commands, ass: Res<AssetServer>) {
//     let ground = ass.load("models/terrain/ground.glb");
//     let cliff_low = ass.load("models/terrain/cliff_low.glb");
//     let cliff_low_corner = ass.load("models/terrain/cliff_low_corner.glb");
//     let cliff_upper = ass.load("models/terrain/cliff_upper.glb");
//     let assets = vec![ground, cliff_low, cliff_low_corner, cliff_upper];
//     cmds.insert_resource(TileAssets(assets));
// }
