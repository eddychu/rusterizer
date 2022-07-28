use std::{fs, io};

use gltf::{self, buffer, buffer::Data, Gltf};

use crate::math::{Vec2, Vec3};

pub struct Vertex {
    pos: Vec3,
    tc: Option<Vec2>,
    normals: Option<Vec3>,
    tangent: Option<Vec3>,
    bitangent: Option<Vec3>,
}

pub struct Model {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Model {
    pub fn from_file(file_path: &str) {
        let (gltf, buffers, _) = gltf::import(&file_path).expect("failed to load file");
        for scene in gltf.scenes() {
            println!("Scene {}", scene.index());
            for node in scene.nodes() {
                Model::print_tree(&node, 1, &buffers);
            }
        }
    }

    pub fn print_tree(node: &gltf::Node, depth: i32, buffers: &Vec<Data>) {
        for _ in 0..(depth - 1) {
            print!("  ");
        }
        print!(" -");

        println!(" Node {}", node.index());
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(iter) = reader.read_positions() {
                    for vertex_position in iter {
                        println!("{:?}", vertex_position);
                    }
                }
                if let Some(iter) = reader.read_indices() {
                    let iter = iter.into_u32();
                    for indice in iter {
                        println!("{}", indice);
                    }
                }
            }
        }
        for child in node.children() {
            Model::print_tree(&child, depth + 1, &buffers);
        }
    }
}
