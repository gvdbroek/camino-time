use crate::components::gpx_map::GpxMap;
use gpx::read;
use gpx::Gpx;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_leaflet::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Stylesheet href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css" />
                <Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ssr-test.css"/>

        // sets the document title
        <Title text="Welcome to Camino Time!"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <GpxMap></GpxMap>
    }
}
#[component]
fn HomePage_old() -> impl IntoView {
    // Creates a reactive value to update the button
    // let count = RwSignal::new(0);
    // let on_click = move |_| *count.write() += 1;
    let resource = OnceResource::new(server_func());

    view! {
            <h1>"Welcome to Camino Time 2025!"</h1>
            // <button on:click=on_click>"Click Me: " {count}</button>
            <Suspense
    fallback= {move || view!{<p>"loading"</p>}}
                >
                {move ||
                    resource.get().map(|d| view!{<NetDataShower data=d.unwrap()> </NetDataShower>})
                }
            </Suspense>
        }
}
#[component]
pub fn NetDataShower_Old(data: ListData) -> impl IntoView {
    log!("Net Data Shower is showing...");
    view! {<ul>
    {data.numbers.into_iter()
        .map(|n| view! { <li>{n.lat}"//"{n.lon}</li>})
        .collect_view()}
    </ul>}
}

#[component]
pub fn NetDataShower(data: ListData) -> impl IntoView {
    let mut positions: Vec<Position> = vec![];
    for netdata in data.numbers.clone() {
        let newp = Position {
            lat: netdata.lat,
            lng: netdata.lon,
        };
        positions.push(newp);
    }
    let first_point = data.numbers[0].clone();
    let first_pos = Position {
        lat: first_point.lat,
        lng: first_point.lon,
    };

    view! {

          <MapContainer style="height: 400px" center=Position::new(51.505, -0.09) zoom=13.0 set_view=true>
              <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
                <Polyline positions=positions weight=5.0/>
            <Circle center=first_pos color="blue" radius=20.0/>
            </MapContainer>
    }
}

#[server]
pub async fn server_func() -> Result<ListData, ServerFnError> {
    // This XML file actually exists — try it for yourself!
    let file = File::open("/home/laika/code/exp/ssr-test/sample.gpx").unwrap();
    let reader = BufReader::new(file);

    // read takes any io::Read and gives a Result<Gpx, Error>.
    let gpx: Gpx = read(reader).unwrap();
    let mut datalist: ListData = ListData { numbers: vec![] };

    for point in gpx.tracks[0].segments[0].points.clone() {
        let d = NetData {
            lat: point.point().y(),
            lon: point.point().x(),
        };
        datalist.numbers.push(d);
    }
    log!("Server func executed");
    Ok(datalist)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetData {
    lat: f64,
    lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListData {
    numbers: Vec<NetData>,
}
