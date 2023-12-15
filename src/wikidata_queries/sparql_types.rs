use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SparqlResponse {
    pub head: Head,
    pub results: Results,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Head {
    pub vars: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    pub bindings: Vec<HashMap<String, Object>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub r#type: String,
    pub datatype: Option<String>,
    pub value: String,
}
