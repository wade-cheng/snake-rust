use std::fs;
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

fn main() {
    /* refactor to a Enum later
       note similarity to numpad
       -1: no movement:
       4: Left
       8: Up
       6: Right
       2: Down
    */
    // let mut queued_movement: i32 = -1;

    // width: 30; height: 10
    let mut map = String::from("..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n..............................\n");

    fs::write("test.txt", "").expect("Unable to write file");
    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(|key| {
        // queued_movement = 0; // cannot assign to `queued_movement`, as it is a captured variable in a `Fn` closure"
        match key {
            &Keycode::Left => fs::write("test.txt", "4").expect("Unable to write file"), //println!("Key Left pressed, i think"), // queued_movement = 4;
            &Keycode::Right => fs::write("test.txt", "6").expect("Unable to write file"), // LMFAO workaround of the century
            &Keycode::Up => fs::write("test.txt", "8").expect("Unable to write file"), 
            &Keycode::Down => fs::write("test.txt", "2").expect("Unable to write file"), 
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
    'game: loop {
        for (i,j) in snake_locs.iter() {
            map.replace_range((j + i*31)..(j + i*31)+1,".");
        }
        
        let s = fs::read_to_string("test.txt").expect("Unable to read file");
        match &s[..] {
            "4" => j = (j + 29) % 30,
            "6" => j = (j + 1) % 30,
            "8" => i = (i + 9) % 10,
            "2" => i = (i + 1) % 10, 
            _ => (),
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
        loop {
            if snake_locs_iter.peek().is_none() {
                // our pos "pointer" points to the head. we can end self-collision detection
                break;
            }

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
