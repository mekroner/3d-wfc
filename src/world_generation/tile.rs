use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use strum::IntoEnumIterator;

use crate::world_generation::Socket;

use super::{Prototype, Prototypes, dir::Rotation, dir::Dir};

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

pub struct AdjacencyRules {
    pub p_x: Vec<TileID>,
    pub n_x: Vec<TileID>,
    pub p_y: Vec<TileID>,
    pub n_y: Vec<TileID>,
    pub p_z: Vec<TileID>,
    pub n_z: Vec<TileID>,
}

impl AdjacencyRules {
    pub fn from_dir(&self, dir: Dir) -> &[TileID] {
        match dir {
            Dir::Forward => &self.n_z,
            Dir::Backward => &self.p_z,
            Dir::Left => &self.n_x,
            Dir::Right => &self.p_x,
            Dir::Up => &self.p_y,
            Dir::Down => &self.n_y,
        }
    }

    pub fn from_dir_mut(&mut self, dir: Dir) -> &mut Vec<TileID> {
        match dir {
            Dir::Forward => &mut self.n_z,
            Dir::Backward => &mut self.p_z,
            Dir::Left => &mut self.n_x,
            Dir::Right => &mut self.p_x,
            Dir::Up => &mut self.p_y,
            Dir::Down => &mut self.n_y,
        }
    }

    pub fn len(&self) -> usize {
        self.p_x.len()
            + self.n_x.len()
            + self.p_y.len()
            + self.n_y.len()
            + self.p_z.len()
            + self.n_z.len()
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
        for rotation in Rotation::iter() {
            info!("New Tile: {} with rotation {:?}", prototype.name, rotation);
            let new_tile = Tile {
                id: TileID(id),
                asset_handle: prototype.asset_handle.clone(),
                y_rotation: rotation,
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
                for other_rotation in Rotation::iter() {
                    append_rule(
                        prototype,
                        rotation,
                        other_prt,
                        other_rotation,
                        &mut rule,
                        other_id,
                    );
                    other_id += 1;
                }
            }
            rule_set.0.insert(TileID(id), rule);
            id += 1;
        }
    }
}

fn append_rule(
    prototype: &Prototype,
    rotation: Rotation,
    other_prt: &Prototype,
    other_rotation: Rotation,
    rule: &mut AdjacencyRules,
    id: u32,
) {
    for dir in Dir::iter() {
        if dir == Dir::Up || dir == Dir::Down {
            continue;
        }
        let rot_dir = dir.rotate_y(rotation);
        let other_rot_dir = dir.rotate_y(other_rotation).opposite();
        let socket_is_not_nil = prototype.socket_from_dir(rot_dir) != Socket::NIL;
        let has_connection =
            prototype.socket_from_dir(rot_dir) == other_prt.socket_from_dir(other_rot_dir);
        if socket_is_not_nil && has_connection {
            rule.from_dir_mut(rot_dir).push(TileID(id));
        }
    }
}

