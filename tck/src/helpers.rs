use std::collections::HashMap;
use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JsonObject {
    String(String),
    I64(i64),
    Bool(bool),
    Vec(Vec<JsonObject>),
    Map(HashMap<String, JsonObject>),
}

impl FromStr for JsonObject {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as i64
        if let Ok(i) = s.parse::<i64>() {
            return Ok(JsonObject::I64(i));
        }

        // Try parsing as bool
        if let Ok(b) = s.parse::<bool>() {
            return Ok(JsonObject::Bool(b));
        }

        // Try parsing as JSON array
        if s.starts_with('[') && s.ends_with(']') {
            if let Ok(array) = serde_json::from_str::<Vec<JsonObject>>(s) {
                return Ok(JsonObject::Vec(array));
            }
        }

        // Try parsing as JSON map
        if s.starts_with('{') && s.ends_with('}') {
            if let Ok(map) = serde_json::from_str::<HashMap<String, JsonObject>>(s) {
                return Ok(JsonObject::Map(map));
            }
        }

        // Default to String
        Ok(JsonObject::String(s.to_string()))
    }
}

const JSON_RPC_VERSION: &str = "2.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRequest {
    pub jsonrpc: String,
    pub id: i64,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<JsonObject>,
}

impl JsonRequest {
    pub fn new(id: i64, method: String, params: JsonObject) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id,
            method,
            params: Some(params),
        }
    }

    pub fn to_dict(&self) -> HashMap<String, JsonObject> {
        let mut dict = HashMap::new();
        dict.insert("jsonrpc".to_string(), JsonObject::String(self.jsonrpc.clone()));
        dict.insert("id".to_string(), JsonObject::I64(self.id));
        dict.insert("method".to_string(), JsonObject::String(self.method.clone()));
        
        if let Some(params) = &self.params {
            dict.insert("params".to_string(), params.clone());
        }

        dict
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<JsonObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonError>,
}

impl JsonResponse {
    pub fn success(id: i64, result: JsonObject) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id: Some(id),
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Option<i64>, error: JsonError) -> Self {
        Self {
            jsonrpc: JSON_RPC_VERSION.to_string(),
            id,
            result: None,
            error: Some(error),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub enum JsonError {
    HederaError {
        message: String,
        data: Option<JsonObject>
    },
    InvalidRequest {
        message: String,
        data: Option<JsonObject>
    },
    MethodNotFound {
        message: String,
        data: Option<JsonObject>
    },
    InvalidParams {
        message: String,
        data: Option<JsonObject>
    },
    InternalError {
        message: String,
        data: Option<JsonObject>
    },
    ParseError {
        message: String,
        data: Option<JsonObject>
    },
}

impl JsonError {
    pub fn code(&self) -> i32 {
        match self {
            Self::HederaError { .. } => -32001,
            Self::InvalidRequest { .. } => -32600,
            Self::MethodNotFound { .. } => -32601,
            Self::InvalidParams { .. } => -32602,
            Self::InternalError { .. } => -32603,
            Self::ParseError { .. } => -32700,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::HederaError { message, .. } |
            Self::InvalidRequest { message, .. } |
            Self::MethodNotFound { message, .. } |
            Self::InvalidParams { message, .. } |
            Self::InternalError { message, .. } |
            Self::ParseError { message, .. } => message,
        }
    }

    pub fn data(&self) -> Option<&JsonObject> {
        match self {
            Self::HederaError { data, .. } |
            Self::InvalidRequest { data, .. } |
            Self::MethodNotFound { data, .. } |
            Self::InvalidParams { data, .. } |
            Self::InternalError { data, .. } |
            Self::ParseError { data, .. } => data.as_ref(),
        }
    }
}

impl Serialize for JsonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("JsonError", 3)?;
        state.serialize_field("code", &self.code())?;
        state.serialize_field("message", &self.message())?;
        if let Some(data) = self.data() {
            state.serialize_field("data", data)?;
        }
        state.end()
    }
}