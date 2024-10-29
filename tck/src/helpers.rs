use std::collections::HashMap;
use std::str::FromStr;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionParamValue {
    String(String),
    I64(i64),
    Bool(bool),
    Array(Vec<String>),
    Map(HashMap<String, String>),
}

impl FromStr for TransactionParamValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as i64
        if let Ok(i) = s.parse::<i64>() {
            return Ok(TransactionParamValue::I64(i));
        }

        // Try parsing as bool
        if let Ok(b) = s.parse::<bool>() {
            return Ok(TransactionParamValue::Bool(b));
        }

        // Try parsing as JSON array
        if s.starts_with('[') && s.ends_with(']') {
            if let Ok(array) = serde_json::from_str::<Vec<String>>(s) {
                return Ok(TransactionParamValue::Array(array));
            }
        }

        // Try parsing as JSON map
        if s.starts_with('{') && s.ends_with('}') {
            if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(s) {
                return Ok(TransactionParamValue::Map(map));
            }
        }

        // Default to String
        Ok(TransactionParamValue::String(s.to_string()))
    }
}
