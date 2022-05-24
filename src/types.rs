/// A radio-based navigational aid.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NavAid {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) kind: NavAidKind,
    pub(crate) frequency_khz: usize,
    pub(crate) latitude: f32,
    pub(crate) longitude: f32,
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
        self.frequency_khz as f32 / 100f32
    }

    /// The latitude of this navaid.
    pub fn latitude(&self) -> f32 {
        self.latitude
    }

    /// The longitude of this navaid.
    pub fn longitude(&self) -> f32 {
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
    pub(crate) latitude: f32,
    pub(crate) longitude: f32,
}

impl Intersection {
    /// The 5 letter intersection designator.
    pub fn designator(&self) -> &String {
        &self.designator
    }

    /// The latitude of the intersection.
    pub fn latitude(&self) -> f32 {
        self.latitude
    }

    /// The longitude of the intersection.
    pub fn longitude(&self) -> f32 {
        self.longitude
    }
}
