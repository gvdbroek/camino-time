#[allow(unused_imports)]
use crate::components::{colors, parser};
// use crate::components::parser;
use crate::types::{Point, Segment, Track};
#[allow(unused_imports)]
use leptos::logging::log;
use leptos::prelude::*;
use leptos_leaflet::prelude::*;

#[component]
pub fn GpxMap() -> impl IntoView {
    let res = 20;
    let tracks_resource = OnceResource::new(get_gpx_tracks(res));

    // let tracks : Vec<Track> = vec![];
    view! {
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
fn GpxMapPlaceholder() -> impl IntoView {
    view! {
        <div style="height: 801px;">
        <div class="lds-ring"><div></div><div></div><div></div><div></div></div>
        </div>
    }
    // view!{<span class="loader"></span>}
    // view!{<h1>"LOADING..."</h1>}
}

#[component]
fn GpxMapTrackViewer(tracks: Vec<Track>) -> impl IntoView {
    let start_pos = Position::new(42.211, -8.443);
    // let base_color: String = "red".to_string();
    let gr_l = "3a7bd5";
    let gr_r = "3a6073";
    let darkmap = "https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png";
    let tilemap = "https://tile.openstreetmap.org/{z}/{x}/{y}.png";
    let topomap = "https://{s}.tile.opentopomap.org/{z}/{x}/{y}.png";
    let stamenmap = "https://stamen-tiles.a.ssl.fastly.net/watercolor/{z}/{x}/{y}.jpg";
    // let stamenmap = "https://stamen-tiles.a.ssl.fastly.net/terrain/{z}/{x}/{y}.jpg";
    let map_url = tilemap;
    view! {
        <MapContainer style="height: 800px" center=start_pos zoom=8.0 set_view=true>
            <TileLayer url=map_url attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
        {
                let mut counter: i32 = 0;
                let mut track_views = vec![];
                let num_tracks = tracks.len() as i32;

                for track in tracks{
                    counter += 1;
                    let fade_factor: f32 = (counter as f32 / num_tracks as f32).into();
                    let lerped_color = colors::color_lerp(gr_l, gr_r, fade_factor);

                    let v = view!{<GpxTrack track=track color=lerped_color.clone() />};
                    track_views.push(v);

                }
                track_views.collect_view()
                // tracks.into_iter()
                //     .map(|track| view!{<GpxTrack track=track color=base_color.clone() />})
                //     .collect_view()
        }
        </MapContainer>
        // <MapContainer style="height: 801px" center=start_pos zoom=9.0 set_view=true>
        //     <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
        // {
        //         tracks.into_iter()
        //             .map(|track| view!{<GpxTrack track=track color=base_color.clone() />})
        //             .collect_view()
        // }
        // </MapContainer>
    }
}

pub fn has_value(value: Option<Point>) -> bool {
    match value {
        Some(_) => true,
        None => false,
    }
}

#[component]
pub fn GpxTrack(track: Track, color: String) -> impl IntoView {
    let last_pos = track.get_last_point();
    view! {
        <GpxTrackEnd endpoint=last_pos.unwrap() color=color.clone() />
         <GpxSegments segments=track.segments.clone() color=color />
    }
}
#[component]
pub fn GpxTrackEnd(endpoint: Point, color: String) -> impl IntoView {
    view! {
        <Circle center=Position::from(endpoint) color=color.clone() radius=400.0 > </Circle>
    }
}

#[component]
pub fn GpxSegments(segments: Vec<Segment>, color: String) -> impl IntoView {
    // log!("Rendering --segments");
    view! {
            {
                segments.into_iter().map(|seg| view!{
                    <GpxSegment segment=seg color=color.clone() />
                }).collect_view()
            }
    }
}

pub fn fade_red(fade_factor: f32) -> String {
    let fade_factor = fade_factor.clamp(0.0, 1.0);
    let red_value = (255.0 * (1.0 - fade_factor)) as u8; // 255 - fade_factor * 255
    format!("#{:02X}{:02X}{:02X}", 255, red_value, red_value)
}
#[component]
pub fn GpxSegment(segment: Segment, color: String) -> impl IntoView {
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
        <Polyline positions=positionsb.clone() weight=5.0 color=color/>
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
pub async fn get_gpx_tracks(resolution: i32) -> Result<Vec<Track>, ServerFnError> {
    let search_path: String = "./uploads".to_string();
    let paths = read_gpx_from_dir(&search_path)?;

    let mut tracks = parser::read_gpx_files(paths, resolution).await.unwrap();

    // sort tracks based on name (for chronological reasons)
    // example: people.sort_by(|a, b| b.age.cmp(&a.age));
    tracks.sort_by(|l, r| r.name.cmp(&l.name));
    log!("Collected {} tracks", &tracks.len());
    Ok(tracks)
}
