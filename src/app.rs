use crate::components::gpx_map::GpxMap;
use crate::components::submit_form::GpxSubmitForm;
// use gpx::read;
// use gpx::Gpx;
// use leptos::logging::log;
use crate::homepage::HomePage;
use leptos::prelude::*;
// use leptos_leaflet::prelude::*;
use crate::types::StatblockData;
use leptos_meta::{provide_meta_context, Link, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Stylesheet href="https://unpkg.com/leaflet@1.9.3/dist/leaflet.css" />
                <Script src="https://unpkg.com/leaflet@1.9.3/dist/leaflet.js"/>
                <Link href="https://fonts.googleapis.com/css2?family=VT323&display=swap" rel="stylesheet"/>
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


    // set_stats.set(StatblockData {
    //     asc_total: 0.0,
    //     days: 10,
    //     km_total: 0.0,
    //     dsc_total: 0.0,
    //     speed_avg: 0.0,
    // });

    view! {
        <Stylesheet id="leptos" href="/pkg/camino-time.css"/>
        <Title text="Welcome to Camino Time!"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    // <Route path=StaticSegment("/submit") view=UploadPage/>
                </Routes>
            </main>
        </Router>
    }
}
