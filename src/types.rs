use crate::prelude::*;
use airac::AIRAC;

/// A radio-based navigational aid.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NavAid {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) kind: NavAidKind,
    pub(crate) frequency_khz: usize,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) elevation: usize,
}

impl NavAid {
    /// The three letter identifier of this navaid.
    pub fn id(&self) -> &String {
        &self.id
    }

    /// The full name of this navaid.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The type of this radio navaid.
    pub fn kind(&self) -> NavAidKind {
        self.kind
    }

    /// The frequency of this radio navaid in kilo-Hertz.
    pub fn frequency_khz(&self) -> usize {
        self.frequency_khz
    }

    /// The frequency of this radio navaid in mega-Hertz.
    pub fn frequency(&self) -> f32 {
        self.frequency_khz as f32 / 1000f32
    }

    /// The latitude of this navaid.
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// The longitude of this navaid.
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// The elevation of this navaid. Note that for NDBs, elevation isn't always given
    /// as it has little effect on the use of the navaid.
    pub fn elevation(&self) -> usize {
        self.elevation
    }
}

/// The kind of navaid this [`NavAid`] is.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NavAidKind {
    /// A VOR (VHF omnidirectional range) navaid
    VOR,
    /// A DME (distance measuring equipment) only navaid
    DME,
    /// A combined VOR/DME navaid
    VORDME,
    /// An NDB (non-directional beacon) navaid
    NDB,
}

impl Default for NavAidKind {
    fn default() -> Self {
        Self::VOR
    }
}

/// An intersection (navigational waypoint).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Intersection {
    pub(crate) designator: String,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
}

impl Intersection {
    /// The 5 letter intersection designator.
    pub fn designator(&self) -> &String {
        &self.designator
    }

    /// The latitude of the intersection.
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// The longitude of the intersection.
    pub fn longitude(&self) -> f64 {
        self.longitude
    }
}

/// An airway.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Airway {
    pub(crate) designator: String,
    pub(crate) waypoints: Vec<AirwayWaypoint>,
}

impl Airway {
    /// The designator for the airway, starting with a 'U' if this is considered an upper airway
    pub fn designator(&self) -> &String {
        &self.designator
    }

    /// The waypoints (in order) of this airway
    pub fn waypoints(&self) -> &Vec<AirwayWaypoint> {
        &self.waypoints
    }
}

/// A waypoint on an airway.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AirwayWaypoint {
    pub(crate) designator: String,
    pub(crate) lower_limit: String,
    pub(crate) upper_limit: String,
}

impl AirwayWaypoint {
    /// The airway waypoint designator. For intersections, this will be 5 characters long, for
    /// radio navaids, this will be 3 characters long.
    pub fn designator(&self) -> &String {
        &self.designator
    }

    /// Is this waypoint a navaid?
    pub fn is_navaid(&self) -> bool {
        self.designator.len() == 3
    }

    /// Is this waypoint an intersection?
    pub fn is_intersection(&self) -> bool {
        self.designator.len() == 5
    }

    /// Get the lower airspace limit of this airway
    pub fn lower_limit(&self) -> &String {
        &self.lower_limit
    }

    /// Get the upper airspace limit of this airway
    pub fn upper_limit(&self) -> &String {
        &self.upper_limit
    }
}

/// Data about an airport
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Airport {
    pub(crate) icao: String,
    pub(crate) name: String,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) elevation: usize,
    pub(crate) charts: Vec<Chart>,
}

impl Airport {
    /// Fetch the data from the given eAIP for the given AIRAC.
    pub async fn from_eaip(eaip: &EAIP, airac: AIRAC, aerodrome: String) -> Result<Self> {
        let egbo = Part::Aerodromes(AD::Aerodromes(aerodrome));
        let data = eaip.get_current_page(egbo.clone(), EAIPType::HTML).await?;
        let mut airport = Airport::parse(&data)?;
        airport.canonicalise_chart_urls(eaip, airac, egbo)?;
        Ok(airport)
    }

    /// Fetch the data from the given eAIP for the current AIRAC.
    pub async fn from_current_eaip(eaip: &EAIP, aerodrome: String) -> Result<Self> {
        Self::from_eaip(eaip, AIRAC::current(), aerodrome).await
    }

    /// Canonicalise chart URLs so they aren't relative.
    pub fn canonicalise_chart_urls(&mut self, eaip: &EAIP, airac: AIRAC, part: Part) -> Result<()> {
        let base_url = url::Url::parse(&eaip.generate_url(airac, part, EAIPType::HTML));
        if base_url.is_err() {
            return Err(Error::EAIPInvalidBaseURL(base_url.unwrap_err()));
        }
        let base_url = base_url.unwrap();
        for chart in &mut self.charts {
            if url::Url::parse(&chart.url) == Err(url::ParseError::RelativeUrlWithoutBase) {
                if let Ok(joined_url) = base_url.join(&chart.url) {
                    chart.url = joined_url.to_string();
                } else {
                    return Err(Error::ChartURLMalformed(chart.url.clone()));
                }
            }
        }
        Ok(())
    }

    /// The ICAO code of the aerodrome
    pub fn icao(&self) -> &String {
        &self.icao
    }

    /// The name of the aerodrome
    pub fn name(&self) -> &String {
        &self.name
    }

    /// The aerodrome's latitude
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// The aerodrome's longitude
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// The aerodrome's elevation
    pub fn elevation(&self) -> usize {
        self.elevation
    }

    /// Charts relating to the aerodrome
    pub fn charts(&self) -> &Vec<Chart> {
        &self.charts
    }
}

/// A chart
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Chart {
    pub(crate) title: String,
    pub(crate) url: String,
}

impl Chart {
    /// The title of the chart
    pub fn title(&self) -> &String {
        &self.title
    }

    /// The URL of the chart
    pub fn url(&self) -> &String {
        &self.url
    }
}
