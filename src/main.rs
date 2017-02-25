// 
// Aircraft carrier: five spaces
// Battleship: four spaces
// Destroyer: three spaces
// Submarine: three spaces
// Patrol: two spaces
// 
// Grids are marked horizontally by letters A through I and vertically by numbers 1 through 10.
//
extern crate rand;
#[macro_use] extern crate text_io;

use rand::Rng;

struct Ship {
    name: String,
    len: i8,
    x: i8,
    y: i8
}

struct Board {
    name: String,
    spaces: [[char; 10]; 9],
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
                if self.spaces[h as usize][w as usize] != '-' {
                    continue
                }

                // true = up/down, false = left/right
                let orientation: bool = rand::thread_rng().gen();

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
                for i in start_point..ship.len + start_point + 1 {
                    let value = if orientation {
                        self.spaces[i as usize][start_point as usize]
                    } else {
                        self.spaces[start_point as usize][i as usize]
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

                // Update the board spaces with the Ship's label (1st character of name).
                let label = ship.name.chars().nth(0).unwrap();
                for i in start_point..ship.len + start_point + 1 {
                    if orientation {
                        self.spaces[i as usize][start_point as usize] = label;
                    } else {
                        self.spaces[start_point as usize][i as usize] = label;
                    }
                }

                // Move on to placing the next Ship.
                break
            }
        }
    }

    fn print_battle_board(&self) {
        print!(">> {0}:\n   ", self.name);

        // Print column labels (1-10).
        for j in 1..11 {
            print!(" {0} ", j);
        }
        println!();

        // A
        let mut i = 65u8;

        // Prefix each row with the label (A-I) and print the row contents.
        for row in &self.spaces {
            print!(" {0} ", i as char);
            i += 1;
            for j in row {
                print!(" {0} ", j);
            }
            println!();
        }
    }
}

fn ship_factory(name: String, len: i8) -> Ship {
    return Ship {
        name: name,
        len: len,
        x: -1, 
        y: -1
    }
}

fn board_factory(name: String) -> Board {
    return Board {
        spaces: [['-'; 10]; 9],
        name: name.to_string(),
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
    let mut computer_board = board_factory("Computer".to_string());
    let mut human_board = board_factory("Human".to_string());

    println!();
    computer_board.initialize();
    computer_board.print_battle_board();

    println!();
    human_board.initialize();
    human_board.print_battle_board();
    

    /*
    whos_turn = true;
    loop {
        let turn: String = read!("{}\n");
    }
    */
}

