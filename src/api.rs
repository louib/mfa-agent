use std::str;

/// The current version of the API.
pub const CURRENT_VERSION: u8 = 0x47;

#[derive(Debug)]
pub enum OperationType {
    Ping,
    Search,
    HMAC,
    U2F,
    /// List the secrets in the remote agent. This will
    /// return a list of *sanitized* secrets.
    List,
    // PGP Encrypt
    // Seed (send an initial secrets payload to a device)
    // Crypto sign a transaction
}
impl OperationType {
    pub fn to_byte(&self) -> u8 {
        match &self {
            OperationType::Ping => 113,
            OperationType::Search => 1,
            OperationType::HMAC => 2,
            OperationType::U2F => 3,
            OperationType::List => 4,
        }
    }

    pub fn from_byte(operation_type: u8) -> Result<OperationType, String> {
        if operation_type == 113 {
            return Ok(OperationType::Ping);
        }
        if operation_type == 1 {
            return Ok(OperationType::Search);
        }
        if operation_type == 2 {
            return Ok(OperationType::HMAC);
        }
        if operation_type == 3 {
            return Ok(OperationType::U2F);
        }
        if operation_type == 4 {
            return Ok(OperationType::List);
        }
        Err(format!("Invalid operation type {}.", operation_type))
    }
}

pub struct Request {
    pub version: u8,
    pub op: OperationType,
    pub payload: Vec<u8>,
}
impl Default for Request {
    fn default() -> Self { Request { version: CURRENT_VERSION, op: OperationType::Ping, payload: vec![] } }
}
impl Request {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];
        response[0] = self.version as u8;
        response[1] = self.op.to_byte();
        // TODO validate that the last index is not included!
        for payload_index in 0..self.payload.len() {
            response[1 + payload_index] = self.payload[payload_index];
        }
        response
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Request, String> {
        let mut response = Self::default();
        response.version = bytes[0];
        if response.version != CURRENT_VERSION {
            return Err(format!("Invalid API version {}", response.version));
        }
        response.op = OperationType::from_byte(bytes[1])?;
        // TODO extract and validate the response.
        Ok(response)
    }
}

pub struct Response {
    pub version: u8,
    pub code: StatusCode,
    pub payload: Vec<u8>,
}
impl Default for Response {
    fn default() -> Self { Response { version: CURRENT_VERSION, code: StatusCode::Ok, payload: vec![] } }
}
impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];
        response[0] = self.version;
        response[1] = self.code.to_byte();
        // TODO add the response!
        response
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Response, String> {
        let mut response = Self::default();
        response.version = bytes[0];
        if response.version != CURRENT_VERSION {
            return Err(format!("Invalid version {}", response.version));
        }
        response.code = match StatusCode::from_byte(bytes[1]) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        Ok(response)
    }
}


pub enum StatusCode {
    Ok,
    Err,
    Locked,
}
impl StatusCode {
    pub fn to_byte(&self) -> u8 {
        match &self {
            StatusCode::Ok => 112,
            StatusCode::Err => 1,
            StatusCode::Locked => 2,
        }
    }

    pub fn from_byte(status_code: u8) -> Result<StatusCode, String> {
        if status_code == 112 {
            return Ok(StatusCode::Ok);
        }
        if status_code == 1 {
            return Ok(StatusCode::Err);
        }
        if status_code == 2 {
            return Ok(StatusCode::Locked);
        }
        Err(format!("Invalid status code {}.", status_code))
    }
}

pub type PingRequest = ();
pub type PingResponse = ();
/// The term to search for.
pub type SearchRequest = String;
/// A list of IDs of the secrets that matched the search term.
pub type SearchResponse = Vec<String>;
pub type ListRequest = ();
pub type ListResponse = Vec<SanitizedSecret>;
pub type HMACRequest = String;
pub type HMACResponse = String;

pub struct SanitizedSecret {
    secretId: String,
    secretName: String,
}

pub async fn handle_request(request: Request) -> Result<Response, String> {
    match request.op {
        Ping => {
            let ping_request: PingRequest = match serde_yaml::from_str(str::from_utf8(&request.payload).unwrap()) {
                Ok(r) => r,
                Err(e) => return Err(e.to_string()),
            };
            let response = crate::api::Response::default();
            return Ok(response);

            // let ping_response: PingResponse = ConnectionType::execute()

        },
        _ => panic!("Operation {:?} not implemented yet.", request.op),
    }
}
