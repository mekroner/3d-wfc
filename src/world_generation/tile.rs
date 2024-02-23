use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use strum_macros::EnumIter;

use crate::world_generation::Socket;

use super::{Prototype, Prototypes, Rotation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tile {
    pub id: TileID,
    pub asset_handle: Handle<Gltf>,
    pub y_rotation: Rotation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileID(pub u32);

#[derive(Resource)]
pub struct Tiles(pub HashMap<TileID, Tile>);

#[derive(EnumIter, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Forward,  //-Z
    Backward, //Z
    Left,     // -X
    Right,    // X
    Up,       // Y
    Down,     // -Y
}

impl Dir {
    pub fn to_vec3(self) -> Vec3 {
        match self {
            Dir::Forward => -Vec3::Z,
            Dir::Backward => Vec3::Z,
            Dir::Left => -Vec3::X,
            Dir::Right => Vec3::X,
            Dir::Up => Vec3::Y,
            Dir::Down => -Vec3::Y,
        }
    }
}

pub struct AdjacencyRules {
    pub p_x: Vec<TileID>,
    pub n_x: Vec<TileID>,
    pub p_y: Vec<TileID>,
    pub n_y: Vec<TileID>,
    pub p_z: Vec<TileID>,
    pub n_z: Vec<TileID>,
}

impl AdjacencyRules {
    pub fn from_dir(&self, dir: Dir) -> &Vec<TileID> {
        match dir {
            Dir::Forward => &self.n_z,
            Dir::Backward => &self.p_z,
            Dir::Left => &self.n_x,
            Dir::Right => &self.p_x,
            Dir::Up => &self.p_y,
            Dir::Down => &self.n_y,
        }
    }

    pub fn len(&self) -> usize {
        self.p_x.len() + self.n_x.len() +
        self.p_y.len() + self.n_y.len() +
        self.p_z.len() + self.n_z.len()
    }
}

#[derive(Resource)]
pub struct AdjRuleSet(pub HashMap<TileID, AdjacencyRules>);

pub fn generate_tiles_and_rules(
    prototypes: Res<Prototypes>,
    mut tiles: ResMut<Tiles>,
    mut rule_set: ResMut<AdjRuleSet>,
) {
    let mut id = 0;
    for prototype in prototypes.0.iter() {
        info!("New Tile: {}", prototype.name);
        let new_tile = Tile {
            id: TileID(id),
            asset_handle: prototype.asset_handle.clone(),
            y_rotation: Rotation::Zero,
        };
        tiles.0.insert(TileID(id), new_tile);
        let mut rule = AdjacencyRules {
            p_x: vec![],
            n_x: vec![],
            p_y: vec![],
            n_y: vec![],
            p_z: vec![],
            n_z: vec![],
        };
        let mut other_id = 0;
        for other_prt in prototypes.0.iter() {
            add_rule(prototype, other_prt, &mut rule, other_id);
            other_id += 1;
        }
        rule_set.0.insert(TileID(id), rule);
        id += 1;
    }
}

fn add_rule(prototype: &Prototype, other_prt: &Prototype, rule: &mut AdjacencyRules, id: u32) {
    if prototype.p_x == other_prt.n_x && prototype.p_x != Socket::NIL {
        info!(
            "New p_x Rule: {} connects to {}",
            prototype.name, other_prt.name
        );
        rule.p_x.push(TileID(id));
    }
    if prototype.n_x == other_prt.p_x && prototype.n_x != Socket::NIL {
        info!(
            "New n_x Rule: {} connects to {}",
            prototype.name, other_prt.name
        );
        rule.n_x.push(TileID(id));
    }
    if prototype.p_y == other_prt.n_y && prototype.p_y != Socket::NIL {
        info!(
            "New p_y Rule: {} connects to {}",
            prototype.name, other_prt.name
        );
        rule.p_y.push(TileID(id));
    }
    if prototype.n_y == other_prt.p_y && prototype.n_y != Socket::NIL {
        info!(
            "New n_y Rule: {} connects to {}",
            prototype.name, other_prt.name
        );
        rule.n_y.push(TileID(id));
    }
    if prototype.p_z == other_prt.n_z && prototype.p_z != Socket::NIL {
        info!(
            "New p_z Rule: {} connects to {}",
            prototype.name, other_prt.name
        );
        rule.p_z.push(TileID(id));
    }
    if prototype.n_z == other_prt.p_z && prototype.n_z != Socket::NIL {
        info!(
            "New n_z Rule: {} connects to {}",
            prototype.name, other_prt.name
        );
        rule.n_z.push(TileID(id));
    }
}
