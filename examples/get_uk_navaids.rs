use eaip::eaip::ais::GB;
use eaip::parse::navaids::Navaids;
use eaip::parse::*;
use eaip::parts::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Get
    let eaip = *GB;
    let page = Part::EnRoute(ENR::RadioNavAids(1));
    let data = eaip.get_current_page(page, Type::HTML).await.unwrap();

    // Parse
    let navaids = Navaids::parse(&data).unwrap();
    println!("{:#?}", navaids);
}
