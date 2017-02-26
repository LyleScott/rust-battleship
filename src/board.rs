extern crate rand;
use ship;

use rand::Rng;

pub const MISS_ICON: char = '🔹';
pub const HIT_ICON: char = '💥';
pub const DEFAULT_ICON: char = '-';
pub const MAX_BOARD_WIDTH: usize = 10;
pub const MAX_BOARD_HEIGHT: usize = 9;

pub struct Board {
    pub name: String,
    pub self_spaces: [[char; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
    pub enemy_spaces: [[char; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
    pub ships: [ship::Ship; 5],
}

impl Board {
    pub fn initialize(&mut self) {
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

                // Make sure any space that the Ship takes up is not taken.
                skip = false;
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
                if skip { continue } // Space is taken, so replace Ship.

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

    pub fn is_hit(&self, row: i8, col: i8) -> bool {
        // Check if the row/col is a hit by checking if the space is in it's default state.
        self.self_spaces[row as usize][col as usize] != DEFAULT_ICON
    }

    pub fn has_destroyed_enemy(&self) -> bool {
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

    pub fn is_human(&self) -> bool {
        self.name == "Human"
    }

    pub fn print_board(&self, is_self: bool) {
        let mut i = 65u8;  // A
        let spaces_ref = if is_self { self.self_spaces } else { self.enemy_spaces };

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

pub fn board_factory(name: String) -> Board {
    return Board {
        name: name.to_string(),
        self_spaces: [[DEFAULT_ICON; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
        enemy_spaces: [[DEFAULT_ICON; MAX_BOARD_WIDTH]; MAX_BOARD_HEIGHT],
        ships: [
            ship::ship_factory("aircraft".to_string(), 5),
            ship::ship_factory("battleship".to_string(), 4),
            ship::ship_factory("destroyer".to_string(), 3),
            ship::ship_factory("submarine".to_string(), 3),
            ship::ship_factory("patrol".to_string(), 2),
        ]
    };
}
