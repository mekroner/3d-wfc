// use bevy::prelude::*;
// use bevy::render::mesh::Indices;
// use bevy::render::render_resource::PrimitiveTopology;
// use bevy::pbr::wireframe::{NoWireframe, Wireframe, WireframeColor, WireframeConfig, WireframePlugin};
// use bevy::render::{ render_resource::WgpuFeatures, settings::{RenderCreation, WgpuSettings}, RenderPlugin,
//     };
// // use bevy::log::once;

// pub mod orbit_pan_camera;
// pub mod fly_camera;
// pub mod world_generation;

// use orbit_pan_camera::OrbitCameraPlugin;
// use fly_camera::{FlyCamPlugin, FlyCam};
// use world_generation::{WorldGenerationPlugin, WorldFocusPoint};

// #[derive(Component)]
// struct CustomUV;

// // NOTE: I can use an enum to get different cases for different terrain cases
// #[derive(Component)]
// struct Ground;

// fn main() {
//     App::new()
//         .add_plugins((DefaultPlugins.set(bevy::log::LogPlugin {
//             // level: bevy::log::Level::DEBUG,
//             ..default()
//         }).set(RenderPlugin {
//                 render_creation: RenderCreation::Automatic(WgpuSettings {
//                     // WARN this is a native only feature. It will not work with webgl or webgpu
//                     features: WgpuFeatures::POLYGON_MODE_LINE,
//                     ..default()
//                 }),
//             }),
//             // You need to add this plugin to enable wireframe rendering
//             WireframePlugin,
//         ))
//         .insert_resource(WireframeConfig {
//             global: true,
//             default_color: Color::WHITE,
//         })
//         .add_plugins(FlyCamPlugin)
//         .add_plugins(WorldGenerationPlugin)
//         .add_systems(Startup, setup)
//         .add_systems(Update, tie_focus_to_cam)
//         // .add_systems(Update, draw_cursor)
//         .run();
// }

// fn setup(mut commands: Commands) {
//     let trnsfrm = Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y);

//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1000.0,
//             range: 100.0,
//             ..default()
//         },
//         transform: trnsfrm,
//         ..default()
//     });
// }

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

// #[rustfmt::skip]
// fn create_cube_mesh() -> Mesh {
//     Mesh::new(PrimitiveTopology::TriangleList)
//     .with_inserted_attribute(
//         Mesh::ATTRIBUTE_POSITION,
//         // Each array is an [x, y, z] coordinate in local space.
//         // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
//         // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
//         vec![
//             // top (facing towards +y)
//             [-0.5, 0.5, -0.5], // vertex with index 0
//             [0.5, 0.5, -0.5], // vertex with index 1
//             [0.5, 0.5, 0.5], // etc. until 23
//             [-0.5, 0.5, 0.5],
//             // bottom   (-y)
//             [-0.5, -0.5, -0.5],
//             [0.5, -0.5, -0.5],
//             [0.5, -0.5, 0.5],
//             [-0.5, -0.5, 0.5],
//             // right    (+x)
//             [0.5, -0.5, -0.5],
//             [0.5, -0.5, 0.5],
//             [0.5, 0.5, 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
//             [0.5, 0.5, -0.5],
//             // left     (-x)
//             [-0.5, -0.5, -0.5],
//             [-0.5, -0.5, 0.5],
//             [-0.5, 0.5, 0.5],
//             [-0.5, 0.5, -0.5],
//             // back     (+z)
//             [-0.5, -0.5, 0.5],
//             [-0.5, 0.5, 0.5],
//             [0.5, 0.5, 0.5],
//             [0.5, -0.5, 0.5],
//             // forward  (-z)
//             [-0.5, -0.5, -0.5],
//             [-0.5, 0.5, -0.5],
//             [0.5, 0.5, -0.5],
//             [0.5, -0.5, -0.5],
//         ],
//     )
//     // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
//     // Take a look at the custom image (assets/textures/array_texture.png)
//     // so the UV coords will make more sense
//     // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
//     .with_inserted_attribute(
//         Mesh::ATTRIBUTE_UV_0,
//         vec![
//             // Assigning the UV coords for the top side.
//             [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.25],
//             // Assigning the UV coords for the bottom side.
//             [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
//             // Assigning the UV coords for the right side.
//             [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
//             // Assigning the UV coords for the left side.
//             [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
//             // Assigning the UV coords for the back side.
//             [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
//             // Assigning the UV coords for the forward side.
//             [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
//         ],
//     )
//     // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
//     // the surface.
//     // Normals are required for correct lighting calculations.
//     // Each array represents a normalized vector, which length should be equal to 1.0.
//     .with_inserted_attribute(
//         Mesh::ATTRIBUTE_NORMAL,
//         vec![
//             // Normals for the top side (towards +y)
//             [0.0, 1.0, 0.0],
//             [0.0, 1.0, 0.0],
//             [0.0, 1.0, 0.0],
//             [0.0, 1.0, 0.0],
//             // Normals for the bottom side (towards -y)
//             [0.0, -1.0, 0.0],
//             [0.0, -1.0, 0.0],
//             [0.0, -1.0, 0.0],
//             [0.0, -1.0, 0.0],
//             // Normals for the right side (towards +x)
//             [1.0, 0.0, 0.0],
//             [1.0, 0.0, 0.0],
//             [1.0, 0.0, 0.0],
//             [1.0, 0.0, 0.0],
//             // Normals for the left side (towards -x)
//             [-1.0, 0.0, 0.0],
//             [-1.0, 0.0, 0.0],
//             [-1.0, 0.0, 0.0],
//             [-1.0, 0.0, 0.0],
//             // Normals for the back side (towards +z)
//             [0.0, 0.0, 1.0],
//             [0.0, 0.0, 1.0],
//             [0.0, 0.0, 1.0],
//             [0.0, 0.0, 1.0],
//             // Normals for the forward side (towards -z)
//             [0.0, 0.0, -1.0],
//             [0.0, 0.0, -1.0],
//             [0.0, 0.0, -1.0],
//             [0.0, 0.0, -1.0],
//         ],
//     )
//     // Create the triangles out of the 24 vertices we created.
//     // To construct a square, we need 2 triangles, therefore 12 triangles in total.
//     // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
//     // by one, in a counter-clockwise order (relative to the position of the viewer, the order
//     // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
//     // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
//     // further examples and the implementation of the built-in shapes.
//     .with_indices(Some(Indices::U32(vec![
//         0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
//         4,5,7 , 5,6,7, // bottom (-y)
//         8,11,9 , 9,11,10, // right (+x)
//         12,13,15 , 13,14,15, // left (-x)
//         16,19,17 , 17,19,18, // back (+z)
//         20,21,23 , 21,22,23, // forward (-z)
//     ])))
// }
