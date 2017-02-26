extern crate rand;
#[macro_use] extern crate text_io;

use rand::Rng;
use std::{thread, time};

const MISS_ICON: char = '🔹';
const HIT_ICON: char = '💥';
static WE_HIT_MESSAGE: &'static str = ":> We HIT them, Captain!";
static WE_MISS_MESSAGE: &'static str = ":> We MISSED them, Captain!";
static THEY_HIT_MESSAGE: &'static str = ":> They HIT us, Captain!";
static THEY_MISS_MESSAGE: &'static str = ":> They MISSED us, Captain!";

struct Ship {
    name: String,
    len: i8,
}

struct Board {
    name: String,
    self_spaces: [[char; 10]; 9],
    enemy_spaces: [[char; 10]; 9],
    ships: [Ship; 5],
}

impl Board {
    fn initialize(&mut self) {
        // Constraints.
        let max_width = 10;
        let max_height = 9;
        let mut skip: bool;

        for ship in self.ships.iter_mut() {
            loop {
                // Pick a (random) starting point.
                let w = rand::thread_rng().gen_range(0, max_width - 1);
                let h = rand::thread_rng().gen_range(0, max_height - 1);

                // Start over if that space is already taken.
                if self.self_spaces[h as usize][w as usize] != '-' {
                    continue
                }

                // true = up/down, false = left/right
                let orientation: bool = rand::thread_rng().gen();

                // Set the upper or most right bound for bounds checking.
                let bound: i8;
                let start_point: i8;
                if orientation {
                    bound = max_height;
                    start_point = h;
                } else {
                    bound = max_width;
                    start_point = w;
                }

                // Make sure the end of the Ship is within the bounds of the board.
                if start_point + ship.len >= bound {
                    continue
                }

                skip = false;
                //let mut value: char;

                // Make sure any space that the Ship takes up is not taken.
                for i in start_point..ship.len + start_point {
                    let value = if orientation {
                        self.self_spaces[i as usize][start_point as usize]
                    } else {
                        self.self_spaces[start_point as usize][i as usize]
                    };

                    // Space is taken, to bail out of this check.
                    if value != '-' {
                        skip = true;
                        break
                    }
                }
                if skip {
                    // Space is taken, so restart placing the Ship.
                    continue
                }

                // Update the board self_spaces with the Ship's label (1st character of name).
                let label = ship.name.chars().nth(0).unwrap();
                for i in start_point..ship.len + start_point {
                    if orientation {
                        self.self_spaces[i as usize][start_point as usize] = label;
                    } else {
                        self.self_spaces[start_point as usize][i as usize] = label;
                    }
                }

                // Move on to placing the next Ship.
                break
            }
        }
    }

    fn is_hit(&self, row: i8, col: i8) -> bool {
        // Check if the row/col is a hit by checking if the space is in it's default state.
        self.self_spaces[row as usize][col as usize] != '-'
    }

    fn destroyed_enemy(&self) -> bool {
        // Check to see if all number of hits == max number of hits possible.
        let mut hits = 0;
        for i in &self.enemy_spaces {
            for j in i {
                if j == &'x' {
                    hits += 1;
                }
            }
        }

        hits == 17
    }

    fn is_human(&self) -> bool {
        self.name == "Human"
    }

    fn print_board(&self, is_self: bool) {
        // A
        let mut i = 65u8;
        let spaces_ref = if is_self {
            self.self_spaces
        } else {
            self.enemy_spaces
        };

        // Prefix each row with the label (A-I) and print the row contents.
        for row in &spaces_ref {
            print!(" {0} ", i as char);
            i += 1;
            for j in row {
                print!(" {0} ", j);
            }
            println!();
        }

        // Print column labels (1-10).
        print!("   ");
        for j in 1..11 {
            print!(" {0} ", j);
        }
        println!();
    }
}

impl PartialEq for Board {
        fn eq(&self, other: &Board) -> bool {
            self.name == other.name
        }
}

fn ship_factory(name: String, len: i8) -> Ship {
    return Ship {
        name: name,
        len: len,
    }
}

fn board_factory(name: String) -> Board {
    return Board {
        name: name.to_string(),
        self_spaces: [['-'; 10]; 9],
        enemy_spaces: [['-'; 10]; 9],
        ships: [
            ship_factory("aircraft".to_string(), 5),
            ship_factory("battleship".to_string(), 4),
            ship_factory("destroyer".to_string(), 3),
            ship_factory("submarine".to_string(), 3),
            ship_factory("patrol".to_string(), 2),
        ]
    };
}

fn main() {
    // Create Boards containing Ships for each player.
    let mut computer_board = board_factory("Computer".to_string());
    computer_board.initialize();
    let mut human_board = board_factory("Human".to_string());
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

                // Grab the rest of the input string, which should be the number of the coordinate.
                col = coordinate[1..coordinate.len()].parse().unwrap_or(0) - 1;

                // Bounds checking.
                if row < 0 || row >= 9 || col < 0 || col >= 10 {
                    // The coodinate the user chose is out of bounds.
                    println!("Bad input!");
                    continue
                }

                hit_message = &WE_HIT_MESSAGE;
                miss_message = &WE_MISS_MESSAGE;
            } else {
                // Basically, guess a coordinate.
                loop {
                    row = rand::thread_rng().gen_range(b'A', b'I') as i8 - offset_a;
                    col = rand::thread_rng().gen_range(0, 9) as i8;
                    if active_board.enemy_spaces[row as usize][col as usize] == '-' {
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
                active_board.enemy_spaces[row as usize][col as usize] = HIT_ICON;
                enemy_board.self_spaces[row as usize][col as usize] = HIT_ICON;
            } else {
                println!("{0}", miss_message);
                active_board.enemy_spaces[row as usize][col as usize] = MISS_ICON;
                enemy_board.self_spaces[row as usize][col as usize] = MISS_ICON;
            }
            thread::sleep(time::Duration::from_millis(1000));

            break
        }

        // Check if someone won.
        if active_board.destroyed_enemy() {
            println!("{0} won!", active_board.name);
            break
        }

        // Swap turns.
        std::mem::swap(active_board, enemy_board);
    }
}

