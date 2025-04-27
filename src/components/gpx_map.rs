use crate::types::{Point, Segment, Track};
use leptos::logging::{log,error};
use leptos::prelude::*;
use leptos_leaflet::prelude::*;

#[component]
pub fn GpxMap() -> impl IntoView {
    let resource = OnceResource::new(get_gpx_filenames());
    log!("Gpx map loading...");
    let start_pos = Position::new(42.211,-8.443);
    view! {
        <MapContainer style="height: 801px" center=start_pos zoom=10.0 set_view=true>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
            <Suspense>
                {
                    move ||
                    resource.get().map(|files| 
                        {
                        files.unwrap().into_iter()
                            .map(|file| view!{<GpxTrackLoader file=file/>})
                            .collect_view()
                        }
                    )
                }
            </Suspense>
        </MapContainer>
    }
}
#[component]
pub fn GpxTrackLoader(file: String) -> impl IntoView{

    let resource = OnceResource::new(get_gpx_track(file.clone()));
    view! {
        <Suspense>
            {
            move || resource.get().map(
                |track| {
                        view!{<GpxTrack track=track.unwrap()/> 
                        }
                })
        }
        </Suspense>
    }

}

#[component]
pub fn GpxTrack(track: Track) -> impl IntoView {
    log!("Rendering -track");
    // let resource = OnceResource::new(get_gpx_track(file.clone()));

    view! {
        <GpxSegments segments=track.segments.clone() />
    }
}

#[component]
pub fn GpxSegments(segments: Vec<Segment>) -> impl IntoView {
    log!("Rendering --segments");
    view! {
    {
            segments.into_iter().map(|seg| view!{
                <GpxSegment segment=seg />
            }).collect_view()
        }


    }
}

#[component]
pub fn GpxSegment(segment: Segment) -> impl IntoView {
    log!("Rendering ---segment");
    let positions: Vec<Position> = segment
        .points
        .into_iter()
        .map(|p| Position {
            lat: p.lat,
            lng: p.lon,
        })
        .collect();
    let positionsb = positions.clone();
    log!("Collected positions");
    view! {
        <Polyline positions=positionsb.clone() weight=3.0 />
    }
}


pub fn read_gpx_from_dir(path: &String) -> Result<Vec<String>, ServerFnError> {
    let readdir = std::fs::read_dir(path)?;
    let mut paths: Vec<String> = vec![];

    for entry in readdir {
        if let Ok(e) = entry {
            let filename_res = e.file_name();
            if let Ok(filename) = filename_res.into_string() {
                // get the filename from the filename.ext
                let file = filename.split(".").collect::<Vec<&str>>()[0];
                paths.push(file.to_string());
            }
        }
    }
    Ok(paths.clone())
}
#[server]
pub async fn get_gpx_track(filename: String) -> Result<Track, ServerFnError> {
    let p0 = Point { lat: 0.0, lon: 0.0 };
    let p1 = Point { lat: 5.0, lon: 5.0 };
    let points: Vec<Point> = vec![p0, p1];
    let seg = Segment { points: points };
    let segs: Vec<Segment> = vec![seg];
    let track = Track {
        segments: segs,
        name: filename.clone(),
    };
    Ok(track)
}

#[server]
pub async fn read_gpx_file(filename: String) -> Result<Segment, ServerFnError> {
    let filepath: String = format!("./uploads/{}.gpx", filename);
    dbg!(&filepath);
    let p0 = Point { lat: 0.0, lon: 0.0 };
    let p1 = Point { lat: 5.0, lon: 5.0 };
    let mut points: Vec<Point> = vec![p0, p1];
    let seg = Segment { points: points };
    Ok(seg)
}

#[server]
pub async fn get_gpx_filenames() -> Result<Vec<String>, ServerFnError> {
    let search_path: String = "./uploads".to_string();
    let paths = read_gpx_from_dir(&search_path)?;

    Ok(paths.clone())
}
