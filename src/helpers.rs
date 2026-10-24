pub fn get_random_temperature() -> f64 {
    let random: i32 = rand::random_range(-400..500);
    (random / 10) as f64
}

pub fn get_random_power() -> f64 {
    let random: u32 = rand::random_range(10..1000);
    (random / 10) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_temperature() {
        for _ in 0..10 {
            let temperature = get_random_temperature();
            assert!(temperature > -40.0 && temperature < 50.0);
        }
    }

    #[test]
    fn test_get_random_power() {
        for _ in 0..10 {
            let power = get_random_power();
            assert!(power > 1.0 && power < 100.0);
        }
    }
}
