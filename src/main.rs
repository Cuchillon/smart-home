use smart_home::{
    Outlet, Reporter, Room, SmartDevice, SmartHome, Thermometer, get_random_power,
    get_random_temperature,
};
use uuid::Uuid;

fn main() {
    let outlet_1_id = Uuid::new_v4();
    let room_1_id = Uuid::new_v4();

    let thermometer_1 = Thermometer::new(get_random_temperature());
    let thermometer_2 = Thermometer::new(get_random_temperature());

    let outlet_1 = Outlet::with_id(outlet_1_id, true, get_random_power());
    let outlet_2 = Outlet::new(true, get_random_power());

    let room_devices_1 = vec![
        SmartDevice::Thermometer(thermometer_1),
        SmartDevice::Outlet(outlet_1),
    ];
    let room_1 = Room::with_id(room_1_id, room_devices_1);

    let room_devices_2 = vec![
        SmartDevice::Thermometer(thermometer_2),
        SmartDevice::Outlet(outlet_2),
    ];
    let room_2 = Room::new(room_devices_2);

    let mut home = SmartHome::new(vec![room_1, room_2]);

    home.print_report();

    let room = home
        .get_room_mut(room_1_id)
        .unwrap_or_else(|| panic!("Room {} not found", room_1_id));
    let device = room
        .get_device_mut(outlet_1_id)
        .unwrap_or_else(|| panic!("Device {} not found", outlet_1_id));

    if let SmartDevice::Outlet(outlet) = device {
        println!("-> Выключаем розетку #{}...", outlet.get_id());
        outlet.turn_off();
    }

    home.print_report();
}
