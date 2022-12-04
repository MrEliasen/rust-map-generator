use rand_seeder::{Seeder};
use rand_pcg::Pcg64;
use image::{ImageBuffer, RgbImage};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::steppers::{Generators, is_valid_cell};
use crate::biomes::Biomes;
use crate::steppers::Stepper;
use crate::steppers::direction::Direction;
use crate::steppers::map_position::MapPosition;

pub type MapData = Vec<Vec<Biomes>>;

pub struct Generator {
    seed: String,
    map_size: u32,
    _rivers: u32,
    steppers: u32,
    steps: u32,
    map_data: MapData,
    _rng: Pcg64,
}

impl Generator {
    pub fn new(seed: String, map_size: u32, _rivers: u32, steppers: u32, steps: u32) -> Self {
        let rng = Seeder::from(&seed).make_rng();

        Self {
            seed,
            map_size,
            _rivers,
            steppers,
            steps,
            map_data: vec![vec![Biomes::Void; map_size as usize]; map_size as usize],
            _rng: rng,
        }
    }

    pub async fn generate(&mut self) {
        self.run().await;
    }

    async fn run(&mut self) {
        let land_stepper = Generators::LandGenerator;

        for index in 0..self.steppers {
            let seed = String::from(&self.seed).push_str(&index.to_string());
            let rng = Seeder::from(seed).make_rng();

            let mut stepper = Stepper::create(
                rng,
                self.map_size,
                self.steps,
            );

            stepper.run(&mut self.map_data, land_stepper).await;
        }

        self.post_proccess();
    }

    fn post_proccess(&mut self) {
        // remove long stragglers
        let mut cleaned_map_data = self.map_data.clone();

        self.remove_stragglers(&mut cleaned_map_data);

        // run from top to bottom
        // for (x, col) in self.map_data.iter().enumerate() {
        //     for (y, tile) in col.iter().enumerate() {
        //         self.clean_tile(&mut cleaned_map_data, tile, &MapPosition {x: x as i32, y: y as i32}, false);
        //     }
        // }

        self.map_data = cleaned_map_data;
    }

    fn remove_stragglers(&mut self, map_data: &mut MapData) {
        for (x, col) in self.map_data.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                // ignore placeholders
                match tile {
                    Biomes::Placeholder => {
                        continue;
                    },
                    _ => (),
                }

                let current_pos = MapPosition {
                    x: x as i32,
                    y: y as i32
                };

                let neighbours: Vec<MapPosition> = self.get_tile_neighbours(&current_pos, tile, true);

                if neighbours.len() <= 2 {
                    map_data[x][y] = Biomes::Void;
                }
            }
        }
    }

    fn clean_tile(&self, map_data: &mut MapData, tile: &Biomes, position: &MapPosition, skip: bool) {
        if skip {
            return;
        }

        // ignore outter rim
        if position.y == 0 // top
            || position.y as u32 == self.map_size - 1 // bottom
            || position.x == 0 // left
            || position.x as u32 == self.map_size - 1 // right
         {
            map_data[position.x_usize()][position.y_usize()] = Biomes::Void;
            return;
        }

        let neighbours = self.get_tile_neighbours(position, tile, false);

        if neighbours.len() > 1 {
            return;
        }

        // if a tile is alone, convert it
        if neighbours.len() < 2 {
            map_data[position.x_usize()][position.y_usize()] = match tile {
                Biomes::Void => Biomes::Placeholder,
                Biomes::FreshWater => Biomes::Placeholder,
                _ => Biomes::Void,
            };
        }

        neighbours.iter().for_each(|position| {
            let tile = map_data[position.x_usize()][position.y_usize()];

            self.clean_tile(map_data, &tile, &position, false)
        });
    }

    fn get_tile_neighbours(&self, position: &MapPosition, biome: &Biomes, cross_direction: bool) -> Vec<MapPosition> {
        let mut neighbours: Vec<MapPosition> = Vec::new();
        let mut directions = Direction::get_standard_directions();

        if cross_direction {
            directions = Direction::get_extended_directions();
        }

        directions
            .iter_mut()
            .for_each(|direction| {
                let n_x = position.x + direction.x;
                let n_y = position.y + direction.y;

                if is_valid_cell(&self.map_data, n_x, n_y) {
                    if self.map_data[n_x as usize][n_y as usize].get_name() == biome.get_name() {
                        neighbours.push(MapPosition { x: n_x, y: n_y });
                    }
                }
            });

        return neighbours;
    }

    pub fn output_file(&self) {
        let path = Path::new("output.txt");
        let display = path.display();

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(msg) => panic!("could not create {}: {}", display, msg),
        };

        // render map
        for col in self.map_data.iter() {
            for tile in col.iter() {
                file.write_all(tile.get_symbol().as_bytes()).unwrap();
            }

            file.write_all("\n".as_bytes()).unwrap();
        }

        file.write_all(format!("\n\nSeed: {}", &self.seed).as_bytes())
            .unwrap();
    }

    pub fn output_image(&self) {
        let mut image: RgbImage = ImageBuffer::new(self.map_size, self.map_size);

        // render map
        for (x, col) in self.map_data.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                image.put_pixel(x as u32, y as u32, tile.get_colour());
            }
        }

        // write it out to a file
        image.save("output.png").unwrap();
    }
}
