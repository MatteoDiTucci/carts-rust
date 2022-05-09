pub mod cart;
pub mod grid;
pub mod movement;
pub mod error;

#[cfg(test)]
mod functional_tests {
    use crate::{Dimensions, Cart, Coordinate, MOVE, Grid, LEFT, RUN_RACE, NORTH};
    use crate::domain::cart::Direction::{SOUTH};
    use crate::domain::error::ForbiddenMovementError;

    #[test]
    fn move_carts() {
        let races =
                (Cart {
                    coordinate: Coordinate { x: 1, y: 2 },
                    direction: NORTH,
                },
                 vec![
                     LEFT, MOVE, LEFT, MOVE, LEFT, MOVE, LEFT, MOVE, MOVE,
                 ]);
        let grid = Grid { boundaries: Dimensions { width: 5, height: 5 }, carts: vec![] };

        let result = RUN_RACE(races, &grid);


        assert_eq!(Cart { coordinate: Coordinate { x: 1, y: 3 }, direction: NORTH }, result.as_ref().unwrap().carts[0]);
    }

    #[test]
    fn move_carts_with_illegal_movement() {
        let cart = Cart {
            coordinate: Coordinate { x: 0, y: 0 },
            direction: SOUTH,
        };
        let race =
            (cart.clone(),
             vec![
                 MOVE, MOVE,
             ]);
        let grid = Grid { boundaries: Dimensions { width: 5, height: 5 }, carts: vec![] };

        let result = RUN_RACE(race, &grid);

        assert_eq!(ForbiddenMovementError { movement: MOVE, cart }, result.unwrap_err());
    }
}