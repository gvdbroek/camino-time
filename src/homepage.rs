use crate::components::gpx_map::GpxMap;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Banner/>
        <GpxMap></GpxMap>
        // <div class="fade-overlay"/>
    }
}

#[component]
pub fn Banner() -> impl IntoView {
    view! {

    <nav class="navbar">
                <div class="container">
                    <a class="navbar-brand" href="#">
                        <img src="CaminoLogo.jpg" alt="Logo" class="logo" />
                        <span class="site-title">CAMINO TIME 2025</span>
                    </a>
                </div>
            </nav>
        }
}
