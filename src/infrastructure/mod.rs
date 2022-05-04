pub mod ui {
    use std::fmt;
    use std::fmt::Formatter;
    use crate::domain::cart::Direction;
    use crate::domain::error::InvalidMovementError;
    use crate::{Cart, Grid, Movement};

    impl fmt::Display for Direction {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match *self {
                Direction::NORTH => write!(f, "NORTH"),
                Direction::WEST => write!(f, "WEST"),
                Direction::SOUTH => write!(f, "SOUTH"),
                Direction::EAST => write!(f, "EAST"),
            }
        }
    }

    impl fmt::Display for Movement {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            match *self {
                Movement::MOVE => write!(f, "MOVE"),
                Movement::LEFT => write!(f, "LEFT"),
                Movement::RIGHT => write!(f, "RIGHT"),
            }
        }
    }

    impl fmt::Display for Cart {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", format!("{} {} {}", self.coordinate.x, self.coordinate.y, self.direction))
        }
    }

    impl fmt::Display for InvalidMovementError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", format!("Cannot apply {} to cart at position {}", self.movement, self.cart))
        }
    }

    pub const PRINT: fn(Result<Grid, InvalidMovementError>) -> () = {
        |result| {
            match result {
                Ok(grid) => {
                    for cart in grid.carts {
                        println!("{} {} {}", cart.coordinate.x, cart.coordinate.y, cart.direction)
                    }
                }
                Err(error) => println!("{}", format!("{}", error))
            }
        }
    };
}
