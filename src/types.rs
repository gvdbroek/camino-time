use leptos_leaflet::prelude::Position;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GpxData {
    pub tracks: Vec<Track>,
    pub underlays: Vec<Track>,
}

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

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct StatblockData {
    pub days: i32,
    pub km_total: f64,
    pub asc_total: f64,
    pub dsc_total: f64,
    pub speed_avg: f64,
}

impl Track {
    pub fn get_last_point(&self) -> Option<Point> {
        if let Some(seg) = self.segments.last() {
            if let Some(point) = seg.points.last() {
                let p = point.clone();
                return Some(p);
            }
        }
        None
    }
}

impl From<Point> for Position {
    fn from(p: Point) -> Position {
        Position {
            lat: p.lat,
            lng: p.lon,
        }
    }
}
