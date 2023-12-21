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

pub mod tile;

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
        self.chunks.insert(chunk.id.clone(), chunk);
    }
}

#[derive(Default, Hash, Eq, PartialEq, Debug, Clone)]
struct ChunkId(IVec2);

impl ChunkId {
    pub fn new(x: i32, z: i32) -> Self {
        Self(IVec2::new(x, z))
    }

    fn x(&self) -> i32 {
        self.0.x
    }

    fn z(&self) -> i32 {
        self.0.y
    }

    fn from_position(pos: Vec3) -> Self {
        let x = (pos.x / CHUNK_SIZE as f32).floor() as i32;
        let z = (pos.z / CHUNK_SIZE as f32).floor() as i32;
        Self::new(x, z)
    }

    fn x_offset(mut self, offset: i32) -> Self {
        self.0.x += offset;
        self
    }
    fn z_offset(mut self, offset: i32) -> Self {
        self.0.y += offset;
        self
    }
}

struct Chunk {
    id: ChunkId,
    tiles: Vec<TileType>,
}

impl Chunk {
    fn new(id: ChunkId) -> Self {
        let mut tiles = vec![TileType::Air; CHUNK_VOLUME];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                tiles[get_index(x, 0, z)] = TileType::Ground;
            }
        }
        Self { id, tiles }
    }

    fn get_tile(&self, x: usize, y: usize, z: usize) -> &TileType {
        &self.tiles[y * CHUNK_AREA + x * CHUNK_SIZE + z]
    }

    fn pos(&self) -> Vec3 {
        let x = self.id.x() as f32;
        let z = self.id.z() as f32;
        let size = CHUNK_SIZE as f32;
        Vec3::new(x, 0.0, z) * size
    }
}

#[inline]
fn get_index(x: usize, y: usize, z: usize) -> usize {
    y * CHUNK_AREA + x * CHUNK_SIZE + z
}

#[inline]
fn from_index(index: usize) -> (usize, usize, usize) {
    let z = index % CHUNK_SIZE;
    let x = index / CHUNK_SIZE % CHUNK_SIZE;
    let y = index / CHUNK_AREA;

    (x,y,z)
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
            p_x: vec![TileType::Ground, TileType::CliffLow],
            n_x: vec![TileType::Ground, ],
            p_y: vec![TileType::Air],
            n_y: vec![TileType::Air],
            p_z: vec![TileType::Ground],
            n_z: vec![TileType::Ground],
        };
        let cliff_low_rules = AdjacencyRules {
            p_x: vec![TileType::Air],
            n_x: vec![TileType::Ground],
            p_y: vec![TileType::Air],
            n_y: vec![TileType::Air],
            p_z: vec![TileType::CliffLow],
            n_z: vec![TileType::CliffLow],
        };
        let cliff_low_corner_rules = AdjacencyRules {
            p_x: vec![TileType::Ground],
            n_x: vec![TileType::Ground],
            p_y: vec![TileType::Air],
            n_y: vec![TileType::Air],
            p_z: vec![TileType::Ground],
            n_z: vec![TileType::Ground],
        };
        let air_rules = AdjacencyRules {
            p_x: vec![TileType::Air],
            n_x: vec![TileType::Air],
            p_y: vec![TileType::Ground, TileType::Air],
            n_y: vec![TileType::Ground, TileType::Air],
            p_z: vec![TileType::Air],
            n_z: vec![TileType::Air],
        };
        let chunk = ChunkBuilder::new(id.clone())
            .add_tile(TileType::Ground, ground_rules)
            .add_tile(TileType::CliffLow, cliff_low_rules)
            // .add_tile(TileType::CliffLowCorner, cliff_low_corner_rules)
            .add_tile(TileType::Air, air_rules)
            .build();

        // let chunk = Chunk::new(id.clone());
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HIGHT {
                    let tile = &chunk.get_tile(x, y, z);
                    if *tile == &TileType::Air {
                        continue;
                    }
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum TileType {
    #[default]
    Air,
    Ground,
    CliffLow,
    CliffLowCorner,
    CliffUpper,
    CliffUpperCorner,
}

#[derive(States, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum TileLoadState {
    #[default]
    Loading,
    Finished,
}

#[derive(Resource)]
struct TileAssets(Vec<Handle<Gltf>>);

impl Index<&TileType> for TileAssets {
    type Output = Handle<Gltf>;

    fn index(&self, index: &TileType) -> &Self::Output {
        match index {
            TileType::Air => todo!(),
            TileType::Ground => &self.0[0],
            TileType::CliffLow => &self.0[1],
            TileType::CliffLowCorner => &self.0[2],
            TileType::CliffUpper => todo!(),
            TileType::CliffUpperCorner => todo!(),
        }
    }
}

fn load_tiles(mut cmds: Commands, ass: Res<AssetServer>) {
    let ground = ass.load("models/terrain/ground.glb");
    let cliff_low = ass.load("models/terrain/cliff_low.glb");
    let cliff_low_corner = ass.load("models/terrain/cliff_low_corner.glb");
    let assets = vec![ground, cliff_low, cliff_low_corner];
    cmds.insert_resource(TileAssets(assets));
}

fn check_tiles_loaded(
    ass: Res<AssetServer>,
    tiles: Res<TileAssets>,
    mut next_state: ResMut<NextState<TileLoadState>>,
) {
    for tile in &tiles.0 {
        if ass.get_load_state(tile).is_none() {
            debug!("not all tiles loaded!");
            return;
        }
    }
    debug!("Tiles loaded!");
    next_state.set(TileLoadState::Finished);
}

