use uuid::Uuid;

pub struct Outlet {
    id: Uuid,
    is_on: bool,
    power: f64,
}

impl Outlet {
    pub fn new(is_on: bool, power: f64) -> Self {
        let id = Uuid::new_v4();
        Self { id, is_on, power }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn is_on(&self) -> bool {
        self.is_on
    }

    pub fn get_power(&self) -> f64 {
        if self.is_on { self.power } else { 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power() {
        let outlet = Outlet::new(true, 10.0);
        assert_eq!(outlet.get_power(), 10.0);
    }

    #[test]
    fn test_turning_off() {
        let mut outlet = Outlet::new(true, 10.0);
        outlet.turn_off();
        assert!(!outlet.is_on());
    }

    #[test]
    fn test_turning_on() {
        let mut outlet = Outlet::new(false, 10.0);
        outlet.turn_on();
        assert!(outlet.is_on());
    }
}
