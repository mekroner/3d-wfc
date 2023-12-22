use bevy::gltf::Gltf;
use bevy::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

mod chunk;
mod tile;
mod util;
mod prototyper;

use chunk::*;
use tile::*;

const CHUNK_SIZE: usize = 24;
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
            // .add_systems(Update, world_gizmo)
            .add_systems(Update, grid_gizmo);
    }
}

fn world_gizmo(mut gizmos: Gizmos, world_map: Res<WorldMap>) {
    for (_, chunk) in world_map.chunks.iter() {
        // draw tiles
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                gizmos.rect(
                    chunk.pos() + Vec3::new(x as f32, 0.0, z as f32),
                    Quat::from_rotation_x(PI / 2.0),
                    Vec2::splat(TILE_SIZE as f32),
                    Color::YELLOW,
                )
            }
        }
        gizmos.ray(chunk.pos(), Vec3::Y * 100.0, Color::WHITE);
    }
}

fn grid_gizmo(mut gizmos: Gizmos) {
    for z in -50..50 {
        for x in -50..50 {
            gizmos.rect(
                Vec3::new(x as f32, 0.0, z as f32),
                Quat::from_rotation_x(PI / 2.0),
                Vec2::splat(TILE_SIZE as f32),
                Color::YELLOW,
            )
        }
    }

    // compas

    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::BLUE);
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::GREEN);
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
    let id = ChunkId::new(0, 0);

    if !world_map.contains(&id) {
        chunks_to_spawn.push(id);
    }
    // }
    // }

    for id in &chunks_to_spawn {
        let air_rules = AdjacencyRules {
            p_x: vec![Tile::Air],
            n_x: vec![Tile::Air],
            p_y: vec![Tile::Air],
            n_y: vec![Tile::Air, Tile::Ground, Tile::CliffUpper],
            p_z: vec![Tile::Air],
            n_z: vec![Tile::Air],
        };

        let solid_rules = AdjacencyRules {
            p_x: vec![Tile::Solid],
            n_x: vec![Tile::Solid],
            p_y: vec![Tile::Solid, Tile::Ground, Tile::CliffLow],
            n_y: vec![Tile::Solid],
            p_z: vec![Tile::Solid],
            n_z: vec![Tile::Solid],
        };

        let ground_rules = AdjacencyRules {
            p_x: vec![Tile::Ground],
            n_x: vec![Tile::Ground],
            p_y: vec![Tile::Air],
            n_y: vec![Tile::Solid],
            p_z: vec![Tile::Ground],
            n_z: vec![Tile::Ground],
        };

        let cliff_low_rules = AdjacencyRules {
            p_x: vec![Tile::Solid],
            n_x: vec![Tile::Ground],
            p_y: vec![Tile::CliffUpper],
            n_y: vec![Tile::Solid],
            p_z: vec![Tile::CliffLow],
            n_z: vec![Tile::CliffLow],
        };

        let cliff_upper_rules = AdjacencyRules {
            p_x: vec![Tile::Ground],
            n_x: vec![Tile::Ground],
            p_y: vec![Tile::Air],
            n_y: vec![Tile::Solid],
            p_z: vec![Tile::CliffUpper],
            n_z: vec![Tile::CliffUpper],
        };

        let chunk = ChunkBuilder::new(id.clone())
            .add_tile(Tile::Air, air_rules)
            .add_tile(Tile::Solid, solid_rules)
            .add_tile(Tile::Ground, ground_rules)
            .add_tile(Tile::CliffLow, cliff_low_rules)
            .add_tile(Tile::CliffUpper, cliff_upper_rules)
            // .add_tile(TileType::CliffLowCorner, cliff_low_corner_rules)
            .build();

        // let chunk = Chunk::new(id.clone());
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HIGHT {
                    let tile = &chunk.get_tile(x, y, z);
                    let Some(handle) = tiles.get(*tile) else {
                        continue;
                    };
                    let gltf = assets_gltf.get(handle).expect("Asset should be loaded");
                    let transform = Transform {
                        translation: chunk.pos() + Vec3::new(x as f32, y as f32, z as f32),
                        ..default()
                    };
                    let _entity = cmds.spawn((SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform,
                        ..default()
                    },));
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
