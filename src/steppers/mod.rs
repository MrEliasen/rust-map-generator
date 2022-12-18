use rand::Rng;
use crate::MapData;
use crate::steppers::direction::pick_random_direction;
use crate::steppers::map_position::MapPosition;
use rand_pcg::Pcg64;

use self::landmass::Landmass;
use self::move_direction::MoveDirection;

pub mod landmass;
pub mod direction;
pub mod map_position;
pub mod move_direction;

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

pub struct Stepper {
    steps: u32,
    map_size: i32,
    rng: Pcg64,
    start_pos: MapPosition,
}

impl Stepper {
    pub fn create(rng: Pcg64, map_size: u32, steps: u32, start_pos: MapPosition) -> Self {
        Self {
            map_size: map_size as i32,
            steps,
            rng,
            start_pos,
        }
    }

    fn calculate_direction_exclusions(&mut self, current_pos: &MapPosition) -> Vec<MoveDirection> {
        let mut excludes: Vec<MoveDirection> = Vec::new();
        let faction_map_sixe = self.map_size / 10;

        if current_pos.x < faction_map_sixe {
            excludes.push(MoveDirection { x: -1, y: 0 });
        }

        if current_pos.y < faction_map_sixe {
            excludes.push(MoveDirection { x: 0, y: -1 });
        }

        if current_pos.x > self.map_size - faction_map_sixe {
            excludes.push(MoveDirection { x: 1, y: 0 });
        }

        if current_pos.y > self.map_size - faction_map_sixe {
            excludes.push(MoveDirection { x: 0, y: 1 });
        }

        excludes
    }

    pub fn run(&mut self, map_data: &mut MapData, generator: Generators) {
        let mut excludes: Vec<MoveDirection> = Vec::new();
        let mut steps_left = self.steps;
        let mut last_direction: MoveDirection = pick_random_direction(
            &mut self.rng,
            Vec::new(),
        );
        let mut current_direction: MoveDirection = last_direction;
        let mut last_direction_steps: u32 = 0;
        let stepper_generator = generator.get_generator();

        // randomised a map position offset
        let mut current_pos = self.start_pos;

        while steps_left > 0 {
            if self.rng.gen_range(0..=1) == 1 {
                excludes.push(last_direction.opposite_direction());

                current_direction = pick_random_direction(
                    &mut self.rng,
                    excludes,
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

            excludes = self.calculate_direction_exclusions(&current_pos);

            if last_direction != current_direction {
                last_direction_steps = 0;
                last_direction = current_direction;
            }

            last_direction_steps += 1;

            let proceed = stepper_generator.on_step(map_data, current_pos, steps_left, last_direction_steps);

            steps_left -= 1;

            if steps_left == 0 {
                stepper_generator.on_last_step(map_data, current_pos);
            }

            if !proceed {
                break;
            }
        }
    }
}
