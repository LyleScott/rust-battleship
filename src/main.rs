extern crate rand;
#[macro_use] extern crate text_io;

use rand::Rng;
use std::{thread, time};

mod board;
mod ship;

static WE_HIT_MESSAGE: &'static str = ":> We HIT them, Captain!";
static WE_MISS_MESSAGE: &'static str = ":> We MISSED them, Captain!";
static THEY_HIT_MESSAGE: &'static str = ":> They HIT us, Captain!";
static THEY_MISS_MESSAGE: &'static str = ":> They MISSED us, Captain!";


fn main() {
    // Create Boards containing Ships for each player.
    let mut computer_board = board::board_factory("Computer".to_string());
    computer_board.initialize();
    let mut human_board = board::board_factory("Human".to_string());
    human_board.initialize();

    // Needed for transforming a char of A-I to zero-based indexing into an array.
    let offset_a = 'A' as i8;

    // Hold references to each board. Then swap them after each turn.
    let active_board = &mut human_board;
    let enemy_board = &mut computer_board;

    // Alter the hit/miss messages based on who's turn it is.
    let mut hit_message: &'static str;
    let mut miss_message: &'static str;

    let mut row: i8;
    let mut col: i8;
    let mut coordinate: String;
    let mut rng = rand::thread_rng();

    loop {
        loop {
            if active_board.is_human() {
                // Print the boards.
                println!("\n>> Our Fleet Status");
                active_board.print_board(true);
                println!("\n>> Enemy Fleet Status");
                active_board.print_board(false);

                // Get the coordinate from the user's keyboard.
                println!("\n?> Coordinate, sir? (ie, C5)");
                coordinate = read!("{}\n");

                // The offset the length of the "number" part of the coordinate.
                // ie, It can be "1" in the case of B1, but also "2" in the case of B10.
                if coordinate.len() < 1 {
                    // The user entered no characters or a single character.
                    println!("Bad input!");
                    continue
                }

                // Deduce the row index from the char value of the letter input.
                row = coordinate.to_uppercase().chars().nth(0).unwrap() as i8 - offset_a;

                // Grab the rest of the input string; ie, the number of the coordinate.
                col = coordinate[1..coordinate.len()].parse().unwrap_or(0) - 1;

                // Bounds checking.
                if row < 0 || row >= board::MAX_BOARD_HEIGHT as i8
                    || col < 0 || col >= board::MAX_BOARD_WIDTH as i8 {
                    // The coodinate the user chose is out of bounds.
                    println!("Bad input!");
                    continue
                }

                hit_message = &WE_HIT_MESSAGE;
                miss_message = &WE_MISS_MESSAGE;
            } else {
                // Basically, guess a coordinate.
                loop {
                    row = rng.gen_range(b'A', b'I') as i8 - offset_a;
                    col = rng.gen_range(0, 9) as i8;
                    if active_board.enemy_spaces[row as usize][col as usize] == board::DEFAULT_ICON {
                        // Let the computer have the advantage of not duping a coordinate.
                        break
                    }
                }

                hit_message = &THEY_HIT_MESSAGE;
                miss_message = &THEY_MISS_MESSAGE;
            }

            // Display a HIT or MISS message.
            if enemy_board.is_hit(row, col) {
                println!("{0}", hit_message);
                active_board.enemy_spaces[row as usize][col as usize] = board::HIT_ICON;
                enemy_board.self_spaces[row as usize][col as usize] = board::HIT_ICON;
            } else {
                println!("{0}", miss_message);
                active_board.enemy_spaces[row as usize][col as usize] = board::MISS_ICON;
                enemy_board.self_spaces[row as usize][col as usize] = board::MISS_ICON;
            }
            thread::sleep(time::Duration::from_millis(1000));

            break
        }

        // Check if someone won.
        if active_board.has_destroyed_enemy() {
            println!("{0} won!", active_board.name);
            break
        }

        // Swap turns.
        std::mem::swap(active_board, enemy_board);
    }
}

