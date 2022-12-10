#[derive(Copy, Clone)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for MapPosition {
    fn eq(&self, other: &Self) -> bool {
        return format!("{}_{}", self.x, self.y) == format!("{}_{}", other.x, other.y);
    }
}

impl MapPosition {
    pub fn x_usize(&self) -> usize {
        self.x as usize
    }

    pub fn y_usize(&self) -> usize {
        self.y as usize
    }
}
