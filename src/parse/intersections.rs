use crate::{parse::get_clean_text, prelude::*};
use async_trait::async_trait;
use scraper::{Html, Selector};

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
        let html = Html::parse_document(data);
        let table_content_selector = Selector::parse("table > tbody").unwrap();
        let intersection_tr_selector = Selector::parse("tr.Table-row-type-3").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let table = html
            .select(&table_content_selector)
            .next()
            .ok_or(Error::CannotScrapeData("base table not present"))?;

        let mut intersections = Vec::new();
        'row: for row in table.select(&intersection_tr_selector) {
            let mut intersection = Intersection::default();
            for (i, cell) in row.select(&td_selector).enumerate() {
                if i == 0 {
                    // Column 1 contains name
                    let clean = get_clean_text(cell.inner_html());
                    if clean.is_empty() {
                        continue 'row;
                    }
                    intersection.designator = clean;
                } else if i == 1 {
                    // Column 2 contains lat long
                    let clean = get_clean_text(cell.inner_html());
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
