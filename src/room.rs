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
