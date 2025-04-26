use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Track {
    pub name: String,
    pub segments: Vec<Segment>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Segment {
    pub points: Vec<Point>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
    // pub ele: Option<f64>,
    // pub time: Option<String>,
}
