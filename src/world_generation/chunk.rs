use std::fmt::Display;
use std::ops::RangeBounds;
use std::usize;

use super::dir::Dir;
use super::{AdjRuleSet, AdjacencyRules, Tile, TileID, Tiles, CHUNK_HIGHT};
use super::{CHUNK_SIZE, CHUNK_VOLUME};
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::rngs::ThreadRng;
use rand::Rng;
use strum::IntoEnumIterator;

use super::util::*;

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
    tiles: Vec<Option<TileID>>,
}

impl Chunk {
    pub fn new(id: ChunkId, ground: Option<TileID>) -> Self {
        let mut tiles = vec![None; CHUNK_VOLUME];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                tiles[get_index(x, 0, z)] = ground.clone();
            }
        }
        Self { id, tiles }
    }

    pub fn id(&self) -> ChunkId {
        self.id.clone()
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Option<TileID> {
        self.tiles[get_index(x, y, z)]
    }

    pub fn pos(&self) -> Vec3 {
        let x = self.id.x() as f32;
        let z = self.id.z() as f32;
        let size = CHUNK_SIZE as f32;
        Vec3::new(x, 0.0, z) * size
    }
}

// Chunk Generatorion

#[derive(Debug, Clone)]
enum WaveState {
    Collapsed(TileID),
    Superpos(Vec<TileID>),
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
    rules: HashMap<TileID, AdjacencyRules>,
    rng: ThreadRng,
}

impl Default for ChunkBuilder {
    fn default() -> Self {
        Self {
            id: ChunkId::default(),
            wave: vec![],
            rules: HashMap::default(),
            rng: rand::thread_rng(),
        }
    }
}

struct WaveError(&'static str);

impl Display for WaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WFC Error: {}", self.0)
    }
}

impl ChunkBuilder {
    pub fn new(id: ChunkId) -> Self {
        Self { id, ..default() }
    }

    pub fn add_rule_set(mut self, set: AdjRuleSet) -> Self {
        self.rules = set.0;
        self
    }

    pub fn build(mut self, tiles: &Tiles) -> Chunk {
        self.init(tiles);
        if let Err(e) = self.branch(tiles) {
            error!("{}", e);
        }
        // while !self.is_collapsed() {
        //     if let Err(e) = self.iterate(tiles) {
        //         error!("{}", e);
        //         break;
        //     }
        // }

        let mut tiles: Vec<Option<TileID>> = Vec::new();
        for wave_state in self.wave {
            let WaveState::Collapsed(tile) = wave_state else {
                // panic!("wave function collapse should not fail");
                tiles.push(None);

                continue;
            };
            tiles.push(Some(tile));
        }

        Chunk { id: self.id, tiles }
    }

