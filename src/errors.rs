use std::{error::Error, fmt};

#[derive(Debug)]
pub struct MapGenError;

impl Error for MapGenError {}

impl fmt::Display for MapGenError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Map generator failed.")
    }
}
