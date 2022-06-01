#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum ApplicationEvent {
    PasswordEntered(String),
    AddKnownDevice(String),
    PairWithDevice(String),
    UnpairWithDevice(String),
    AllowSecretAccess(String),
}
