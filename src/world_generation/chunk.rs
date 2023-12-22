use std::collections::HashMap;
use std::thread::current;
use std::usize;

use crate::Ground;

use super::Tile;
use super::{CHUNK_HIGHT, CHUNK_SIZE, CHUNK_VOLUME};
use bevy::prelude::*;
use rand::{thread_rng, Rng};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::util::*;

use WaveState::*;

#[derive(Default, Hash, Eq, PartialEq, Debug, Clone)]
pub struct ChunkId(IVec2);

impl ChunkId {
    pub fn new(x: i32, z: i32) -> Self {
        Self(IVec2::new(x, z))
    }

    pub fn x(&self) -> i32 {
        self.0.x
    }

    pub fn z(&self) -> i32 {
        self.0.y
    }

    pub fn from_position(pos: Vec3) -> Self {
        let x = (pos.x / CHUNK_SIZE as f32).floor() as i32;
        let z = (pos.z / CHUNK_SIZE as f32).floor() as i32;
        Self::new(x, z)
    }

    pub fn x_offset(mut self, offset: i32) -> Self {
        self.0.x += offset;
        self
    }

    pub fn z_offset(mut self, offset: i32) -> Self {
        self.0.y += offset;
        self
    }
}

pub struct Chunk {
    id: ChunkId,
    tiles: Vec<Tile>,
}

