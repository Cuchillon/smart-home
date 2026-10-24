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
        if self.is_on {
            self.power
        } else {
            0.0
        }
    }
}
