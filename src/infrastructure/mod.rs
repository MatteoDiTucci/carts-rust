pub mod ui {
    use std::fmt;
    use crate::domain::cart::Direction;
    use crate::Grid;


    impl fmt::Display for Direction {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                Direction::NORTH => write!(f, "NORTH"),
                Direction::WEST => write!(f, "WEST"),
                Direction::SOUTH => write!(f, "SOUTH"),
                Direction::EAST => write!(f, "EAST"),
            }
        }
    }

    pub const PRINT: fn(Grid) -> () = {
        |grid| {
            for cart in grid.carts {
                println!("{} {} {}", cart.coordinate.x, cart.coordinate.y, cart.direction)
            }
        }
    };
}
