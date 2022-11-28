// LAUNCH GRAPHICALLY: ONLY WHEN RELEASING
#![windows_subsystem = "windows"]

extern crate piston_window;
extern crate graphics;
extern crate rand;

mod pixel;

use piston_window::*;
use pixel::*;

fn main() {
    let mut world = [(0 as u8, 0 as u8, 0 as u8, [0 as i32, 0 as i32], true); (WIDTH * HEIGHT) as usize];

    // make a new window called Pixel Physics
    let window: PistonWindow =
        WindowSettings::new("Pixel Physics", [(WIDTH * SCALE_FACTOR) as u32, (HEIGHT * SCALE_FACTOR) as u32])
            .resizable(false).build().unwrap();
    let window = window.max_fps(30);
    // let window = window.ups(1);
    let mut window = window.exit_on_esc(true);

    let (mut mouse_x, mut mouse_y) = (0, 0);

    let mut brush_type = SAND;
    let mut cycle = 0;
    let mut draw = false;

    // game loop
    while let Some(event) = window.next() {
        cycle = (cycle + 1) % 2;

        // if the 1 key is pressed, change the brush to rock
        if let Some(Button::Keyboard(Key::D1)) = event.press_args() {
            brush_type = ROCK;
        }
        // if the 2 key is pressed, change the brush to sand
        if let Some(Button::Keyboard(Key::D2)) = event.press_args() {
            brush_type = SAND;
        }
        // if the 3 key is pressed, change the brush to water
        if let Some(Button::Keyboard(Key::D3)) = event.press_args() {
            brush_type = WATER;
        }
        // if the 4 key is pressed, change the brush to air
        if let Some(Button::Keyboard(Key::D4)) = event.press_args() {
            brush_type = AIR;
        }

        // get mouse position
        event.mouse_cursor(|pos| {
            mouse_x = (pos[0] as f64/SCALE_FACTOR as f64) as i32;
            mouse_y = HEIGHT - 1 - (pos[1] as f64/SCALE_FACTOR as f64) as i32;
            if mouse_x < 0 { mouse_x = 0; }
            if mouse_x >= WIDTH { mouse_x = WIDTH - 1; }
            if mouse_y < 0 { mouse_y = 0; }
            if mouse_y >= HEIGHT { mouse_y = HEIGHT - 1; }
        });

        // check if the mouse is pressed
        if let Some(Button::Mouse(_)) = event.press_args() {
            draw = true;
        }

        // check if the mouse is released
        if let Some(Button::Mouse(_)) = event.release_args() {
            draw = false;
        }

        // if the mouse is pressed, draw a pixel
        if draw {
            if pixel_at(&world, mouse_x, mouse_y).0 != brush_type as u8 {
                // create_pixel(&mut world, brush_type, cycle, density(brush_type), mouse_x, mouse_y);
                create_pixel_rect(&mut world, brush_type, cycle, density(brush_type), mouse_x - 5, mouse_y - 5, 11, 11);
            }
        }

        // move pixels
        for i in (0..world.len() as u64).rev() {
            let pixel = world[i as usize];
            // if the pixel has already been moved this cycle, skip it
            // if the pixel is air or rock, it will never move
            if match_cycle(pixel, cycle) || match_type(pixel, AIR) || match_type(pixel, ROCK) {
                // reset the update flag
                if ! match_type(pixel, AIR) {
                    world[i as usize].4 = true;
                }
                continue;
            } else /* if get_vector_y(pixel) != 0 */ {
                update_pixel(&mut world, i, cycle);
            }
        }

        // render pixels on screen
        window.draw_2d(&event, |context, graphics, _device| {

            // display_world(&world, 0, world.len() as u64);
            clear([0.0; 4], graphics);

            // loop over each pixel
            for i in (0..world.len() as u64) {
                // if the pixel is air then skip drawing it
                let pixel = world[i as usize];
                if ! match_type(pixel, AIR) {
                    // let (x, y) = loc(i);
                    // let mut col = color(get_type(pixel));
                    // col[3] = 0.1;
                    // rectangle(col,
                    //           // square at x, y with width of the pixel size
                    //           draw_pixel(x, y),
                    //           context.transform,
                    //           graphics);
                }
                if world[i as usize].4 {
                    // println!("{}", world[i as usize].4);
                    let (x, y) = loc(i);

                    let chunk_size = find_chunk_size(&world, x, y);

                    update_pixels_in_chunk(&mut world, x, y, chunk_size);
                    // display_world(&world, 0, world.len() as u64);
                    let mut col = color(get_type(pixel));
                    if DEBUG {
                        if get_type(pixel) == WATER {
                            col = [(x as f32)/(WIDTH as f32 - 1.0),(y as f32)/(HEIGHT as f32-1.0), 1.0, 0.1];
                        } else {
                            col = [(x as f32)/(WIDTH as f32 - 1.0),(y as f32)/(HEIGHT as f32-1.0), 0.0, 1.0]
                        }
                    }
                    rectangle(col,
                              // square at x, y with width of the pixel size
                              draw_chunk(x, y, chunk_size),
                              context.transform,
                              graphics);
                }
            }

            // draw mouse cursor
            rectangle([1.0, 1.0, 1.0, 0.3],
                      // square at x, y with width of the pixel size
                      draw_pixel(mouse_x, mouse_y),
                      context.transform,
                      graphics);
            }
        );
    }
}

