use crate::SmartDevice;

pub struct Room {
    devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(devices: Vec<SmartDevice>) -> Self {
        Self { devices }
    }

    pub fn get_device(&self, index: usize) -> &SmartDevice {
        &self.devices[index]
    }

    pub fn get_device_mut(&mut self, index: usize) -> &mut SmartDevice {
        &mut self.devices[index]
    }

    pub fn print_report(&self) {
        println!("Отчет по комнате (устройств: {}):", self.devices.len());
        for device in &self.devices {
            device.print_state();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::outlet::Outlet;
    use crate::thermometer::Thermometer;

    #[test]
    fn test_devices() {
        let room = Room::new(vec![
            SmartDevice::Thermometer(Thermometer::new(22.0)),
            SmartDevice::Outlet(Outlet::new(true, 150.0)),
            SmartDevice::Thermometer(Thermometer::new(19.5)),
        ]);
        assert_eq!(room.devices.len(), 3);
    }

    #[test]
    fn test_get_device() {
        let room = Room::new(vec![SmartDevice::Thermometer(Thermometer::new(22.0))]);

        if let SmartDevice::Thermometer(thermometer) = room.get_device(0) {
            assert_eq!(thermometer.get_temperature(), 22.0);
        } else {
            panic!("Must be thermometer");
        }
    }

    #[test]
    #[should_panic]
    fn test_get_device_out_of_bounds() {
        let room = Room::new(vec![SmartDevice::Thermometer(Thermometer::new(22.0))]);
        let _ = room.get_device(1);
    }
}
