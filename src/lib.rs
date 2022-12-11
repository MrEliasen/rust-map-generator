use crate::biomes::Biome;

pub mod biomes;
pub mod generator;
pub mod steppers;
pub mod helper;

pub type MapData = Vec<Vec<Biome>>;