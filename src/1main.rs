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

struct CommonProps {
    name: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    classes: Vec<String>
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
enum FieldA {
    #[serde(rename = "text")]
    Text {
        #[serde(flatten)]
        common: CommonProps
    },
    #[serde(rename = "group")]
    Group {
        name: String,
        label: String,
        #[serde(default)]
        classes: Vec<String>,
        fields: Vec<FieldA>
    }
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
        <Suspense
            fallback = move | | view! {<i>"Loading..."</i>}
        >
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
                <Fields fields = form.def.fields/>
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
fn Fields(fields: Vec<FieldA>) -> impl IntoView {
    view! {
        {
            fields.into_iter().map(| f | {
                view! {
                    <Field field = f/>
                }
            }).collect_view()
        }
    }
}

#[component]
fn Field(field: FieldA) -> impl IntoView{
    view! {
        <div class="field-wrap">
            {

                match field {
                    FieldA::Text { name, label, classes } => view! {
                        <input type="text"/>
                    }.into_any(),
                    FieldA::Group { name, label, classes, fields } => view! {
                        "B"
                    }.into_any()
                }

            }
        </div>
    }
}

#[component]
fn FText(name: String, label: String, classes: Vec<String>) -> impl IntoView {

}
