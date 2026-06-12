use leptos::{attr::{r#async, list}, form, html::div, leptos_dom::logging::console_log, prelude::*, svg::view};
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
    def: FormDef,
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
struct ChoiceLike {
    options: Vec<Opt>,
}

#[derive(Deserialize, Clone, Debug)]
struct ContainerLike {
    fields: Vec<FieldA>,
}


#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
enum SpecificFields {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "select")]
    Select(ChoiceLike),
    #[serde(rename = "group")]
    Group(ContainerLike)
}

#[derive(Deserialize, Clone, Debug)]
struct FieldA {
        name: String,
        #[serde(default)]
        label: String,
        #[serde(default)]
        classes: Vec<String>,
        #[serde(default)]
        default: Value,
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
    let (data, set_data) = signal(form.data);
    let memo = Memo::new( move | _ | data.get());
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
                <Fields fields = form.def.fields path="form".to_string() data=memo/>
            </div>

            <div class="buttons">
                {
                    form.def.buttons.into_iter().map(| b | view! {
                        <button type = {b.btype} class = {b.classes.join(" ")}  >{b.text}</button>
                    }).collect_view()
                }
            </div>

            <button
                on:click= move |_| update_data(set_data, "stuff--name".into(), "ASD!!".into())
            >
                CLICK ME
            </button>

        </div>
    }
}

// ---

fn update_data(w: WriteSignal<Value>, path: String, value: Value) {
    w.update(| p |  {
        let mut f = p;
        for pe in path.split("--") {
            f = f.get_mut(pe).unwrap();
        }
        *f = value
    })
}

// ---

#[component]
fn Fields(fields: Vec<FieldA>, path: String, data:Memo<Value>) -> impl IntoView {
    view! {
        {
            fields.into_iter().map(| f | {
                let name = f.name.clone();
                let path = format!("{}--{}", path, name);
                let fd = Memo::new( move |_| data.get()[name.clone()].clone() );
                view! {
                    <Field field = f path=path data = fd/>
                }
            }).collect_view()
        }
    }
}

#[component]
fn Field(field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {

    view! {

        {
            match &field.specific {
                SpecificFields::Text => view! {
                    <FText field = field.clone() path data />
                }.into_any(),
                SpecificFields::Group(s) => view! {
                    <FGroup field=field.clone() specific=s.clone() path  data/>
                }.into_any(),

                SpecificFields::Select(s) => view! {
                    <FSelect field=field.clone() specific=s.clone() path=path />
                }.into_any(),

                _ => view! {
                    <div>Not implemented yet</div>
                }.into_any()
            }
        }

    }
}


#[component]
fn FText(field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    // console_log(format!("{}", data).as_str());

    let clean_val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            // <input id={id} name={field.name} type="text"  value={field.default}/>
            <input id={id} name={field.name} type="text" value=clean_val/>
        </div>
    }


}




#[component]
fn FSelect(field: FieldA, specific: ChoiceLike, path: String) -> impl IntoView {
    let id = format!("_{}",path);
    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            <select id={id} name={field.name}>
                {
                    specific.options.into_iter().map(| o | view! {
                        <option value={o.value}>{o.label}</option>
                    }).collect_view()
                }
            </select>
        </div>
    }
}

#[component]
fn FGroup(field: FieldA, specific: ContainerLike, path: String, data: Memo<Value>) -> impl IntoView {
    let path  = format!("{}--{}",path, field.name);
    view! {
        <div class={format!("group {}", field.classes.join(" "))}>
            <label for={path.clone()}>{field.label}</label>
            <Fields fields= specific.fields path=path data/>
        </div>
    }
}
