use std::collections::HashMap;
use serde_json::Value;

pub fn convert_inputs_to_run_program(inputs: HashMap<String, String>) -> HashMap<String, serde_json::Value> {
    let mut successful_parses = HashMap::new();
    for (key, value) in inputs.iter() {
        if let Ok(num) = value.parse::<u32>() {
            successful_parses.insert(key.clone(), num.into());
            println!("The value for '{}' is a valid u32: {}", key, num);
        } 
        else if let Ok(num) = value.parse::<u64>() {
            successful_parses.insert(key.clone(), num.into());
        } 
        else if let Ok(vec) = value.parse::<Value>() {
            successful_parses.insert(key.clone(), vec.into());
        }
        
        else {
            println!("The value for '{}' is not a valid u32.", key);
        }
    }

    successful_parses
}

// pub fn deserialize_inputs<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let value = Value::deserialize(deserializer)?;
//     if let Value::Object(map) = value {
//         let result = map
//             .into_iter()
//             .map(|(k, v)| {
//                 v.as_str()
//                     .map(|s| (k, s.to_string()))
//                     .ok_or_else(|| serde::de::Error::custom("All values must be strings"))
//             })
//             .collect();
//         result
//     } else {
//         Err(serde::de::Error::custom("inputs must be an object"))
//     }
// }

// fn deserialize_inputs<'de, D>(deserializer: D) -> Result<HashMap<String, String>,
// D::Error> where
//     D: Deserializer<'de>,
// {
//     let val: Value = Deserialize::deserialize(deserializer)?;
//     match val {
//         Value::Object(map) => map
//             .into_iter()
//             .map(|(k, v)| match v.as_str() {
//                 Some(str_val) => Ok((k, str_val.to_string())),
//                 None => Err(serde::de::Error::custom(
//                     "Expected a string value in the map",
//                 )),
//             })
//             .collect(),
//         _ => Err(serde::de::Error::custom("Expected a map for inputs")),
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct GenerateZKPJobRequest<T> {
//     pub request: T,
//     pub program: ProgramParams,
// }

// impl<T> GenerateZKPJobRequest<T> {
//     pub fn new(request: T, program: ProgramParams) -> Self {
//         Self { request, program }
//     }
// }
