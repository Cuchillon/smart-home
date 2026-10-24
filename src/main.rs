use smart_home::{
    Outlet, Room, SmartDevice, SmartHome, Thermometer, get_random_power, get_random_temperature,
};

fn main() {
    let thermometer_1 = Thermometer::new(get_random_temperature());
    let thermometer_2 = Thermometer::new(get_random_temperature());

    let outlet_1 = Outlet::new(true, get_random_power());
    let outlet_2 = Outlet::new(true, get_random_power());

    let room_devices_1 = vec![
        SmartDevice::Thermometer(thermometer_1),
        SmartDevice::Outlet(outlet_1),
    ];
    let room_1 = Room::new(room_devices_1);

    let room_devices_2 = vec![
        SmartDevice::Thermometer(thermometer_2),
        SmartDevice::Outlet(outlet_2),
    ];
    let room_2 = Room::new(room_devices_2);

    let mut home = SmartHome::new(vec![room_1, room_2]);

    home.print_report();

    let room = home.get_room_mut(0);
    let device = room.get_device_mut(1);

    if let SmartDevice::Outlet(outlet) = device {
        println!("-> Выключаем розетку #{}...", outlet.get_id());
        outlet.turn_off();
    }

    home.print_report();
}
