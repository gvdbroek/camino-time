use crate::components::gpx_map::GpxMap;
use crate::types::StatblockData;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <Banner/>
            <div class="app">
                <div class="map-container">
                    <GpxMap></GpxMap>
                </div>
                <div class="infobox">
                    <Statblock />
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="navbar-container">
                <a class="navbar-brand" href="#">
                    <img src="CaminoLogo.jpg" alt="Logo" class="logo" />
                    <span class="site-title">CAMINO TIME 2025</span>
                </a>
            </div>
        </nav>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {
    view! {
        <div class="sidebar">
            <Statblock />
        </div>
    }
}
#[component]
pub fn Statblock() -> impl IntoView {
    let stats = use_context::<ReadSignal<StatblockData>>();

    view! {
        <div class="statblock">
            <h3>Stats</h3>
            <p>
                <span>"Days walked: " { move || stats.get().unwrap().days}</span><br/>
                <span>"Km's walked: " {move || stats.get().unwrap().km_total}</span><br/>
                <span>"Total ascended  (m): "{ move|| stats.get().unwrap().asc_total}</span><br/>
                <span>"Total descended (m): "{ move || stats.get().unwrap().dsc_total}</span><br/>
                <span>"Average speed (km/h): "{ move ||stats.get().unwrap().speed_avg}</span><br/>
            </p>
        </div>

    }
}
