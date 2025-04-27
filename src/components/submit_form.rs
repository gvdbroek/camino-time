// use leptos::html::Form;
use leptos::prelude::*;
use leptos_router::components::Form;

#[component]
pub fn GpxSubmitForm() -> impl IntoView {
    view! {
        <Form method="POST" action="">
            <p>"This is the upload form yo"</p>
        </Form>
    }
}
