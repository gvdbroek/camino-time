use leptos::logging::log;
use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use crate::types::{Track,Segment,Point};

#[component]
pub fn GpxMap() -> impl IntoView {
    let resource = OnceResource::new(get_gpx_filenames());
    log!("Gpx map loading...");
    view! {
        <MapContainer style="height: 800px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
            <Suspense>
                {
                    move ||
                    resource.get().map(|files| view!{<GpxTracks files=files.unwrap().clone()/>})
                }
            </Suspense>
        </MapContainer>
    }
}

#[component]
pub fn GpxTrack(file: String) -> impl IntoView{
    let resource = OnceResource::new(get_gpx_track(file.clone()));
    view!{
        <Suspense>
            {
            move || resource.get().map(|track| view!{<GpxSegments track=track.unwrap().clone()> </GpxSegments>})
        }
        </Suspense>
    }
}
#[component]
pub fn GpxSegments(track: Track) -> impl IntoView {
    view!{
    {
            track.segments.into_iter().map(|seg| view!{
                <GpxSegment segment=seg />
            }).collect_view()
        }


    }
}
#[component]
pub fn GpxTracks(files: Vec<String>) -> impl IntoView {
    view! {
            {
                files.into_iter()
                    .map(|f| view!{<GpxTrack file=f></GpxTrack>})
                    .collect_view()
            }
    }
}

#[component]
pub fn GpxSegmentsOld(files: Vec<String>) -> impl IntoView {
    view! {
            {
                files.into_iter()
                    .map(|f| view!{<GpxSegmentOld file=f></GpxSegmentOld>})
                    .collect_view()
            }
    }
}
#[component]
pub fn GpxSegment(segment: Segment) -> impl IntoView {
    let positions: Vec<Position> = segment.points.into_iter().map(|p| Position{lat:p.lat, lng: p.lon}).collect();
    view!{
        <Polyline positions=positions weight=3.0 />
    }


}

#[component]
pub fn GpxSegmentOld(file: String) -> impl IntoView {
    let resource = OnceResource::new(read_gpx_file(file.clone()));
    // view! {
        // <Suspense>
        // { move ||
        //     resource.get()
        //         // TODO: gpx_segment.points moet worden omgezet naar Leaflet Positions
        //         .map( |gpx_segment| view!{<Polyline positions=gpx_segment.points />})
        // }
        // </Suspense>
    // }

}

pub fn read_gpx_from_dir(path: &String) -> Result<Vec<String>, ServerFnError> {
    let readdir = std::fs::read_dir(path)?;
    let mut paths: Vec<String> = vec![];

    for entry in readdir {
        if let Ok(e) = entry{
                let filename_res = e.file_name();
                if let Ok(filename) = filename_res.into_string(){

                    // get the filename from the filename.ext
                    let file = filename.split(".").collect::<Vec<&str>>()[0];
                    paths.push(file.to_string());
            }
        }
    }
    Ok(paths.clone())
}
#[server]
pub async fn get_gpx_track(filename: String) ->Result<Track, ServerFnError>{

    let p0 = Point{
        lat:0.0,
        lon:0.0
    };
    let p1 = Point{
        lat:5.0,
        lon:5.0
    };
    let points : Vec<Point> = vec![p0, p1];
    let seg = Segment{
        points: points
    };
    let segs : Vec<Segment> = vec![seg];
    let track  = Track{
        segments: segs,
        name: filename.clone()
    };
    Ok(track)
}


#[server]
pub async fn read_gpx_file(filename: String)->Result<Segment,ServerFnError>{
    let filepath: String = format!("./uploads/{}.gpx", filename);
    dbg!(&filepath);
    let p0 = Point{
        lat:0.0,
        lon:0.0
    };
    let p1 = Point{
        lat:5.0,
        lon:5.0
    };
    let mut points : Vec<Point> = vec![p0, p1];
    let seg = Segment{
        points: points
    };
    Ok(seg)

}

#[server]
pub async fn get_gpx_filenames() -> Result<Vec<String>, ServerFnError> {
    let search_path: String = "./uploads".to_string();
    let paths = read_gpx_from_dir(&search_path)?;
    
    Ok(paths.clone())
}
