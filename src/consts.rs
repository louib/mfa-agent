pub const APP_ID: &str = "net.louib.mfa-agent";
pub const APP_NAME: &str = "mfa-agent";
pub const APP_TITLE: &str = "MFA Agent";

/// Bluetooth Service UUID.
// FIXME change this UUID, it was taken from the examples.
pub const APP_BT_SERVICE_ID: uuid::Uuid = uuid::Uuid::from_u128(0xFEEDC0DE00002);

/// Characteristic UUID for TOTP requests
pub const APP_BT_TOTP_CHARACTERISTIC_ID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);

/// Characteristic UUID for HMAC (Yubikey) requests
pub const APP_BT_HMAC_CHARACTERISTIC_ID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
