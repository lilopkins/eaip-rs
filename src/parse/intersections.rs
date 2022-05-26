use table_extract::Table;

use crate::{parse::get_clean_text, prelude::*};
use async_trait::async_trait;

use super::parse_latlong;

/// A list of intersections that can be parsed with a [`Parser`] from data from
/// an [`EAIP`](crate::eaip::EAIP).
pub type Intersections = Vec<Intersection>;

#[async_trait]
impl FromEAIP for Intersections {
    type Output = Self;

    async fn from_eaip(eaip: &EAIP, airac: airac::AIRAC) -> Result<Self::Output> {
        let page = Part::EnRoute(ENR::RadioNavAids(4));
        let data = eaip.get_page(airac, page, EAIPType::HTML).await?;
        let intersections = Intersections::parse(&data)?;
        Ok(intersections)
    }
}

impl<'a> Parser<'a> for Intersections {
    type Output = Self;

    fn parse(data: &'a str) -> Result<Self::Output> {
        let table = Table::find_first(data);
        if table.is_none() {
            return Err(Error::CannotScrapeData("base table is not present"));
        }
        let table = table.unwrap();

        let mut intersections = Vec::new();
        'row: for row in &table {
            let mut intersection = Intersection::default();
            for (i, cell) in row.into_iter().enumerate() {
                if i == 0 {
                    // Column 1 contains name
                    let clean = get_clean_text(cell.clone());
                    if clean.is_empty() {
                        continue 'row;
                    }
                    intersection.designator = clean;
                } else if i == 1 {
                    // Column 2 contains lat long
                    let clean = get_clean_text(cell.clone());
                    for line in clean.split('\n') {
                        let (lat, lon) = parse_latlong(line)?;
                        if let Some(lat) = lat {
                            intersection.latitude = lat;
                        }
                        if let Some(lon) = lon {
                            intersection.longitude = lon;
                        }
                    }
                }
            }

            if intersection != Intersection::default() {
                intersections.push(intersection);
            }
        }

        Ok(intersections)
    }
}
