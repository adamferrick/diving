pub fn weighted_average(value_1: f32, weight_1: f32, value_2: f32, weight_2: f32) -> f32 {
    (weight_1 * value_1 + weight_2 * value_2) / (weight_1 + weight_2)
}
