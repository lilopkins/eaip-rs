use eaip::eaip::ais::GB;
use eaip::parse::*;
use eaip::parts::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Get
    let eaip = *GB;
    let egbo = Part::Aerodromes(AD::TableOfContents);
    let data = eaip.get_current_page(egbo, Type::HTML).await.unwrap();

    // Parse
    let toc = TableOfContents::parse(&data);

    println!("{:#?}", toc);
}
