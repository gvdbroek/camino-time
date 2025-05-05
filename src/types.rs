use leptos_leaflet::prelude::Position;
use serde::{Deserialize, Serialize};
use is_close::is_close;

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
    pub length: f64,
    pub ascent: f64,
    pub descent: f64
}


#[derive(Clone, Serialize, Deserialize, Debug, Copy)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
    pub ele: f64,
    // pub time: Option<String>,
}
impl Point{
    pub fn add(self, other: &Point) -> Point{
        Point{
            lat: self.lat - other.lat,
            lon: self.lon - other.lon,
            ele: self.ele - other.ele
        }
    }
    // Distance in meters
    pub fn distance_to(self, other: &Point) -> f64{
        // 
        //     Latitude: 1 deg = 110.574 km.
        //    Longitude: 1 deg = 111.320*cos(latitude) km.

        let x = self.lon - other.lon;
        let y = self.lat - other.lat;

        let y_km = y * 110.574;
        let x_km = x * (111.320 * y.to_radians().cos());

        // ((x_km * x_km ) + (y_km * y_km)).sqrt()
        haversine_km(self.lat, self.lon, other.lat, other.lon)
    }

    pub fn is_close(&self, other: &Point) -> bool{
        is_close!(self.lat, other.lat) && is_close!(self.lon, other.lon) && is_close!(self.ele, other.ele)
    }
    pub fn is_zero(&self) -> bool{
        self.is_close(&Point::default())
    }

}
    fn haversine_km(lat1 : f64, lon1 : f64, lat2: f64, lon2: f64 )-> f64{
        // https://stackoverflow.com/questions/365826/calculate-distance-between-2-gps-coordinates
    let dlong = (lon2 - lon1).to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let a = (dlat/2.0).sin().powi(2) + (lat1.to_radians()).cos() * lat2.to_radians().cos() * (dlong/2.0).sin().powi(2);
    let c = 2.0 * (a.sqrt().atan2((1.0-a).sqrt()));
    let d = 6367.0 * c;
    // double a = pow(sin(dlat/2.0), 2) + cos(lat1*d2r) * cos(lat2*d2r) * pow(sin(dlong/2.0), 2);
    // double c = 2 * atan2(sqrt(a), sqrt(1-a));
    // double d = 6367 * c;
    //
    // return d;
    d
    }
impl Default for Point {
    fn default() -> Self {
        Point {
            lat: 0.0,
            lon: 0.0,
            ele: 0.0,
        }
    }
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
