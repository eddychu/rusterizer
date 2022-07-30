// use std::primitive;

// use crate::{math::Vec2, math::Vec3, scene::Material, texture::Texture};
// use gltf::{
//     self,
//     buffer::{Data, View},
//     image::Source,
//     texture::{MagFilter, MinFilter, Sampler, WrappingMode},
//     Document, Gltf,
// };

// #[derive(Debug, Clone)]
// pub struct IMesh {
//     pub primitives: Vec<IMeshPrimitive>,
// }

// #[derive(Debug, Clone)]
// pub struct IMeshPrimitive {
//     pub positions: Vec<Vec3>,
//     pub indices: Vec<usize>,
//     pub tex_coords: Vec<Vec2>,
//     pub material: Option<usize>,
// }

// #[derive(Debug, Clone)]
// pub struct ITexture {
//     sampler: usize,
//     image: usize,
// }

// #[derive(Debug, Clone)]
// pub struct IMaterial {
//     base_color_texture: usize,
// }
// #[derive(Debug, Clone)]
// pub struct ISceneNode {
//     pub parent: Option<usize>,
//     pub children: Vec<usize>,
//     pub mesh: Option<usize>,
// }
// #[derive(Debug, Clone)]
// pub struct TextureSampler {
//     wrap_s: WrappingMode,
//     wrap_t: WrappingMode,
//     min_filter: MinFilter,
//     mag_filter: MagFilter,
// }

// impl ISceneNode {
//     pub fn new() -> Self {
//         ISceneNode {
//             parent: None,
//             children: Vec::new(),
//             mesh: None,
//         }
//     }
// }
// #[derive(Debug, Clone)]
// pub struct IScene {
//     pub nodes: Vec<usize>,
// }
// #[derive(Debug, Clone)]
// pub struct IGltf {
//     pub scene: Option<usize>,
//     pub scenes: Vec<IScene>,
//     pub nodes: Vec<ISceneNode>,
//     pub materials: Vec<IMaterial>,
//     pub meshes: Vec<IMesh>,
//     pub textures: Vec<ITexture>,
//     pub samplers: Vec<TextureSampler>,
//     pub images: Vec<Texture>,
// }

// impl IGltf {
//     pub fn new() -> Self {
//         IGltf {
//             scene: None,
//             scenes: Vec::new(),
//             nodes: Vec::new(),
//             materials: Vec::new(),
//             meshes: Vec::new(),
//             textures: Vec::new(),
//             samplers: Vec::new(),
//             images: Vec::new(),
//         }
//     }

//     pub fn load(path: &str) -> Self {
//         let mut igltf = IGltf::new();
//         let (document, buffers, _) = gltf::import(path).expect("failed to load file");
//         let mut comps: Vec<&str> = path.split("/").collect();
//         comps.pop();
//         let dir = comps.join("/");

//         if let Some(default_scene) = document.default_scene() {
//             igltf.scene = Some(default_scene.index());
//         }

//         IGltf::process_meshes(&document, &buffers, &mut igltf);
//         IGltf::process_materials(&document, &mut igltf);
//         IGltf::process_textures(&document, &mut igltf);
//         IGltf::process_samplers(&document, &mut igltf);
//         IGltf::process_images(&document, &mut igltf, &dir.as_str());
//         IGltf::process_nodes(&document, &mut igltf);
//         IGltf::process_scenes(&document, &mut igltf);
//         igltf
//     }

//     pub fn process_scenes(document: &Document, igltf: &mut IGltf) {
//         let scenes = document.scenes();
//         for scene in scenes {
//             let mut iscene = IScene { nodes: Vec::new() };
//             for node in scene.nodes() {
//                 iscene.nodes.push(node.index());
//             }
//             igltf.scenes.push(iscene);
//         }
//     }

//     pub fn process_nodes(document: &Document, igltf: &mut IGltf) {
//         let nodes = document.nodes();
//         let size = nodes.len();
//         igltf.nodes.resize(
//             size,
//             ISceneNode {
//                 parent: None,
//                 children: Vec::new(),
//                 mesh: None,
//             },
//         );
//         for node in nodes {
//             IGltf::process_node(&node, None, igltf);
//         }
//     }

