use crate::math::Vec3;

pub struct FrameBuffer {
    pub pixels: Vec<u32>,
    pub depths: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub gamma: f32,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let length = width * height;
        FrameBuffer {
            pixels: vec![0; length],
            depths: vec![1.0f32; length],
            width,
            height,
            gamma: 2.2,
        }
    }

    pub fn clear(&mut self) {
        let dim = self.width * self.height;
        self.pixels = vec![0; dim];
        self.depths = vec![1.0f32; dim];
    }

    fn map_vec3_to_u32(&self, color: Vec3) -> u32 {
        let srgb = color.pow(1.0 / self.gamma);
        let (r, g, b) = (
            (srgb.x * 255.9f32) as u32,
            (srgb.y * 255.9f32) as u32,
            (srgb.z * 255.9f32) as u32,
        );

        let res = r << 16 | g << 8 | b;
        // println!("{:?}", res);
        return res;
    }

    pub fn write_pixel_vec3(&mut self, x: usize, y: usize, color: Vec3) {
        self.write_pixel(x, y, self.map_vec3_to_u32(color));
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, value: u32) {
        let index = y * self.width + x;
        self.pixels[index] = value;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        let index = y * self.width + x;
        self.pixels[index]
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        let index = y * self.width + x;
        self.depths[index]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, value: f32) {
        let index = y * self.width + x;
        self.depths[index] = value;
    }
}
