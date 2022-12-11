use image::Rgb;

#[derive(Debug, Copy, Clone)]
pub enum Biomes {
    Void,
    Placeholder,
    FreshWater,
    SaltWater,
    Land,
    Beach,
    // whittaker Biomes
    SubtropicalDesert,
    Grassland,
    TropicalSeasonalForest,
    TropicalRainForest,
    TemperateDesert,
    TemperateDeciduousForest,
    TemperateRainForest,
    Shrubland,
    Taiga,
    Scorched,
    Bare,
    Tundra,
    Snow,
}

const WHITTAKER: [[Biomes; 6]; 4] = [
    [
        Biomes::SubtropicalDesert,
        Biomes::Grassland,
        Biomes::TropicalSeasonalForest,
        Biomes::TropicalSeasonalForest,
        Biomes::TropicalRainForest,
        Biomes::TropicalRainForest,
    ],
    [
        Biomes::TemperateDesert,
        Biomes::Grassland,
        Biomes::Grassland,
        Biomes::TemperateDeciduousForest,
        Biomes::TemperateDeciduousForest,
        Biomes::TemperateRainForest,
    ],
    [
        Biomes::TemperateDesert,
        Biomes::TemperateDesert,
        Biomes::Shrubland,
        Biomes::Shrubland,
        Biomes::Taiga,
        Biomes::Taiga,
    ],
    [
        Biomes::Scorched,
        Biomes::Bare,
        Biomes::Tundra,
        Biomes::Snow,
        Biomes::Snow,
        Biomes::Snow,
    ],
];

impl Biomes {
    pub fn get_symbol(&self) -> &str {
        match self {
            Biomes::Placeholder => " ",
            Biomes::Void => "",
            Biomes::FreshWater => "=",
            Biomes::SaltWater => "~",
            Biomes::Land => "",
            Biomes::Beach => "B",
            Biomes::Taiga => "1",
            Biomes::SubtropicalDesert => "2",
            Biomes::Grassland => "3",
            Biomes::TropicalSeasonalForest => "4",
            Biomes::TropicalRainForest => "5",
            Biomes::TemperateDesert => "6",
            Biomes::TemperateDeciduousForest => "7",
            Biomes::TemperateRainForest => "8",
            Biomes::Shrubland => "9",
            Biomes::Scorched => "0",
            Biomes::Bare => ".",
            Biomes::Tundra => "t",
            Biomes::Snow => "s",
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Biomes::Void => "Void",
            Biomes::FreshWater => "Fresh Water",
            Biomes::SaltWater => "Salt Water",
            Biomes::Land => "Land",
            Biomes::Beach => "Beach",
            Biomes::Placeholder => "Placeholder",
            Biomes::Taiga => "Taiga",
            Biomes::SubtropicalDesert => "SubtropicalDesert",
            Biomes::Grassland => "Grassland",
            Biomes::TropicalSeasonalForest => "TropicalSeasonalForest",
            Biomes::TropicalRainForest => "TropicalRainForest",
            Biomes::TemperateDesert => "TemperateDesert",
            Biomes::TemperateDeciduousForest => "TemperateDeciduousForest",
            Biomes::TemperateRainForest => "TemperateRainForest",
            Biomes::Shrubland => "Shrubland",
            Biomes::Scorched => "Scorched",
            Biomes::Bare => "Bare",
            Biomes::Tundra => "Tundra",
            Biomes::Snow => "Snow",
        }
    }

    pub fn get_colour(&self) -> Rgb<u8> {
        match self {
            Biomes::Void => image::Rgb([255, 0, 0]),
            Biomes::FreshWater => image::Rgb([41, 95, 255]),
            Biomes::SaltWater => image::Rgb([0, 5, 206]),
            Biomes::Land => image::Rgb([64, 171, 0]),
            Biomes::Beach => image::Rgb([255, 195, 0]),
            Biomes::Placeholder => image::Rgb([0, 0, 0]),
            Biomes::Taiga => image::Rgb([203, 212, 187]),
            Biomes::SubtropicalDesert => image::Rgb([233, 220, 198]),
            Biomes::Grassland => image::Rgb([196, 211, 170]),
            Biomes::TropicalSeasonalForest => image::Rgb([169, 204, 163]),
            Biomes::TropicalRainForest => image::Rgb([156, 187, 169]),
            Biomes::TemperateDesert => image::Rgb([228, 232, 202]),
            Biomes::TemperateDeciduousForest => image::Rgb([180, 200, 169]),
            Biomes::TemperateRainForest => image::Rgb([163, 196, 168]),
            Biomes::Shrubland => image::Rgb([195, 204, 186]),
            Biomes::Scorched => image::Rgb([153, 153, 153]),
            Biomes::Bare => image::Rgb([187, 187, 187]),
            Biomes::Tundra => image::Rgb([221, 221, 186]),
            Biomes::Snow => image::Rgb([255, 255, 255]),
        }
    }
}

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
