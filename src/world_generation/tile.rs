use bevy::prelude::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Ground,
    CliffLow,
    CliffLowCorner,
    CliffUpper,
    CliffUpperCorner,
}

#[derive(States, Default, Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TileLoadState {
    #[default]
    Loading,
    Finished,
}

#[derive(Resource)]
pub struct TileAssets(Vec<Handle<Gltf>>);

impl Index<&Tile> for TileAssets {
    type Output = Handle<Gltf>;

    fn index(&self, index: &Tile) -> &Self::Output {
        match index {
            Tile::Ground => &self.0[0],
            Tile::CliffLow => &self.0[1],
            Tile::CliffLowCorner => &self.0[2],
            Tile::CliffUpper => todo!(),
            Tile::CliffUpperCorner => todo!(),
        }
    }
}


pub fn check_tiles_loaded(
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

pub fn load_tiles(mut cmds: Commands, ass: Res<AssetServer>) {
    let ground = ass.load("models/terrain/ground.glb");
    let cliff_low = ass.load("models/terrain/cliff_low.glb");
    let cliff_low_corner = ass.load("models/terrain/cliff_low_corner.glb");
    let assets = vec![ground, cliff_low, cliff_low_corner];
    cmds.insert_resource(TileAssets(assets));
}

pub fn setup_tiles(mut commands: Commands, tiles: Res<TileAssets>, assets_gltf: Res<Assets<Gltf>>) {
    if let Some(gltf) = assets_gltf.get(&tiles.0[1]) {
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..default()
        });
    }
}
