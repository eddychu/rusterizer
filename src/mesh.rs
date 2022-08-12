use crate::{
    math::{Vec2, Vec3, Vec4},
    renderstate::RenderState,
};

use rayon::prelude::*;

fn orient2d(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    return (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x);
}

pub struct Vertex {
    pub position: Vec3,
    pub tex_coord: Option<Vec2>,
    pub normal: Option<Vec3>,
    pub tangent: Option<Vec3>,
    pub bitanget: Option<Vec3>,
}

pub trait Mesh {
    fn indice(&self, i: usize) -> usize;
    fn vertex(&self, i: usize) -> &Vertex;
    fn num_of_indices(&self) -> usize;
    fn num_of_vertices(&self) -> usize;
    fn draw(&self, state: &mut RenderState);
}

pub struct Cube {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<usize>,
}

impl Cube {
    pub fn new(side: f32) -> Self {
        let side2 = side / 2.0;
        let p = vec![
            // Front
            Vec3::new(-side2, -side2, side2),
            Vec3::new(side2, -side2, side2),
            Vec3::new(side2, side2, side2),
            Vec3::new(-side2, side2, side2),
            // Right
            Vec3::new(side2, -side2, side2),
            Vec3::new(side2, -side2, -side2),
            Vec3::new(side2, side2, -side2),
            Vec3::new(side2, side2, side2),
            // Back
            Vec3::new(-side2, -side2, -side2),
            Vec3::new(-side2, side2, -side2),
            Vec3::new(side2, side2, -side2),
            Vec3::new(side2, -side2, -side2),
            // Left
            Vec3::new(-side2, -side2, side2),
            Vec3::new(-side2, side2, side2),
            Vec3::new(-side2, side2, -side2),
            Vec3::new(-side2, -side2, -side2),
            // Bottom
            Vec3::new(-side2, -side2, side2),
            Vec3::new(-side2, -side2, -side2),
            Vec3::new(side2, -side2, -side2),
            Vec3::new(side2, -side2, side2),
            // Top
            Vec3::new(-side2, side2, side2),
            Vec3::new(side2, side2, side2),
            Vec3::new(side2, side2, -side2),
            Vec3::new(-side2, side2, -side2),
        ];

        let n = vec![
            // Front
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0), // Right
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0), // Back
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0), // Left
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0), // Bottom
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0), // Top
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let tc = vec![
            // Front
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Right
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Back
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Left
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Bottom
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
            // Top
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];

        let i = vec![
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
        ];

        let mut vertices: Vec<Vertex> = Vec::with_capacity(p.len());
        for i in 0..p.len() {
            let vertex = Vertex {
                position: p[i],
                tex_coord: Some(tc[i]),
                normal: Some(n[i]),
                tangent: None,
                bitanget: None,
            };
            vertices.push(vertex);
        }

        Cube {
            vertices,
            indices: i,
        }
    }

    fn draw_experiment(&self, state: &mut RenderState) {
        self.indices
            .par_chunks(3)
            .map(|indices| {
                let i0 = indices[0];
                let i1 = indices[1];
                let i2 = indices[2];
                // println!("{} {} {}", i, i + 1, i + 2);
                let v0 = self.vertex(i0).position;
                let v1 = self.vertex(i1).position;
                let v2 = self.vertex(i2).position;

                let t0 = self.vertex(i0).tex_coord.unwrap();
                let t1 = self.vertex(i1).tex_coord.unwrap();
                let t2 = self.vertex(i2).tex_coord.unwrap();

                let mut clip_v0 = state.mvp * Vec4::from_vec3(&v0, 1.0);
                let mut clip_v1 = state.mvp * Vec4::from_vec3(&v1, 1.0);
                let mut clip_v2 = state.mvp * Vec4::from_vec3(&v2, 1.0);

                // perspective devide
                clip_v0 = Vec4::new(
                    clip_v0.x / clip_v0.w,
                    clip_v0.y / clip_v0.w,
                    clip_v0.z / clip_v0.w,
                    clip_v0.w,
                );
                clip_v1 = Vec4::new(
                    clip_v1.x / clip_v1.w,
                    clip_v1.y / clip_v1.w,
                    clip_v1.z / clip_v1.w,
                    clip_v1.w,
                );
                clip_v2 = Vec4::new(
                    clip_v2.x / clip_v2.w,
                    clip_v2.y / clip_v2.w,
                    clip_v2.z / clip_v2.w,
                    clip_v2.w,
                );

                // println!("{:?} {:?} {:?}", clip_v0, clip_v1, clip_v2);

                let width = state.target.width as f32;
                let height = state.target.height as f32;
                // view_port_transform
                clip_v0.x = (clip_v0.x + 1.0) * 0.5 * width;
                clip_v0.y = (1.0 - clip_v0.y) * 0.5 * height;
                clip_v1.x = (clip_v1.x + 1.0) * 0.5 * width;
                clip_v1.y = (1.0 - clip_v1.y) * 0.5 * height;
                clip_v2.x = (clip_v2.x + 1.0) * 0.5 * width;
                clip_v2.y = (1.0 - clip_v2.y) * 0.5 * height;

                // let c0: Vec3 = Vec3::new(1.0, 0.0, 0.0);
                // let c1: Vec3 = Vec3::new(0.0, 1.0, 0.0);
                // let c2: Vec3 = Vec3::new(0.0, 0.0, 1.0);
                let min_x = clip_v0.x.min(clip_v1.x).min(clip_v2.x).max(0.0) as usize;
                let min_y = clip_v0.y.min(clip_v1.y).min(clip_v2.y).max(0.0) as usize;
                let max_x = clip_v0.x.max(clip_v1.x).max(clip_v2.x).min(width - 1.0) as usize;
                let max_y = clip_v0.y.max(clip_v1.y).max(clip_v2.y).min(height - 1.0) as usize;

                let a = Vec2::new(clip_v0.x + 0.5, clip_v0.y + 0.5);
                let b = Vec2::new(clip_v1.x + 0.5, clip_v1.y + 0.5);
                let c = Vec2::new(clip_v2.x + 0.5, clip_v2.y + 0.5);

                [a, b, c]
            })
            .filter(|screen_coords| {
                orient2d(screen_coords[0], screen_coords[1], screen_coords[2]) > 0.0
            })
            .map(|screen_coords| {});
    }
}

