use smart_home::{
    GetDeviceError, Outlet, Reporter, Room, SmartDevice, SmartHome, Thermometer, build_room,
    get_random_power, get_random_temperature,
};
use uuid::Uuid;

fn print_report(reporter: &impl Reporter) {
    reporter.print_report();
}

fn main() {
    let (outlet_1_id, outlet_2_id, outlet_3_id) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
    let room_1_id = Uuid::new_v4();

    let outlet_1 = Outlet::with_id(outlet_1_id, true, get_random_power());
    let outlet_2 = Outlet::with_id(outlet_2_id, true, get_random_power());
    let outlet_3 = Outlet::with_id(outlet_3_id, false, get_random_power());

    let room_1 = Room::with_id(
        room_1_id,
        vec![
            SmartDevice::Thermometer(Thermometer::new(get_random_temperature())),
            SmartDevice::Outlet(outlet_1),
        ],
    );

    // Комната через макрос build_room
    let room_2 = build_room!(
        Thermometer::new(get_random_temperature()).into(),
        SmartDevice::Outlet(outlet_2),
    );
    let room_2_id = room_2.get_id();

    let mut home = SmartHome::new(vec![room_1, room_2]);

    // Отчеты о доме, отдельной комнате и отдельном устройстве
    println!("### Отчеты через обобщенную функцию\n");
    println!("-- Отчет по дому:");
    print_report(&home);
    println!("-- Отчет по комнате {room_1_id}:");
    if let Some(room) = home.get_room(room_1_id) {
        print_report(room);
    }
    println!("-- Отчет по устройству {outlet_1_id}:");
    match home.get_room_device(room_1_id, outlet_1_id) {
        Ok(device) => print_report(device),
        Err(error) => println!("Не удалось получить устройство: {error}"),
    }

    // Динамическое добавление/удаление комнат
    println!("\n### Динамическое добавление/удаление комнат\n");
    let room_3 = build_room!(Thermometer::new(get_random_temperature()).into());
    let room_3_id = room_3.get_id();
    println!("-> Добавляем комнату {room_3_id}");
    home.add_room(room_3);
    print_report(&home);

    println!("-> Удаляем комнату {room_2_id}");
    if let Some(removed_room) = home.remove_room(room_2_id) {
        println!("Комната {} удалена", removed_room.get_id());
    }
    print_report(&home);

    // Динамическое добавление/удаление устройств
    println!("\n### Динамическое добавление/удаление устройств\n");
    println!("-> Добавляем розетку {outlet_3_id} в комнату {room_1_id}");
    if let Some(room) = home.get_room_mut(room_1_id) {
        room.add_device(SmartDevice::Outlet(outlet_3));
    }
    if let Some(room) = home.get_room(room_1_id) {
        print_report(room);
    }

    println!("-> Удаляем розетку {outlet_3_id} из комнаты {room_1_id}");
    if let Some(room) = home.get_room_mut(room_1_id)
        && let Some(device) = room.remove_device(outlet_3_id)
    {
        println!("Устройство {} удалено", device.get_id());
    }
    if let Some(room) = home.get_room(room_1_id) {
        print_report(room);
    }

    // Обработка ошибок
    println!("\n### Обработка ошибок\n");

    println!("-> Ищем розетку {outlet_1_id} в комнате {room_1_id}:");
    match home.get_room_device(room_1_id, outlet_1_id) {
        Ok(SmartDevice::Outlet(outlet)) => {
            println!(
                "   Розетка найдена, сейчас: {}",
                if outlet.is_on() { "ВКЛ" } else { "ВЫКЛ" }
            );
        }
        Ok(_) => println!("   Устройство не является розеткой"),
        Err(error) => println!("   Ошибка: {error}"),
    }

    println!("-> Ищем устройство в несуществующей комнате:");
    let unknown_id = Uuid::new_v4();
    match home.get_room_device(unknown_id, unknown_id) {
        Ok(_) => println!("   Устройство неожиданно найдено"),
        Err(GetDeviceError::RoomNotFound(room_id)) => {
            println!("   Комната {room_id} не найдена");
        }
        Err(error) => println!("   {error}"),
    }

    println!("-> Выключаем розетку {outlet_1_id} через изменяемый доступ:");
    if let Some(room) = home.get_room_mut(room_1_id)
        && let Some(SmartDevice::Outlet(outlet)) = room.get_device_mut(outlet_1_id)
    {
        outlet.turn_off();
        println!("   Розетка выключена");
    }

    println!("-> Ищем несуществующее устройство в комнате {room_1_id}:");
    match home.get_room_device(room_1_id, unknown_id) {
        Ok(_) => println!("   Устройство неожиданно найдено"),
        Err(error) => println!("   {error}"),
    }

    println!("\n### Итоговый отчет\n");
    print_report(&home);
}
