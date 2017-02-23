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

    fn initialize(&mut self) {

    }
}

fn main() {
    let mut computer_board = Board {
        spaces: [['-'; 10]; 9],
        name: "Computer".to_string(),
        ships: [
            Ship { len: 5, x: -1, y: -1, name: "aircraft".to_string()},
            Ship { len: 4, x: -1, y: -1, name: "battleship".to_string() },
            Ship { len: 3, x: -1, y: -1, name: "destroyer".to_string() },
            Ship { len: 3, x: -1, y: -1, name: "submarine".to_string() },
            Ship { len: 2, x: -1, y: -1, name: "patrol".to_string() }
        ]
    };

    let mut human_board = Board {
        spaces: [['-'; 10]; 9],
        name: "human".to_string(),
        ships: [
            Ship { len: 5, x: -1, y: -1, name: "aircraft".to_string() },
            Ship { len: 4, x: -1, y: -1, name: "battleship".to_string() },
            Ship { len: 3, x: -1, y: -1, name: "destroyer".to_string() },
            Ship { len: 3, x: -1, y: -1, name: "submarine".to_string() },
            Ship { len: 2, x: -1, y: -1, name: "patrol".to_string() }
        ]
    };

    human_board.print_board();
}

//fn place_pieces(ships: &mut [&mut Ship; 5]) {
//    // Pick a starting point.
//    let num = rand::thread_rng().gen_range(1, 100);
//    for ship in ships {
//        ship.x = 3;
//    }
//}


