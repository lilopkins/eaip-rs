use table_extract::Table;

use crate::{parse::get_clean_text, prelude::*};

/// A list of radio-based navaids that can be parsed with a [`Parser`] from data from
/// an [`EAIP`](crate::eaip::EAIP).
pub type Navaids = Vec<NavAid>;

impl<'a> Parser<'a> for Navaids {
    type Output = Self;
    type Error = ();

    fn parse(data: &'a str) -> Result<Self::Output, Self::Error> {
        let table = Table::find_first(data).unwrap();

        let mut navaids = Vec::new();
        'row: for row in &table {
            let mut i = 0;

            let mut navaid = NavAid::default();
            for cell in row {
                if i == 0 {
                    // Column 1 contains name and type
                    let clean = get_clean_text(cell.clone());
                    let lines = clean.split("\n").collect::<Vec<&str>>();
                    navaid.name = lines[0].trim().to_string();
                    let kind = lines[1].trim();
                    let typ = if kind == "VOR" {
                        NavAidKind::VOR
                    } else if kind == "VOR/DME" {
                        NavAidKind::VORDME
                    } else if kind == "DME" {
                        NavAidKind::DME
                    } else if kind == "NDB" {
                        NavAidKind::NDB
                    } else if kind == "" {
                        continue 'row;
                    } else {
                        panic!("Unknown NavAidKind: {}", kind)
                    };
                    navaid.kind = typ;
                } else if i == 1 {
                    // Column 2 contains ID
                    navaid.id = get_clean_text(cell.clone());
                } else if i == 2 {
                    // Column 3 contains frequency
                    let clean = get_clean_text(cell.clone());
                    for line in clean.split("\n") {
                        if line.ends_with("MHz") {
                            let freq = &line[0..(line.len() - 3)];
                            let freq = freq.parse::<f32>().unwrap();
                            navaid.frequency_khz = (freq * 1000f32) as usize;
                        }
                        if line.ends_with("kHz") {
                            let freq = &line[0..(line.len() - 3)];
                            let freq = freq.parse::<f32>().unwrap();
                            navaid.frequency_khz = freq as usize;
                        }
                    }
                } else if i == 4 {
                    // Column 5 contains lat long
                    let clean = get_clean_text(cell.clone());
                    for line in clean.split("\n") {
                        if line.ends_with("N") {
                            let lon = &line[0..(line.len() - 1)];
                            let lon = lon.parse::<f32>().unwrap();
                            navaid.longitude = lon / 10000f32;
                        } else if line.ends_with("S") {
                            let lon = &line[0..(line.len() - 1)];
                            let lon = lon.parse::<f32>().unwrap();
                            navaid.longitude = lon / -10000f32;
                        } else if line.ends_with("E") {
                            let lat = &line[0..(line.len() - 1)];
                            let lat = lat.parse::<f32>().unwrap();
                            navaid.latitude = lat / 10000f32;
                        } else if line.ends_with("W") {
                            let lat = &line[0..(line.len() - 1)];
                            let lat = lat.parse::<f32>().unwrap();
                            navaid.latitude = lat / -10000f32;
                        }
                    }
                } else if i == 5 {
                    // Column 6 contains elevation
                    let clean = get_clean_text(cell.clone());
                    if clean.ends_with("FT") {
                        navaid.elevation = clean[0..(clean.len() - 2)].parse::<usize>().unwrap();
                    }
                }

                i += 1;
            }

            if navaid != NavAid::default() {
                navaids.push(navaid);
            }
        }

        Ok(navaids)
    }
}
