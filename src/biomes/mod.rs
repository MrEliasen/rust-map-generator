use crate::biomes::biomes::WHITTAKER;
use image::Rgb;

use self::biomes::Biomes;

pub mod biomes;

#[derive(Clone, Debug)]
pub struct Biome {
    pub tile_type: Biomes,
    pub distance_from_sea: u32,
    pub distance_from_fresh_water: u32,
    pub elevation: u32,
    pub moisture: u32,
}

impl Biome {
    pub fn new(biome: Biomes) -> Self {
        Self {
            tile_type: biome,
            distance_from_sea: 0,
            distance_from_fresh_water: 0,
            elevation: 1,
            moisture: 0,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            tile_type: Biomes::Void,
            distance_from_sea: 0,
            distance_from_fresh_water: 0,
            elevation: 1,
            moisture: 0,
        }
    }

    pub fn get_tile_symbol(&self) -> &str {
        self.tile_type.get_symbol()
    }

    pub fn get_tile_name(&self) -> &str {
        self.tile_type.get_name()
    }

    pub fn get_tile_colour(&self) -> Rgb<u8> {
        self.tile_type.get_colour()
    }

    pub fn get_elevation_colour(&self) -> Rgb<u8> {
        if self.get_tile_name() == Biomes::SaltWater.get_name() {
            return self.get_tile_colour();
        }

        match self.elevation {
            1 => image::Rgb([0, 0, 0]),
            2 => image::Rgb([89, 89, 89]),
            3 => image::Rgb([184, 184, 184]),
            4 => image::Rgb([255, 255, 255]),
            _ => image::Rgb([199, 0, 57]),
        }
    }

    pub fn get_moisture_colour(&self) -> Rgb<u8> {
        if self.get_tile_name() == Biomes::SaltWater.get_name() {
            return image::Rgb([199, 0, 57]);
        }

        match self.moisture {
            1 => image::Rgb([224, 224, 224]),
            2 => image::Rgb([112, 112, 112]),
            3 => image::Rgb([234, 242, 255]),
            4 => image::Rgb([125, 174, 254]),
            5 => image::Rgb([16, 106, 255]),
            6 => image::Rgb([0, 11, 213]),
            _ => image::Rgb([0, 0, 0]),
        }
    }

    pub fn calculate_biome(&mut self) {
        let mut moisture = self.moisture;
        let mut elevation = self.elevation;

        if elevation < 1 {
            elevation = 1;
        }

        if elevation > 4 {
            elevation = 4;
        }

        if moisture < 1 {
            moisture = 1;
        }

        if moisture > 6 {
            moisture = 6;
        }

        self.tile_type = WHITTAKER[(elevation - 1) as usize][(moisture - 1) as usize];
    }
}
