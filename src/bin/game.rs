use bevy::prelude::*;
// use bevy::render::mesh::Indices;
// use bevy::render::render_resource::PrimitiveTopology;
// use bevy::pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin};
// use bevy::render::{ render_resource::WgpuFeatures, settings::{RenderCreation, WgpuSettings}, RenderPlugin,
//     };
// // use bevy::log::once;

use utg::fly_camera::{FlyCamPlugin, FlyCam};
use utg::world_generation::{WorldGenerationPlugin, WorldFocusPoint};

// #[derive(Component)]
// struct CustomUV;

// // NOTE: I can use an enum to get different cases for different terrain cases
// #[derive(Component)]
// struct Ground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            // level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(FlyCamPlugin)
        .add_plugins(WorldGenerationPlugin)
        .insert_resource(ClearColor(Color::hex("61adb0").unwrap()))
        .add_systems(Startup, setup)
        // .add_systems(Update, tie_focus_to_cam)
        // .add_systems(Update, draw_cursor)
        .run();
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            range: 100.0,
            ..default()
        },
        transform,
        ..default()
    });
}

// fn tie_focus_to_cam(
//     mut focus: ResMut<WorldFocusPoint>,
//     query: Query<&Transform, With<FlyCam>>,
// ) {
//     for trans in query.iter() {
//         focus.pos = trans.translation.clone();
//     }
// }

// // fn draw_cursor(
// //     camera_query: Query<(&Camera, &GlobalTransform)>,
// //     ground_query: Query<&GlobalTransform, With<Ground>>,
// //     windows: Query<&Window>,
// //     mut gizmos: Gizmos,
// // ) {
// //     let (camera, camera_transform) = camera_query.single();
// //     let ground = ground_query.single();

// //     let Some(cursor_position) = windows.single().cursor_position() else {
// //         return;
// //     };

// //     let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
// //         return;
// //     };

// //     let Some(distance) = ray.intersect_plane(ground.translation(), ground.up()) else {
// //         return;
// //     };

// //     let point = ray.get_point(distance);

// //     gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);
// // }
