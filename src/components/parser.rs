use gpx::Gpx;
use std::io::BufReader;
// use std::fs::File;
use leptos::prelude::*;
use crate::types::{Point, Segment, Track};
use gpx::read;
use futures::future::join_all;
use leptos::logging::log;


fn sample_track()-> Track{
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

pub async fn read_gpx_files(filepaths: Vec<String>)->Result<Vec<Track>, ServerFnError> {

    // read a bunch of files using some threads
    let futures = filepaths.into_iter().map(
        |path| {
            async move {
                read_gpx_file(path).await.unwrap()
            }
        }
    );
    let res = join_all(futures).await;

    Ok(res)
}
pub async fn read_gpx_file(filepath: String) -> Result<Track, ServerFnError> {
    // dbg!(&filepath);
    log!("Parsing: {}", &filepath);
    use tokio::fs;

    // This XML file actually exists — try it for yourself!
    // let file = File::open(&filepath).unwrap();
    let file = fs::read(&filepath).await?;
    let reader = BufReader::new(file);
    let gpx: Gpx = read(reader).unwrap();
    let mut track_segments: Vec<Segment> = vec![];

    for file_track in &gpx.tracks {

        let mut points : Vec<Point> = vec![];

        for seg in &file_track.segments{
            let mut skipper: i32 = 0;
            for waypoint in &seg.points{
                skipper +=1;
                 if skipper % 50 != 0 {
                    continue;
                 }
                let p = Point{
                    lat: waypoint.point().y(),
                    lon: waypoint.point().x()
                };
                points.push(p);

            }
            let new_seg = Segment{
                points:points.clone()
            };
            track_segments.push(new_seg);
        }
    }
    let track = Track{
        segments: track_segments.clone(),
        name: filepath
    };

    Ok(track)
}
