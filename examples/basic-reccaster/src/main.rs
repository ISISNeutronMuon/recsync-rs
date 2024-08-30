// This file is part of Recsync-rs.
// Copyright (c) 2024 UK Research and Innovation, Science and Technology Facilities Council
//
// This project is licensed under both the MIT License and the BSD 3-Clause License.
// You must comply with both licenses to use, modify, or distribute this software.
// See the LICENSE file for details.

use reccaster::{record::Record, Reccaster};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt().init();

    let mut record = Record::new("DEV:RECASTER:RUST".to_string(), "ai".to_string());
    record.properties.insert("recordDesc".to_string(), "Rust Recaster".to_string());
    let records: Vec<Record> = vec![record];

    let mut caster = Reccaster::new(records).await;
    caster.run().await;
}
