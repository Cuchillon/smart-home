pub fn get_random_temperature() -> f64 {
    rand::random_range(-40.0..50.0)
}

pub fn get_random_power() -> f64 {
    rand::random_range(1.0..100.0)
}

#[macro_export]
macro_rules! build_room {
    ($($device:expr),* $(,)?) => {{
        let mut room = $crate::Room::new(vec![]);
        $(room.add_device($device);)*
        room
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Outlet, Thermometer};
    use uuid::Uuid;

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

    #[test]
    fn test_build_room() {
        let (thermometer_id, outlet_id) = (Uuid::new_v4(), Uuid::new_v4());

        let room = build_room!(
            Thermometer::with_id(thermometer_id, 22.0).into(),
            Outlet::with_id(outlet_id, true, 10.0).into(),
        );

        assert!(room.get_device(thermometer_id).is_some());
        assert!(room.get_device(outlet_id).is_some());
    }
}
