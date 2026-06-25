use std::collections::HashMap;

use leptos::{
    // leptos_dom::logging::console_log,
    // leptos_dom::logging::console_debug_log,
    attr::r#for, leptos_dom::logging::console_log, prelude::*, task::spawn_local
};
use wasm_bindgen::JsCast;
use gloo_net::http::*;
use serde::{Deserialize};
use serde_json::Value;


// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum StringOrArray {
//     Single(String),
//     Nested(Vec<HashMap<String, StringOrArray>>)
// }



// fn main() {
//     console_error_panic_hook::set_once();
//     let h = mount_to(
//         document().get_element_by_id("app").unwrap().unchecked_into()
//         , App);
//     h.forget();
//     spawn_local(async {
//         let resp = Request::get("http://localhost:3000/").send().await.unwrap();
//         let def: HashMap<String, StringOrArray> = resp.json().await.unwrap();
//         for (key, value) in &def {
//             let f = match value {
//                 StringOrArray::Single(_s) => format!("{}: single", key),
//                 StringOrArray::Nested(_n) => format!("{}: vec", key )
//             };
//             console_log(f.as_str());
//         }
//     });
// }

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum StringOrArray {
//     Single(String),
//     Nested(Vec<HashMap<String, StringOrArray>>)
// }



// fn main() {
//     console_error_panic_hook::set_once();
//     let h = mount_to(
//         document().get_element_by_id("app").unwrap().unchecked_into()
//         , App);
//     h.forget();
//     spawn_local(async {
//         let resp = Request::get("http://localhost:3000/").send().await.unwrap();
//         let def: HashMap<String, StringOrArray> = resp.json().await.unwrap();
//         for (key, value) in &def {
//             let f = match value {
//                 StringOrArray::Single(_s) => format!("{}: single", key),
//                 StringOrArray::Nested(_n) => format!("{}: vec", key )
//             };
//             console_log(f.as_str());
//         }
//     });
// }

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum StringOrArray {
//     Single(String),
//     Nested(Vec<HashMap<String, StringOrArray>>)
// }


fn main() {
    console_error_panic_hook::set_once();
    let h = mount_to(
        document().get_element_by_id("app").unwrap().unchecked_into()
        , App);
    h.forget();
}
#[derive(Deserialize)]
struct Cat {
    id: String,
    url: String,
    width: usize,
    height: usize
}

// async fn fetch_cats(count: usize) -> Result<Vec<String>, Error> {
//         let res = reqwasm::http::Request::get(&format!(
//             "https://api.thecatapi.com/v1/images/search?limit={count}",
//         ))
//         .send()
//         .await?
//         .json::<Vec<Cat>>()
//         .await?
//         .into_iter()
//         .take(count)
//         .map(|cat| cat.url)
//         .collect::<Vec<_>>();
//         Ok(res)
// }

async fn fetch_cats(count: usize) -> Result<Vec<String>, Error> {
        let res = Request::get(&format!(
            "https://api.thecatapi.com/v1/images/search?limit={count}",
        ))
        .send()
        .await?
        .json::<Vec<Cat>>()
        .await?
        .into_iter()
        // .take(count)
        .map(|cat| cat.url)
        .collect::<Vec<_>>();
        Ok(res)
}

// #[component]
// pub fn LocalDataViewer() -> impl IntoView {
//     // 1. Create a reactive source (e.g., a signal)
//     let (count, set_count) = signal(0);

//     // 2. Create the LocalResource
//     // It will run the async 'fetcher' whenever 'count' changes.
//     let async_data = LocalResource::new(move || {
//         let current_count = count.get();
//         async move {
//             // Simulate a local async task, like a browser API call
//             TimeoutFuture::new(1_000).await;
//             format!("Data for count: {}", current_count)
//         }
//     });

//     view! {
//         <button on:click=move |_| set_count.update(|n| *n += 1)>
//             "Increment: " {count}
//         </button>

//         // 3. Use Suspense to handle the loading state
//         <Suspense fallback=move || view! { <p>"Loading..."</p> }>
//             <p>
//                 {move || async_data.get()}
//             </p>
//         </Suspense>
//     }
// }

#[component]
fn App() -> impl IntoView {

    // let form_def  = LocalResource::new(| | async move {
    //     let resp = Request::get("http://localhost:3000/").send().await.unwrap();
    //     resp.json().await.unwrap()
    // });
    let  cats = LocalResource::new(move | | fetch_cats(10));

    let fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect::<Vec<_>>()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };


    view! {

        <Await
            future = fetch_cats(10)
            let:data
        >

                <h1>AAAA</h1>

        </Await>
    }

    // console_log(format!("{:?}", fd()).as_str());

    // spawn_local(async {
    //     let resp = Request::get("http://localhost:3000/").send().await.unwrap();
    //     let form_o: Value = resp.json().await.unwrap();
    //     let form = form_o.as_object().unwrap();

    // });
    // let def = form["def"].as_object().u
    // view! {
    //     <Transition fallback=|| view! { <div>"Loading..."</div> }>
    //          <ErrorBoundary fallback>
    //              <ul>
    //                  {move || Suspend::new(async move {
    //                      cats.await
    //                          .map(|cats| {
    //                              cats.iter()
    //                                  .map(|s| {
    //                                      view! {
    //                                          <li>
    //                                              <img src=s.clone() />
    //                                          </li>
    //                                      }
    //                                  })
    //                                  .collect::<Vec<_>>()
    //                          })
    //                  })}

    //              </ul>
    //          </ErrorBoundary>
    //      </Transition>
    // }


}

// #[component]
// fn CatView(cats: Vec<String>) -> impl IntoView {

//     view!{
//         <ul>
//             {
//                 cats
//             }
//         </ul>
//     }
// }

// #[component]
// fn FormView(def: Value) -> impl IntoView {
//     console_log(format!("{:?}", def).as_str());


// }