fn update_pixel(mut world: &mut[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize],
                array_pos: u64, cycle: i32) {
    // if we are updating a pixel that doesnt exist, return
    if array_pos < 0 || array_pos >= world.len() as u64 {
        return;
    }
    let pixel = world[array_pos as usize];
    // if the pixel has already been updated this cycle, return
    if get_cycle(pixel) == cycle {
        return;
    }
    // if the pixel is air or rock, return
    if match_type(pixel, AIR) || match_type(pixel, ROCK) {
        return;
    }

    if match_type(pixel, SAND) {
        let (x, y) = loc(array_pos);
        if compare_density(pixel, pixel_at(&world, x, y - 1)) {
            // set x vector to 0
            world[array_pos as usize].3[0] = 0;
            swap_pixels(&mut world, x, y, x, y - 1, cycle);
        } else {
            // if there is no air below it, it will move down and to the left or right if there is air there
            if compare_density(pixel, pixel_at(&world, x - 1, y - 1)) {
                // set x vector to -1
                world[array_pos as usize].3[0] = -1;
                swap_pixels(&mut world, x, y, x - 1, y - 1, cycle);
            } else if compare_density(pixel, pixel_at(&world, x + 1, y - 1)) {
                // set x vector to 1
                world[array_pos as usize].3[0] = 1;
                swap_pixels(&mut world, x, y, x + 1, y - 1, cycle);
            } else {
                world[array_pos as usize].3[0] = 0;
                world[array_pos as usize].3[1] = 0;
            }
        }
    }

    if match_type(pixel, WATER) {
        let (x, y) = loc(array_pos);
        if compare_density(pixel, pixel_at(&world, x, y - 1)) {
            world[array_pos as usize].3[0] = 0;
            // if the pixel below is water, use rng to decide if it should move left or right or stay still
            if match_type(pixel_at(&world, x, y - 1), WATER) {
                // random bool
                let rand = rand::random::<bool>();
                if rand {
                    // update cycle
                    world[array_pos as usize].1 = cycle as u8;
                    return;
                } else {
                    swap_pixels(&mut world, x, y, x, y - 1, cycle);
                }
            } else {
                swap_pixels(&mut world, x, y, x, y - 1, cycle);
            }
        } else {
            if compare_density(pixel, pixel_at(&world, x - 1, y - 1)) {
                // down and to the left
                world[array_pos as usize].3[0] = -1;
                swap_pixels(&mut world, x, y, x - 1, y - 1, cycle);
            } else if compare_density(pixel, pixel_at(&world, x + 1, y - 1)) {
                // down and to the right
                world[array_pos as usize].3[0] = 1;
                swap_pixels(&mut world, x, y, x + 1, y - 1, cycle);
            } else {
                if get_vector_x(pixel) == 0 {
                    // pick randomly true or false which direction to move
                    let rand = rand::random::<bool>();
                    if rand {
                        world[array_pos as usize].3[0] = 1;
                    } else {
                        world[array_pos as usize].3[0] = -1;
                    }
                } else if get_vector_x(pixel) == 1 {
                    // move to the right if we already have right movement
                    if compare_density(pixel, pixel_at(&world, x + 1, y)) {
                        swap_pixels(&mut world, x, y, x + 1, y, cycle);
                    } else {
                        // swap directions if we cant move right
                        world[array_pos as usize].3[0] = -1;
                    }
                } else if get_vector_x(pixel) == -1 {
                    // move to the left if we already have left movement
                    if compare_density(pixel, pixel_at(&world, x - 1, y)) {
                        swap_pixels(&mut world, x, y, x - 1, y, cycle);
                    } else {
                        // swap directions if we cant move left
                        world[array_pos as usize].3[0] = 1;
                    }
                } else {
                    world[array_pos as usize].3[0] = 0;
                    world[array_pos as usize].3[1] = 0;
                }
            }
        }
    }
}

fn color(pixel_type: i32) -> [f32; 4] {
    let color = match pixel_type {
        ROCK => COL_ROCK,
        SAND => COL_SAND,
        WATER => COL_WATER,
        _ => COL_AIR
    };
    return color;
}

fn density(pixel_type: i32) -> i32 {
    let density = match pixel_type {
        ROCK => D_ROCK,
        SAND => D_SAND,
        WATER => D_WATER,
        _ => D_AIR
    };
    return density;
}

//<editor-fold desc="DEBUG">
fn display_world(world: &[(u8, u8, u8, [i32; 2], bool); (WIDTH * HEIGHT) as usize], start: u64, end: u64) {
    for i in (start..end).rev() {
        let (t, cycle, density, position, display_update) = world[i as usize];
        let (pos_x, pos_y) = loc(i);
        println!("Type: {0}, Cycle: {1}, Density: {2}, Vector: ({3}, {4}), Pos: ({5}, {6}), Update Sc: {7}", t, cycle, density, position[0], position[1], pos_x, pos_y, display_update);
    }
}
//</editor-fold>