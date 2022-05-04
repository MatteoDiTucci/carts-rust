use crate::domain::movement::Movement;
use crate::domain::movement::Movement::{MOVE, LEFT, RIGHT};
use crate::domain::grid::{Dimensions, Grid};
use crate::domain::cart::{Coordinate, Cart};
use crate::domain::cart::Direction::{EAST, NORTH, SOUTH};
use crate::domain::move_carts::{MOVE_CART};
use crate::infrastructure::ui;
use crate::ui::PRINT;

mod domain;
mod infrastructure;

fn main() {
    let missions =
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
        ];
    let grid = Grid {
        boundaries: Dimensions {
            width: 5,
            height: 5,
        },
        carts: vec![],
    };

    for mission in missions {
        let result = MOVE_CART(mission, &grid);
        PRINT(result);
    }
}
