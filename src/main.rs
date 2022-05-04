use crate::domain::movement::Movement;
use crate::domain::movement::Movement::{MOVE, LEFT, RIGHT};
use crate::domain::grid::{Dimensions, Grid};
use crate::domain::cart::{Coordinate, Cart};
use crate::domain::cart::Direction::{EAST, NORTH, SOUTH};
use crate::infrastructure::ui;
use crate::ui::PRINT;
use crate::use_cases::{Mission, MOVE_CART};

mod domain;
mod infrastructure;
mod use_cases;

fn main() {
    let grid = grid();

    for mission in missions() {
        let result = MOVE_CART(mission, &grid);
        PRINT(result);
    }
}

fn grid() -> Grid {
    Grid {
        boundaries: Dimensions {
            width: 5,
            height: 5,
        },
        carts: vec![],
    }
}

fn missions() -> Vec<Mission> {
    vec![
        (Cart {
            coordinate: Coordinate { x: 1, y: 2 },
            direction: NORTH,
        },
         vec![
             LEFT, MOVE, LEFT, MOVE, LEFT, MOVE, LEFT, MOVE, MOVE,
         ]),
        (Cart {
            coordinate: Coordinate { x: 3, y: 3 },
            direction: EAST,
        },
         vec![
             MOVE, MOVE, RIGHT, MOVE, MOVE, RIGHT, MOVE, RIGHT, RIGHT, MOVE,
         ]),
        (Cart {
            coordinate: Coordinate { x: 0, y: 0 },
            direction: SOUTH,
        },
         vec![
             MOVE, MOVE,
         ]),
    ]
}
