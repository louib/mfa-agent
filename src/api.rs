use std::str;

/// The current version of the API.
pub const CURRENT_VERSION: u8 = 0x47;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum OperationType {
    Ping,
    Search,
    HMAC,
    U2F,
    /// List the secrets in the remote agent. This will
    /// return a list of *sanitized* secrets.
    List,
    // SSH connect
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
    fn default() -> Self {
        Request {
            version: CURRENT_VERSION,
            op: OperationType::Ping,
            payload: vec![],
        }
    }
}
impl Request {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];
        response.push(self.version as u8);
        response.push(self.op.to_byte());
        // TODO validate that the last index is not included!
        for payload_index in 0..self.payload.len() {
            response.push(self.payload[payload_index]);
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
        response.payload = vec![];
        for i in 2..bytes.len() {
            response.payload.push(bytes[i]);
        }
        Ok(response)
    }
}

pub struct Response {
    pub version: u8,
    pub code: StatusCode,
    pub payload: Vec<u8>,
}
impl Default for Response {
    fn default() -> Self {
        Response {
            version: CURRENT_VERSION,
            code: StatusCode::Ok,
            payload: vec![],
        }
    }
}
impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response: Vec<u8> = vec![];
        response.push(self.version);
        response.push(self.code.to_byte());
        for i in 0..self.payload.len() {
            response.push(self.payload[i]);
        }
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
        response.payload = vec![];
        for i in 2..bytes.len() {
            response.payload.push(bytes[i]);
        }
        Ok(response)
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
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
    id: String,
    name: String,
}

pub async fn handle_ping_request(request: crate::api::Request) -> Result<crate::api::PingResponse, String> {
    log::info!("Handling ping request");
    Ok(())
}

pub async fn handle_search_request(request: crate::api::Request) -> Result<crate::api::SearchResponse, String> {
    log::info!("Handling search request");
    let search_term: &str = str::from_utf8(&request.payload).map_err(|e| e.to_string())?;
    let search_term: crate::api::SearchRequest =
        serde_json::from_str(search_term).map_err(|e| e.to_string())?;

    log::info!("Received search request for {}.", search_term);

    Ok(vec!["allo".to_string(), "toi".to_string()])
}

pub async fn handle_request(request: crate::api::Request) -> Result<crate::api::Response, String> {
    let mut response = crate::api::Response::default();

    match request.op {
        crate::api::OperationType::Ping => {
            let op_response = handle_ping_request(request).await?;
            response.payload = serde_json::to_string(&op_response)
                .map_err(|e| e.to_string())?
                .as_bytes()
                .to_vec();
            // response.code = ???
        }
        crate::api::OperationType::Search => {
            let op_response = handle_search_request(request).await?;
            response.payload = serde_json::to_string(&op_response)
                .map_err(|e| e.to_string())?
                .as_bytes()
                .to_vec();
            // response.code = ???
        }
        _ => return Err(format!("Operation {:?} not implemented yet.", request.op)),
    }

    Ok(response)
}

mod api_tests {

    #[test]
    pub fn test_request_serialization_without_payload() {
        let request = crate::api::Request::default();
        let bytes = request.to_bytes();
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    pub fn test_request_serialization_with_payload() {
        let mut request = crate::api::Request::default();
        let bytes = request.to_bytes();
        assert_eq!(bytes.len(), 2);
        request.payload = vec![0x12, 0x13, 0x14, 0x15];
        let bytes = request.to_bytes();
        assert_eq!(bytes.len(), 6);
    }

    #[test]
    pub fn test_request_deserialization_without_payload() {
        let operation = crate::api::OperationType::HMAC;

        let mut request = crate::api::Request::default();
        request.op = operation.clone();

        let bytes = request.to_bytes();
        assert_eq!(bytes.len(), 2);
        let request = crate::api::Request::from_bytes(&bytes).unwrap();
        assert_eq!(request.op, operation);
    }

    #[test]
    pub fn test_request_deserialization_with_payload() {
        let operation = crate::api::OperationType::HMAC;
        let payload = vec![0x66, 0x77, 0x88, 0x99];

        let mut request = crate::api::Request::default();
        request.op = operation.clone();
        request.payload = payload.clone();

        let bytes = request.to_bytes();
        assert_eq!(bytes.len(), 6);
        let request = crate::api::Request::from_bytes(&bytes).unwrap();
        assert_eq!(request.op, operation);
        assert_eq!(request.payload, payload);
    }

    #[test]
    pub fn test_response_serialization_without_payload() {
        let response = crate::api::Response::default();
        let bytes = response.to_bytes();
        assert_eq!(bytes.len(), 2);
    }

    #[test]
    pub fn test_response_serialization_with_payload() {
        let mut response = crate::api::Response::default();
        let bytes = response.to_bytes();
        assert_eq!(bytes.len(), 2);
        response.payload = vec![0x12, 0x13, 0x14, 0x15];
        let bytes = response.to_bytes();
        assert_eq!(bytes.len(), 6);
    }

    #[test]
    pub fn test_response_deserialization_without_payload() {
        let code = crate::api::StatusCode::Ok;

        let mut response = crate::api::Response::default();
        response.code = code.clone();

        let bytes = response.to_bytes();
        assert_eq!(bytes.len(), 2);
        let response = crate::api::Response::from_bytes(&bytes).unwrap();
        assert_eq!(response.code, code);
    }

    #[test]
    pub fn test_response_deserialization_with_payload() {
        let code = crate::api::StatusCode::Ok;
        let payload = vec![0x66, 0x77, 0x88, 0x99];

        let mut response = crate::api::Response::default();
        response.code = code.clone();
        response.payload = payload.clone();

        let bytes = response.to_bytes();
        assert_eq!(bytes.len(), 6);
        let response = crate::api::Response::from_bytes(&bytes).unwrap();
        assert_eq!(response.code, code);
        assert_eq!(response.payload, payload);
    }

    pub fn test_request_deserialization_invalid_version() {
        // TODO
    }
}
