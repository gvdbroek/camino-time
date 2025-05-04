use crate::components::gpx_map::GpxMap;
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
        // <div class="container">
        //     <Banner/>
        //     <div class="app">
        //         <div id="left">
        //             <GpxMap></GpxMap>
        //         </div>
        //         <div id="right">
        //             <SideBar />
        //         </div>
        //     </div>
        // </div>
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
    view! {
        <div class="statblock">
        <h3>Stats</h3>
        <p>
            <span>"Days walked: 10"</span><br/>
            <span>"Km's walked: 200km"</span><br/>
            <span>"Total ascended: x"</span><br/>
            <span>"Total descended: x"</span><br/>
            <span>"Average speed: x"</span><br/>
        </p>
        </div>

    }
}
