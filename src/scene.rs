// use crate::math::Mat4;
// use crate::math::{Vec2, Vec3};
// use crate::texture::Texture;
// use gltf::json::extensions::material;
// use gltf::{self, buffer::Data, image::Source, Gltf};

// // #[derive(Debug, Clone, Copy)]
// // pub struct Vertex {
// //     pos: Vec3,
// //     tc: Option<Vec2>,
// //     normals: Option<Vec3>,
// //     tangent: Option<Vec3>,
// //     bitangent: Option<Vec3>,
// // }
// #[derive(Debug, Clone)]
// pub struct Material {
//     pub albedo: Texture,
// }

// #[derive(Debug, Clone)]
// pub struct Mesh {
//     pub vertices: Vec<Vec3>,
//     pub tex_coords: Vec<Vec2>,
//     pub indicies: Vec<u32>,
//     pub material: Option<Material>,
// }

// #[derive(Debug, Clone)]
// pub struct SceneNode {
//     pub parent: Option<usize>,
//     pub children: Vec<usize>,
//     pub meshes: Vec<Mesh>,
// }

// impl SceneNode {
//     pub fn new() -> SceneNode {
//         SceneNode {
//             parent: None,
//             children: Vec::new(),
//             meshes: Vec::new(),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Scene {
//     pub nodes: Vec<SceneNode>,
// }

// impl Scene {
//     pub fn new() -> Self {
//         Scene { nodes: Vec::new() }
//     }

//     pub fn load(path: &str) -> Self {
//         let mut scene = Scene::new();
//         let (gltf, buffers, images) = gltf::import(&path).expect("failed to load file");
//         let gltf_scene = gltf.scenes().next().unwrap();
//         let mut comps: Vec<&str> = path.split("/").collect();
//         comps.pop();
//         let dir = comps.join("/");

//         for node in gltf_scene.nodes() {
//             Scene::process_node(&node, &buffers, None, &mut scene, dir.as_str());
//         }
//         scene
//     }

//     pub fn process_node(
//         gltf_node: &gltf::Node,
//         gltf_buffers: &Vec<Data>,
//         parent: Option<usize>,
//         scene: &mut Self,
//         dir: &str,
//     ) -> usize {
//         // if let Some(name) = gltf_node.name() {
//         //     scene.names.push(name.to_string());
//         // }
//         let mut node = SceneNode::new();
//         node.parent = parent;
//         scene.nodes.push(node);
//         let current_index = scene.nodes.len() - 1;
//         if let Some(mesh) = gltf_node.mesh() {
//             let mut positions: Vec<Vec3> = Vec::new();
//             let mut indicies: Vec<u32> = Vec::new();
//             let mut tc: Vec<Vec2> = Vec::new();
//             for primitive in mesh.primitives() {
//                 let reader = primitive.reader(|buffer| Some(&gltf_buffers[buffer.index()]));

//                 if let Some(iter) = reader.read_positions() {
//                     for vertex_position in iter {
//                         positions.push(Vec3::new(
//                             vertex_position[0],
//                             vertex_position[1],
//                             vertex_position[2],
//                         ))
//                     }
//                 }
//                 if let Some(iter) = reader.read_indices() {
//                     let iter = iter.into_u32();
//                     for indice in iter {
//                         indicies.push(indice);
//                     }
//                 }
//                 if let Some(iter) = reader.read_tex_coords(0) {
//                     for tex_coord in iter.into_f32() {
//                         tc.push(Vec2::new(tex_coord[0], tex_coord[1]))
//                     }
//                 }
//                 // let mut material: Option<Material> = None;
//                 // if let Some(base_color) = primitive
//                 //     .material()
//                 //     .pbr_metallic_roughness()
//                 //     .base_color_texture()
//                 // {
//                 //     let image = base_color.texture().source().index();
//                 // }
//                 if positions.len() > 0 {
//                     scene.nodes[current_index].meshes.push(Mesh {
//                         vertices: positions.clone(),
//                         indicies: indicies.clone(),
//                         tex_coords: tc.clone(),
//                         material: None,
//                     })
//                 }
//             }
//         }

//         for child in gltf_node.children() {
//             let child_index =
//                 Scene::process_node(&child, &gltf_buffers, Some(current_index), scene, dir);
//             scene.nodes[current_index].children.push(child_index);
//         }
//         current_index
//     }
// }
