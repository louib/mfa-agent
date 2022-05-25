use serde::{Deserialize, Deserializer, Serializer};

pub const BLUETOOTH: &str = "bluetooth";
pub const TCP: &str = "tcp";
pub const USB: &str = "usb";

/// This enumerates all the connection types that the
/// proxy can use to connect to the remote agent.
pub enum ConnectionType {
    Bluetooth,
    Tcp,
    Usb,
}

impl ConnectionType {
    pub fn to_string(&self) -> String {
        match &self {
            ConnectionType::Bluetooth => BLUETOOTH.to_string(),
            ConnectionType::Tcp => TCP.to_string(),
            ConnectionType::Usb => USB.to_string(),
        }
    }

    pub fn from_string(connection_type: &str) -> Result<ConnectionType, String> {
        if connection_type == BLUETOOTH {
            return Ok(ConnectionType::Bluetooth);
        }
        if connection_type == TCP {
            return Ok(ConnectionType::Tcp);
        }
        if connection_type == USB {
            return Ok(ConnectionType::Usb);
        }
        Err(format!("Invalid connection type {}.", connection_type))
    }

    pub fn serialize<S>(x: &Option<ConnectionType>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(build_system) = x {
            return s.serialize_str(&build_system.to_string());
        }
        panic!("This should not happen.");
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ConnectionType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        match ConnectionType::from_string(&buf) {
            Ok(b) => Ok(Some(b)),
            Err(e) => Err(e).map_err(serde::de::Error::custom),
        }
    }
}
