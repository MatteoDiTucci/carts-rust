use crate::domain::cart::{Coordinate, Direction, Cart};

pub enum Movement {
    MOVE,
    LEFT,
    RIGHT,
}

impl Movement {
    pub fn apply(&self, cart: &Cart) -> Cart {
        match self {
            Movement::MOVE => Self::move_forward(cart),
            Movement::LEFT => Self::left(cart),
            Movement::RIGHT => Self::right(cart),
        }
    }

    fn move_forward(cart: &Cart) -> Cart {
        let coordinate = match &cart.direction {
            Direction::NORTH => Coordinate { x: cart.coordinate.x, y: cart.coordinate.y + 1 },
            Direction::WEST => Coordinate { x: cart.coordinate.x - 1, y: cart.coordinate.y },
            Direction::SOUTH => Coordinate { x: cart.coordinate.x, y: cart.coordinate.y - 1 },
            Direction::EAST => Coordinate { x: cart.coordinate.x + 1, y: cart.coordinate.y }
        };

        Cart { coordinate, direction: cart.direction.clone() }
    }

    fn left(cart: &Cart) -> Cart {
        let direction = match &cart.direction {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH
        };

        Cart { coordinate: cart.coordinate.clone(), direction }
    }

    fn right(cart: &Cart) -> Cart {
        let direction = match &cart.direction {
            Direction::NORTH => Direction::EAST,
            Direction::WEST => Direction::NORTH,
            Direction::SOUTH => Direction::WEST,
            Direction::EAST => Direction::SOUTH
        };

        Cart { coordinate: cart.coordinate.clone(), direction }
    }
}