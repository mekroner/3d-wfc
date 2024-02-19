// This first analizes the tileset, it defines sockets, as well as the which side is up and which
// side is down

use bevy::prelude::*;
use bevy::gltf::Gltf;

// use super::tile::Tile;

pub enum Rotation {
    Zero,
    Quarter,
    Half,
    ThreeQuarter
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Socket(pub u32);

pub struct Prototype {
    pub name: &'static str,
    pub asset_handle: Handle<Gltf>,
    pub p_x: Socket,
    pub n_x: Socket,
    pub p_y: Socket,
    pub n_y: Socket,
    pub p_z: Socket,
    pub n_z: Socket,

    pub frequency: u32,
}


#[derive(Resource)]
pub struct Prototypes(pub Vec<Prototype>);

// FIXME: This can be done using a new asset type
pub fn load_prototypes(mut cmds: Commands, ass: Res<AssetServer>) {
    let ground = ass.load("models/terrain/ground.glb");
    let cliff_low = ass.load("models/terrain/cliff_low.glb");
    let cliff_low_corner = ass.load("models/terrain/cliff_low_corner.glb");
    let cliff_upper = ass.load("models/terrain/cliff_upper.glb");
    let cliff_upper_corner = ass.load("models/terrain/cliff_upper_corner.glb");
    let ground_prt = Prototype {
        name: "ground",
        asset_handle: ground,
        p_x: Socket(1),
        n_x: Socket(1),
        p_y: Socket(0),
        n_y: Socket(0),
        p_z: Socket(1),
        n_z: Socket(1),
        frequency: 0,
    };

    let cliff_low_prt = Prototype {
        name: "cliff_low",
        asset_handle: cliff_low,
        p_x: Socket(0),
        n_x: Socket(1),
        p_y: Socket(2),
        n_y: Socket(0),
        p_z: Socket(3),
        n_z: Socket(3),
        frequency: 0,
    };

    let cliff_low_corner_prt = Prototype {
        name: "cliff_low_corner",
        asset_handle: cliff_low_corner,
        p_x: Socket(0),
        n_x: Socket(1),
        p_y: Socket(0),
        n_y: Socket(0),
        p_z: Socket(1),
        n_z: Socket(3),
        frequency: 0,
    };

    let cliff_upper_prt = Prototype {
        name: "cliff_upper",
        asset_handle: cliff_upper,
        p_x: Socket(1),
        n_x: Socket(0),
        p_y: Socket(0),
        n_y: Socket(2),
        p_z: Socket(0),
        n_z: Socket(0),
        frequency: 0,
    };

    let cliff_upper_corner_prt = Prototype {
        name: "cliff_upper_corner",
        asset_handle: cliff_upper_corner,
        p_x: Socket(0),
        n_x: Socket(0),
        p_y: Socket(0),
        n_y: Socket(0),
        p_z: Socket(0),
        n_z: Socket(0),
        frequency: 0,
    };

    let assets = vec![
        ground_prt,
        cliff_low_prt,
        cliff_low_corner_prt,
        cliff_upper_prt,
        cliff_upper_corner_prt,
    ];
    cmds.insert_resource(Prototypes(assets));
}

#[derive(States, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PrototypesLoadState {
    #[default]
    Loading,
    Finished,
}

pub fn check_prototypes_loaded(
    ass: Res<AssetServer>,
    prts: Res<Prototypes>,
    mut next_state: ResMut<NextState<PrototypesLoadState>>,
) {
    for prt in &prts.0 {
        if ass.get_load_state(&prt.asset_handle).is_none() {
            debug!("Not all prototypes loaded!");
            return;
        }
    }
    debug!("Prototypes loaded!");
    next_state.set(PrototypesLoadState::Finished);
}
