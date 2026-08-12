use std::path;

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

async fn get_form(path: String) -> Result<FormA, Error> {
    let t = Request::get(path.as_str())
        .send().await?.json::<FormA>().await?;
    Ok(t)
}

// ---

#[component]
fn App() -> impl  IntoView {
    let (form_index, set_form_index) = signal(0);
    let form_lr = LocalResource::new(move | | get_form(format!("/forms/form-{}.json",form_index.get())));
    view!{
        <FormSelector form_index = set_form_index />
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

// ---

#[component]
fn FormSelector(form_index: WriteSignal<usize>) -> impl IntoView {
    view! {
        <div class="form-selector">
            <div class="field-wrap">
                <label for ="form_select">Form </label>
                <select class="field-input"
                    id="form_select"
                    on:change = move |evt|  form_index.set(event_target_value(&evt).parse::<usize>().unwrap())
                >
                    {
                        (0..5).into_iter().map(|i| view! {
                            <option value={i}>{format!("Form - {}", i)}</option>
                        }).collect_view()
                    }

                </select>
            </div>
        </div>
    }
}
