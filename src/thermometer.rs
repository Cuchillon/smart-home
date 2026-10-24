use uuid::Uuid;

pub struct Thermometer {
    id: Uuid,
    temperature: f64,
}

impl Thermometer {
    pub fn new(temperature: f64) -> Self {
        let id = Uuid::new_v4();
        Self { id, temperature }
    }
    
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_temperature(&self) -> f64 {
        self.temperature
    }
}
