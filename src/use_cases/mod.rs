use crate::{Cart, Grid, Movement};
use crate::domain::error::ForbiddenMovementError;
use crate::domain::grid::{RUN_RACE_ON_GRID};

pub type Race = (Cart, Vec<Movement>);
pub type RunRace = fn(Race, &Grid) -> Result<Grid, ForbiddenMovementError>;

pub const RUN_RACE: RunRace = {
    |race, grid| {
        RUN_RACE_ON_GRID(race, grid)
    }
};