use std::collections::HashMap;

use serde_json::Value;

pub fn try_convert_string_into_vec<T: for<'a> serde::Deserialize<'a>>(
    string: String,
) -> Result<Vec<T>, serde_json::Error> {
    let str_vec: serde_json::Result<Vec<T>> = serde_json::from_str(string.clone().as_str());
    str_vec.map_err(serde_json::Error::from)
}

pub fn convert_inputs_to_run_program(
    inputs: HashMap<String, String>,
) -> HashMap<String, serde_json::Value> {
    let mut successful_parses = HashMap::new();
    for (key, value) in inputs.iter() {
        if let Ok(num) = value.parse::<u32>() {
            successful_parses.insert(key.clone(), num.into());
            println!("The value for '{}' is a valid u32: {}", key, num);
        } else if let Ok(num) = value.parse::<u64>() {
            successful_parses.insert(key.clone(), num.into());
        } else if let Ok(value) = value.parse::<Value>() {
            successful_parses.insert(key.clone(), value);
        } else if let Ok(vec) = try_convert_string_into_vec::<String>(value.to_string()) {
            successful_parses.insert(key.clone(), vec.into());
        } else if let Ok(vec) = try_convert_string_into_vec::<u32>(value.to_string()) {
            successful_parses.insert(key.clone(), vec.into());
        } else if let Ok(vec) = try_convert_string_into_vec::<u64>(value.to_string()) {
            successful_parses.insert(key.clone(), vec.into());
        } else if let Ok(vec) = try_convert_string_into_vec::<Value>(value.to_string()) {
            successful_parses.insert(key.clone(), vec.into());
        } else {
            println!("The value for '{}' is not a valid type.", key);
        }
    }

    successful_parses
}
