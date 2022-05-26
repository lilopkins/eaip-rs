use eaip::eaip::ais::GB;
use eaip::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let eaip = &*GB;
    let intersections = Intersections::from_current_eaip(eaip)
        .await
        .with_context(|| "Failed to get intersections")?;
    println!("{:#?}", intersections);

    Ok(())
}
