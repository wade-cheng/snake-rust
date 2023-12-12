use std::sync::{Arc, Mutex};
use device_query::{DeviceEvents, DeviceState, Keycode};
use std::{thread, time};
use std::collections::VecDeque;


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


    let mut i: usize = 0;
    let mut j: usize = 0;
    let q = Arc::clone(&queued_movement);
    'game: loop {
        for (i,j) in snake_locs.iter() {
            map.replace_range((j + i*31)..(j + i*31)+1,".");
        }
        
        let mvt: Movement = *q.lock().unwrap();
        match mvt {
            Movement::Left => j = (j + 29) % 30,
            Movement::Right => j = (j + 1) % 30,
            Movement::Up => i = (i + 9) % 10,
            Movement::Down => i = (i + 1) % 10, 
            Movement::None => (),
        }
        
        snake_locs.push_back((i,j));

        for (i,j) in apple_locs.iter() {
            map.replace_range((j + i*31)..(j + i*31)+1,"o");
        }

        let snake_head_pos = snake_locs.back().unwrap();
        match apple_locs.iter().position(|x| x == snake_head_pos) {
            None => { snake_locs.pop_front(); },  // if we didn't eat an apple, the snake moves along
            Some(pos) => { apple_locs.remove(pos); }, // if we did, remove that apple and "grow" by not moving along
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
