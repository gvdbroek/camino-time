// use leptos::html::Form;
use leptos::prelude::*;
use leptos_router::components::Form;
use serde::{Deserialize, Serialize};

use server_fn::codec::{
    MultipartData, MultipartFormData
};
use leptos::web_sys::{FormData, HtmlFormElement, SubmitEvent};
// guided using https://github.com/leptos-rs/leptos/blob/main/examples/server_fns_axum/src/app.rs#L333


#[component]
pub fn GpxSubmitForm() -> impl IntoView{


}


#[server(
        input = MultipartFormData,
    )]
pub async fn file_length(data: MultipartData)-> Result<usize, ServerFnError> {
    let mut data = data.into_inner().unwrap();

        // this will just measure the total number of bytes uploaded
        let mut count = 0;
        while let Ok(Some(mut field)) = data.next_field().await {
            println!("\n[NEXT FIELD]\n");
            let name = field.name().unwrap_or_default().to_string();
            println!("  [NAME] {name}");
            while let Ok(Some(chunk)) = field.chunk().await {
                let len = chunk.len();
                count += len;
                println!("      [CHUNK] {len}");
                // in a real server function, you'd do something like saving the file here
            }
        }

        Ok(count)

}

#[component]
fn FileUpload() -> impl IntoView{


    // let upload_action = Action::new_local(|data: &FormData| {
    //     // `MultipartData` implements `From<FormData>`
    //     file_length(data.clone().into())
    // });
    //
    // view! {
    //     <h3>File Upload</h3>
    //     <p>Uploading files is fairly easy using multipart form data.</p>
    //
    //     <form on:submit=move |ev: SubmitEvent| {
    //         ev.prevent_default();
    //         let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
    //         let form_data = FormData::new_with_form(&target).unwrap();
    //         upload_action.dispatch_local(form_data);
    //     }>
    //         <input type="file" name="file_to_upload" />
    //         <input type="submit" />
    //     </form>
    //     <p>
    //         {move || {
    //             if upload_action.input().read().is_none() && upload_action.value().read().is_none()
    //             {
    //                 "Upload a file.".to_string()
    //             } else if upload_action.pending().get() {
    //                 "Uploading...".to_string()
    //             } else if let Some(Ok(value)) = upload_action.value().get() {
    //                 value.to_string()
    //             } else {
    //                 format!("{:?}", upload_action.value().get())
    //             }
    //         }}
    //
    //     </p>
    // }
}
