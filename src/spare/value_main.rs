use leptos::{attr::{r#async, list}, form, leptos_dom::logging::console_log, prelude::*, svg::view};
use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::JsCast;
use gloo_net::http::Request;

fn main() {
    mount_to(
        document().get_element_by_id("app").unwrap().unchecked_into(),
        App
    )
    .forget();
}

// ---

async fn get_form() -> Result<Value, Error> {
    let t = Request::get("http://localhost:3000/")
        .send().await?.json::<Value>().await?;
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

// ---

#[component]
fn AForm(form: Value) -> impl IntoView {
    let def = form["def"].clone();
    let title = def.get("title").unwrap();
    let buttons = def["buttons"].as_array().unwrap();

    view! {
        <div class="form-wrap">
            {
                if title  !="" {
                    view! {
                        <div class="title">{title.as_str()}</div>
                    }.into_any()
                } else {
                    ().into_any()
                }

            }


            <div class="buttons">
                {
                    buttons.into_iter().map(| b | {
                        let classes = b["classes"].clone().as_array().unwrap().na;
                        view! {
                            <button type = {b["btype"].clone().to_string()} class = {classes}>
                                {b["text"].clone().to_string()}
                            </button>
                        }
                    }).collect_view()
                }
            </div>
        </div>
    }
}

// ---

// #[component]
// fn Fields(fields: Vec<FieldA>, path: String) -> impl IntoView {
//     view! {
//         {
//             fields.into_iter().map(| f | {
//                 view! {
//                     <Field field = f path=path.clone()/>
//                 }
//             }).collect_view()
//         }
//     }
// }

// #[component]
// fn Field(field: FieldA, path: String) -> impl IntoView{
//     view! {
//         {
//             match field {
//                 FieldA::Text (f) => view! {
//                     <FText props=f path=path/>
//                 }.into_any(),
//                 FieldA::Group (g) => view! {
//                     <FGroup props=g path=path/>
//                 }.into_any(),
//                 FieldA::Select(s) => view! {
//                     <FSelect props=s path=path/>
//                 }.into_any()
//             }
//         }
//     }
// }

// #[component]
// fn FText(props: TextLike, path: String) -> impl IntoView {
//     let id = format!("_{}_{}",path, props.common.name);
//     view! {
//         <div class={format!("field-container {}", props.common.classes.join(" "))}>
//             <label for={id.clone()}>{props.common.label}</label>
//             <input id={id} name={props.common.name} type="text"  value={props.common.default}/>
//         </div>
//     }
// }

// #[component]
// fn FSelect(props: ChoiceLike, path: String) -> impl IntoView {
//     let id = format!("_{}_{}",path, props.common.name);
//     view! {
//         <div class={format!("field-container {}", props.common.classes.join(" "))}>
//             <label for={id.clone()}>{props.common.label}</label>
//             <select id={id} name={props.common.name}>
//                 {
//                     props.options.into_iter().map(| o | view! {
//                         <option value={o.value}>{o.label}</option>
//                     }).collect_view()
//                 }
//             </select>
//         </div>
//     }
// }


// #[component]
// fn FGroup(props: ContainerLike, path: String) -> impl IntoView {
//     let new_path = format!("{}_{}",path, props.common.name);
//     view! {
//         <div class={format!("group {}", props.common.classes.join(" "))}>
//             <label for={new_path.clone()}>{props.common.label}</label>
//             <Fields fields= props.fields path=new_path/>
//         </div>
//     }
// }
