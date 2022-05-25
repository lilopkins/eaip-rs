use eaip::eaip::ais::GB;
use eaip::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Get
    let eaip = &*GB;
    let list = Airports::from_current_eaip(eaip).await.unwrap();

    println!("{:#?}", list);
}
