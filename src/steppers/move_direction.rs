#[derive(Copy, Clone, Debug)]
pub struct MoveDirection {
    pub x: i32,
    pub y: i32,
}

impl MoveDirection {
    pub fn opposite_direction(&self) -> MoveDirection {
        MoveDirection {
            x: self.x * -1,
            y: self.y * -1,
        }
    }
}

impl PartialEq for MoveDirection {
    fn eq(&self, other: &Self) -> bool {
        return format!("{}_{}", self.x, self.y) == format!("{}_{}", other.x, other.y);
    }
}