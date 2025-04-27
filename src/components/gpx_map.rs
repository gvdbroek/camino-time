use crate::types::{Point, Segment, Track};
use leptos::logging::{log,error};
use leptos::prelude::*;
use leptos_leaflet::prelude::*;

use crate::components::parser;

#[component]
pub fn GpxMap()-> impl IntoView{

    let tracks_resource = OnceResource::new(get_gpx_tracks());

    // let tracks : Vec<Track> = vec![];
    view!{
        <Suspense
            fallback=move || view!{<GpxMapPlaceholder/>}
        >
            {
            move ||
            // log!("Getting server gpx files!");
                tracks_resource.get().map(|re| view!{
                    <GpxMapTrackViewer tracks=re.unwrap() />

                })
        }
             // <GpxMapTrackViewer tracks=tracks />
        </Suspense>
    }
}
#[component]
fn GpxMapPlaceholder() -> impl IntoView{
    view!{
        <div style="height: 801px;">
        <div class="lds-ring"><div></div><div></div><div></div><div></div></div>
        </div>
    }
    // view!{<span class="loader"></span>}
    // view!{<h1>"LOADING..."</h1>}

}

#[component]
fn GpxMapTrackViewer(tracks: Vec<Track>) -> impl IntoView{
    let start_pos = Position::new(42.211,-8.443);
    view! {
        <MapContainer style="height: 801px" center=start_pos zoom=10.0 set_view=true>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
    { tracks.into_iter()
                    .map(|track| view!{<GpxTrack track=track />})
                    .collect_view()
            }
        </MapContainer>
    }

}




#[component]
pub fn GpxTrack(track: Track) -> impl IntoView {
    // log!("Rendering -track");

    view! {
        <GpxSegments segments=track.segments.clone() />
    }
}

#[component]
pub fn GpxSegments(segments: Vec<Segment>) -> impl IntoView {
    // log!("Rendering --segments");
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
    // log!("Rendering ---segment");
    let positions: Vec<Position> = segment
        .points
        .into_iter()
        .map(|p| Position {
            lat: p.lat,
            lng: p.lon,
        })
        .collect();
    let positionsb = positions.clone();
    // log!("Collected positions");
    view! {
        // <p>"point"</p>
        <Polyline positions=positionsb.clone() weight=3.0 />
    }
}

pub fn read_gpx_from_dir(path: &String) -> Result<Vec<String>, ServerFnError> {
    let readdir = std::fs::read_dir(path)?;
    let mut paths: Vec<String> = vec![];

    for entry in readdir {
                // dbg!(&entry);
        if let Ok(e) = entry {
            let path = e.path().into_os_string().into_string().unwrap();
            paths.push(path);
        }
    }
    Ok(paths.clone())
}


#[server]
pub async fn get_gpx_tracks()-> Result<Vec<Track>, ServerFnError>{
    let search_path: String = "./uploads".to_string();
    let paths = read_gpx_from_dir(&search_path)?;

    let tracks = parser::read_gpx_files(paths).await.unwrap();

    log!("Collected {} tracks", &tracks.len());

    Ok(tracks)
}
