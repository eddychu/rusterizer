use std::os::windows::process;

use crate::{math::Vec2, math::Vec3, texture::Texture};
use gltf::{
    self,
    buffer::Data,
    image::Source,
    texture::{MagFilter, MinFilter, Sampler, WrappingMode},
    Document, Gltf,
};
struct IMesh {
    position: Vec<Vec3>,
    indices: Vec<usize>,
    material: Option<usize>,
}

struct ITexture {
    sampler: usize,
    image: usize,
}

struct IMaterial {
    base_color_texture: usize,
}

struct ISceneNode {
    parent: Option<usize>,
    children: Vec<usize>,
    meshes: Vec<usize>,
}

// pub enum MagFilter {
//     Nearest,
//     Linear,
// }

// pub enum MinFilter {
//     Nearest,
//     Linear,
//     NearestMipmapNearest,
//     LinearMipmapNearest,
//     NearestMipmapLinear,
//     LinearMipmapLinear,
// }

// pub enum WrappingMode {
//     ClampToEdge,
//     MirroredRepeat,
//     Repeat,
// }

struct TextureSampler {
    wrap_s: WrappingMode,
    wrap_t: WrappingMode,
    min_filter: Option<MinFilter>,
    mag_filter: Option<MagFilter>,
}

impl ISceneNode {
    pub fn new() -> Self {
        ISceneNode {
            parent: None,
            children: Vec::new(),
            meshes: Vec::new(),
        }
    }
}

struct IScene {
    nodes: Vec<ISceneNode>,
    meshes: Vec<IMesh>,
    textures: Vec<ITexture>,
    materials: Vec<IMaterial>,
    samplers: Vec<TextureSampler>,
}

impl IScene {
    pub fn new() -> Self {
        IScene {
            nodes: Vec::new(),
            meshes: Vec::new(),
            textures: Vec::new(),
            materials: Vec::new(),
            samplers: Vec::new(),
        }
    }

    pub fn load(path: &str) {
        let mut scene = IScene::new();
        let (document, buffers, _) = gltf::import(path).expect("failed to load file");
        IScene::process_meshes(&document, &buffers, &mut scene);
        IScene::process_materials(&document, &mut scene);
        IScene::process_textures(&document, &mut scene);
    }

    pub fn process_meshes(document: &Document, buffers: &Vec<Data>, scene: &mut IScene) {
        let meshes = document.meshes();
        for mesh in meshes {
            let mut positions: Vec<Vec3> = Vec::new();
            let mut indicies: Vec<usize> = Vec::new();
            let mut tc: Vec<Vec2> = Vec::new();
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    for vertex_position in iter {
                        positions.push(Vec3::new(
                            vertex_position[0],
                            vertex_position[1],
                            vertex_position[2],
                        ))
                    }
                }
                if let Some(iter) = reader.read_indices() {
                    let iter = iter.into_u32();
                    for indice in iter {
                        indicies.push(indice as usize);
                    }
                }
                if let Some(iter) = reader.read_tex_coords(0) {
                    for tex_coord in iter.into_f32() {
                        tc.push(Vec2::new(tex_coord[0], tex_coord[1]))
                    }
                }
            }
            let imesh = IMesh {
                position: positions,
                indices: indicies,
                material: None,
            };
            scene.meshes.push(imesh);
        }
    }

    pub fn process_materials(document: &Document, scene: &mut IScene) {
        let materials = document.materials();
        for material in materials {
            let base_color_factor = material.pbr_metallic_roughness().base_color_factor();
            let base_color_texture = material
                .pbr_metallic_roughness()
                .base_color_texture()
                .expect("base color texture missing")
                .texture()
                .index();
            let imaterial = IMaterial { base_color_texture };
            scene.materials.push(imaterial);
        }
    }

    pub fn process_textures(document: &Document, scene: &mut IScene) {
        let textures = document.textures();
        let size = textures.len();
        scene.textures = Vec::with_capacity(size);
        for texture in textures {
            let index = texture.index();
            let sampler_index = texture.sampler().index().or(Some(0)).unwrap();
            let image_index = texture.source().index();
            scene.textures[index] = ITexture {
                sampler: sampler_index,
                image: image_index,
            }
        }
    }

    pub fn process_samplers(document: &Document, scene: &mut IScene) {
        let samplers = document.samplers();
        let size = samplers.len();
        scene.samplers = Vec::with_capacity(size);
        for sampler in samplers {
            let index = sampler.index().or(Some(0)).expect("need sampler index");
            let min_filter = sampler.min_filter();
            let mag_filter = sampler.mag_filter();
            let wrap_s = sampler.wrap_s();
            let wrap_t = sampler.wrap_t();

            scene.samplers[index] = TextureSampler {
                min_filter,
                mag_filter,
                wrap_s,
                wrap_t,
            }
        }
    }

    pub fn process_images(document: &Document, scene: &mut IScene) {
        let images = document.images();
    }

    pub fn process_node(
        gltf_node: &gltf::Node,
        gltf_buffers: &Vec<Data>,
        parent: Option<usize>,
        scene: &mut Self,
        dir: &str,
    ) -> usize {
        let mut node = ISceneNode::new();
        node.parent = parent;
        scene.nodes.push(node);
        let current_index = scene.nodes.len() - 1;
        if let Some(mesh) = gltf_node.mesh() {
            let mut positions: Vec<Vec3> = Vec::new();
            let mut indicies: Vec<u32> = Vec::new();
            let mut tc: Vec<Vec2> = Vec::new();
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&gltf_buffers[buffer.index()]));

                if let Some(iter) = reader.read_positions() {
                    for vertex_position in iter {
                        positions.push(Vec3::new(
                            vertex_position[0],
                            vertex_position[1],
                            vertex_position[2],
                        ))
                    }
                }
                if let Some(iter) = reader.read_indices() {
                    let iter = iter.into_u32();
                    for indice in iter {
                        indicies.push(indice);
                    }
                }
                if let Some(iter) = reader.read_tex_coords(0) {
                    for tex_coord in iter.into_f32() {
                        tc.push(Vec2::new(tex_coord[0], tex_coord[1]))
                    }
                }
                // let mut material: Option<Material> = None;
                // if let Some(base_color) = primitive
                //     .material()
                //     .pbr_metallic_roughness()
                //     .base_color_texture()
                // {
                //     let image = base_color.texture().source().index();
                // }
                if positions.len() > 0 {
                    scene.nodes[current_index].meshes.push(Mesh {
                        vertices: positions.clone(),
                        indicies: indicies.clone(),
                        tex_coords: tc.clone(),
                        material: None,
                    })
                }
            }
        }

        for child in gltf_node.children() {
            let child_index =
                Scene::process_node(&child, &gltf_buffers, Some(current_index), scene, dir);
            scene.nodes[current_index].children.push(child_index);
        }
        current_index
    }
}
