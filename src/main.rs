use std::sync::{Arc, Mutex};
use device_query::{DeviceEvents, DeviceState, Keycode};
use std::{thread, time};
use std::collections::VecDeque;
use rand::thread_rng;
use rand::seq::SliceRandom;

// const MAP_ROWS: i32 = 20;
// const MAP_COLS: i32 = 20;
const TICK_RATE_MS: u64 = 250;

fn print_big_l() {
    println!("LLL");
    println!("LLL");
    println!("LLL");
    println!("LLL");
    println!("LLL");
    println!("LLLLLLLLL");
    println!("LLLLLLLLL");
}

#[derive(Copy, Clone)]
enum Movement {
    Left,
    Up,
    Right,
    Down,
    None,
}

fn main() {
    let queued_movement: Arc<Mutex<Movement>> = Arc::new(Mutex::new(Movement::None));

    // width: 30; height: 10
    let mut map = String::from("..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n");

    let device_state = DeviceState::new();
    // when we recieve a key_down event, call the callback closure function with argument `key``
    let q = Arc::clone(&queued_movement);
    let _guard = device_state.on_key_down(move |key| {
        let mut q = q.lock().unwrap();
        match key {
            &Keycode::Left => *q = Movement::Left,
            &Keycode::Right => *q = Movement::Right,
            &Keycode::Up => *q = Movement::Up,
            &Keycode::Down => *q = Movement::Down,
            _ => (),
        }
    });

    let mut snake_locs = VecDeque::from([(0,0)]);
    let mut apple_locs = Vec::new();
    apple_locs.push((5,5));
    apple_locs.push((7,5));
    apple_locs.push((9,5));
    apple_locs.push((9,7));
    apple_locs.push((9,9));
    let mut apple_open_locs_1d = [0; 10 * 30]; // apple open locations, saved as a single integer. i,j is known if width and height of map is known

    let mut rng = thread_rng();

    let mut i: usize = 0;
    let mut j: usize = 0;
    let q = Arc::clone(&queued_movement);
    'game: loop {
        
        let mvt: Movement = *q.lock().unwrap();
        match mvt {
            Movement::Left => j = (j + 29) % 30,
            Movement::Right => j = (j + 1) % 30,
            Movement::Up => i = (i + 9) % 10,
            Movement::Down => i = (i + 1) % 10, 
            Movement::None => (),
        }
        
        snake_locs.push_back((i,j));

        let snake_head_pos = snake_locs.back().unwrap();
        match apple_locs.iter().position(|x| x == snake_head_pos) {
            Some(pos) => { 
                // if an apple's position is where we just moved to, remove that apple and "grow" by not moving along
                // spawn a new apple
                apple_locs.remove(pos); 

                // temp below

                let mut curr_loc_idx = 0; // the index into apple_open_locs_1d
                for loc_1d in 0..10*30 {
                    let loc_2d: (usize, usize) = (loc_1d / 30, loc_1d % 30);
                    println!("{:?}", loc_2d);
                    if !snake_locs.contains(&loc_2d) && !apple_locs.contains(&loc_2d) {
                        apple_open_locs_1d[curr_loc_idx] = loc_1d;
                        curr_loc_idx += 1;
                    }
                } // assert(curr_loc_idx is the number of open spaces.)

                // println!("{:?}", &apple_open_locs_1d[0..curr_loc_idx].choose(&mut rng));
                let new_apple_loc_1d = (&apple_open_locs_1d[0..curr_loc_idx]).choose(&mut rng).unwrap();
                let new_apple_i = new_apple_loc_1d / 30;
                let new_apple_j = new_apple_loc_1d % 30;
                apple_locs.push((new_apple_i, new_apple_j));
            }, 
            None => { 
                // if there is no such position, we didn't eat an apple; the snake just moves along
                let (tail_i, tail_j) = snake_locs.pop_front().unwrap(); 
                map.replace_range((tail_j + tail_i*31)..(tail_j + tail_i*31)+1,".");
            },  
        }

        for (i,j) in apple_locs.iter() {
            map.replace_range((j + i*31)..(j + i*31)+1,"o");
        }

        let mut snake_locs_iter = snake_locs.iter().peekable();
        let snake_head_pos = snake_locs.back().unwrap();
        let mut pos = snake_locs_iter.next().unwrap();
        while snake_locs_iter.peek().is_some() {
            // until pos "pointer" points to the head, we continue self-collision detection

            if pos == snake_head_pos {
                println!("\n\nyou lost.");
                print_big_l();
                break 'game;
            }

            pos = snake_locs_iter.next().unwrap();
        }

        for (i,j) in snake_locs.iter() {
            map.replace_range((j + i*31)..(j + i*31)+1,"X");
        }
        print!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n{}", map);

        thread::sleep(time::Duration::from_millis(TICK_RATE_MS));
    }
}
