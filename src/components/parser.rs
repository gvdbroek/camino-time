use crate::types::{Point, Segment, Track};

#[cfg(feature = "ssr")]
use leptos::logging::log;
#[cfg(feature = "ssr")]
use leptos::prelude::*;

#[allow(dead_code)]
fn sample_track() -> Track {
    let p0 = Point::default();
    let p1 = Point { lat: 5.0, lon: 5.0, ele:0.0 };
    let points: Vec<Point> = vec![p0, p1];
    let seg = Segment { points: points, length: 0.0 , ascent: 0.0, descent: 0.0};
    let segs: Vec<Segment> = vec![seg];
    let track = Track {
        segments: segs,
        name: "SampleTrack".to_string(),
    };
    track
}

#[cfg(feature = "ssr")]
pub async fn read_gpx_files(
    filepaths: Vec<String>,
    resolution: i32,
) -> Result<Vec<Track>, ServerFnError> {
    // read a bunch of files using some threads
    use futures::future::join_all;
    let futures = filepaths
        .into_iter()
        .map(|path| async move { read_gpx_file(path, resolution, true).await.unwrap() });
    let res = join_all(futures).await;

    Ok(res)
}

#[cfg(feature = "ssr")]
pub async fn read_gpx_file(filepath: String, resolution: i32, calculate_distance: bool) -> Result<Track, ServerFnError> {
    use gpx::read;
    use gpx::Gpx;
    log!("Parsing: {}", &filepath);
    use tokio::fs;

    let file = fs::read(&filepath).await?;
    let cursor = std::io::Cursor::new(&file);
    let gpx: Gpx = read(cursor).unwrap();
    let mut track_segments: Vec<Segment> = vec![];

    let mut previous_elevation: f64 = 0.0;
    for file_track in &gpx.tracks {
        let mut points: Vec<Point> = vec![];

        for seg in &file_track.segments {
            let mut seg_len = 0.0;
            let mut seg_rise = 0.0;
            let mut seg_dropped = 0.0;

            let mut skipper: i32 = 0;
            let mut previous_point = Point::default();

            for waypoint in &seg.points {
                skipper += 1;
                if skipper % resolution != 0 { continue; }

                let elevation = match(waypoint.elevation){
                    Some(v) => v,
                    None => previous_elevation
                };
                previous_elevation = elevation;
                let p = Point {
                    lat: waypoint.point().y(),
                    lon: waypoint.point().x(),
                    ele: elevation
                };
                points.push(p.clone());

                if previous_point.is_zero() == false && calculate_distance{
                    let delta_length = previous_point.distance_to(&p);

                    let delta_elev = p.ele - previous_point.ele;
                    if delta_elev > 0.0 {seg_rise += delta_elev.abs() ;}
                    else {seg_dropped += delta_elev.abs();}
                    seg_len += delta_length;

                }

                previous_point = p;
                
            }
            let new_seg = Segment {
                points: points.clone(),
                length: seg_len,
                ascent: seg_rise,
                descent: seg_dropped

            };
            track_segments.push(new_seg);
        }
    }
    let track = Track {
        segments: track_segments.clone(),
        name: filepath,
    };

    Ok(track)
}
