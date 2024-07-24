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
