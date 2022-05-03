pub mod cart;
pub mod grid;
pub mod movement;

#[cfg(test)]
mod functional_tests {
    use crate::{Dimensions, Cart, Coordinate, MOVE, Grid, LEFT, MOVE_CARTS, NORTH, RIGHT};
    use crate::domain::cart::Direction::EAST;

    #[test]
    fn move_carts() {
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
            ];
        let grid = Grid { boundaries: Dimensions { width: 5, height: 5 }, carts: vec![] };

        let result = MOVE_CARTS(missions, &grid);


        assert_eq!(Cart { coordinate: Coordinate { x: 1, y: 3 }, direction: NORTH }, result.as_ref().unwrap().carts[0]);
        assert_eq!(Cart { coordinate: Coordinate { x: 5, y: 1 }, direction: EAST }, result.as_ref().unwrap().carts[1]);
    }
}