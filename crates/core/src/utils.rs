//! Input coercion helpers.
//!
//! NIP-90 `param` tags carry strings; programs expect typed JSON. These
//! helpers bridge the two with best-effort, total (non-panicking) conversions.

use std::collections::HashMap;

use serde_json::Value;

/// Converts string inputs into typed JSON values.
///
/// Each value is coerced with [`coerce_value`]; nothing is dropped.
pub fn convert_inputs_to_run_program(
    inputs: HashMap<String, String>,
) -> HashMap<String, serde_json::Value> {
    inputs
        .into_iter()
        .map(|(key, value)| (key, coerce_value(&value)))
        .collect()
}

/// Coerces a single string input to JSON: `u32` first, then `u64`, then any
/// JSON value (objects, arrays, quoted strings), and finally the raw string.
fn coerce_value(value: &str) -> Value {
    if let Ok(n) = value.parse::<u32>() {
        return n.into();
    }
    if let Ok(n) = value.parse::<u64>() {
        return n.into();
    }
    if let Ok(json) = value.parse::<Value>() {
        return json;
    }
    Value::String(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coerces_unsigned_integers() {
        assert_eq!(coerce_value("5"), Value::from(5u32));
        // Larger than u32::MAX falls through to u64.
        assert_eq!(coerce_value("4294967296"), Value::from(4_294_967_296u64));
    }

    #[test]
    fn coerces_json_values() {
        assert_eq!(
            coerce_value("[1, 2, 3]"),
            Value::Array(vec![1.into(), 2.into(), 3.into()])
        );
        assert_eq!(coerce_value("true"), Value::Bool(true));
    }

    #[test]
    fn falls_back_to_plain_string() {
        assert_eq!(
            coerce_value("text/json"),
            Value::String("text/json".to_owned())
        );
    }

    #[test]
    fn converts_full_input_maps() {
        let inputs = HashMap::from([
            ("log_size".to_owned(), "5".to_owned()),
            ("output".to_owned(), "text/json".to_owned()),
        ]);
        let converted = convert_inputs_to_run_program(inputs);
        assert_eq!(converted.get("log_size"), Some(&Value::from(5u32)));
        assert_eq!(
            converted.get("output"),
            Some(&Value::String("text/json".to_owned()))
        );
    }
}
