use gltf::image::{Data, Format};
use image::GenericImageView;
use image::{self, Pixel};

use crate::math::{Vec2, Vec4};

#[derive(Debug, Clone)]
pub struct Texture {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    // pub fn from_data(data: &Data) {
    //     let format = data.format;
    //     if format == Format::R8G8B8A8 {

    //     }
    // }

    pub fn from_file(file_path: &str) -> Self {
        let img = image::open(file_path).expect("File not found!");
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut data: Vec<f32> = Vec::with_capacity(width * height);
        for pixel in img.pixels() {
            let rgba = pixel.2.channels();
            // TODO: not handling gamma correction at this point
            data.push(rgba[0] as f32 / 255.0);
            data.push(rgba[1] as f32 / 255.0);
            data.push(rgba[2] as f32 / 255.0);
            data.push(rgba[3] as f32 / 255.0);
        }
        Texture {
            data,
            width,
            height,
        }
    }

    pub fn sample_repeat(&self, texcoord: &Vec2) -> Vec4 {
        let u = texcoord.x - texcoord.x.floor();
        let v = texcoord.y - texcoord.y.floor();
        self.sample_point(u, v)
    }

    pub fn sample_clamp(&self, texcoord: &Vec2) -> Vec4 {
        let u = texcoord.x.max(0.0).min(1.0);
        let v = texcoord.y.max(0.0).min(1.0);
        self.sample_point(u, v)
    }

    pub fn sample_point(&self, u: f32, v: f32) -> Vec4 {
        let c = ((self.width as f32 - 1.0) * u) as usize;
        let r = ((self.height as f32 - 1.0) * v) as usize;
        let index = (r * self.width + c) * 4;
        Vec4::new(
            self.data[index],
            self.data[index + 1],
            self.data[index + 2],
            self.data[index + 3],
        )
    }
}
