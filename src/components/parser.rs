use crate::types::{Point, Segment, Track};

#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos::logging::log;


#[allow(dead_code)]
fn sample_track() -> Track {
    let p0 = Point { lat: 0.0, lon: 0.0 };
    let p1 = Point { lat: 5.0, lon: 5.0 };
    let points: Vec<Point> = vec![p0, p1];
    let seg = Segment { points: points };
    let segs: Vec<Segment> = vec![seg];
    let track = Track {
        segments: segs,
        name: "SampleTrack".to_string(),
    };
    track
}

#[cfg(feature = "ssr")]
pub async fn read_gpx_files(filepaths: Vec<String>, resolution:i32) -> Result<Vec<Track>, ServerFnError> {
    // read a bunch of files using some threads
    use futures::future::join_all;
    let futures = filepaths
        .into_iter()
        .map(|path| async move { read_gpx_file(path, resolution).await.unwrap() });
    let res = join_all(futures).await;

    Ok(res)
}

#[cfg(feature = "ssr")]
pub async fn read_gpx_file(filepath: String, resolution:i32) -> Result<Track, ServerFnError> {
    use gpx::read;
    use gpx::Gpx;
    log!("Parsing: {}", &filepath);
    use tokio::fs;

    // This XML file actually exists — try it for yourself!
    // let file = File::open(&filepath).unwrap();
    let file = fs::read(&filepath).await?;
    let cursor = std::io::Cursor::new(&file);
    // let reader = BufReader::new(file);
    let gpx: Gpx = read(cursor).unwrap();
    let mut track_segments: Vec<Segment> = vec![];

    for file_track in &gpx.tracks {
        let mut points: Vec<Point> = vec![];

        for seg in &file_track.segments {
            let mut skipper: i32 = 0;
            for waypoint in &seg.points {
                skipper += 1;
                if skipper % resolution != 0 {
                    continue;
                }
                let p = Point {
                    lat: waypoint.point().y(),
                    lon: waypoint.point().x(),
                };
                points.push(p);
            }
            let new_seg = Segment {
                points: points.clone(),
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
