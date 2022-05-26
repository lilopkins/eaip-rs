use eaip::eaip::ais::GB;
use eaip::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    // Get
    let eaip = &*GB;
    let list = Airports::from_current_eaip(eaip)
        .await
        .with_context(|| "Failed to get list of airports.")?;

    println!("{:#?}", list);

    Ok(())
}
