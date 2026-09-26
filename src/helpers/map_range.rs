pub fn map_range(val: f32, x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    y1 + ((val - x1) * (y2 - y1)) / (x2 - x1)
}