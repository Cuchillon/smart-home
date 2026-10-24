pub fn get_random_temperature() -> f64 {
    let random: i32 = rand::random_range(-400..500);
    (random / 10) as f64
}

pub fn get_random_power() -> f64 {
    let random: u32 = rand::random_range(10..1000);
    (random / 10) as f64
}
