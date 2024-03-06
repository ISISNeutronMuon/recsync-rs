use reccaster::Reccaster;

#[tokio::main]
async fn main() {
    let mut caster = Reccaster::new().await;
    caster.run().await;
}
