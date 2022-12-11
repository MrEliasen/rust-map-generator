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

pub const WHITTAKER: [[Biomes; 6]; 4] = [
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