fn setup_tiles(mut commands: Commands, tiles: Res<TileAssets>, assets_gltf: Res<Assets<Gltf>>) {
    if let Some(gltf) = assets_gltf.get(&tiles.0[1]) {
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..default()
        });
    }
}

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Forward,  //-Z
    Backward, //Z
    Left,     // -X
    Right,    // X
    Up,       // Y
    Down,     // -Y
}

// Chunk Generatorion

struct AdjacencyRules {
    p_x: Vec<TileType>,
    n_x: Vec<TileType>,
    p_y: Vec<TileType>,
    n_y: Vec<TileType>,
    p_z: Vec<TileType>,
    n_z: Vec<TileType>,
}

impl AdjacencyRules {
    fn from_dir(&self, dir: Dir) -> &Vec<TileType> {
        match dir {
            Dir::Forward => &self.n_z,
            Dir::Backward => &self.p_z,
            Dir::Left => &self.n_x,
            Dir::Right => &self.p_x,
            Dir::Up => &self.p_y,
            Dir::Down => &self.n_y,
        }
    }
}

struct ChunkBuilder {
    id: ChunkId,
    wave: Vec<Vec<TileType>>,
    output: [Option<TileType>; CHUNK_VOLUME],

    rules: HashMap<TileType, AdjacencyRules>,
}

impl Default for ChunkBuilder {
    fn default() -> Self {
        Self {
            id: ChunkId::default(),
            wave: vec![Vec::new(); CHUNK_VOLUME],
            output: [None; CHUNK_VOLUME],
            rules: HashMap::default(),
        }
    }
}

impl ChunkBuilder {
    pub fn new(id: ChunkId) -> Self {
        Self { id, ..default() }
    }

    pub fn add_tile(mut self, tile: TileType, rules: AdjacencyRules) -> Self {
        self.rules.insert(tile, rules);
        self
    }

    // pub fn add_tile_with_rotation(mut self, tile: TileType, rules: AdjacencyRules) -> Self {
    //     todo!();
    // }

    pub fn build(mut self) -> Chunk {
        self.init();
        while !self.is_collapsed() {
            self.iterate();
        }

        let mut tiles: Vec<TileType> = Vec::new();
        for tile in self.output {
            if tile.is_none() {
                panic!("wave function collapse should not fail");
            }
            tiles.push(tile.unwrap());
        }
        Chunk { id: self.id, tiles }
    }

    fn init(&mut self) {
        let tiles: Vec<TileType> = self.rules.keys().cloned().collect();
        for list in self.wave.iter_mut() {
            *list = tiles.clone();
        }
    }

    fn iterate(&mut self) {
        // eprintln!("WFC: ITERATE");
        // find superposition with smallest entropy
        let mut pos = None;
        let mut min = usize::MAX;
        for (i, p) in self.wave.iter().enumerate() {
            let entropy = p.len();
            if entropy != 0 && entropy < min {
                min = entropy;
                pos = Some(i);
            }
        }
        let Some(pos) = pos else {
            panic!("Wave function Collapse failed");
        };

        // collapse superposition in random element
        let superposition = &mut self.wave[pos];
        let mut len = superposition.len();
        if len > 1 && superposition.contains(&TileType::Air){
            let air = superposition.iter().position(|&t| t == TileType::Air).unwrap();
            superposition.remove(air);
            len-= 1;
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..len);
        let tile = superposition[index];
        superposition.clear();
        // eprintln!("WFC: collapsed {:?} at {:?}", tile, from_index(pos));
        self.output[pos] = Some(tile);

        // propagate
        for dir in Dir::iter() {
            let Some(index) = self.neighbor(pos, dir) else {
                // eprintln!("WFC: skip for {:?}", dir);
                continue;
            };
            // eprintln!("WFC: propagate to {:?}", from_index(index));
            let rule = self.rules[&tile].from_dir(dir);
            let new_neigh = self.wave[index].iter().filter(|t| rule.contains(t)).cloned().collect();
            // eprintln!("WFC: update {:?} to {:?}", self.wave[index], new_neigh);
            self.wave[index] = new_neigh;
        }
    }

    fn neighbor(&self, pos: usize, dir: Dir) -> Option<usize> {
        let (x,y,z) = from_index(pos);
        let (x_off, y_off, z_off): (isize, isize, isize) = match dir {
            Dir::Forward => (0,0,-1),
            Dir::Backward => (0,0,1),
            Dir::Left => (-1,0,0),
            Dir::Right => (1,0,0),
            Dir::Up => (0,1,0),
            Dir::Down => (0,-1,0)
        };
        let x_res = if (x == 0 && x_off == -1) || (x == CHUNK_SIZE-1 && x_off == 1) {
            return None;
        } else {
            (x as isize + x_off) as usize
        };

        let y_res = if (y == 0 && y_off == -1) || (y == CHUNK_HIGHT-1 && y_off == 1) {
            return None;
        } else {
            (y as isize + y_off) as usize
        };

        let z_res = if (z == 0 && z_off == -1) || (z == CHUNK_SIZE-1 && z_off == 1) {
            return None;
        } else {
            (z as isize + z_off) as usize
        };

        Some(get_index(x_res, y_res, z_res))
    }

    fn is_collapsed(&self) -> bool {
        for tile in self.output {
            if tile.is_none() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_index() {
        for z in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_HIGHT {
                    let index = get_index(x,y,z);
                    let (x0,y0,z0) = from_index(index);
                    assert_eq!(x, x0, "compared x={x} and x0={x0}, for index={index}");
                    assert_eq!(y, y0, "compared y={y} and y0={y0}, for index={index}");
                    assert_eq!(z, z0, "compared z={z} and z0={z0}, for index={index}");
                }
            }
        }
    }
}
