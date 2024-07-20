use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;
use framebuffer::Framebuffer;

fn render(framebuffer: &mut Framebuffer, current_state: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let width = framebuffer.width;
    let height = framebuffer.height;

    let mut next_state = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            let live_neighbors = get_live_neighbors(current_state, x, y);

            if current_state[y][x] {
                if live_neighbors < 2 || live_neighbors > 3 {
                    next_state[y][x] = false;
                } else {
                    next_state[y][x] = true;
                }
            } else {
                if live_neighbors == 3 {
                    next_state[y][x] = true;
                } else {
                    next_state[y][x] = false;
                }
            }

            if next_state[y][x] {
                framebuffer.set_current_color(0x81C14B); 
            } else {
                framebuffer.set_current_color(0x204E4A); 
            }

            framebuffer.point(x, y);
        }
    }

    next_state
}

fn get_live_neighbors(state: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let width = state[0].len();
    let height = state.len();
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let nx = x as isize + i;
            let ny = y as isize + j;

            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                if state[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }

    count
}

fn set_pattern(state: &mut Vec<Vec<bool>>, pattern: &[(usize, usize)], x: usize, y: usize) {
    for &(dx, dy) in pattern {
        if x + dx < state[0].len() && y + dy < state.len() {
            state[y + dy][x + dx] = true;
        }
    }
}

fn main() {
    let window_width = 600;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut current_state = vec![vec![false; framebuffer_width]; framebuffer_height];

    // Organismos
    let glider = &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let toad = &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    let loaf = &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)];
    let boat = &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
    let middle_weight_spaceship = &[(2, 0), (3, 0), (4, 0), (1, 1), (5, 1), (0, 2), (5, 2), (5, 3), (0, 3), (1, 4), (3, 4)];
    let tub = &[(1, 0), (0, 1), (2, 1), (1, 2)];
    let block = &[(0, 0), (1, 0), (0, 1), (1, 1)];
    let bee_hive = &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
    let pulsar = &[
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2), (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4), (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8), (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10), (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12)
    ];

    fn set_pattern(state: &mut Vec<Vec<bool>>, pattern: &[(usize, usize)], offset_x: usize, offset_y: usize) {
        for &(x, y) in pattern {
            state[y + offset_y][x + offset_x] = true;
        }
    }

    // Set patterns to form 3 diagonal lines

    // Diagonal from top-left to bottom-right
    for i in (0..80).step_by(10) {
        set_pattern(&mut current_state, glider, i, i);
        set_pattern(&mut current_state, toad, i + 5, i);
        set_pattern(&mut current_state, loaf, i, i + 5);
        set_pattern(&mut current_state, boat, i + 5, i + 5);
    }

    // Diagonal from bottom-left to top-right
    for i in (0..80).step_by(10) {
        set_pattern(&mut current_state, middle_weight_spaceship, i, 90 - i);
        set_pattern(&mut current_state, tub, i + 5, 90 - i);
        set_pattern(&mut current_state, block, i, 85 - i);
        set_pattern(&mut current_state, bee_hive, i + 5, 85 - i);
    }

    // Diagonal in the center
    for i in (0..80).step_by(10) {
        set_pattern(&mut current_state, pulsar, i + 10, i + 10);
        set_pattern(&mut current_state, glider, i + 15, i + 10);
        set_pattern(&mut current_state, toad, i + 10, i + 15);
        set_pattern(&mut current_state, loaf, i + 15, i + 15);
    }
    set_pattern(&mut current_state, pulsar, 45, 80);
    // Patron inicial
    /*
    set_pattern(&mut current_state, glider, 5, 5);
    set_pattern(&mut current_state, glider, 0, 10);
    set_pattern(&mut current_state, toad, 10, 20);
    set_pattern(&mut current_state, loaf, 20, 35);
    set_pattern(&mut current_state, loaf, 80, 35);
    set_pattern(&mut current_state, boat, 30, 50);
    set_pattern(&mut current_state, middle_weight_spaceship, 50, 10);
    set_pattern(&mut current_state, tub, 70, 20);
    set_pattern(&mut current_state, block, 15, 75);
    set_pattern(&mut current_state, bee_hive, 50, 50);
    set_pattern(&mut current_state, pulsar, 70, 70);
    set_pattern(&mut current_state, glider, 50, 60);
    set_pattern(&mut current_state, toad, 60, 30);
    set_pattern(&mut current_state, toad, 70, 65);
    set_pattern(&mut current_state, loaf, 40, 80);
    set_pattern(&mut current_state, boat, 25, 90);
    set_pattern(&mut current_state, pulsar, 10, 50);
    set_pattern(&mut current_state, pulsar, 80, 10);
   
    
    
    set_pattern(&mut current_state, loaf, 20, 5);
    set_pattern(&mut current_state, loaf, 40, 5);
    set_pattern(&mut current_state, loaf, 60, 5);
    set_pattern(&mut current_state, loaf, 80, 5);
    set_pattern(&mut current_state, loaf, 80, 90);
    set_pattern(&mut current_state, loaf, 60, 90);
    set_pattern(&mut current_state, loaf, 40, 90);
    set_pattern(&mut current_state, loaf, 20, 90);

    set_pattern(&mut current_state, pulsar, 45, 45);
    set_pattern(&mut current_state, pulsar, 40, 50);

    set_pattern(&mut current_state, boat, 30, 80);
    set_pattern(&mut current_state, boat, 50, 80);
    set_pattern(&mut current_state, boat, 70, 80);

    set_pattern(&mut current_state, tub, 30, 15);
    set_pattern(&mut current_state, tub, 50, 15);
    set_pattern(&mut current_state, tub, 70, 15);

    set_pattern(&mut current_state, bee_hive, 40, 25);
    set_pattern(&mut current_state, bee_hive, 60, 25);

    set_pattern(&mut current_state, block, 40, 70);
    set_pattern(&mut current_state, block, 60, 70);

    set_pattern(&mut current_state, toad, 35, 50);
    set_pattern(&mut current_state, toad, 65, 50);

    set_pattern(&mut current_state, middle_weight_spaceship, 50, 40);
    set_pattern(&mut current_state, middle_weight_spaceship, 50, 55);

    set_pattern(&mut current_state, glider, 40, 40);
    set_pattern(&mut current_state, glider, 30, 40);
    set_pattern(&mut current_state, glider, 50, 40);
    set_pattern(&mut current_state, glider, 50, 40);

    set_pattern(&mut current_state, pulsar, 45, 10);
    set_pattern(&mut current_state, pulsar, 45, 90);*/

    while window.is_open() {
        // Listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Render and get next state
        let next_state = render(&mut framebuffer, &current_state);

        // Update current state
        current_state = next_state;

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.pixels, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}