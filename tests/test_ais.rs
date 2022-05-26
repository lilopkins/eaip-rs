#[cfg(feature = "test-online")]
use airac::AIRAC;
#[cfg(feature = "test-online")]
use eaip::prelude::*;

macro_rules! test_ais {
    ($id: ident, $ais: expr) => {
        #[cfg(feature = "test-online")]
        #[tokio::test]
        async fn $id() -> Result<()> {
            let ais = $ais;
            let eaip = ais.eaip();

            eprintln!("Test this AIRAC");
            let airac = AIRAC::current();
            eprintln!("Test navaids");
            let _navaids = Navaids::from_eaip(eaip, airac.clone()).await?;
            eprintln!("Test intersections");
            let _intersections = Intersections::from_eaip(eaip, airac.clone()).await?;
            eprintln!("Test airways");
            let _airways = Airways::from_eaip(eaip, airac.clone()).await?;
            eprintln!("Test airport list");
            let airports = Airports::from_eaip(eaip, airac.clone()).await?;
            if let Some(airport) = airports.get(0) {
                eprintln!("Test individual airport");
                let _airport =
                    Airport::from_eaip(eaip, airac.clone(), airport.icao().to_string()).await?;
            }

            eprintln!("Test next AIRAC");
            let airac = airac.next();
            eprintln!("Test navaids");
            let _navaids = Navaids::from_eaip(eaip, airac.clone()).await?;
            eprintln!("Test intersections");
            let _intersections = Intersections::from_eaip(eaip, airac.clone()).await?;
            eprintln!("Test airways");
            let _airways = Airways::from_eaip(eaip, airac.clone()).await?;
            eprintln!("Test airport list");
            let airports = Airports::from_eaip(eaip, airac.clone()).await?;
            if let Some(airport) = airports.get(0) {
                eprintln!("Test individual airport");
                let _airport =
                    Airport::from_eaip(eaip, airac.clone(), airport.icao().to_string()).await?;
            }

            Ok(())
        }
    };
}

test_ais!(test_gb, &*ais::GB);
test_ais!(test_nl, &*ais::NL);
