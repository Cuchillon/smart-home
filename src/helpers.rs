pub fn get_random_temperature() -> f64 {
    rand::random_range(-40.0..50.0)
}

pub fn get_random_power() -> f64 {
    rand::random_range(1.0..100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_temperature() {
        for _ in 0..10 {
            let temperature = get_random_temperature();
            assert!(temperature >= -40.0 && temperature < 50.0);
        }
    }

    #[test]
    fn test_get_random_power() {
        for _ in 0..10 {
            let power = get_random_power();
            assert!(power >= 1.0 && power < 100.0);
        }
    }
}
