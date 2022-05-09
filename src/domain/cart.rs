use crate::Movement;

#[derive(PartialEq, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Direction {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Cart {
    pub coordinate: Coordinate,
    pub direction: Direction,
}

pub type MoveCart = fn(&Cart, &Movement) -> Cart;

pub const MOVE_CART: MoveCart = {
    |cart, movement| {
        match movement {
            Movement::MOVE => MOVE_FORWARD(cart),
            Movement::LEFT => LEFT(cart),
            Movement::RIGHT => RIGHT(cart),
        }
    }

};

const MOVE_FORWARD: fn(&Cart) -> Cart = {
    |cart| {
        let coordinate = match &cart.direction {
            Direction::NORTH => Coordinate { x: cart.coordinate.x, y: cart.coordinate.y + 1 },
            Direction::WEST => Coordinate { x: cart.coordinate.x - 1, y: cart.coordinate.y },
            Direction::SOUTH => Coordinate { x: cart.coordinate.x, y: cart.coordinate.y - 1 },
            Direction::EAST => Coordinate { x: cart.coordinate.x + 1, y: cart.coordinate.y }
        };

        Cart { coordinate, direction: cart.direction.clone() }
    }
};

const LEFT: fn(&Cart) -> Cart = {
    |cart| {
        let direction = match &cart.direction {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH
        };

        Cart { coordinate: cart.coordinate.clone(), direction }
    }
};

const RIGHT: fn(&Cart) -> Cart = {
    |cart| {
        let direction = match &cart.direction {
            Direction::NORTH => Direction::EAST,
            Direction::WEST => Direction::NORTH,
            Direction::SOUTH => Direction::WEST,
            Direction::EAST => Direction::SOUTH
        };

        Cart { coordinate: cart.coordinate.clone(), direction }
    }
};
