use rand::Rng;
use crate::generator::MapData;
use crate::steppers::direction::pick_random_direction;
use crate::steppers::map_position::MapPosition;
use rand_pcg::Pcg64;

use self::direction::MoveDirection;
use self::landmass::Landmass;

pub mod landmass;
pub mod direction;
pub mod map_position;

#[derive(Clone, Copy)]
pub enum Generators {
    LandGenerator,
}

impl Generators {
    pub fn get_generator(&self) -> Landmass{
        match self {
            Generators::LandGenerator => Landmass::create(),
        }
    }
}

pub fn is_valid_cell(map_size: &u32, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    if x >= *map_size as i32 || y >= *map_size as i32 {
        return false;
    }

    return true;
}

pub struct Stepper {
    steps: u32,
    map_size: i32,
    rng: Pcg64,
    //startLocation: MapPosition,
    //blockedDirections: Vec<MapPosition>,
    //priorityDirection: Direction,
    //priorityPercent: f32,
    // stepsHistory: Vec<Vec<MapPosition>>,
}

impl Stepper {
    pub fn create(rng: Pcg64, map_size: u32, steps: u32) -> Self {
        Self {
            map_size: map_size as i32,
            steps: steps,
            rng: rng,
        }
    }

    pub async fn run(&mut self, map_data: &mut MapData, generator: Generators) {
        let mut steps_left = self.steps;
        let mut current_pos = MapPosition {
            x: self.map_size / 2,
            y: self.map_size / 2,
        };
        let mut last_direction: MoveDirection = pick_random_direction(
            &mut self.rng,
            None,
        );
        let mut current_direction: MoveDirection = last_direction;
        let mut last_direction_steps: u32 = 0;
        let stepper_generator = generator.get_generator();

        while steps_left > 0 {
            if self.rng.gen_range(0..=1) == 1 {
                current_direction = pick_random_direction(
                    &mut self.rng,
                    Some(last_direction.opposite_direction()),
                );
            }

            current_pos.x = current_pos.x + current_direction.x;
            current_pos.y = current_pos.y + current_direction.y;

            if current_pos.x <= 0 || current_pos.x >= self.map_size - 1 {
                current_pos.x = current_pos.x + (current_direction.x * 2) * -1;
            }

            if current_pos.y <= 0 || current_pos.y >= self.map_size - 1 {
                current_pos.y += (current_direction.y * 2) * -1;
            }

            if last_direction != current_direction {
                last_direction_steps = 0;
                last_direction = current_direction;
            }

            last_direction_steps += 1;

            // this.stepsHistory.push({ x: stepperX, y: stepperY });

            let proceed = stepper_generator.on_step(map_data, current_pos, last_direction_steps);

            steps_left -= 1;

            if !proceed {
                break;
            }
        }
    }
}
