use crate::Room;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct SmartHome {
    rooms: HashMap<Uuid, Room>,
}

impl SmartHome {
    pub fn new(rooms: impl IntoIterator<Item = Room>) -> Self {
        Self {
            rooms: rooms.into_iter().map(|r| (r.get_id(), r)).collect(),
        }
    }

    pub fn get_room(&self, id: Uuid) -> Option<&Room> {
        self.rooms.get(&id)
    }

    pub fn get_room_mut(&mut self, id: Uuid) -> Option<&mut Room> {
        self.rooms.get_mut(&id)
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.get_id(), room);
    }

    pub fn remove_room(&mut self, id: Uuid) -> Option<Room> {
        self.rooms.remove(&id)
    }

    pub fn print_report(&self) {
        println!("=== ОТЧЕТ ПО УМНОМУ ДОМУ ===");
        for (id, room) in self.rooms.iter() {
            println!("Комната {}", id);
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
        let device_id = Uuid::new_v4();
        let room_id = Uuid::new_v4();
        let home = SmartHome::new(vec![
            Room::new(vec![SmartDevice::Thermometer(Thermometer::new(22.0))]),
            Room::with_id(
                room_id,
                vec![SmartDevice::Outlet(Outlet::with_id(device_id, true, 150.0))],
            ),
        ]);

        let room = home.get_room(room_id).unwrap_or_else(|| {
            panic!("Room with id {} not found", room_id);
        });
        if let SmartDevice::Outlet(outlet) = room.get_device(device_id).unwrap_or_else(|| {
            panic!("Device with id {} not found", device_id);
        }) {
            assert!(outlet.is_on());
            assert_eq!(outlet.get_power(), 150.0);
        } else {
            panic!("Must be outlet");
        }
    }

    #[test]
    fn test_turn_off_outlet_via_mutable_access() {
        let device_id = Uuid::new_v4();
        let room_id = Uuid::new_v4();
        let mut home = SmartHome::new(vec![Room::with_id(
            room_id,
            vec![SmartDevice::Outlet(Outlet::with_id(device_id, true, 15.0))],
        )]);

        let room = home.get_room_mut(room_id).unwrap_or_else(|| {
            panic!("Room with id {} not found", room_id);
        });
        if let SmartDevice::Outlet(outlet) = room.get_device_mut(device_id).unwrap_or_else(|| {
            panic!("Device with id {} not found", device_id);
        }) {
            assert_eq!(outlet.get_power(), 15.0);
            outlet.turn_off();
            assert!(!outlet.is_on());
            assert_eq!(outlet.get_power(), 0.0);
        } else {
            panic!("Must be outlet");
        }
    }

    #[test]
    fn test_get_room_not_found() {
        let id = Uuid::new_v4();
        let home = SmartHome::new(vec![Room::new(vec![SmartDevice::Thermometer(
            Thermometer::new(22.0),
        )])]);
        let not_found_room = home.get_room(id);
        assert!(not_found_room.is_none());
    }

    #[test]
    fn test_add_room() {
        let id = Uuid::new_v4();
        let mut home = SmartHome::new(vec![]);
        let room = Room::with_id(id, vec![]);
        home.add_room(room);
        assert!(home.get_room(id).is_some());
        assert_eq!(home.rooms.len(), 1);
    }

    #[test]
    fn test_remove_room() {
        let id = Uuid::new_v4();
        let room = Room::with_id(id, vec![]);
        let mut home = SmartHome::new(vec![room]);
        assert!(home.remove_room(id).is_some());
        assert!(home.remove_room(id).is_none());
        assert!(home.rooms.is_empty());
    }
}
