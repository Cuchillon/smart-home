use crate::Room;

pub struct SmartHome {
    rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(rooms: Vec<Room>) -> Self {
        Self { rooms }
    }

    pub fn get_room(&self, index: usize) -> &Room {
        &self.rooms[index]
    }

    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        &mut self.rooms[index]
    }

    pub fn print_report(&self) {
        println!("=== ОТЧЕТ ПО УМНОМУ ДОМУ ===");
        for (i, room) in self.rooms.iter().enumerate() {
            println!("Комната #{}", i);
            room.print_report();
            println!("-------------------------");
        }
        println!("=============================\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SmartDevice;
    use crate::outlet::Outlet;
    use crate::thermometer::Thermometer;

    #[test]
    fn test_rooms() {
        let home = SmartHome::new(vec![
            Room::new(vec![
                SmartDevice::Thermometer(Thermometer::new(22.0)),
                SmartDevice::Outlet(Outlet::new(true, 150.0)),
            ]),
            Room::new(vec![SmartDevice::Thermometer(Thermometer::new(19.5))]),
        ]);
        assert_eq!(home.rooms.len(), 2);
    }

    #[test]
    fn test_get_room() {
        let home = SmartHome::new(vec![
            Room::new(vec![SmartDevice::Thermometer(Thermometer::new(22.0))]),
            Room::new(vec![SmartDevice::Outlet(Outlet::new(true, 150.0))]),
        ]);

        let room = home.get_room(1);
        if let SmartDevice::Outlet(outlet) = room.get_device(0) {
            assert!(outlet.is_on());
            assert_eq!(outlet.get_power(), 150.0);
        } else {
            panic!("Must be outlet");
        }
    }

    #[test]
    #[should_panic]
    fn test_get_room_out_of_bounds() {
        let home = SmartHome::new(vec![Room::new(vec![SmartDevice::Thermometer(
            Thermometer::new(22.0),
        )])]);
        let _ = home.get_room(1);
    }
}
