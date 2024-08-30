// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.

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