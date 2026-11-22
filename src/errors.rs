use std::fmt;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum GetDeviceError {
    RoomNotFound(Uuid),
    DeviceNotFound { room_id: Uuid, device_id: Uuid },
}

impl fmt::Display for GetDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RoomNotFound(room_id) => write!(f, "Комната {room_id} не найдена"),
            Self::DeviceNotFound { room_id, device_id } => {
                write!(f, "Устройство {device_id} не найдено в комнате {room_id}")
            }
        }
    }
}

impl std::error::Error for GetDeviceError {}
