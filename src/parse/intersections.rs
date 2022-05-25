use table_extract::Table;

use crate::{parse::get_clean_text, prelude::*};
use async_trait::async_trait;

/// A list of intersections that can be parsed with a [`Parser`] from data from
/// an [`EAIP`](crate::eaip::EAIP).
pub type Intersections = Vec<Intersection>;

#[async_trait]
impl FromEAIP for Intersections {
    type Output = Self;

    type Error = ();

    async fn from_eaip(eaip: &EAIP, airac: airac::AIRAC) -> Result<Self::Output, Self::Error> {
        let page = Part::EnRoute(ENR::RadioNavAids(4));
        let data = eaip.get_page(airac, page, EAIPType::HTML).await.unwrap();
        let intersections = Intersections::parse(&data).unwrap();
        Ok(intersections)
    }
}

impl<'a> Parser<'a> for Intersections {
    type Output = Self;
    type Error = ();

    fn parse(data: &'a str) -> Result<Self::Output, Self::Error> {
        let table = Table::find_first(data).unwrap();

        let mut intersections = Vec::new();
        for row in &table {
            let mut i = 0;

            let mut intersection = Intersection::default();
            for cell in row {
                if i == 0 {
                    // Column 1 contains name
                    let clean = get_clean_text(cell.clone());
                    intersection.designator = clean;
                } else if i == 1 {
                    // Column 2 contains lat long
                    let clean = get_clean_text(cell.clone());
                    for line in clean.split("\n") {
                        if line.ends_with("N") {
                            let lat = &line[0..(line.len() - 1)];
                            let lat = lat.parse::<f32>().unwrap();
                            intersection.latitude = lat / 10000f32;
                        } else if line.ends_with("S") {
                            let lat = &line[0..(line.len() - 1)];
                            let lat = lat.parse::<f32>().unwrap();
                            intersection.latitude = lat / -10000f32;
                        } else if line.ends_with("E") {
                            let lon = &line[0..(line.len() - 1)];
                            let lon = lon.parse::<f32>().unwrap();
                            intersection.longitude = lon / 10000f32;
                        } else if line.ends_with("W") {
                            let lon = &line[0..(line.len() - 1)];
                            let lon = lon.parse::<f32>().unwrap();
                            intersection.longitude = lon / -10000f32;
                        }
                    }
                }

                i += 1;
            }

            if intersection != Intersection::default() {
                intersections.push(intersection);
            }
        }

        Ok(intersections)
    }
}
