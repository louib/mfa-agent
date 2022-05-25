pub const APP_ID: &str = "net.louib.mfa-agent";
pub const DEV_APP_ID: &str = "net.louib.mfa-agent-dev";
pub const APP_NAME: &str = "mfa-agent";
pub const APP_TITLE: &str = "MFA Agent";
pub const PROXY_TITLE_SUFFIX: &str = "(proxy)";
pub const AGENT_TITLE_SUFFIX: &str = "(remote)";

pub const IS_PROXY_VAR_NAME: &str = "MFA_AGENT_IS_PROXY";
pub const IS_DEV_VAR_NAME: &str = "MFA_AGENT_IS_DEV";
pub const CONNECTION_TYPE_VAR_NAME: &str = "MFA_AGENT_CONNECTION_TYPE";

/// Bluetooth Service UUID.
// FIXME change this UUID, it was taken from the examples.
pub const APP_BT_SERVICE_ID: uuid::Uuid = uuid::Uuid::from_u128(0xFEEDC0DE00002);

/// Characteristic UUID for Bluetooth requests
// FIXME change this UUID, it was taken from the examples.
pub const APP_BT_CHARACTERISTIC_ID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
