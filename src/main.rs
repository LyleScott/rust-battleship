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
        &self.self_spaces[row as usize][col as usize] == &'-'
    }

    fn destroyed_enemy(&self) -> bool {
        let mut hits = 0;
        for i in &self.enemy_spaces {
            for j in i {
                if *j == 'x' {
                    hits += 1;
                }
            }
        }

        println!("hits: {0}", hits);

        hits == 17
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
        for row in &self.self_spaces {
            print!(" {0} ", i as char);
            i += 1;
            for j in row {
                print!(" {0} ", j);
            }
            println!();
        }
    }

    fn print_enemy_board(&self) {
        println!();

        // A
        let mut i = 65u8;

        // Prefix each row with the label (A-I) and print the row contents.
        for row in &self.enemy_spaces {
            print!(" {0} ", i as char);
            i += 1;
            for j in row {
                print!(" {0} ", j);
            }
            println!();
        }

        print!("   ");

        // Print column labels (1-10).
        for j in 1..11 {
            print!(" {0} ", j);
        }
        println!();
    }
}

fn ship_factory(name: String, len: i8) -> Ship {
    return Ship {
        name: name,
        len: len,
        x: -1, 
        y: -1,
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
    let mut enemy_board = board_factory("Computer".to_string());
    let mut human_board = board_factory("Human".to_string());

    println!();
    enemy_board.initialize();

    println!();
    human_board.initialize();

    let mut is_humans_turn = true;
    let offset_a = 'A' as i8;
    loop {
        if is_humans_turn {
            println!("\n\n\n\n\n\n\n");
            enemy_board.print_battle_board();
            human_board.print_battle_board();
            human_board.print_enemy_board();

            if human_board.destroyed_enemy() {
                println!("You sank them all!");
                break
            }

            loop {
                let coordinate: String = read!("{}\n");
                let offset = coordinate.len();

                if offset < 1{
                    println!("Bad input!");
                    continue;
                }

                let row: i8 = coordinate.to_uppercase().chars().nth(0).unwrap() as i8 - offset_a;
                let col: i8 = coordinate[1..offset].parse().unwrap_or(0) - 1;

                if row < 0 || row >= 9 || col < 0 || col >= 10 {
                    println!("Bad input!");
                    continue;
                }

                human_board.enemy_spaces[row as usize][col as usize] = if enemy_board.is_hit(row, col) {
                    '#' 
                } else {
                    'x'
                };
                break;
            }

        } else {
        }

        is_humans_turn = !is_humans_turn
    }
}