//     pub fn process_node(node: &gltf::Node, parent: Option<usize>, igltf: &mut IGltf) {
//         let index = node.index();
//         for child in node.children() {
//             igltf.nodes[index].children.push(child.index());
//         }
//         if let Some(mesh) = node.mesh() {
//             igltf.nodes[index].mesh = Some(mesh.index());
//         }

//         // TODO: handle camera / transform etc.
//     }

//     pub fn process_meshes(document: &Document, buffers: &Vec<Data>, igltf: &mut IGltf) {
//         let meshes = document.meshes();
//         let size = meshes.len();
//         igltf.meshes.resize(
//             size,
//             IMesh {
//                 primitives: Vec::new(),
//             },
//         );
//         for mesh in meshes {
//             let index = mesh.index();
//             for primitive in mesh.primitives() {
//                 let mut positions: Vec<Vec3> = Vec::new();
//                 let mut indices: Vec<usize> = Vec::new();
//                 let mut tex_coords: Vec<Vec2> = Vec::new();
//                 let material = primitive.material().index();
//                 let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
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
//                         indices.push(indice as usize);
//                     }
//                 }
//                 if let Some(iter) = reader.read_tex_coords(0) {
//                     for tex_coord in iter.into_f32() {
//                         tex_coords.push(Vec2::new(tex_coord[0], tex_coord[1]))
//                     }
//                 }
//                 igltf.meshes[index].primitives.push(IMeshPrimitive {
//                     positions,
//                     indices,
//                     tex_coords,
//                     material,
//                 })
//             }
//         }
//     }

//     pub fn process_materials(document: &Document, igltf: &mut IGltf) {
//         let materials = document.materials();
//         for material in materials {
//             // let base_color_factor = material.pbr_metallic_roughness().base_color_factor();
//             let base_color_texture = material
//                 .pbr_metallic_roughness()
//                 .base_color_texture()
//                 .expect("base color texture missing")
//                 .texture()
//                 .index();
//             let imaterial = IMaterial { base_color_texture };
//             igltf.materials.push(imaterial);
//         }
//     }

//     pub fn process_textures(document: &Document, igltf: &mut IGltf) {
//         let textures = document.textures();
//         let size = textures.len();
//         igltf.textures.resize(
//             size,
//             ITexture {
//                 sampler: 0,
//                 image: 0,
//             },
//         );
//         for texture in textures {
//             let index = texture.index();
//             let sampler_index = texture.sampler().index().or(Some(0)).unwrap();
//             let image_index = texture.source().index();
//             igltf.textures[index] = ITexture {
//                 sampler: sampler_index,
//                 image: image_index,
//             }
//         }
//     }

//     pub fn process_samplers(document: &Document, igltf: &mut IGltf) {
//         let samplers = document.samplers();
//         let size = samplers.len();
//         igltf.samplers.resize(
//             size,
//             TextureSampler {
//                 wrap_s: WrappingMode::Repeat,
//                 wrap_t: WrappingMode::Repeat,
//                 min_filter: MinFilter::Linear,
//                 mag_filter: MagFilter::Linear,
//             },
//         );
//         for sampler in samplers {
//             let index = sampler.index().or(Some(0)).expect("need sampler index");

//             let mut min_filter: MinFilter = MinFilter::Linear;
//             let mut mag_filter: MagFilter = MagFilter::Nearest;
//             if let Some(filter) = sampler.min_filter() {
//                 min_filter = filter;
//             }
//             if let Some(filter) = sampler.mag_filter() {
//                 mag_filter = filter;
//             }
//             let wrap_s = sampler.wrap_s();
//             let wrap_t = sampler.wrap_t();

//             igltf.samplers[index] = TextureSampler {
//                 min_filter,
//                 mag_filter,
//                 wrap_s,
//                 wrap_t,
//             }
//         }
//     }

//     pub fn process_images(document: &Document, igltf: &mut IGltf, dir: &str) {
//         let images = document.images();
//         let size = images.len();
//         igltf.images.resize(size, Texture::new());
//         for image in images {
//             match image.source() {
//                 Source::Uri { uri, mime_type } => {
//                     let image_path = [dir, uri].join("/");
//                     let tex = Texture::from_file(&image_path);
//                     igltf.images[image.index()] = tex;
//                 }
//                 Source::View { view, mime_type } => {
//                     todo!("TODO: handle source view")
//                 }
//             }
//         }
//     }
// }
