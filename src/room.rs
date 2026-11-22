use crate::{Reporter, SmartDevice};
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

    pub fn add_device(&mut self, device: SmartDevice) {
        self.devices.insert(device.get_id(), device);
    }

    pub fn remove_device(&mut self, id: Uuid) -> Option<SmartDevice> {
        self.devices.remove(&id)
    }
}

impl Reporter for Room {
    fn print_report(&self) {
        println!("Отчет по комнате (устройств: {}):", self.devices.len());
        for device in self.devices.values() {
            device.print_report();
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

    #[test]
    fn test_add_device() {
        let id = Uuid::new_v4();
        let mut room = Room::new(vec![]);
        let device = SmartDevice::Thermometer(Thermometer::with_id(id, 22.0));
        room.add_device(device);
        assert!(room.get_device(id).is_some());
        assert_eq!(room.devices.len(), 1);
    }

    #[test]
    fn test_remove_device() {
        let id = Uuid::new_v4();
        let device = SmartDevice::Thermometer(Thermometer::with_id(id, 22.0));
        let mut room = Room::new(vec![device]);
        assert!(room.remove_device(id).is_some());
        assert!(room.remove_device(id).is_none());
        assert!(room.devices.is_empty());
    }
}
