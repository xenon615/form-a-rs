use std::collections::HashMap;

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
// #[derive(Deserialize, Clone, Debug)]
// #[serde(untagged)]
// enum FormDataValue {
//     Text(String),
//     Number(i32),
//     Array(Vec<FormDataValue>),
//     Map(FormData)
// }


// type FormData = HashMap<String, FormDataValue>;

#[derive(Deserialize, Clone)]
struct FormA {
    def: FormDef,
    // data: FormData
    data: Value
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
#[derive(Deserialize, Clone, Debug)]
struct CommonProps {
    name: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    classes: Vec<String>,
    #[serde(default)]
    default: String
}

#[derive(Deserialize, Clone, Debug)]
struct TextLike {
    #[serde(flatten)]
    common: CommonProps
}

#[derive(Deserialize, Clone, Debug)]
struct ContainerLike {
    fields: Vec<FieldA>,
    #[serde(flatten)]
    common: CommonProps
}

#[derive(Deserialize, Clone, Debug)]
struct Opt {
    value: String,
    label: String
}

#[derive(Deserialize, Clone, Debug)]
struct ChoiceLike {
    options: Vec<Opt>,
    #[serde(flatten)]
    common: CommonProps
}


#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum FieldA {
    #[serde(rename = "text")]
    Text(TextLike),

    #[serde(rename = "group")]
    Group(ContainerLike),

    #[serde(rename = "select")]
    Select(ChoiceLike)
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
    console_log(format!("{:?}", form.data).as_str());
    let (data, set_data) = signal(form.data);
    provide_context(data);
    provide_context(set_data);

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
fn Field(field: FieldA, path: String) -> impl IntoView{

    view! {
        {
            match field {
                FieldA::Text (f) => view! {
                    <FText props=f path=path/>
                }.into_any(),
                FieldA::Group (g) => view! {
                    <FGroup props=g path=path/>
                }.into_any(),
                FieldA::Select(s) => view! {
                    <FSelect props=s path=path/>
                }.into_any()
            }
        }
    }
}

#[component]
fn FText(props: TextLike, path: String) -> impl IntoView {

    let path  = format!("{}--{}",path, props.common.name);
    console_log(format!("path {}", path).as_str());



    let getter = use_context::<ReadSignal<Value>>().expect("getter error");

        let val = getter.with(| u |  value[])

    console_log(format!("{:?}", getter).as_str());

    view! {
        <div class={format!("field-container {}", props.common.classes.join(" "))}>
            <label for={path.clone()}>{props.common.label}</label>
            // <input id={id} name={props.common.name} type="text"  value={props.common.default}/>
            <input  type="text"        />
        </div>
    }
}

#[component]
fn FSelect(props: ChoiceLike, path: String) -> impl IntoView {
    let path  = format!("{}--{}",path, props.common.name);
    view! {
        <div class={format!("field-container {}", props.common.classes.join(" "))}>
            <label for={path.clone()}>{props.common.label}</label>
            <select id={path} name={props.common.name}>
                {
                    props.options.into_iter().map(| o | view! {
                        <option value={o.value}>{o.label}</option>
                    }).collect_view()
                }
            </select>
        </div>
    }
}


#[component]
fn FGroup(props: ContainerLike, path: String) -> impl IntoView {
    let path  = format!("{}--{}",path, props.common.name);
    view! {
        <div class={format!("group {}", props.common.classes.join(" "))}>
            <label for={path.clone()}>{props.common.label}</label>
            <Fields fields= props.fields path=path/>
        </div>
    }
}
