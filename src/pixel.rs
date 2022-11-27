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

pub const COL_AIR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const COL_ROCK: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
pub const COL_SAND: [f32; 4] = [1.0, 0.53, 0.0, 1.0];
pub const COL_WATER: [f32; 4] = [0.0, 0.25, 1.0, 1.0];

const X: usize = 0;
const Y: usize = 1;

// edit the world array
pub fn create_pixel(world: &mut[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize],
                t: i32, cycle: i32, dens: i32, x: i32, y: i32) {
    let mut vector = [0; 2];
    if t == WATER || t == SAND {
        vector = [0, -1];
    }
    return world[coord(x, y)] = (t as u8, cycle as u8, dens as u8, vector, true);
}

pub fn draw_pixel(x: i32, y: i32) -> [f64; 4] {
    return [(x * SCALE_FACTOR) as f64, ((HEIGHT - y - 1) * SCALE_FACTOR) as f64, SCALE_FACTOR as f64, SCALE_FACTOR as f64];
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
    return (x as i32, y);
}

// get pixel info
pub fn pixel_at(world: &[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize], x: i32, y: i32) -> (u8, u8, u8, [i32; 2], bool) {
    // if the pixel is out of bounds, return air
    if x < 0 || x >= WIDTH || y < 0 || y >= HEIGHT {
        return (ROCK as u8, 0, 3, [0, 0], false);
    }
    return world[coord(x, y)];
}

pub fn get_type(pixel: (u8, u8, u8, [i32; 2], bool)) -> i32 {
    return pixel.0 as i32;
}

pub fn get_update_state(pixel: (u8, u8, u8, [i32; 2], bool)) -> bool {
    return pixel.4;
}

pub fn get_cycle(pixel: (u8, u8, u8, [i32; 2], bool)) -> i32 {
    return pixel.1 as i32;
}

pub fn get_density(pixel: (u8, u8, u8, [i32; 2], bool)) -> i32 {
    return pixel.2 as i32;
}

pub fn get_vector(pixel: (u8, u8, u8, [i32; 2], bool)) -> [i32; 2] {
    return [pixel.3[X] as i32, pixel.3[Y] as i32];
}

pub fn get_vector_x(pixel: (u8, u8, u8, [i32; 2], bool)) -> i32 {
    return pixel.3[X] as i32;
}

pub fn get_vector_y(pixel: (u8, u8, u8, [i32; 2], bool)) -> i32 {
    return pixel.3[Y] as i32;
}

// pixel comparison
pub fn match_type(pixel: (u8, u8, u8, [i32; 2], bool), t: i32) -> bool {
    return pixel.0 as i32 == t;
}

pub fn match_cycle(pixel: (u8, u8, u8, [i32; 2], bool), c: i32) -> bool {
    return pixel.1 as i32 == c;
}

pub fn compare_density(pixel: (u8, u8, u8, [i32; 2], bool), d: (u8, u8, u8, [i32; 2], bool)) -> bool {
    return pixel.2 as i32 > get_density(d);
}

pub fn swap_pixels(world: &mut[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize], x1: i32, y1: i32, x2: i32, y2: i32, cycle: i32) {
    // update the cycle
    world[coord(x1, y1)].1 = cycle as u8;

    // tell renderer to redraw the pixels
    world[coord(x1, y1)].4 = true;
    world[coord(x2, y2)].4 = true;

    // swap the pixels
    let temp = world[coord(x1, y1)];
    world[coord(x1, y1)] = world[coord(x2, y2)];
    world[coord(x2, y2)] = temp;
}

pub fn update_state(world: &mut[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize], arr_index: usize, state: bool) {
    world[arr_index].4 = state;
}