#[derive(Debug)]
pub enum OperationType {
    Ping,
    Search,
    HMAC,
}
impl OperationType {
    pub fn to_byte(&self) -> u8 {
        match &self {
            OperationType::Ping => 0,
            OperationType::Search => 1,
            OperationType::HMAC => 2,
        }
    }

    pub fn from_byte(operation_type: u8) -> Result<OperationType, String> {
        if operation_type == 0 {
            return Ok(OperationType::Ping);
        }
        if operation_type == 1 {
            return Ok(OperationType::Search);
        }
        if operation_type == 2 {
            return Ok(OperationType::HMAC);
        }
        Err(format!("Invalid operation type {}.", operation_type))
    }
}

pub enum ResponseType {
    Ping,
    Search,
    HMAC,
}

pub struct SearchRequest {
    term: String,
}
pub struct SearchResponse {
    term: String,
}
pub struct HMACRequest {
    secretId: String,
}
pub struct HMACResponse {
    secretId: String,
}
