#[allow(unused_imports)]
use crate::components::{colors, parser};
// use crate::components::parser;
use crate::types::{GpxData, Point, Segment, StatblockData, Track};
#[allow(unused_imports)]
use leptos::logging::log;
use leptos::prelude::*;
use leptos_leaflet::prelude::*;

#[component]
pub fn GpxMap() -> impl IntoView {
    let res = 20;
    // let tracks_resource = OnceResource::new(get_gpx_tracks(res));
    let gpx_data = OnceResource::new(get_all_gpx_data(res));

    view! {
        <Suspense
            fallback=move || view!{<GpxMapPlaceholder/>}
        >
        {
            move ||
                gpx_data.get().map(|re| view!{
                    <GpxMapTrackViewer gpx_data=re.unwrap() />
                })
        }
        </Suspense>
    }
}

fn load_statsblock_data(data: &GpxData) -> StatblockData {
    log!("Loading new statblock");
    let num_tracks = data.tracks.len();
    let total_distance : f64 = data.tracks.iter().map( |track| track.segments.iter().map(|seg| seg.length).sum::<f64>() ).sum();
    let total_rise : f64 = data.tracks.iter().map( |track| track.segments.iter().map(|seg| seg.ascent).sum::<f64>() ).sum();
    let total_desc : f64 = data.tracks.iter().map( |track| track.segments.iter().map(|seg| seg.descent).sum::<f64>() ).sum();

    StatblockData {
        asc_total: total_rise,
        days: num_tracks as i32,
        km_total: total_distance,
        dsc_total: total_desc,
        speed_avg: 3.5,
    }
}

#[component]
fn GpxMapPlaceholder() -> impl IntoView {
    view! {
        <div style="height: 800px;">
        <div class="lds-ring"><div></div><div></div><div></div><div></div></div>
        </div>
    }
}

#[component]
fn GpxMapTrackViewer(gpx_data: GpxData) -> impl IntoView {
    let start_pos = Position::new(42.211, -8.443);
    let gr_l = "3a7bd5".to_string();
    let gr_r = "3a6073".to_string();

    // let darkmap = "https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png";
    let tilemap = "https://tile.openstreetmap.org/{z}/{x}/{y}.png";
    // let topomap load_= "https://{s}.tile.opentopomap.org/{z}/{x}/{y}.png";
    // let stamenmap = "https://stamen-tiles.a.ssl.fastly.net/watercolor/{z}/{x}/{y}.jpg";
    // let stamenmap = "https://stamen-tiles.a.ssl.fastly.net/terrain/{z}/{x}/{y}.jpg";
    let map_url = tilemap;

    // log!("Retrieving global state statblock");
    let set_stats = use_context::<WriteSignal<StatblockData>>().unwrap();
    // log!("Got global state statblock");
    let new_data = load_statsblock_data(&gpx_data);
    set_stats.set(new_data);
    // log!("Rendering view");

    view! {
        <MapContainer style="height: 90vh; width: 100vw;" center=start_pos zoom=8.0 set_view=true>
            <TileLayer url=map_url attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
            <TrackCollection tracks=gpx_data.clone().underlays gradient_left="abb5d6".to_string() gradient_right="abb5d6".to_string() />
            <TrackCollection tracks=gpx_data.clone().tracks gradient_left=gr_l gradient_right=gr_r />
        </MapContainer>

    }
}

#[component]
pub fn TrackCollection(
    tracks: Vec<Track>,
    gradient_left: String,
    gradient_right: String,
) -> impl IntoView {
    view! {

        {
            let mut counter: i32 = 0;
            let mut track_views = vec![];
            let num_tracks = tracks.len() as i32;

            for track in tracks{
                counter += 1;
                let fade_factor: f32 = (counter as f32 / num_tracks as f32).into();
                let lerped_color = colors::color_lerp(gradient_left.as_str(), gradient_right.as_str(), fade_factor);
                let v = view!{<GpxTrack track=track color=lerped_color.clone() />};
                track_views.push(v);

            }
            track_views.collect_view()

        }
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

    for entry in readdir.flatten() {
        // if let Ok(e) = entry {
        let entrypath = entry.path();
        if let Some(extension) = &entrypath.extension() {
            if extension.to_str().unwrap() != "gpx" {
                continue;
            }
        }

        let path = entry.path().into_os_string().into_string().unwrap();
        paths.push(path);
    }
    Ok(paths.clone())
}

#[cfg(feature = "ssr")]
// #[server]
pub async fn get_gpx_tracks(resolution: i32, sub_path: &str) -> Result<Vec<Track>, ServerFnError> {
    let search_path: String = sub_path.to_string();
    let paths = read_gpx_from_dir(&search_path)?;

    let mut tracks = parser::read_gpx_files(paths, resolution).await.unwrap();

    // sort tracks based on name (for chronological reasons)
    // example: people.sort_by(|a, b| b.age.cmp(&a.age));
    tracks.sort_by(|l, r| r.name.cmp(&l.name));
    log!("Collected {} tracks", &tracks.len());
    Ok(tracks)
}

#[server]
pub async fn get_all_gpx_data(resolution: i32) -> Result<GpxData, ServerFnError> {
    let gpx_tracks = get_gpx_tracks(resolution, "./uploads").await;
    let tracks = match gpx_tracks {
        Ok(tracks_list) => tracks_list,
        Err(_) => vec![],
    };

    let underlay_tracks = get_gpx_tracks(resolution, "./underlays").await;
    let underlays = match underlay_tracks {
        Ok(underlays_list) => underlays_list,
        Err(_) => vec![],
    };
    Ok(GpxData {
        tracks: tracks,
        underlays: underlays,
    })
}
