use image::Rgb;

#[derive(Debug, Copy, Clone)]
pub enum Biomes {
    Void,
    Placeholder,
    FreshWater,
    SaltWater,
    Land,
}

impl Biomes {
    pub fn get_symbol(&self) -> &str {
        match self {
            Biomes::Void => "ø",
            Biomes::FreshWater => "~",
            Biomes::SaltWater => "=",
            Biomes::Land => "#",
            Biomes::Placeholder => " ",
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Biomes::Void => "Void",
            Biomes::FreshWater => "Fresh Water",
            Biomes::SaltWater => "Salt Water",
            Biomes::Land => "Land",
            Biomes::Placeholder => "Placeholder",
        }
    }

    pub fn get_colour(&self) -> Rgb<u8> {
        match self {
            Biomes::Void => image::Rgb([255, 0, 0]),
            Biomes::FreshWater => image::Rgb([41, 95, 246]),
            Biomes::SaltWater => image::Rgb([0, 5, 206]),
            Biomes::Land => image::Rgb([64, 171, 0]),
            Biomes::Placeholder => image::Rgb([0, 0, 0]),
        }
    }
}
