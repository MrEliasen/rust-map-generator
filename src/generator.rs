use rand_seeder::{Seeder};
use rand_pcg::Pcg64;
use image::{ImageBuffer, RgbImage};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::steppers::{Generators, is_valid_cell};
use crate::biomes::{Biomes, Biome};
use crate::steppers::Stepper;
use crate::steppers::direction::{Direction};
use crate::steppers::map_position::MapPosition;
use std::collections::VecDeque;

pub type MapData = Vec<Vec<Biome>>;

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
            map_data: vec![vec![Biome::new_empty(); map_size as usize]; map_size as usize],
            _rng: rng,
        }
    }

    pub async fn generate(&mut self) {
        self.run().await;
    }

    async fn run(&mut self) {
        let land_stepper = Generators::LandGenerator;

        for index in 0..self.steppers {
            let mut seed = String::from(&self.seed);
            seed.push_str(&index.to_string());

            let rng = Seeder::from(seed).make_rng();

            let mut stepper = Stepper::create(
                rng,
                self.map_size,
                self.steps,
            );

            stepper.run(&mut self.map_data, land_stepper).await;
        }

        // post process the raw map
        self.post_proccess();

        let mut ignored_tiles: Vec<MapPosition> = Vec::new();
        let void = Biomes::Void.get_name();

        for x in 0..self.map_size {
            for y in 0..self.map_size {
                if self.map_data[x as usize][y as usize].get_tile_name() != void {
                    ignored_tiles.push(MapPosition {x: x as i32, y: y as i32});
                }
            }
        }

        // flood fill with salt water
        self.flood_fill(Biomes::SaltWater, 0, 0, &mut ignored_tiles);

        // Replace last void tiles with fresh water
        self.find_replace(Biomes::Void, Biomes::FreshWater);

        // generate elevation
        self.generate_elevation();

        // create rivers

        // calculate the moisture for each placeholder cell
        self.generate_moisture();        

        // generate beaches
        self.generate_beaches();        

        // generate biomes
        for x in 0..self.map_size {
            for y in 0..self.map_size {
                let tile = &self.map_data[x as usize][y as usize];

                if tile.get_tile_name() != Biomes::Placeholder.get_name() {
                    continue;
                }

                self.map_data[x as usize][y as usize].calculate_biome();
            }
        }
    }

    fn generate_elevation(&mut self) {
        let mut ground_elevations: Vec<(u32, u32, f32)> = Vec::new();
        let map_size = self.map_size;
        let placeholder = Biomes::Placeholder.get_name();

        for x in 0..map_size {
            for y in 0..map_size {
                let tile = &self.map_data[x as usize][y as usize];

                if tile.get_tile_name() != placeholder {
                    continue;
                }

                let (loc_x, loc_y, distance) = self.find_nearest(x, y, Biomes::SaltWater);

                if distance.is_none() {
                    continue;
                }

                ground_elevations.push((loc_x.unwrap(), loc_y.unwrap(), distance.unwrap()));
            }
        }

        // // sort by distance
        ground_elevations.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        dbg!(ground_elevations.first(), ground_elevations.last());

        // normalise the distances by deviding the biggest distance by 4
        let per_elevation = ground_elevations.first().unwrap().2 / 4.0;
        dbg!(per_elevation);

        for tile in ground_elevations {
            let mut elevation = tile.2 / per_elevation;

            if elevation == 0.0 {
                elevation = 1.0;
            }

            self.map_data[tile.0 as usize][tile.1 as usize].distance_from_sea = tile.2 as u32;
            self.map_data[tile.0 as usize][tile.1 as usize].elevation = elevation as u32;
        }
    }

    fn generate_moisture(&mut self) {
        let mut moistures: Vec<(u32, u32, f32)> = Vec::new();
        let map_size = self.map_size;

        for x in 0..map_size {
            for y in 0..map_size {
                let tile = &self.map_data[x as usize][y as usize];

                if tile.get_tile_name() != Biomes::Placeholder.get_name() {
                    continue;
                }

                let (loc_x, loc_y, distance) = self.find_nearest(x, y, Biomes::FreshWater);

                if distance.is_none() {
                    continue;
                }

                moistures.push((loc_x.unwrap(), loc_y.unwrap(), distance.unwrap()));
            }
        }

        // // sort by distance
        moistures.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        dbg!(moistures.first(), moistures.last());

        // // normalise the distances by deviding the biggest distance by 4
        let per_moisture_stage = moistures.first().unwrap().2 / 6.0;
        dbg!(per_moisture_stage);

        for tile in moistures {
            let mut moisture = 6 - (tile.2 / per_moisture_stage).ceil() as u32 + 1;

            if moisture > 6 {
                moisture = 6;
            }

            self.map_data[tile.0 as usize][tile.1 as usize].distance_from_fresh_water = tile.2 as u32;
            self.map_data[tile.0 as usize][tile.1 as usize].moisture = moisture;
        }
    }

    fn generate_beaches(&mut self) {
        for x in 0..self.map_size {
            for y in 0..self.map_size {
                let tile = &self.map_data[x as usize][y as usize];

                if tile.get_tile_name() != Biomes::Placeholder.get_name() {
                    continue;
                }

                if tile.elevation != 1 || tile.moisture > 2 {
                    continue;
                }

                let neighbours = self.get_tile_neighbours(
                    &MapPosition { x: x as i32, y: y as i32 },
                    &Biomes::SaltWater,
                    true
                );

                if neighbours.len() >= 1 {
                    self.map_data[x as usize][y as usize].tile_type = Biomes::Beach;
                }
            }
        }
    }

    fn get_distance(&self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> f32{
        let y = to_x as f32 - from_x as f32;
        let x = to_y as f32 - from_y as f32;

        return f32::sqrt(x * x + y * y);
    }

    fn find_nearest(&mut self, x: u32, y: u32, of_type: Biomes) -> (Option<u32>, Option<u32>, Option<f32>) {
        let type_name = of_type.get_name();

        let mut closest_x: Option<u32> = None;
        let mut closest_y: Option<u32> = None;
        let mut closest_distance: Option<f32> = None;

        // Iterate over the outer Vec.
        for (nx, col) in self.map_data.iter().enumerate() {
            // Iterate over the inner Vec.
            for (ny, tile) in col.iter().enumerate() {
                if tile.get_tile_name() != type_name {
                    continue;
                }

                let distance = self.get_distance(x, y, nx as u32, ny as u32) as u32;

                if closest_distance.is_none() || (distance as f32) < closest_distance.unwrap() {
                    closest_x = Some(nx as u32);
                    closest_y = Some(ny as u32);
                    closest_distance = Some(distance as f32);
                }
            }
        }

        return (closest_x, closest_y, closest_distance);
    }

    fn find_replace(&mut self, find: Biomes, replace: Biomes) {
        let find_type = find.get_name();

        for x in 0..self.map_size {
            for y in 0..self.map_size {
                if self.map_data[x as usize][y as usize].get_tile_name() != find_type {
                    continue;
                }

                self.map_data[x as usize][y as usize].tile_type = replace;
            }
        }
    }

    fn flood_fill(&mut self, biome: Biomes, start_row: i32, start_col: i32, ignore: &mut Vec<MapPosition>) {
        let rows = self.map_size as i32;
        let cols = self.map_size as i32;
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; cols as usize]; rows as usize];

        for pos in ignore {
            visited[pos.x as usize][pos.y as usize] = true;
        }

        queue.push_back((start_row, start_col));

        while let Some((row, col)) = queue.pop_front() {
            if row >= 0 && row < rows && col >= 0 && col < cols && !visited[row as usize][col as usize] {
                visited[row as usize][col as usize] = true;
                self.map_data[row as usize][col as usize].tile_type = biome;

                queue.push_back((row + 1, col));
                queue.push_back((row - 1, col));
                queue.push_back((row, col + 1));
                queue.push_back((row, col - 1));
            }
        }
    }

    fn post_proccess(&mut self) {
        // remove long stragglers
        self.remove_stragglers();

        for x in 0..self.map_size {
            for y in 0..self.map_size {
                let tile = self.map_data[x as usize][y as usize].tile_type;
                self.clean_tile(&tile, &MapPosition {x: x as i32, y: y as i32}, false);
            }
        }
    }

    fn remove_stragglers(&mut self) {
        for x in 0..self.map_size {
            for y in 0..self.map_size {
                let tile = self.map_data[x as usize][y as usize].tile_type;

                // only remove land placeholders
                if let Biomes::Placeholder = tile {
                    let current_pos = MapPosition {
                        x: x as i32,
                        y: y as i32
                    };

                    let neighbours: Vec<MapPosition> = self.get_tile_neighbours(&current_pos, &tile, true);

                    if neighbours.len() <= 2 {
                        self.map_data[x as usize][y as usize].tile_type = Biomes::Void;
                    }
                }
            }
        }
    }

    fn clean_tile(&mut self, tile: &Biomes, position: &MapPosition, skip: bool) {
        if skip {
            return;
        }

        // ignore outter rim
        if position.y == 0 // top
            || position.y as u32 == self.map_size - 1 // bottom
            || position.x == 0 // left
            || position.x as u32 == self.map_size - 1 // right
         {
            self.map_data[position.x_usize()][position.y_usize()].tile_type = Biomes::Void;
            return;
        }

        let neighbours = self.get_tile_neighbours(position, tile, false);

        if neighbours.len() > 1 {
            return;
        }

        // if a tile is alone, convert it
        if neighbours.len() < 2 {
            self.map_data[position.x_usize()][position.y_usize()].tile_type = match tile {
                Biomes::Void => Biomes::Placeholder,
                Biomes::FreshWater => Biomes::Placeholder,
                _ => Biomes::Void,
            };
        }

        neighbours.iter().for_each(|position| {
            let tile = self.map_data[position.x_usize()][position.y_usize()].tile_type;

            self.clean_tile(&tile, &position, false)
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

                if is_valid_cell(&self.map_size, n_x, n_y) {
                    if self.map_data[n_x as usize][n_y as usize].tile_type.get_name() == biome.get_name() {
                        neighbours.push(MapPosition { x: n_x, y: n_y });
                    }
                }
            });

        return neighbours;
    }

    pub fn output_file(&self, file_name: String) {
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(msg) => panic!("could not create {}: {}", display, msg),
        };


        file.write_all(format!("Seed: {}", &self.seed).as_bytes()).unwrap();
        file.write_all("\n\nElevation:\n".as_bytes()).unwrap();

        // render map
        for col in self.map_data.iter() {
            for tile in col.iter() {
                file.write_all(tile.elevation.to_string().as_bytes()).unwrap();
            }

            file.write_all("\n".as_bytes()).unwrap();
        }

        file.write_all("\n\nmoisture:\n".as_bytes()).unwrap();
        // render map
        for col in self.map_data.iter() {
            for tile in col.iter() {
                file.write_all(tile.moisture.to_string().as_bytes()).unwrap();
            }

            file.write_all("\n".as_bytes()).unwrap();
        }

        file.write_all(format!("\n\nSeed: {}", &self.seed).as_bytes())
            .unwrap();
    }

    pub fn output_image(&self, file_name: String, multiplier: u32) {
        let mut image: RgbImage = ImageBuffer::new(self.map_size * multiplier, self.map_size * multiplier);

        // render map
        for (x, col) in self.map_data.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                let tile_colour = tile.get_tile_colour();

                for x_step in 0..multiplier {
                    for y_step in 0..multiplier {
                        let my = y as u32 * multiplier + y_step;
                        let mx = x as u32 * multiplier + x_step;

                        image.put_pixel(mx as u32, my as u32, tile_colour);
                    }
                }
            }
        }

        // write it out to a file
        image.save(&file_name).unwrap();
    }
}
