pub fn get_distance(from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> f32{
    let y = to_x as f32 - from_x as f32;
    let x = to_y as f32 - from_y as f32;

    return f32::sqrt(x * x + y * y);
}