use super::*;

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
    tiles: Vec<Option<Tile>>,
}

impl Chunk {
    pub fn new(id: ChunkId) -> Self {
        let mut tiles = vec![None; CHUNK_VOLUME];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                tiles[get_index(x, 0, z)] = Some(Tile::Ground);
            }
        }
        Self { id, tiles }
    }

    pub fn id(&self) -> ChunkId {
        self.id
    }

    pub fn get_tile(&self, x: usize, y: usize, z: usize) -> Option<Tile> {
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

pub struct ChunkBuilder {
    id: ChunkId,
    wave: Vec<Vec<Tile>>,
    output: [Option<Tile>; CHUNK_VOLUME],

    rules: HashMap<Tile, AdjacencyRules>,
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

    pub fn add_tile(mut self, tile: Tile, rules: AdjacencyRules) -> Self {
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

        let mut tiles: Vec<Tile> = Vec::new();
        for tile in self.output {
            if tile.is_none() {
                panic!("wave function collapse should not fail");
            }
            tiles.push(tile.unwrap());
        }
        Chunk { id: self.id, tiles }
    }

    fn init(&mut self) {
        let tiles: Vec<Tile> = self.rules.keys().cloned().collect();
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
        if len > 1 && superposition.contains(&Tile::Air) {
            let air = superposition.iter().position(|&t| t == Tile::Air).unwrap();
            superposition.remove(air);
            len -= 1;
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
            let new_neigh = self.wave[index]
                .iter()
                .filter(|t| rule.contains(t))
                .cloned()
                .collect();
            // eprintln!("WFC: update {:?} to {:?}", self.wave[index], new_neigh);
            self.wave[index] = new_neigh;
        }
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
        for tile in self.output {
            if tile.is_none() {
                return false;
            }
        }
        true
    }
}
