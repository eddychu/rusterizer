use rayon::vec;

#[derive(Clone)]

pub struct Tile {
    pub x_offset: usize,
    pub y_offset: usize,
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
    pub depths: Vec<f32>,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            x_offset: 0,
            y_offset: 0,
            width: 0,
            height: 0,
            pixels: Vec::new(),
            depths: Vec::new(),
        }
    }
}

/**
 * num has to be a sqr of int such as 4, 16,
 */
pub fn generate_tiles(width: usize, height: usize, num: usize) -> Vec<Tile> {
    let num_sqrt = (num as f32).sqrt() as usize;
    let tile_width = (width / num_sqrt) as usize;
    let tile_height = (height / num_sqrt) as usize;
    let mut tiles = vec![Tile::new(); num];
    let mut x_offset = 0usize;
    let mut y_offset = 0usize;
    let mut index = 0;
    for i in 0..num_sqrt {
        for j in 0..num_sqrt {
            x_offset += i * tile_width;
            y_offset += j * tile_height;
            tiles[index].x_offset = x_offset;
            tiles[index].y_offset = y_offset;
            tiles[index].width = tile_width;
            tiles[index].height = tile_height;
            tiles[index].pixels = vec![0; tile_width * tile_height];
            tiles[index].depths = vec![1.0; tile_height * tile_width];
            index += 1;
        }
    }
    tiles
}
