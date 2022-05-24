use eaip::eaip::ais::GB;
use eaip::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let eaip = &*GB;
    let airways = Airways::from_current_eaip(eaip).await.unwrap();
    println!("{:#?}", airways);
}
