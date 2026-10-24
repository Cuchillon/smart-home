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
