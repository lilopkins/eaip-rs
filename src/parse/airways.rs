use regex::Regex;
use scraper::{Html, Selector};

use crate::{parse::get_clean_text, prelude::*};
use async_trait::async_trait;

/// A list of intersections that can be parsed with a [`Parser`] from data from
/// an [`EAIP`](crate::eaip::EAIP).
pub type Airways = Vec<Airway>;

#[async_trait]
impl FromEAIP for Airways {
    type Output = Self;

    type Error = ();

    async fn from_eaip(eaip: &EAIP, airac: airac::AIRAC) -> Result<Self::Output, Self::Error> {
        let page = Part::EnRoute(ENR::ATSRoutes(3));
        let data = eaip.get_page(airac, page, EAIPType::HTML).await.unwrap();
        let airways = Airways::parse(&data).unwrap();
        Ok(airways)
    }
}

impl<'a> Parser<'a> for Airways {
    type Output = Self;
    type Error = ();

    fn parse(data: &'a str) -> Result<Self::Output, Self::Error> {
        let mut airways = Vec::new();

        let html = Html::parse_document(data);
        let tbody_selector = Selector::parse("tbody").unwrap();
        let designator_selector = Selector::parse("tr.Table-row-type-1 > td:first-child").unwrap();
        let waypoint_selector = Selector::parse("tr.Table-row-type-2 > td:nth-child(2)").unwrap();
        let detail_selector = Selector::parse("tr.Table-row-type-3").unwrap();
        let upper_limit_selector = Selector::parse("td:nth-child(4) td.Upper").unwrap();
        let lower_limit_selector = Selector::parse("td:nth-child(4) td.Lower").unwrap();
        // Capture either an intersection (group 1), or a navaid (group 2)
        let intersection_or_vor_re = Regex::new(r#"(^[A-Z]{5})|\(([A-Z]{3})\)"#).unwrap();

        for tbody in html.select(&tbody_selector) {
            let mut airway = Airway::default();
            let designator = tbody.select(&designator_selector).next();
            if designator.is_none() {
                continue;
            }
            let designator = designator.unwrap();
            let clean = get_clean_text(designator.inner_html());
            let parts = clean.split("\n").collect::<Vec<&str>>();
            airway.designator = parts[0].to_string();

            for waypoint in tbody.select(&waypoint_selector) {
                let clean = get_clean_text(waypoint.inner_html());
                let caps = intersection_or_vor_re.captures(&clean).unwrap();
                if let Some(cap) = caps.get(2) {
                    // VOR, must be checked first to make sure a VOR name isn't picked up as an intersection
                    airway.waypoints.push(AirwayWaypoint {
                        designator: cap.as_str().to_string(),
                        ..Default::default()
                    })
                } else if let Some(cap) = caps.get(1) {
                    // Intersection
                    airway.waypoints.push(AirwayWaypoint {
                        designator: cap.as_str().to_string(),
                        ..Default::default()
                    })
                }
            }

            let mut i = 0;
            for detail in tbody.select(&detail_selector) {
                if let Some(upper) = detail.select(&upper_limit_selector).next() {
                    let upper = get_clean_text(upper.inner_html());
                    airway.waypoints[i].upper_limit = upper;
                }
                
                if let Some(lower) = detail.select(&lower_limit_selector).next() {
                    let lower = get_clean_text(lower.inner_html());
                    airway.waypoints[i].lower_limit = lower;
                }

                i += 1;
            }

            if airway != Airway::default() {
                airways.push(airway);
            }
        }

        Ok(airways)
    }
}