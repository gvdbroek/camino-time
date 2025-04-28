use crate::components::gpx_map::GpxMap;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <GpxMap></GpxMap>
        <div class="fade-overlay"/>
    }
}
