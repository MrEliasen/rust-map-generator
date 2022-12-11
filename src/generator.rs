use rand_seeder::{Seeder};
use rand_pcg::Pcg64;
use image::{ImageBuffer, RgbImage};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::helper::get_distance;
use crate::steppers::{Generators, is_valid_cell};
use crate::biomes::{Biomes, Biome};
use crate::steppers::Stepper;
use crate::steppers::direction::{Direction};
use crate::steppers::map_position::MapPosition;
use std::collections::VecDeque;

pub type MapData = Vec<Vec<Biome>>;

pub struct Generator {
    debug: bool,
    seed: String,
    map_size: u32,
    _rivers: u32,
    steppers: u32,
    steps: u32,
    map_data: MapData,
    _rng: Pcg64,
}

impl Generator {
    pub fn new(debug: bool, seed: String, map_size: u32, _rivers: u32, steppers: u32, steps: u32) -> Self {
        let rng = Seeder::from(&seed).make_rng();

        Self {
            debug,
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
        self.find_replace(Biomes::Void, Biomes::FreshWater, true);
        self.find_replace(Biomes::Void, Biomes::Placeholder, false);

        // generate elevation
        self.generate_elevation();
     
        // create rivers
        // to-do

        // calculate the moisture for each placeholder cell
        self.generate_moisture(); 

        // generate beaches
        self.generate_beaches();        

        // generate biomes
        for x in 0..self.map_size {
            for y in 0..self.map_size {
                if self.map_data[x as usize][y as usize].get_tile_name() != Biomes::Placeholder.get_name() {
                    continue;
                }

                self.map_data[x as usize][y as usize].calculate_biome();
            }
        }
    }

    fn generate_elevation(&mut self) {
        let locations = self.find_tiles_near_type(Biomes::Placeholder, Biomes::SaltWater);

        // normalise the distances by deviding the biggest distance by 4 (the heights elevation possible)
        let per_elevation = locations.first().unwrap().2 / 4.0;

        for tile in locations {
            let mut elevation = (tile.2 / per_elevation).floor();

            if elevation < 1.0 {
                elevation = 1.0;
            }

            self.map_data[tile.0 as usize][tile.1 as usize].distance_from_sea = tile.2 as u32;
            self.map_data[tile.0 as usize][tile.1 as usize].elevation = elevation as u32;
        }
    }

    fn generate_moisture(&mut self) {
        let locations = self.find_tiles_near_type(Biomes::Placeholder, Biomes::FreshWater);

        // normalise the distances by deviding the biggest distance by 6 (the height level of moisture)
        let per_moisture_stage = locations.first().unwrap().2 / 6.0;

        for tile in locations {
            let mut moisture = 6 - (tile.2 / per_moisture_stage).ceil() as u32 + 1;

            if moisture < 1 {
                moisture = 1;
            }

            self.map_data[tile.0 as usize][tile.1 as usize].distance_from_fresh_water = tile.2 as u32;
            self.map_data[tile.0 as usize][tile.1 as usize].moisture = moisture as u32;
        }
    }

    fn find_tiles_near_type(&mut self, find_biome: Biomes, near_biome: Biomes) -> Vec<(u32, u32, f32)> {
        let mut locations: Vec<(u32, u32, f32)> = Vec::new();
        let biome_type = find_biome.get_name();

        for x in 0..self.map_size {
            for y in 0..self.map_size {
                if self.map_data[x as usize][y as usize].get_tile_name() != biome_type {
                    continue;
                }

                let nearest = self.find_nearest(x, y, near_biome);

                if nearest.is_none() {
                    continue;
                }

                locations.push((x, y, nearest.unwrap().2));
            }
        }

        // // sort by distance
        locations.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        return locations;
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

    fn find_nearest(&mut self, x_origin: u32, y_origin: u32, find_biome: Biomes) -> Option<(u32, u32, f32)> {
        let type_name = find_biome.get_name();
        let mut closest: Vec<(u32, u32, f32)> = Vec::new();
        let mut x_start = x_origin as i32;
        let mut x_max = x_origin as i32;
        let mut y_start = y_origin as i32;
        let mut y_max = y_origin as i32;
        let map_size_i32 = self.map_size as i32;

        while closest.is_empty() {
            x_start -= 1;
            y_start -= 1;
            x_max += 1;
            y_max += 1;

            if x_start < 0 && y_start < 0 && x_max >= map_size_i32 && y_max >= map_size_i32 {
                break;
            }

            // let mut nx = x_start;
            // this prevents it from looping over coordinates which are not within the vector
            let optimised_x_start  = if x_max > map_size_i32 { map_size_i32 } else { x_max };
            let mut nx = if x_start < 0 { 0 } else { x_start };
            let optimised_x_max = if x_max > map_size_i32 { map_size_i32 } else { x_max };

            while nx <= optimised_x_max {
                nx += 1;

                // let mut ny = y_start;
                // this prevents it from looping over coordinates which are not within the vector
                let optimised_y_start  = if y_start < 0 { 0 } else { y_start };
                let mut ny  = if y_start < 0 { 0 } else { y_start };
                let optimised_y_max = if y_max > map_size_i32 { map_size_i32 } else { y_max };

                while ny <= optimised_y_max {
                    ny += 1;

                    // ignore tiles which we have already checked
                    if ny > optimised_y_start && ny < y_max && nx > optimised_x_start && nx < x_max {
                        continue;
                    }

                    if !is_valid_cell(&self.map_size, nx, ny) {
                        continue;
                    }

                    if type_name != self.map_data[nx as usize][ny as usize].get_tile_name() {
                        continue;
                    }

                    closest.push((
                        nx as u32,
                        ny as u32,
                        get_distance(x_origin, y_origin, nx as u32, ny as u32)
                    ));
                }
            }
        }

        closest.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        return closest.first().copied();
    }

    fn find_replace(&mut self, find: Biomes, replace: Biomes, ignore_solo_tiles: bool) {
        let find_type = find.get_name();
        let mut replacements: Vec<(usize, usize)> = vec![];

        for x in 0..self.map_size {
            for y in 0..self.map_size {
                if self.map_data[x as usize][y as usize].get_tile_name() != find_type {
                    continue;
                }

                if ignore_solo_tiles {
                    let neighbours: Vec<MapPosition> = self.get_tile_neighbours(
                        &MapPosition { x: x as i32, y: y as i32 },
                        &self.map_data[x as usize][y as usize].tile_type,
                        false
                    );

                    if neighbours.len() == 0 {
                        continue;
                    }
                }

                replacements.push((x as usize, y as usize));
            }
        }

        for tile in replacements {
            self.map_data[tile.0][tile.1].tile_type = replace;
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

        file.write_all("\n\nsymbols:\n".as_bytes()).unwrap();
        // render map
        for col in self.map_data.iter() {
            for tile in col.iter() {
                file.write_all(tile.get_tile_symbol().as_bytes()).unwrap();
            }

            file.write_all("\n".as_bytes()).unwrap();
        }

        file.write_all(format!("\n\nSeed: {}", &self.seed).as_bytes())
            .unwrap();
    }

    pub fn output_image(&self, file_name: String, draw_multiplier: u32) {
        let debug_multiplier = if self.debug { 3 } else { 1 };
        let mut image: RgbImage = ImageBuffer::new(self.map_size * draw_multiplier, (self.map_size * draw_multiplier) * debug_multiplier);
        let mut offset = 0;

        // render map
        for (x, col) in self.map_data.iter().enumerate() {
            for (y, tile) in col.iter().enumerate() {
                let tile_colour = tile.get_tile_colour();

                for x_step in 0..draw_multiplier {
                    for y_step in 0..draw_multiplier {
                        let my = y as u32 * draw_multiplier + y_step;
                        let mx = x as u32 * draw_multiplier + x_step;

                        image.put_pixel(mx as u32, my as u32, tile_colour);
                    }
                }
            }
        }

        if self.debug {
            offset += self.map_size * draw_multiplier;

            // render map
            for (x, col) in self.map_data.iter().enumerate() {
                for (y, tile) in col.iter().enumerate() {
                    let moisture = tile.get_moisture_colour();

                    for x_step in 0..draw_multiplier {
                        for y_step in 0..draw_multiplier {
                            let my = y as u32 * draw_multiplier + y_step + offset;
                            let mx = x as u32 * draw_multiplier + x_step;

                            image.put_pixel(mx as u32, my as u32, moisture);
                        }
                    }
                }
            }

            offset += self.map_size * draw_multiplier;

            // render map
            for (x, col) in self.map_data.iter().enumerate() {
                for (y, tile) in col.iter().enumerate() {
                    let elevation = tile.get_elevation_colour();

                    for x_step in 0..draw_multiplier {
                        for y_step in 0..draw_multiplier {
                            let my = y as u32 * draw_multiplier + y_step + offset;
                            let mx = x as u32 * draw_multiplier + x_step;

                            image.put_pixel(mx as u32, my as u32, elevation);
                        }
                    }
                }
            }
        }

        // write it out to a file
        image.save(&file_name).unwrap();
    }
}
