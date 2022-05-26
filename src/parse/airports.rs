use regex::Regex;
use scraper::{Html, Selector};

use crate::{parse::get_clean_text, prelude::*};
use async_trait::async_trait;

/// A list of airport ICAO codes.
pub type Airports = Vec<Airport>;

#[async_trait]
impl FromEAIP for Airports {
    type Output = Self;

    /// **IMPORTANT**: For airports, `from_eaip` only fetches a list and populates ICAO code and name.
    /// For more details, each airport must be fetched individually.
    async fn from_eaip(eaip: &EAIP, airac: airac::AIRAC) -> Result<Self::Output> {
        let page = Part::Aerodromes(AD::TableOfContents);
        let data = eaip.get_page(airac, page, EAIPType::HTML).await.unwrap();
        let html = Html::parse_document(&data);

        let toc_block_selector =
            Selector::parse(".toc-block:nth-of-type(2) > .toc-block a").unwrap();
        let ad_re = Regex::new(r"^([A-Z]{4})\s*—?\s*(.*)$").unwrap();

        let mut airports = Airports::new();

        for ad_toc_block in html.select(&toc_block_selector) {
            let clean = get_clean_text(ad_toc_block.inner_html());
            if let Some(caps) = ad_re.captures(&clean) {
                airports.push(Airport {
                    icao: caps[1].to_string(),
                    name: caps[2].to_string(),
                    ..Default::default()
                });
            }
        }

        Ok(airports)
    }
}

impl<'a> Parser<'a> for Airport {
    type Output = Self;

    fn parse(data: &'a str) -> Result<Self::Output> {
        let mut airport = Self::default();

        let html = Html::parse_document(data);
        let div_selector = Selector::parse("div").unwrap();
        let title_ad_selector = Selector::parse(".TitleAD").unwrap();
        let first_row_selector = Selector::parse("tr:nth-child(1)").unwrap();
        let third_row_selector = Selector::parse("tr:nth-child(3)").unwrap();
        let data_td_selector = Selector::parse("td:last-child").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let a_selector = Selector::parse("a").unwrap();
        let elevation_re = Regex::new(r"(\d+)\s*(?:FT|ft)").unwrap();
        let latitude_re = Regex::new(r"([0-9]{6}[NS])").unwrap();
        let longitude_re = Regex::new(r"([0-9]{7}[EW])").unwrap();

        for main_elem in html.select(&div_selector) {
            // .TitleAD contains ICAO code and Airport Name
            let title_elem = main_elem.select(&title_ad_selector).next();
            if title_elem.is_none() {
                continue;
            }
            let title_elem = title_elem.unwrap();
            let clean = get_clean_text(title_elem.inner_html());
            let title_parts = clean.split("—").collect::<Vec<&str>>();
            airport.icao = title_parts[0].trim().to_string();
            airport.name = title_parts[1].trim().to_string();

            // Parent of TitleAD contains all divs for processing
            for div in main_elem.select(&div_selector) {
                if let Some(id) = div.value().attr("id") {
                    if id.ends_with("-2.2") {
                        // .<icao>-AD-2.2 contains main data table
                        // 1st row contains lat/long
                        // 3rd row contains elevation
                        let first_row = div.select(&first_row_selector).next().unwrap();
                        let latlong = first_row.select(&data_td_selector).next().unwrap();
                        let clean = get_clean_text(latlong.inner_html());

                        let long_cap = longitude_re.captures(&clean).unwrap();
                        let lat_cap = latitude_re.captures(&clean).unwrap();

                        let lat = &lat_cap[1];
                        if lat.ends_with("N") {
                            let lat = &lat[0..(lat.len() - 1)];
                            let lat = lat.parse::<f32>().unwrap();
                            airport.latitude = lat / 10000f32;
                        } else if lat.ends_with("S") {
                            let lat = &lat[0..(lat.len() - 1)];
                            let lat = lat.parse::<f32>().unwrap();
                            airport.latitude = lat / -10000f32;
                        }

                        let long = &long_cap[1];
                        if long.ends_with("E") {
                            let long = &long[0..(long.len() - 1)];
                            let long = long.parse::<f32>().unwrap();
                            airport.longitude = long / 10000f32;
                        } else if long.ends_with("W") {
                            let long = &long[0..(long.len() - 1)];
                            let long = long.parse::<f32>().unwrap();
                            airport.longitude = long / -10000f32;
                        }

                        let third_row = div.select(&third_row_selector).next().unwrap();
                        let elevation = third_row.select(&data_td_selector).next().unwrap();
                        let clean = get_clean_text(elevation.inner_html());

                        let elev_cap = elevation_re.captures(&clean).unwrap();
                        let elev = &elev_cap[1];
                        airport.elevation = elev.parse().unwrap();
                    } else if id.ends_with("-2.24") {
                        // .<icao>-ad-2.24 contains charts
                        // iterate through <td>, alternate between title and chart link
                        let mut chart_title = None;
                        for td in div.select(&td_selector) {
                            if chart_title.is_none() {
                                chart_title = Some(get_clean_text(td.inner_html()));
                            } else {
                                let a = td.select(&a_selector).next().unwrap();
                                let href = a.value().attr("href").unwrap();
                                airport.charts.push(Chart {
                                    title: chart_title.unwrap(),
                                    url: href.to_string(),
                                });
                                chart_title = None;
                            }
                        }
                    }
                }
            }

            break; // once the data is found, stop. don't traverse deeper
        }

        Ok(airport)
    }
}
