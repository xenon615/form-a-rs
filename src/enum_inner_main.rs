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

#[derive(Deserialize, Clone)]
struct FormA {
    def: FormDef
}

#[derive(Deserialize, Clone)]
struct FormDef {
    #[serde(default)]
    title: String,
    #[serde(rename = "remoteSubmit")]
    remote_submit: bool,
    fields: Vec<FieldA>,
    buttons: Vec<Button>
}

#[derive(Deserialize, Clone)]
struct Button {
    text: String,
    classes: Vec<String>,
    #[serde(rename = "type")]
    btype: String
}
// #[derive(Deserialize, Clone, Debug)]
// struct CommonProps {
//     name: String,
//     #[serde(default)]
//     label: String,
//     #[serde(default)]
//     classes: Vec<String>,
//     #[serde(default)]
//     default: String
// }

// #[derive(Deserialize, Clone, Debug)]
// struct TextLike {
//     #[serde(flatten)]
//     common: CommonProps
// }

// #[derive(Deserialize, Clone, Debug)]
// struct ContainerLike {
//     fields: Vec<FieldA>,
//     #[serde(flatten)]
//     common: CommonProps
// }

#[derive(Deserialize, Clone, Debug)]
struct Opt {
    value: String,
    label: String
}

// #[derive(Deserialize, Clone, Debug)]
// struct ChoiceLike {
//     options: Vec<Opt>,
//     #[serde(flatten)]
//     common: CommonProps
// }


// #[derive(Deserialize, Clone, Debug)]
// #[serde(tag = "type")]
// enum FieldA {
//     #[serde(rename = "text")]
//     Text(TextLike),

//     #[serde(rename = "group")]
//     Group(ContainerLike),

//     #[serde(rename = "select")]
//     Select(ChoiceLike)
// }


#[derive(Deserialize, Clone, Debug)]
struct ContainerLike {
    #[serde(rename = "type")]
    ftype: String,

    fields: Vec<FieldA>,
}

#[derive(Deserialize, Clone, Debug)]
struct ChoiceLike {
    options: Vec<Opt>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum SpecificFields {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "group")]
    Group(ContainerLike),
    #[serde(rename = "select")]
    Select(ChoiceLike)
}

#[derive(Deserialize, Clone, Debug)]
struct CommonFields {
            // #[serde(rename = "type")]
            // ftype: String,
            #[serde(default)]
            name: String,
            #[serde(default)]
            label: String,
            // #[serde(default)]
            // classes: Vec<String>,
            #[serde(default)]
            default: String,
}

// #[derive(Deserialize, Clone, Debug)]
// struct FieldA {
//         name: String,
//         #[serde(default)]
//         label: String,
//         #[serde(default)]
//         classes: Vec<String>,
//         #[serde(default)]
//         default: String,
//         #[serde(flatten)]
//         specific: SpecificFields
// }

#[derive(Deserialize, Clone, Debug)]
struct FieldA {
    // #[serde(rename = "type")]
    // ftype: String,
    #[serde(default)]
    classes: Vec<String>,
    #[serde(flatten)]
    common: CommonFields,
    #[serde(flatten)]
    specific: SpecificFields
}

// ---

async fn get_form() -> Result<FormA, Error> {
    let t = Request::get("http://localhost:3000/")
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

// ---

#[component]
fn AForm(form: FormA) -> impl IntoView {
    view! {
        <div class="form-wrap">
            {
                if !form.def.title.is_empty() {
                    view! {
                        <div class="title">{form.def.title}</div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }
            <div class="fields-wrap">
                <Fields fields = form.def.fields path="form".to_string()/>
            </div>

            <div class="buttons">
                {
                    form.def.buttons.into_iter().map(| b | view! {
                        <button type = {b.btype} class = {b.classes.join(" ")}  >{b.text}</button>
                    }).collect_view()
                }
            </div>

        </div>
    }
}

// ---

#[component]
fn Fields(fields: Vec<FieldA>, path: String) -> impl IntoView {
    view! {
        {
            fields.into_iter().map(| f | {
                view! {
                    <Field field = f path=path.clone()/>
                }
            }).collect_view()
        }
    }
}

#[component]
fn Field(field: FieldA, path: String) -> impl IntoView {
    view! {
        // <div class={format!("field-container {} {}", field.specific.f, field.classes.join(" ")) }> >
        {
            match &field.specific {
                SpecificFields::Text => view! {
                    <FText field = field.common path=path />
                }.into_any(),
                SpecificFields::Group(c) => view! {
                    <FGroup field=field.common specific=c.clone() path=path />
                }.into_any(),
                SpecificFields::Select(s) => view! {
                    <FSelect field=field.common specific=s.clone() path=path />
                }.into_any(),

                _ => view! {
                    <div>Not implemented yet</div>
                }.into_any()
            }
        }
        // </div>
    }
}


#[component]
fn FText(field: CommonFields, path: String) -> impl IntoView {
    let id = format!("_{}_{}",path, field.name);
    view! {
        // <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            <input id={id} name={field.name} type="text"  value={field.default}/>
        // </div>
    }
}


#[component]
fn FGroup(field: CommonFields, specific: ContainerLike,    path: String) -> impl IntoView {
    let new_path = format!("{}_{}",path, field.name);
    view! {
        // <div class={format!("group {}", field.classes.join(" "))}>
            <label for={new_path.clone()}>{field.label}</label>
            <Fields fields= specific.fields path=new_path/>
        // </div>
    }
}


#[component]
fn FSelect(field: CommonFields, specific: ChoiceLike, path: String) -> impl IntoView {
    let id = format!("_{}_{}",path, field.name);
    view! {
        // <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            <select id={id} name={field.name}>
                {
                    specific.options.into_iter().map(| o | view! {
                        <option value={o.value}>{o.label}</option>
                    }).collect_view()
                }
            </select>
        // </div>
    }
}


// #[component]
// fn FText(field: FieldA, path: String) -> impl IntoView {
//     let id = format!("_{}_{}",path, field.name);
//     view! {
//         <div class={format!("field-container {}", field.classes.join(" "))}>
//             <label for={id.clone()}>{field.label}</label>
//             <input id={id} name={field.name} type="text"  value={field.default}/>
//         </div>
//     }
// }

// #[component]
// fn FGroup(field: FieldA,   path: String) -> impl IntoView {
//     let new_path = format!("{}_{}",path, field.name);
//     view! {
//         <div class={format!("group {}", field.classes.join(" "))}>
//             <label for={new_path.clone()}>{field.label}</label>
//             // <Fields fields= specific.fields path=new_path/>
//         </div>
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
