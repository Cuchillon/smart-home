use crate::outlet::Outlet;
use crate::thermometer::Thermometer;

pub enum SmartDevice {
    Thermometer(Thermometer),
    Outlet(Outlet),
}

impl SmartDevice {

    pub fn print_state(&self) {
        match self {
            SmartDevice::Thermometer(thermometer) => {
                println!(
                    "  [Термометр #{}] Температура: {:.1} С",
                    thermometer.get_id(),
                    thermometer.get_temperature(),
                );
            }
            SmartDevice::Outlet(outlet) => {
                let state = if outlet.is_on() { "ВКЛ" } else { "ВЫКЛ" };
                println!(
                    "  [Розетка #{}] Состояние: {} | Мощность: {:.1} Вт",
                    outlet.get_id(),
                    state,
                    outlet.get_power(),
                );
            }
        }
    }
}
