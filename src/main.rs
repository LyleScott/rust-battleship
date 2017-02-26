extern crate rand;
#[macro_use] extern crate text_io;

use rand::Rng;
use std::{thread, time};

const MISS_ICON: char = '🔹';
const HIT_ICON: char = '💥';
const DEFAULT_ICON: char = '-';
const MAX_BOARD_WIDTH: usize = 10;
const MAX_BOARD_HEIGHT: usize = 9;
static WE_HIT_MESSAGE: &'static str = ":> We HIT them, Captain!";
static WE_MISS_MESSAGE: &'static str = ":> We MISSED them, Captain!";
static THEY_HIT_MESSAGE: &'static str = ":> They HIT us, Captain!";
static THEY_MISS_MESSAGE: &'static str = ":> They MISSED us, Captain!";

struct Ship {
    name: String,
    n_spaces: i8,
}

impl Ship {
    fn get_label(&self) -> char {
        self.name.chars().nth(0).unwrap()
    }
}

struct Board {
    name: String,
    self_spaces: [[char; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
    enemy_spaces: [[char; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
    ships: [Ship; 5],
}

impl Board {
    fn initialize(&mut self) {
        // Constraints.
        let mut skip: bool;
        let mut orientation: bool;
        let mut bound: i8;
        let mut start_point: i8;
        let mut rng = rand::thread_rng();

        for ship in self.ships.iter_mut() {
            loop {
                // Pick a (random) starting point.
                let w = rng.gen_range(0, MAX_BOARD_WIDTH - 1);
                let h = rng.gen_range(0, MAX_BOARD_HEIGHT - 1);

                // Start over if that space is already taken.
                if self.self_spaces[h as usize][w as usize] != DEFAULT_ICON {
                    continue
                }

                // true = up/down, false = left/right
                orientation = rng.gen();

                // Set the upper or most right bound for bounds checking.
                if orientation {
                    bound = MAX_BOARD_HEIGHT as i8;
                    start_point = h as i8;
                } else {
                    bound = MAX_BOARD_WIDTH as i8;
                    start_point = w as i8;
                }

                // Make sure the end of the Ship is within the bounds of the board.
                if start_point + ship.n_spaces >= bound {
                    continue
                }

                skip = false;
                //let mut value: char;

                // Make sure any space that the Ship takes up is not taken.
                for i in start_point..ship.n_spaces + start_point {
                    let value = if orientation {
                        self.self_spaces[i as usize][start_point as usize]
                    } else {
                        self.self_spaces[start_point as usize][i as usize]
                    };

                    // Space is taken, to bail out of this check.
                    if value != DEFAULT_ICON {
                        skip = true;
                        break
                    }
                }
                if skip {
                    // Space is taken, so restart placing the Ship.
                    continue
                }

                // Update the board self_spaces with the Ship's label (1st character of name).
                for i in start_point..ship.n_spaces + start_point {
                    if orientation {
                        self.self_spaces[i as usize][start_point as usize] = ship.get_label();
                    } else {
                        self.self_spaces[start_point as usize][i as usize] = ship.get_label();
                    }
                }

                // Move on to placing the next Ship.
                break
            }
        }
    }

    fn is_hit(&self, row: i8, col: i8) -> bool {
        // Check if the row/col is a hit by checking if the space is in it's default state.
        self.self_spaces[row as usize][col as usize] != DEFAULT_ICON
    }

    fn has_destroyed_enemy(&self) -> bool {
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
        for j in 0..MAX_BOARD_WIDTH {
            print!(" {0} ", j + 1);
        }
        println!();
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        self.name == other.name
    }
}

fn ship_factory(name: String, n_spaces: i8) -> Ship {
    return Ship {
        name: name,
        n_spaces: n_spaces,
    }
}

fn board_factory(name: String) -> Board {
    return Board {
        name: name.to_string(),
        self_spaces: [[DEFAULT_ICON; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
        enemy_spaces: [[DEFAULT_ICON; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
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
                if row < 0 || row >= MAX_BOARD_HEIGHT as i8
                    || col < 0 || col >= MAX_BOARD_WIDTH as i8 {
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
                    if active_board.enemy_spaces[row as usize][col as usize] == DEFAULT_ICON {
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
        if active_board.has_destroyed_enemy() {
            println!("{0} won!", active_board.name);
            break
        }

        // Swap turns.
        std::mem::swap(active_board, enemy_board);
    }
}

