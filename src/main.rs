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

use rand::Rng;

struct Ship {
    name: String,
    len: i8,
    x: i8,
    y: i8
}

impl Ship {
    fn move_to(&mut self, x: i8, y: i8) {
        self.x = x;
        self.y = y;
    }
}

struct Board {
    name: String,
    spaces: [[char; 10]; 9],
    ships: [Ship; 5]
}

impl Board {
    fn initialize(&mut self) {
        // Constraints.
        let max_height = 9;
        let max_width = 10;

        //self.ships[0].x = 3;

        for ship in self.ships.iter_mut() {
            // true = up/down, false = left/right
            let orientation: bool = rand::thread_rng().gen();

            // Pick a starting point.
            let x = rand::thread_rng().gen_range(0, 9);
            let y = rand::thread_rng().gen_range(0, 10);

            // Check if this overlaps with an existing ship.
            
            ship.x = x;
            ship.y = y;
        }
    }

    fn print_board(&self) {
        print!(">> {0}:\n   ", self.name);

        for j in 1..11 {
            print!(" {0} ", j);
        }
        println!();

        let mut i = 65u8;
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

    human_board.print_board();
    human_board.initialize();
}

