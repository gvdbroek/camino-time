use crate::components::gpx_map::GpxMap;
use crate::types::StatblockData;
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (stats, set_stats) = signal(StatblockData {
        asc_total: 0.0,
        days: 0,
        km_total: 0.0,
        dsc_total: 0.0,
        speed_avg: 0.0,
    });

    provide_context(stats);
    provide_context(set_stats);
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

    let (km_walked, set_km_walked) = signal("0.00".to_string());
    let (days_walked, set_days_walked) = signal(0);
    let (asc_total, set_asc_total) = signal("0.00".to_string());
    let (dsc_total, set_dsc_total) = signal("0.00".to_string());
    let (avg_speed, set_avg_speed) = signal(0.0);

    Effect::new(move |_| {
        let val = stats.get().unwrap();
        set_days_walked(val.days);
        set_km_walked(format!("{:.2}", val.km_total));
        set_asc_total(format!("{:.2}", val.asc_total));
        set_dsc_total(format!("{:.2}", val.dsc_total));
        set_avg_speed(val.speed_avg);
    });

    view! {
        <div class="statblock">
            <h3>Stats</h3>
            <p>
                <span>"Days walked: " { days_walked }</span><br/>
                <span>"Km's walked: " { km_walked }</span><br/>
                <span>"Total ascended  (m): "{ asc_total }</span><br/>
                <span>"Total descended (m): "{ dsc_total }</span><br/>
                // <span>"Average speed (km/h): "{ avg_speed }</span><br/>
            </p>
        </div>

    }
}
