use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Record {
    pub name: String,
    pub r#type: String,
    pub alias: Option<String>,
    pub properties: HashMap<String, String>
}

impl Record {
    pub fn new(name: String, r#type: String) -> Record {
        let map: HashMap<String, String> = HashMap::new();
        Record { name, r#type, alias: None, properties: map}
    }
}