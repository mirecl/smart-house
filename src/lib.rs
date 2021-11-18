#![allow(unused_variables)]
#![allow(dead_code)]

// Заглушка для типа возврата результата, где возможна ошибка
struct SomeResult;

// Список доступных устройств в умном доме
enum Device {
    Thermometer(Thermometer), // Термометр со структурой методов
    Socket(Socket),           // Розетка со структурой методов
}

// Реализация общих методов для всех устройств
// Условный контракт - обязательный для всех структур устройств
impl Device {
    // Имя устройства
    fn name(&self) -> String {
        todo!()
    }
}

// Структура для умного дома
struct House {
    name: String,        // Имя дома
    description: String, // Описание дома
    rooms: Vec<Room>,    // Список помещений в доме
}

// Реализация методов умного дома
impl House {
    // Создание дома
    fn new(name: String, description: String) -> Self {
        todo!()
    }
    // Добавить помещение
    // Имя помещения уникальное в доме и возможна ошибка
    fn add_room(&self, room: Room) -> SomeResult {
        todo!()
    }
    // Получить список всех помещений в доме
    // Помещения в доме может не быть и может вернуться ошибка
    fn rooms(&self) -> SomeResult {
        todo!()
    }
    // Получить помещение по имени
    // Помещения может не быть в доме и может вернуться ошибка
    fn room_by_name(&self, name: String) -> SomeResult {
        todo!()
    }
    // Получить помещение по имени для дальнейшего изменения
    // Помещения может не быть в доме и может вернуться ошибка
    fn room_by_name_mut(&mut self, name: String) -> SomeResult {
        todo!()
    }
    // Удалить помещение по имени
    // Помещения может не быть в доме и может вернуться ошибка
    fn remove_room_by_name(&self, name: String) -> SomeResult {
        todo!()
    }
    // Получить список всех устройств в доме
    // Устройства/помещения может не быть в доме и может вернуться ошибка
    fn devices(&self) -> SomeResult {
        todo!()
    }
    // Получить отчет по всем устройствам в доме
    // Устройства может не быть и может вернуться ошибка
    fn report_devices(&self) -> SomeResult {
        todo!()
    }
}

// Структура для помещения
struct Room {
    name: String,        // Имя помещения
    description: String, // Описание помещения
    // Единый список со всеми устройствами в помещения
    // Для расширения доступных устройств необходимо расширить enum Device
    devices: Vec<Device>,
}

// Реализация методов помещения
impl Room {
    // Создание помещения
    fn new(name: String, description: String) -> Self {
        todo!()
    }
    // Добавить устройство в помещение
    // Имя устройства уникальное в помещении и может вернуться ошибка
    fn add_device(&self, device: Device) -> SomeResult {
        todo!()
    }
    // Получить устройство по имени
    // Устройства может не быть в помещении и может вернуться ошибка
    fn device_by_name(&self, name: String) -> SomeResult {
        todo!()
    }
    // Удалить устройство из помещения
    // Устройства может не быть в помещении и может вернуться ошибка
    fn remove_device_by_name(&self, name: String) -> SomeResult {
        todo!()
    }
    // Получить все устройства в помещении
    // Устройства может не быть в помещении и может вернуться ошибка
    fn devices(&self) -> SomeResult {
        todo!()
    }
}

// Струкутра устройства - розетки
struct Socket {
    name: String,        // Имя розетки
    description: String, // Описание розетки
    status: bool,        // Статус розетки (вкл/выкл)
    watts: usize,        // Полезная инфоромация
}

// Реализация методов для розетки
impl Socket {
    // Создание розетки
    fn new(name: String, description: String) -> Self {
        todo!()
    }
    // Выключить розетку
    // Розетка может не работать или быть уже выкл и может вернуться ошибка
    fn off(&self) -> SomeResult {
        todo!()
    }
    // Включить розетку
    // Розетка может не работать или быть уже вкл, то возможна ошибка
    fn on(&self) -> SomeResult {
        todo!()
    }
    // Получить полезную информацию от розетки
    fn watts(&self) -> usize {
        todo!()
    }
}

// Струкутра устройства - термометр
struct Thermometer {
    name: String,        // Имя термометра
    description: String, // Описание термометра
    temperature: usize,  // Полезная информация
}

// Реализация методов для термометра
impl Thermometer {
    // Создание термометра
    fn new(name: String, description: String) -> Self {
        todo!()
    }
    // Получить полезную информацию от термометра
    fn temperature(&self) -> usize {
        todo!()
    }
}
