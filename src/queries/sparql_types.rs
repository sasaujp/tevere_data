use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SparqlResponse {
    head: Head,
    results: Results,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Head {
    vars: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    bindings: Vec<HashMap<String, Object>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    r#type: String,
    datatype: Option<String>,
    value: String,
}
