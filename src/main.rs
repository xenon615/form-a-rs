use leptos::prelude::*;
use wasm_bindgen::JsCast;
use gloo_net::http::Request;

mod data;
mod fields;
mod form;
use form::{FormA, AForm};


fn main() {
    console_error_panic_hook::set_once();
    mount_to(
        document().get_element_by_id("app").unwrap().unchecked_into(),
        App
    )
    .forget();
}

// ---

// async fn get_form() -> Result<FormA, Error> {
//     let t = Request::get("http://localhost:3000/")
//         .send().await?.json::<FormA>().await?;
//     Ok(t)
// }
    // TimeoutFuture::new(1_000).await;

async fn get_form() -> Result<FormA, Error> {
    let t = Request::get("/files/def4.json")
        .send().await?.json::<FormA>().await?;
    Ok(t)
}

// ---

#[component]
fn App() -> impl  IntoView {
    let form_lr = LocalResource::new(move | | get_form());
    view!{
        <Suspense fallback = move | | view! {<i>"Loading..."</i>} >
            {
                move | | Suspend::new( async move {
                    match form_lr.await {
                        Ok(form) => view! {<AForm form = form/>}.into_any(),
                        Err(e) => view! {<span>{format!("{:?}", e)}</span>}.into_any()
                    }
                })
            }
        </Suspense>
    }
}