impl Chunk {
    pub fn new(id: ChunkId) -> Self {
        let mut tiles = vec![Tile::Air; CHUNK_VOLUME];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                tiles[get_index(x, 0, z)] = Tile::Ground;
            }
        }
        Self { id, tiles }
    }

    pub fn id(&self) -> ChunkId {
        self.id.clone()
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Tile {
        self.tiles[get_index(x, y, z)]
    }

    pub fn pos(&self) -> Vec3 {
        let x = self.id.x() as f32;
        let z = self.id.z() as f32;
        let size = CHUNK_SIZE as f32;
        Vec3::new(x, 0.0, z) * size
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

pub struct AdjacencyRules {
    pub p_x: Vec<Tile>,
    pub n_x: Vec<Tile>,
    pub p_y: Vec<Tile>,
    pub n_y: Vec<Tile>,
    pub p_z: Vec<Tile>,
    pub n_z: Vec<Tile>,
}

impl AdjacencyRules {
    fn from_dir(&self, dir: Dir) -> &Vec<Tile> {
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

#[derive(Debug, Clone)]
enum WaveState {
    Collapsed(Tile),
    Superpos(Vec<Tile>),
}

impl WaveState {
    fn is_collapsed(&self) -> bool {
        match self {
            Self::Collapsed(_) => true,
            Self::Superpos(_) => false,
        }
    }
}

pub struct ChunkBuilder {
    id: ChunkId,
    wave: Vec<WaveState>,

    rules: HashMap<Tile, AdjacencyRules>,
}

impl Default for ChunkBuilder {
    fn default() -> Self {
        Self {
            id: ChunkId::default(),
            wave: vec![],
            rules: HashMap::default(),
        }
    }
}

struct WaveError;

impl ChunkBuilder {
    pub fn new(id: ChunkId) -> Self {
        Self { id, ..default() }
    }

    pub fn add_tile(mut self, tile: Tile, rules: AdjacencyRules) -> Self {
        self.rules.insert(tile, rules);
        self
    }

    pub fn build(mut self) -> Chunk {
        self.init();
        while !self.is_collapsed() {
            if self.iterate().is_err() {
                eprint!("WFC: Error");
                break;
            }
        }

        // dbg!(&self.wave);

        let mut tiles: Vec<Tile> = Vec::new();
        for wave_state in self.wave {
            let Collapsed(tile) = wave_state else {
                // panic!("wave function collapse should not fail");
                tiles.push(Tile::Dbg);
                
                continue;
            };
            tiles.push(tile);
        }

        Chunk { id: self.id, tiles }
    }

    fn init(&mut self) {
        let tiles: Vec<Tile> = self.rules.keys().cloned().collect();
        self.wave = vec![Superpos(tiles.clone()); CHUNK_VOLUME];
        self.wave[get_index(0,0,0)] = Collapsed(Tile::Ground);
        self.propagate(get_index(0,0,0));
    }

    // find superposition with lowest non zero entropy
    fn lowest_entropy(&self) -> Option<usize> {
        let mut pos = None;
        let mut min = usize::MAX;
        for (i, wave_state) in self.wave.iter().enumerate() {
            let Superpos(p) = wave_state else {
                continue;
            };
            let entropy = p.len();
            if entropy != 0 && entropy < min {
                min = entropy;
                pos = Some(i);
            }
        }
        pos
    }

    // collapse superposition in random element
    fn collapse(&mut self, pos: usize) -> Result<Tile, WaveError> {
        let Superpos(superpos) = &mut self.wave[pos] else {
            // panic!("WFC: Cannot Collapse an already collapsed element");
            return Err(WaveError);
        };
        let len = superpos.len();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..len);
        let tile = superpos[index];
        superpos.clear();
        self.wave[pos] = Collapsed(tile);
        Ok(tile)
    }

    // propagate the rules for a tile colapsed at pos
    fn propagate(&mut self, pos: usize) {
        let mut stack = Vec::new();
        stack.push(pos);
        while !stack.is_empty() {
            let current_pos = stack.pop().unwrap();
            for dir in Dir::iter() {
                let Some(neighbor_pos) = self.neighbor(pos, dir) else {
                    continue;
                };
                let Superpos(neighbor) = self.wave[neighbor_pos].clone() else {
                    continue;
                };
                let allowed_neighbors: Vec<Tile> = match &self.wave[current_pos] {
                    Collapsed(tile) => self.rules[tile].from_dir(dir).clone(),
                    Superpos(list) => {
                        let mut res = Vec::new();
                        for tile in list {
                            res.append(&mut self.rules[tile].from_dir(dir).clone());
                        }
                        res
                    }
                };

                for tile in neighbor {
                    if allowed_neighbors.contains(&tile) {
                        continue;
                    }

                    // TODO remove tile from neighbors
                    if let Superpos(n) = &mut self.wave[neighbor_pos] {
                        let index = n.iter().position(|&i| i == tile).unwrap();
                        n.remove(index);
                    }

                    if stack.contains(&neighbor_pos) {
                        stack.push(neighbor_pos)
                    }
                }
            }
        }
    }

    fn iterate(&mut self) -> Result<(), WaveError> {
        eprintln!();
        eprintln!("WFC: INTERATE");
        let Some(pos) = self.lowest_entropy() else {
            return Err(WaveError);
        };

        let tile = self.collapse(pos)?;
        eprintln!("WFC: Collapse {:?} {:?}", from_index(pos), tile);
        self.propagate(pos);
        Ok(())
    }

    fn neighbor(&self, pos: usize, dir: Dir) -> Option<usize> {
        let (x, y, z) = from_index(pos);
        let (x_off, y_off, z_off): (isize, isize, isize) = match dir {
            Dir::Forward => (0, 0, -1),
            Dir::Backward => (0, 0, 1),
            Dir::Left => (-1, 0, 0),
            Dir::Right => (1, 0, 0),
            Dir::Up => (0, 1, 0),
            Dir::Down => (0, -1, 0),
        };
        let x_res = if (x == 0 && x_off == -1) || (x == CHUNK_SIZE - 1 && x_off == 1) {
            return None;
        } else {
            (x as isize + x_off) as usize
        };

        let y_res = if (y == 0 && y_off == -1) || (y == CHUNK_HIGHT - 1 && y_off == 1) {
            return None;
        } else {
            (y as isize + y_off) as usize
        };

        let z_res = if (z == 0 && z_off == -1) || (z == CHUNK_SIZE - 1 && z_off == 1) {
            return None;
        } else {
            (z as isize + z_off) as usize
        };

        Some(get_index(x_res, y_res, z_res))
    }

    fn is_collapsed(&self) -> bool {
        for wave_state in &self.wave {
            if !wave_state.is_collapsed() {
                return false;
            }
        }
        true
    }
}
