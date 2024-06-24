use reccaster::{record::Record, Reccaster};

#[tokio::main]
async fn main() {

    let mut record = Record::new("DEV:PYRECASTER".to_string(), "ai".to_string());
    record.properties.insert("recordDesc".to_string(), "P4P4ISIS PyRecaster".to_string());
    let records: Vec<Record> = vec![record];

    let mut caster = Reccaster::new(records).await;
    caster.run().await;
}
