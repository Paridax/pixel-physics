pub const WIDTH: i32 = 256;
pub const HEIGHT: i32 = 256;
pub const SCALE_FACTOR: i32 = 3;

pub const AIR: i32 = 0;
pub const ROCK: i32 = 1;
pub const SAND: i32 = 2;
pub const WATER: i32 = 3;

pub const D_AIR: i32 = 0;
pub const D_ROCK: i32 = 3;
pub const D_SAND: i32 = 2;
pub const D_WATER: i32 = 1;

pub const COL_AIR: [f32; 4] = [0.0; 4];
pub const COL_ROCK: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
pub const COL_SAND: [f32; 4] = [1.0, 0.53, 0.0, 1.0];
pub const COL_WATER: [f32; 4] = [0.0, 0.25, 1.0, 1.0];

const X: usize = 0;
const Y: usize = 1;

// edit the world array
pub fn create_pixel(world: &mut[(u8, u8, u8, [u32; 2]); (WIDTH * HEIGHT) as usize],
                t: i32, cycle: i32, dens: i32, x: i32, y: i32) {
    world[coord(x, y)] = (t as u8, cycle as u8, dens as u8, [0, 0]);
}

pub fn pixel(x: i32, y: i32) -> [f64; 4] {
    return [(x * SCALE_FACTOR) as f64, ((HEIGHT - y) * SCALE_FACTOR) as f64, SCALE_FACTOR as f64, SCALE_FACTOR as f64]
}

// turn x, y into array loc
pub fn coord(x: i32, y: i32) -> usize
{
    let res = (x + y * WIDTH) as u64;
    return res as usize;
}

// turn an array location back into x and y
pub fn loc(array_pos: u64) -> (i32, i32) {
    let x = array_pos % WIDTH as u64;
    let y = (array_pos / WIDTH as u64) as i32;
    (x as i32, y)
}

// get pixel info
pub fn get_type(pixel: (u8, u8, u8, [u32; 2])) -> i32 {
    pixel.0 as i32
}

pub fn get_cycle(pixel: (u8, u8, u8, [u32; 2])) -> i32 {
    pixel.1 as i32
}

pub fn get_density(pixel: (u8, u8, u8, [u32; 2])) -> i32 {
    pixel.2 as i32
}

pub fn get_vector(pixel: (u8, u8, u8, [u32; 2])) -> [i32; 2] {
    [pixel.3[X] as i32, pixel.3[Y] as i32]
}

pub fn get_vector_x(pixel: (u8, u8, u8, [u32; 2])) -> i32 {
    pixel.3[X] as i32
}

pub fn get_vector_y(pixel: (u8, u8, u8, [u32; 2])) -> i32 {
    pixel.3[Y] as i32
}

// pixel comparison
pub fn match_type(pixel: (u8, u8, u8, [u32; 2]), t: i32) -> bool {
    pixel.0 as i32 == t
}

pub fn match_cycle(pixel: (u8, u8, u8, [u32; 2]), c: i32) -> bool {
    pixel.1 as i32 == c
}

pub fn compare_density(pixel: (u8, u8, u8, [u32; 2]), d: i32) -> bool {
    pixel.2 as i32 >= d
}