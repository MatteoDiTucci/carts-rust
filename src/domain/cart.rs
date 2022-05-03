#[derive(PartialEq, Clone, Debug)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
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