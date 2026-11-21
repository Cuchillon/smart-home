use crate::SmartDevice;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Room {
    id: Uuid,
    devices: HashMap<Uuid, SmartDevice>,
}

impl Room {
    pub fn new(devices: impl IntoIterator<Item = SmartDevice>) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            devices: devices.into_iter().map(|d| (d.get_id(), d)).collect(),
        }
    }

    pub fn with_id(id: Uuid, devices: impl IntoIterator<Item = SmartDevice>) -> Self {
        Self {
            id,
            devices: devices.into_iter().map(|d| (d.get_id(), d)).collect(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_device(&self, id: Uuid) -> Option<&SmartDevice> {
        self.devices.get(&id)
    }

    pub fn get_device_mut(&mut self, id: Uuid) -> Option<&mut SmartDevice> {
        self.devices.get_mut(&id)
    }

    pub fn print_report(&self) {
        println!("Отчет по комнате (устройств: {}):", self.devices.len());
        for device in self.devices.values() {
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
        let id = Uuid::new_v4();
        let room = Room::new(vec![SmartDevice::Thermometer(Thermometer::with_id(
            id, 22.0,
        ))]);

        if let SmartDevice::Thermometer(thermometer) = room
            .get_device(id)
            .unwrap_or_else(|| panic!("Thermometer for {} not found", id))
        {
            assert_eq!(thermometer.get_temperature(), 22.0);
        } else {
            panic!("Must be thermometer");
        }
    }

    #[test]
    fn test_get_device_not_found() {
        let id = Uuid::new_v4();
        let room = Room::new(vec![SmartDevice::Thermometer(Thermometer::new(22.0))]);
        let not_found_device = room.get_device(id);
        assert!(not_found_device.is_none());
    }
}
