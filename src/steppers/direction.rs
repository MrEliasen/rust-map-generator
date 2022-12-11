use rand::Rng;
use rand_pcg::Pcg64;

#[derive(Copy, Clone, Debug)]
pub struct MoveDirection {
    pub x: i32,
    pub y: i32,
}

impl PartialEq for MoveDirection {
    fn eq(&self, other: &Self) -> bool {
        return format!("{}_{}", self.x, self.y) == format!("{}_{}", other.x, other.y);
    }
}

impl MoveDirection {
    pub fn opposite_direction(&self) -> MoveDirection {
        MoveDirection {
            x: self.x * -1,
            y: self.y * -1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    pub fn get_move_direction(&self) -> MoveDirection {
        match self {
            Direction::North => MoveDirection { x: 0, y: -1 },
            Direction::NorthWest => MoveDirection { x: -1, y: -1 },
            Direction::NorthEast => MoveDirection { x: 1, y: -1 },
            Direction::South => MoveDirection { x: 0, y: 1 },
            Direction::SouthEast => MoveDirection { x: 1, y: 1 },
            Direction::SouthWest => MoveDirection { x: -1, y: 1 },
            Direction::East => MoveDirection { x: 1, y: 0 },
            Direction::West => MoveDirection { x: -1, y: 0 },
        }
    }

    pub fn get_standard_directions() -> Vec<MoveDirection> {
        let mut dirs: Vec<MoveDirection> = Vec::with_capacity(4);
        // North
        dirs.push(MoveDirection { x: 0, y: -1 });
        // South
        dirs.push(MoveDirection { x: 0, y: 1 });
        // East
        dirs.push(MoveDirection { x: 1, y: 0 });
        // West
        dirs.push(MoveDirection { x: -1, y: 0 });

        dirs
    }

    pub fn get_extended_directions() -> Vec<MoveDirection> {
        let mut dirs: Vec<MoveDirection> = Vec::from(Direction::get_standard_directions());
        // NorthWest
        dirs.push(MoveDirection { x: -1, y: -1 });
        // NorthEast
        dirs.push(MoveDirection { x: 1, y: -1 });
        // SouthEast
        dirs.push(MoveDirection { x: 1, y: 1 });
        // SouthWest
        dirs.push(MoveDirection { x: -1, y: 1 });

        dirs
    }
}

pub fn pick_random_direction(rng: &mut Pcg64, exlude_direction: Option<MoveDirection>) -> MoveDirection {
    let mut dirs = Direction::get_standard_directions();

    if exlude_direction.is_some() {
        dirs = dirs.into_iter().filter(|a| *a != exlude_direction.unwrap()).collect();
    }

    let x = rng.gen_range(0..dirs.len());
    dirs[x]
}
