use eaip::eaip::ais::GB;
use eaip::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let eaip = &*GB;
    let airport = Airport::from_current_eaip(eaip, "EGBO".to_string())
        .await
        .unwrap();
    println!("{:#?}", airport);
}
