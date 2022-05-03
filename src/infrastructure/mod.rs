pub mod ui {
    use std::fmt;
    use std::fmt::Formatter;
    use crate::domain::cart::Direction;
    use crate::domain::error::InvalidMove;
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

    impl fmt::Display for InvalidMove {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", format!("Cannot apply {} to cart at position {}", self.movement, self.cart))
        }
    }

    pub const PRINT: fn(Result<Grid, InvalidMove>) -> () = {
        |result| {
            if result.is_ok() {
                for cart in result.unwrap().carts {
                    println!("{} {} {}", cart.coordinate.x, cart.coordinate.y, cart.direction)
                }
            } else {
                println!("{}", format!("{}", result.unwrap_err()))
            }
        }
    };
}