impl Mesh for Cube {
    fn indice(&self, i: usize) -> usize {
        return self.indices[i];
    }

    fn vertex(&self, i: usize) -> &Vertex {
        return &self.vertices[i];
    }

    fn num_of_indices(&self) -> usize {
        self.indices.len()
    }

    fn num_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn draw(&self, state: &mut RenderState) {
        for i in (0..self.num_of_indices() - 2).step_by(3) {
            let i0 = self.indice(i);
            let i1 = self.indice(i + 1);
            let i2 = self.indice(i + 2);
            // println!("{} {} {}", i, i + 1, i + 2);
            let v0 = self.vertex(i0).position;
            let v1 = self.vertex(i1).position;
            let v2 = self.vertex(i2).position;

            let t0 = self.vertex(i0).tex_coord.unwrap();
            let t1 = self.vertex(i1).tex_coord.unwrap();
            let t2 = self.vertex(i2).tex_coord.unwrap();

            let mut clip_v0 = state.mvp * Vec4::from_vec3(&v0, 1.0);
            let mut clip_v1 = state.mvp * Vec4::from_vec3(&v1, 1.0);
            let mut clip_v2 = state.mvp * Vec4::from_vec3(&v2, 1.0);

            // perspective devide
            clip_v0 = Vec4::new(
                clip_v0.x / clip_v0.w,
                clip_v0.y / clip_v0.w,
                clip_v0.z / clip_v0.w,
                clip_v0.w,
            );
            clip_v1 = Vec4::new(
                clip_v1.x / clip_v1.w,
                clip_v1.y / clip_v1.w,
                clip_v1.z / clip_v1.w,
                clip_v1.w,
            );
            clip_v2 = Vec4::new(
                clip_v2.x / clip_v2.w,
                clip_v2.y / clip_v2.w,
                clip_v2.z / clip_v2.w,
                clip_v2.w,
            );

            // println!("{:?} {:?} {:?}", clip_v0, clip_v1, clip_v2);

            let width = state.target.width as f32;
            let height = state.target.height as f32;
            // view_port_transform
            clip_v0.x = (clip_v0.x + 1.0) * 0.5 * width;
            clip_v0.y = (1.0 - clip_v0.y) * 0.5 * height;
            clip_v1.x = (clip_v1.x + 1.0) * 0.5 * width;
            clip_v1.y = (1.0 - clip_v1.y) * 0.5 * height;
            clip_v2.x = (clip_v2.x + 1.0) * 0.5 * width;
            clip_v2.y = (1.0 - clip_v2.y) * 0.5 * height;

            // let c0: Vec3 = Vec3::new(1.0, 0.0, 0.0);
            // let c1: Vec3 = Vec3::new(0.0, 1.0, 0.0);
            // let c2: Vec3 = Vec3::new(0.0, 0.0, 1.0);
            let min_x = clip_v0.x.min(clip_v1.x).min(clip_v2.x).max(0.0) as usize;
            let min_y = clip_v0.y.min(clip_v1.y).min(clip_v2.y).max(0.0) as usize;
            let max_x = clip_v0.x.max(clip_v1.x).max(clip_v2.x).min(width - 1.0) as usize;
            let max_y = clip_v0.y.max(clip_v1.y).max(clip_v2.y).min(height - 1.0) as usize;

            let a = Vec2::new(clip_v0.x + 0.5, clip_v0.y + 0.5);
            let b = Vec2::new(clip_v1.x + 0.5, clip_v1.y + 0.5);
            let c = Vec2::new(clip_v2.x + 0.5, clip_v2.y + 0.5);

            let area = orient2d(a, b, c);
            if area <= 0.0 {
                continue;
            }

            for x in min_x..max_x + 1 {
                for y in min_y..max_y + 1 {
                    let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                    let mut w0 = orient2d(b, c, p);
                    let mut w1 = orient2d(c, a, p);
                    let mut w2 = orient2d(a, b, p);

                    // let bary = barycentric(p, a, b, c);

                    if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                        w0 /= area;
                        w1 /= area;
                        w2 /= area;

                        let z = w0 * clip_v0.z + w1 * clip_v1.z + w2 * clip_v2.z;

                        if z < state.target.get_depth(x, y) {
                            let w0_perp = w0 / clip_v0.w;
                            let w1_perp = w1 / clip_v1.w;
                            let w2_perp = w2 / clip_v2.w;
                            let l = w0_perp + w1_perp + w2_perp;
                            let w0_perp = w0_perp / l;
                            let w1_perp = w1_perp / l;
                            let w2_perp = w2_perp / l;

                            let t = t0 * w0_perp + t1 * w1_perp + t2 * w2_perp;

                            state.target.set_depth(x, y, z);
                            let color = state.albedo.sample_repeat(&t);

                            // println!("{:?}", color);
                            state.target.write_pixel_vec3(x, y, color.to_vec3());
                        }
                    }
                }
            }
        }
    }
}
