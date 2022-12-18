use crate::helper::is_valid_cell;
use crate::MapData;
use crate::biomes::biomes::Biomes;
use crate::steppers::MapPosition;

pub struct Landmass {}

impl Landmass {
    pub fn create() -> Self {
        Self {}
    }

    pub fn on_step(
        &self,
        map_data: &mut MapData,
        current_pos: MapPosition,
        steps_left: u32,
        last_direction_steps: u32,
    ) -> bool {
        if !is_valid_cell(&(map_data.len() as u32), current_pos.x, current_pos.y) {
            return false;
        }

        if steps_left % 10 == 0 || last_direction_steps == 4 {
            self.fill_area(map_data, current_pos, 4);
        }

        map_data[current_pos.x_usize()][current_pos.y_usize()].tile_type = Biomes::Placeholder;

        true
    }

    pub fn on_last_step(&self, map_data: &mut MapData, current_pos: MapPosition) {
        self.fill_area(map_data, current_pos, 4);
    }

    fn fill_area(&self, map_data: &mut MapData, current_pos: MapPosition, fill_size: u32) {
        let fill_neg = fill_size as i32 * -1;
        let fill_pos = fill_size as i32;
        let mut x_offset = fill_neg;
        let mut y_offset = fill_neg;
        let max_x_offset = fill_pos;
        let max_y_offset = fill_pos;

        while x_offset <= max_x_offset {
            let new_x = current_pos.y + x_offset;

            while y_offset <= max_y_offset {
                let new_y = current_pos.x + y_offset;

                // do not touch the far corners, to round it off a bit
                if (x_offset == fill_neg && y_offset == fill_neg)
                    && (x_offset == fill_pos && y_offset == fill_pos)
                    && (x_offset == fill_neg && y_offset == fill_pos)
                    && (x_offset == fill_pos && y_offset == fill_neg)
                {
                    y_offset += 1;
                    continue;
                }

                if is_valid_cell(&(map_data.len() as u32), new_x, new_y) {
                    map_data[new_x as usize][new_y as usize].tile_type = Biomes::Placeholder;
                }

                y_offset += 1;
            }

            y_offset = fill_neg;
            x_offset += 1;
        }
    }
}
