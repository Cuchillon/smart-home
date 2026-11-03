use crate::outlet::Outlet;
use crate::thermometer::Thermometer;

pub enum SmartDevice {
    Thermometer(Thermometer),
    Outlet(Outlet),
}

impl SmartDevice {
    pub fn state_string(&self) -> String {
        match self {
            SmartDevice::Thermometer(thermometer) => format!(
                "[Термометр #{}] Температура: {:.1} С",
                thermometer.get_id(),
                thermometer.get_temperature(),
            ),
            SmartDevice::Outlet(outlet) => {
                let state = if outlet.is_on() { "ВКЛ" } else { "ВЫКЛ" };
                format!(
                    "[Розетка #{}] Состояние: {} | Мощность: {:.1} Вт",
                    outlet.get_id(),
                    state,
                    outlet.get_power(),
                )
            }
        }
    }

    pub fn print_state(&self) {
        println!("  {}", self.state_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer_state_string() {
        let thermometer = Thermometer::new(25.0);
        let id = thermometer.get_id();
        let device = SmartDevice::Thermometer(thermometer);

        let state = device.state_string();
        assert!(state.contains("Термометр"), "state = {state}");
        assert!(state.contains(&format!("#{id}")), "state = {state}");
        assert!(state.contains("25.0"), "state = {state}");
    }

    #[test]
    fn test_outlet_state_string_on() {
        let outlet = Outlet::new(true, 300.0);
        let id = outlet.get_id();
        let device = SmartDevice::Outlet(outlet);

        let state = device.state_string();
        assert!(state.contains("Розетка"), "state = {state}");
        assert!(state.contains(&format!("#{id}")), "state = {state}");
        assert!(state.contains("ВКЛ"), "state = {state}");
        assert!(state.contains("300.0"), "state = {state}");
    }

    #[test]
    fn test_outlet_state_string_off() {
        let device = SmartDevice::Outlet(Outlet::new(false, 30.0));

        let state = device.state_string();
        assert!(state.contains("ВЫКЛ"), "state = {state}");
        // Выключенная розетка не отдает мощность
        assert!(state.contains("Мощность: 0.0 Вт"), "state = {state}");
        assert!(!state.contains("30.0"), "state = {state}");
    }
}
