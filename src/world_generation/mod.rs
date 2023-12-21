use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::dbg;
use core::panic;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::consts::PI;
use std::ops::Index;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod util;
mod chunk;
mod tile;

use util::*;
use chunk::*;
use tile::*;

const CHUNK_SIZE: usize = 16;
const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_HIGHT: usize = 4;
const CHUNK_VOLUME: usize = CHUNK_AREA * CHUNK_HIGHT;
const CHUNK_SPAWN_DISTANCE: i32 = 1;
const TILE_SIZE: f32 = 1.0;

pub struct WorldGenerationPlugin;
impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldFocusPoint { pos: Vec3::ZERO })
            .init_resource::<WorldMap>()
            .add_state::<TileLoadState>()
            .add_systems(OnEnter(TileLoadState::Loading), load_tiles)
            .add_systems(
                Update,
                check_tiles_loaded.run_if(in_state(TileLoadState::Loading)),
            )
            // .add_systems(OnEnter(TileLoadState::Finished), setup_tiles)
            .add_systems(
                Update,
                spawn_chunks.run_if(in_state(TileLoadState::Finished)),
            )
            .add_systems(Update, world_gizmo);
    }
}

fn world_gizmo(mut gizmos: Gizmos, world_map: Res<WorldMap>) {
    // let offset = Vec3::new(TILE_SIZE / 2., 0.0, TILE_SIZE / 2.0);
    for (_, chunk) in world_map.chunks.iter() {
        // draw tiles
        gizmos.ray(chunk.pos(), Vec3::Y * 100.0, Color::WHITE);
    }
}

fn spawn_chunks(
    focus: Res<WorldFocusPoint>,
    mut world_map: ResMut<WorldMap>,
    assets_gltf: Res<Assets<Gltf>>,
    tiles: Res<TileAssets>,
    mut cmds: Commands,
) {
    let mut chunks_to_spawn: Vec<ChunkId> = Vec::new();
    // for z in (-CHUNK_SPAWN_DISTANCE)..CHUNK_SPAWN_DISTANCE {
    //     for x in (-CHUNK_SPAWN_DISTANCE)..CHUNK_SPAWN_DISTANCE {
            // let id = ChunkId::from_position(focus.pos).x_offset(x).z_offset(z);
            let id = ChunkId::new(0,0);

            if !world_map.contains(&id) {
                chunks_to_spawn.push(id);
            }
        // }
    // }

    for id in &chunks_to_spawn {
        let ground_rules = AdjacencyRules {
            p_x: vec![Tile::Ground],
            n_x: vec![Tile::Ground, ],
            p_y: vec![],
            n_y: vec![],
            p_z: vec![Tile::Ground],
            n_z: vec![Tile::Ground],
        };

        // let cliff_low_rules = AdjacencyRules {
        //     p_x: vec![],
        //     n_x: vec![Tile::Ground],
        //     p_y: vec![],
        //     n_y: vec![],
        //     p_z: vec![Tile::CliffLow],
        //     n_z: vec![Tile::CliffLow],
        // };

        let chunk = ChunkBuilder::new(id.clone())
            .add_tile(Tile::Ground, ground_rules)
            // .add_tile(Tile::CliffLow, cliff_low_rules)
            // .add_tile(TileType::CliffLowCorner, cliff_low_corner_rules)
            .build();

        let chunk = Chunk::new(id.clone());
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HIGHT {
                    let Some(tile) = &chunk.get_tile(x, y, z) else {continue;};
                    let gltf = assets_gltf.get(&tiles[&tile]).unwrap();
                    let transform = Transform {
                        translation: chunk.pos() + Vec3::new(x as f32, y as f32, z as f32),
                        ..default()
                    };
                    let _entity = cmds.spawn((SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform,
                        ..default()
                    },
                    ));
                }
            }
        }
        info!("spawned chunk with {:?}", &id);
        world_map.add_chunk(chunk);
    }
}

#[derive(Resource)]
pub struct WorldFocusPoint {
    pub pos: Vec3,
}

#[derive(Resource, Default)]
struct WorldMap {
    chunks: HashMap<ChunkId, Chunk>,
}

impl WorldMap {
    fn contains(&self, id: &ChunkId) -> bool {
        self.chunks.contains_key(id)
    }

    fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.id(), chunk);
    }
}

