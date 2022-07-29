extern crate minifb;
mod framebuffer;
mod loader;
mod math;
mod model;
mod renderstate;
mod scene;
mod texture;
use framebuffer::FrameBuffer;
use gltf::Mesh;
use image::Frame;
use loader::{IGltf, IMesh};
use math::{Mat4, Quat, Vec2, Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};
use renderstate::RenderState;
use scene::Scene;
use texture::Texture;
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn barycentric(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> Vec3 {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;
    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);
    let denom = d00 * d11 - d01 * d01;
    // if denom < f32::EPSILON {
    //     println!("{:?}", denom);
    // }
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    return Vec3::new(u, v, w);
}

fn render(igltf: &IGltf, buffer: &mut FrameBuffer, state: &RenderState) {
    let scene = &igltf.scenes[0];
    for node in scene.nodes.iter() {
        let node = &igltf.nodes[node.clone()];
        if let Some(mesh) = node.mesh {
            println!("{:?}", mesh);
            let mesh = &igltf.meshes[mesh];
            render_mesh(mesh, buffer, state);
        }
        for child in node.children.iter() {
            let child_node = &igltf.nodes[child.clone()];
            if let Some(mesh) = child_node.mesh {
                let mesh = &igltf.meshes[mesh];
                render_mesh(mesh, buffer, state)
            }
        }
    }
}

fn render_mesh(mesh: &IMesh, buffer: &mut FrameBuffer, state: &RenderState) {
    for primitive in mesh.primitives.iter() {
        println!("{:?}", primitive);
        for i in (0..(primitive.indices.len() - 2)).step_by(3) {
            let i0 = primitive.indices[i];
            let i1 = primitive.indices[i + 1];
            let i2 = primitive.indices[i + 2];

            let v0 = primitive.positions[i0 as usize];
            let v1 = primitive.positions[i1 as usize];
            let v2 = primitive.positions[i2 as usize];

            let t0 = primitive.tex_coords[i0 as usize];
            let t1 = primitive.tex_coords[i1 as usize];
            let t2 = primitive.tex_coords[i2 as usize];

            let mut clip_v0 = state.mvp * Vec4::from_vec3(&v0, 1.0);
            let mut clip_v1 = state.mvp * Vec4::from_vec3(&v1, 1.0);
            let mut clip_v2 = state.mvp * Vec4::from_vec3(&v2, 1.0);

            // perspective devide
            clip_v0 = clip_v0 * (1.0 / clip_v0.w);
            clip_v1 = clip_v1 * (1.0 / clip_v1.w);
            clip_v2 = clip_v2 * (1.0 / clip_v2.w);

            // view_port_transform
            clip_v0.x = (clip_v0.x + 1.0) * 0.5 * WIDTH as f32;
            clip_v0.y = (1.0 - clip_v0.y) * 0.5 * HEIGHT as f32;
            clip_v1.x = (clip_v1.x + 1.0) * 0.5 * WIDTH as f32;
            clip_v1.y = (1.0 - clip_v1.y) * 0.5 * HEIGHT as f32;
            clip_v2.x = (clip_v2.x + 1.0) * 0.5 * WIDTH as f32;
            clip_v2.y = (1.0 - clip_v2.y) * 0.5 * HEIGHT as f32;

            let c0: Vec3 = Vec3::new(1.0, 0.0, 0.0);
            let c1: Vec3 = Vec3::new(0.0, 1.0, 0.0);
            let c2: Vec3 = Vec3::new(0.0, 0.0, 1.0);
            let min_x = clip_v0.x.min(clip_v1.x).min(clip_v2.x).max(0.0) as usize;
            let min_y = clip_v0.y.min(clip_v1.y).min(clip_v2.y).max(0.0) as usize;
            let max_x = clip_v0
                .x
                .max(clip_v1.x)
                .max(clip_v2.x)
                .min((WIDTH - 1) as f32) as usize;
            let max_y = clip_v0
                .y
                .max(clip_v1.y)
                .max(clip_v2.y)
                .min((HEIGHT - 1) as f32) as usize;

            for x in min_x..max_x + 1 {
                for y in min_y..max_y + 1 {
                    let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                    let a = Vec2::new(clip_v0.x, clip_v0.y);
                    let b = Vec2::new(clip_v1.x, clip_v1.y);
                    let c = Vec2::new(clip_v2.x, clip_v2.y);
                    let bar = barycentric(p, a, b, c);

                    if bar.x >= 0.0 && bar.y >= 0.0 && bar.z >= 0.0 {
                        let z = bar.x * clip_v0.z + bar.y * clip_v1.z + bar.z * clip_v2.z;
                        let t = t0 * bar.x + t1 * bar.y + t2 * bar.z;
                        if z < buffer.get_depth(x, y) {
                            buffer.set_depth(x, y, z);
                            let color = state.albedo.sample_repeat(&t);

                            // println!("{:?}", color);
                            buffer.write_pixel_vec3(x, y, color.to_vec3());
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let igltf = IGltf::load("asset/2.0/BoxTextured/glTF/BoxTextured.gltf");
    let mut buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let projection = Mat4::perspective(60.0, (WIDTH as f32) / (HEIGHT as f32), 0.1, 100.0);
    let view = Mat4::lookat(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let albedo = Texture::from_file("asset/2.0/BoxTextured/glTF/CesiumLogoFlat.png");

    let mut state = RenderState {
        projection,
        view,
        mv: view,
        mvp: projection * view,
        albedo,
    };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut angles = 0.0f32;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way

        let rotation_quat = Quat::from_angle_axis(angles, Vec3::new(1.0, 1.0, 0.0));
        state.mvp = projection * view * (rotation_quat).to_mat4();
        buffer.clear();
        render(&igltf, &mut buffer, &state);
        // break;
        window
            .update_with_buffer(&buffer.pixels, WIDTH, HEIGHT)
            .unwrap();
        angles += 0.02;
    }
}