    fn init(&mut self, tiles: &Tiles) {
        self.wave = vec![WaveState::Superpos(vec![]); CHUNK_VOLUME];
        for y in 0..CHUNK_HIGHT {
            let ids: Vec<TileID> = self
                .rules
                .keys()
                .filter(|&id| {
                    let tile: &Tile = &tiles.0[id];
                    let Some(ref range) = tile.y_level else {
                        return true;
                    };
                    range.contains(&y)
                })
                .cloned()
                .collect();
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = get_index(x, y, z);
                    self.wave[index] = WaveState::Superpos(ids.clone());
                }
            }
        }
        for y in 0..CHUNK_HIGHT {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let index = get_index(x, y, z);
                    self.propagate(index);
                }
            }
        }
    }

    fn shannon_entropy(&self, ids: &[TileID], tiles: &Tiles) -> Option<f32> {
        if ids.len() == 0 {
            return None;
        }
        let weights = ids
            .iter()
            .filter_map(|id| tiles.0.get(id).map(|tile| tile.weight as f32));
        let sum: f32 = weights.clone().sum();
        let sum2: f32 = weights.fold(0.0, |acc, weight| acc + weight * f32::log2(weight));
        Some(sum - sum2 / sum)
    }

    // find superposition with lowest non zero entropy
    fn lowest_entropy(&self, tiles: &Tiles) -> Option<usize> {
        let mut pos = None;
        let mut min = f32::MAX;
        for (i, wave_state) in self.wave.iter().enumerate() {
            let WaveState::Superpos(p) = wave_state else {
                continue;
            };
            let Some(entropy) = self.shannon_entropy(p, tiles) else {
                continue;
            };
            if entropy < min {
                min = entropy;
                pos = Some(i);
            }
        }
        pos
    }

    fn random_by_weight(&self, ids: &[TileID], tiles: &Tiles) -> Option<usize> {
        let weights = ids
            .iter()
            .filter_map(|id| tiles.0.get(id).map(|tile| tile.weight));
        let sum: usize = weights.clone().sum();
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..sum);
        let mut cursor = 0;
        for (i, weight) in weights.enumerate() {
            cursor += weight;
            if cursor > random {
                return Some(i);
            }
        }
        None
    }

    // collapse superposition in random element
    fn collapse(&mut self, pos: usize, tiles: &Tiles) -> Result<TileID, WaveError> {
        let WaveState::Superpos(ref superpos) = self.wave[pos] else {
            return Err(WaveError("Cannot Collapse an already collapsed element."));
        };
        let Some(index) = self.random_by_weight(&superpos, tiles) else {
            return Err(WaveError("No Element to collapse found!"));
        };
        let tile = superpos[index];
        info!("WFC: Selected {:?} from Superpos {:?}", tile, superpos);
        // superpos.clear();
        self.wave[pos] = WaveState::Collapsed(tile);
        Ok(tile)
    }

    // propagate the rules for a tile collapsed at pos
    fn propagate(&mut self, pos: usize) {
        let mut stack = Vec::new();
        stack.push(pos);
        while !stack.is_empty() {
            let current_pos = stack.pop().unwrap();
            for dir in Dir::iter() {
                let Some(neighbor_pos) = self.neighbor(pos, dir) else {
                    continue;
                };
                let WaveState::Superpos(neighbor) = self.wave[neighbor_pos].clone() else {
                    continue;
                };
                let allowed_neighbors: Vec<TileID> = match &self.wave[current_pos] {
                    WaveState::Collapsed(tile) => self.rules[tile].from_dir(dir).to_vec(),
                    WaveState::Superpos(list) => {
                        let mut res = Vec::new();
                        for tile in list {
                            res.append(&mut self.rules[tile].from_dir(dir).to_vec());
                        }
                        res
                    }
                };

                for tile in neighbor {
                    if allowed_neighbors.contains(&tile) {
                        continue;
                    }

                    // remove tile from neighbors
                    if let WaveState::Superpos(n) = &mut self.wave[neighbor_pos] {
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

    fn iterate(&mut self, tiles: &Tiles) -> Result<(), WaveError> {
        // info!("WFC: INTERATE");
        let Some(pos) = self.lowest_entropy(tiles) else {
            return Err(WaveError("No element with lowest entropy found."));
        };
        let tile = self.collapse(pos, tiles)?;
        // info!("WFC: Collapse {:?} {:?}", from_index(pos), tile);
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

    fn branch(&mut self, tiles: &Tiles) -> Result<(), WaveError> {
        // info!("WFC: BRANCH");
        if self.is_collapsed() {
            return Ok(());
        }

        let Some(min_ent) = self
            .wave
            .iter()
            .filter_map(|state| match state {
                WaveState::Superpos(vec) => Some(vec.len()),
                WaveState::Collapsed(_) => None,
            })
            .min()
        else {
            return Err(WaveError("No lowest_entropy found"));
        };

        if min_ent == 0 {
            return Err(WaveError("Lowest entropy cannot be zero!"));
        }

        let positions: Vec<usize> = self
            .wave
            .iter()
            .enumerate()
            .filter_map(|(i, state)| match state {
                WaveState::Superpos(vec) if vec.len() == min_ent => Some(i),
                _ => None,
            })
            .collect();

        for pos in positions {
            let wave = self.wave.clone();
            self.collapse(pos, tiles)?;
            self.propagate(pos);
            if let Err(_) = self.branch(tiles) {
                self.wave = wave.clone();
                continue;
            }
            if self.is_collapsed() {
                return Ok(());
            }
            self.wave = wave.clone();
        }

        Err(WaveError("No solution found"))
    }
}
