use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use std::f32::consts::PI;

pub mod chunk;
pub mod dir;
pub mod prototype;
pub mod tile;
pub mod util;

use chunk::*;
use prototype::*;
use tile::*;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_HIGHT: usize = 8;
pub const CHUNK_VOLUME: usize = CHUNK_AREA * CHUNK_HIGHT;
pub const CHUNK_SPAWN_DISTANCE: i32 = 1;
pub const TILE_SIZE: f32 = 1.0;

pub struct WorldGenerationPlugin;
impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        use PrototypesLoadState as PLS;
        app.insert_resource(WorldFocusPoint { pos: Vec3::ZERO })
            .init_resource::<WorldMap>()
            .add_state::<PLS>()
            .add_systems(OnEnter(PLS::Loading), load_prototypes)
            .add_systems(
                Update,
                check_prototypes_loaded.run_if(in_state(PLS::Loading)),
            )
            .insert_resource(Tiles(HashMap::new()))
            .insert_resource(AdjRuleSet(HashMap::new()))
            .add_systems(OnEnter(PLS::Finished), generate_tiles_and_rules)
            .add_systems(Update, spawn_chunks.run_if(in_state(PLS::Finished)))
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

pub fn grid_gizmo(mut gizmos: Gizmos) {
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

    // compass

    gizmos.ray(Vec3::ZERO, Vec3::Y, Color::BLUE);
    gizmos.ray(Vec3::ZERO, Vec3::X, Color::RED);
    gizmos.ray(Vec3::ZERO, Vec3::Z, Color::GREEN);
}

fn spawn_chunks(
    mut world_map: ResMut<WorldMap>,
    assets_gltf: Res<Assets<Gltf>>,
    tiles: Res<Tiles>,
    rule_set: Res<AdjRuleSet>,
    mut cmds: Commands,
    focus: Res<WorldFocusPoint>,
) {
    let mut chunks_to_spawn: Vec<ChunkId> = Vec::new();
    // for z in (-CHUNK_SPAWN_DISTANCE)..CHUNK_SPAWN_DISTANCE {
    //     for x in (-CHUNK_SPAWN_DISTANCE)..CHUNK_SPAWN_DISTANCE {
    //         let id = ChunkId::from_position(focus.pos).x_offset(x).z_offset(z);
    //         if !world_map.contains(&id) {
    //             chunks_to_spawn.push(id);
    //         }
    //     }
    // }
    let id = ChunkId::new(0, 0);
    if !world_map.contains(&id) {
        chunks_to_spawn.push(id);
    }

    for id in &chunks_to_spawn {
        // let chunk = Chunk::new(id.clone(), Some(TileID(0)));
        let chunk = ChunkBuilder::new(id.clone())
            .add_rule_set(rule_set.clone())
            .build(&tiles);
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HIGHT {
                    let Some(tile_id) = &chunk.get_tile(x, y, z) else {
                        continue;
                    };
                    let Some(tile) = tiles.0.get(tile_id) else {
                        continue;
                    };
                    let Some(handle) = &tile.asset_handle else {
                        continue;
                    };
                    let gltf = assets_gltf.get(handle).expect("Asset should be loaded");
                    let transform = Transform {
                        translation: chunk.pos() + Vec3::new(x as f32, y as f32, z as f32),
                        rotation: tile.y_rotation.to_quat(),
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
pub struct WorldMap {
    chunks: HashMap<ChunkId, Chunk>,
}

impl WorldMap {
    pub fn contains(&self, id: &ChunkId) -> bool {
        self.chunks.contains_key(id)
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.insert(chunk.id(), chunk);
    }
}
