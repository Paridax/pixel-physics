pub const DEBUG: bool = false;

pub const WIDTH: i32 = 256;
pub const HEIGHT: i32 = WIDTH;

pub const MAX_CHUNK_SIZE: i32 = WIDTH;
const SQUARE_CHUNK_SCAN: bool = true;
const RECT_SCAN_X_BIAS: bool = false;
const CHUNK_COARSENESS: i32 = 8;

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
pub const COL_SAND: [f32; 4] = [1.0, 0.6, 0.0, 1.0];
pub const COL_WATER: [f32; 4] = [0.0, 0.4, 1.0, 1.0];

const X: usize = 0;
const Y: usize = 1;

// edit the world array
pub fn create_pixel(world: &mut[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize],
                t: i32, cycle: i32, dens: i32, x: i32, y: i32) {
    let mut vector = [0; 2];
    if t == WATER || t == SAND {
        vector = [0, -1];
    }
    if x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT {
        // if the pixel is already this type, don't change it
        if ! match_type(world[coord(x, y)], t) {
            world[coord(x, y)] = (t as u8, cycle as u8, dens as u8, vector, true);
        }
    }
}

pub fn create_pixel_rect(world: &mut[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize],
                    t: i32, cycle: i32, dens: i32, x: i32, y: i32, width: i32, height: i32) {
    for i in 0..width {
        for j in 0..height {
            create_pixel(world, t, cycle, dens, x + i, y + j);
        }
    }
}

pub fn draw_pixel(x: i32, y: i32) -> [f64; 4] {
    if DEBUG {
        return [(x * SCALE_FACTOR + 1) as f64 , ((HEIGHT - y - 1) * SCALE_FACTOR + 1) as f64, SCALE_FACTOR as f64 - 1.0, SCALE_FACTOR as f64 - 1.0];
    }
    return [(x * SCALE_FACTOR) as f64 , ((HEIGHT - y - 1) * SCALE_FACTOR) as f64, SCALE_FACTOR as f64, SCALE_FACTOR as f64];
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
        return (10 as u8, 0, 3, [0, 0], false);
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

// pub fn get_vector(pixel: (u8, u8, u8, [i32; 2], bool)) -> [i32; 2] {
//     return [pixel.3[X] as i32, pixel.3[Y] as i32];
// }

pub fn get_vector_x(pixel: (u8, u8, u8, [i32; 2], bool)) -> i32 {
    return pixel.3[X] as i32;
}

// pub fn get_vector_y(pixel: (u8, u8, u8, [i32; 2], bool)) -> i32 {
//     return pixel.3[Y] as i32;
// }

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
    world[coord(x2, y2)].1 = cycle as u8;

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


pub fn find_chunk_size(world: &[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize], x: i32, y: i32) -> (u32, u32) {
    // check if the pixel is already updated
    if !get_update_state(pixel_at(world, x, y)) {
        return (0, 0);
    }
    let t = get_type(pixel_at(world, x, y));

    let mut blocked_x = false;
    let mut blocked_y = false;

    // scan larger and larger squares up and to the right of the first pixel until we find a pixel that isn't the same type
    // as the first pixel
    let mut size = 1;
    for i in 1..MAX_CHUNK_SIZE {
        let mut found = false;
        for j in 0..i {
            if get_type(pixel_at(world, x + j, y + i)) != t as i32 || pixel_at(world, x + j, y + i).4 == false {
                found = true;
                blocked_y = true;
                break;
            }
        }
        for j in 0..i {
            if get_type(pixel_at(world, x + i, y + j)) != t as i32 || pixel_at(world, x + i, y + j).4 == false {
                found = true;
                blocked_x = true;
                break;
            }
        }
        if get_type(pixel_at(world, x + i, y + i)) != t as i32 || pixel_at(world, x + i, y + i).4 == false {
            if RECT_SCAN_X_BIAS {
                blocked_x = true;
            } else {
                blocked_y = true;
            }
            found = true;
        }
        if found {
            break;
        }
        size += 1;
    }
    if SQUARE_CHUNK_SCAN {
        return (size, size);
    } else {
        let mut size_x = size;
        let mut size_y = size;

        if blocked_x {
            // if the rectangle is not tall enough, it isn't worth it to make a really skinny rect
            // because it is more likely a more efficient rect can be found further up the scan
            if size_x < CHUNK_COARSENESS as u32 {
                return(size_x, size_y);
            }
            // keep increasing the y size until we find a pixel that isn't the same type as the first pixel
            for i in size_y as i32..MAX_CHUNK_SIZE {
                let mut found = false;
                for j in 0..size as i32 {
                    if get_type(pixel_at(world, x + j, y + i)) != t as i32 || pixel_at(world, x + j, y + i).4 == false {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
                size_y += 1;
            }
        } else if blocked_y {
            // same as above, but for the y size
            if size_y < CHUNK_COARSENESS as u32 {
                return(size_x, size_y);
            }
            // keep increasing the x size until we find a pixel that isn't the same type as the first pixel
            for i in size_x as i32..MAX_CHUNK_SIZE {
                let mut found = false;
                for j in 0..size as i32 {
                    if get_type(pixel_at(world, x + i, y + j)) != t as i32 || pixel_at(world, x + i, y + j).4 == false {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
                size_x += 1;
            }
        }

        return (size_x, size_y);
    }
}

pub fn update_pixels_in_chunk(world: &mut[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize], x: i32, y: i32, chunk_size: (u32, u32)) {
    // update the pixels in the area
    for i in 0..chunk_size.0 as i32 {
        for j in 0..chunk_size.1 as i32 {
            update_state(world, coord(x + i, y + j), false);
        }
    }
}

pub fn draw_chunk(x: i32, y: i32, chunk_size: (u32, u32)) -> [f64; 4] {
    return [(x * SCALE_FACTOR) as f64, ((HEIGHT - y - chunk_size.1 as i32) * SCALE_FACTOR) as f64, (chunk_size.0 as i32 * SCALE_FACTOR) as f64, (chunk_size.1 as i32 * SCALE_FACTOR) as f64];
}