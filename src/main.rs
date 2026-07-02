use leptos::{leptos_dom::logging::console_log, prelude::*, svg::view};
use serde::Deserialize;
use serde_json::{Value, json};
use wasm_bindgen::JsCast;
use gloo_net::http::Request;
mod functions;
use functions::{update_data, delete_data, is_show};

use crate::functions::get_data;

fn main() {
    console_error_panic_hook::set_once();
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

    #[allow(dead_code)]
    remote_submit: bool,
    fields: Vec<FieldA>,
    buttons: Vec<Button>
}

#[derive(Deserialize, Clone)]
struct Button {
    text: String,
    classes: Vec<String>,
    #[serde(rename = "type")]
    btype: String,
    #[serde(default)]
    action: String
}

#[derive(Deserialize, Clone, Debug)]
struct Opt {
    value: String,
    label: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct OptionsLike {
    options: Vec<Opt>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ObjectLike {
    fields: Vec<FieldA>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TextareaLike {
    #[serde(default)]
    rows: u8
}


#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]

pub enum SpecificFields {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "textarea")]
    Textarea(TextareaLike),
    #[serde(rename = "select")]
    Select(OptionsLike),

    #[serde(rename = "radio")]
    Radio(OptionsLike),

    #[serde(rename = "checkbox")]
    CheckBox(OptionsLike),
    #[serde(rename = "true-false")]
    TrueFalse,

    #[serde(rename = "group")]
    Group(ObjectLike),

    #[serde(rename = "repeater")]
    Repeater(ObjectLike),

    #[serde(rename = "table")]
    Table(ObjectLike)

}

#[derive(Deserialize, Debug, Clone)]
pub enum Relation {
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "and")]
    And
}

#[derive(Deserialize, Debug, Clone)]
pub enum Compare {
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = "!=")]
    MotEq,
    #[serde(rename = ">")]
    More,
    #[serde(rename = "<")]
    Less
}

#[derive(Deserialize, Debug, Clone)]
pub struct Logic {
    path: String,
    value: Value,
    // compare: String,
    compare: Compare,
    relation: Relation
}

#[derive(Deserialize, Clone, Debug)]
pub struct FieldA {
        name: String,
        #[serde(default)]
        label: String,
        #[serde(default)]
        classes: Vec<String>,
        #[serde(default)]
        default: Value,
        #[serde(rename = "cLogic")]
        #[serde(default)]
        c_logic: Vec<Logic>,
        #[serde(flatten)]
        specific: SpecificFields
}

impl FieldA {
    fn empty_value(&self) -> Value {
        match self.specific {
            SpecificFields::Group(_) => json!({}),
            _ => json!(())
        }
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
    provide_context(set_data);
    provide_context(data);
    provide_context(form.def.fields.clone());
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
                <Fields fields = form.def.fields path="".to_string() data=memo/>
            </div>

            <div class="buttons">
                {
                    form.def.buttons.into_iter().map(| b | view! {
                        <button
                            type = {b.btype}
                            class = {b.classes.join(" ")}
                            on:click = move |_| submit(b.action.as_str())
                        >
                            {b.text}
                        </button>
                    }).collect_view()
                }
            </div>
            <Jachc/>
            <Pretty data/>

        </div>
    }
}

// ---

fn submit(action:&str) {
    update_data("__action".to_string(), action.into());
    console_log(action);
}

// ---

#[component]
fn Pretty (data: ReadSignal<Value>) -> impl IntoView {
    view! {
        <div style="margin: 10px">
            { move || data.with(| d |  serde_json::to_string_pretty(d).unwrap_or_default()) }
        </div>
    }
}

// ---

#[component]
fn Jachc () -> impl IntoView {

    view! {
        <button
            class="primary"
            on:click= move |_| update_data("sex".into(), "m".into())
            // on:click = move |_| {
            //     let path = vec!["stuff","backup","where"];
            //     let f = get_field(&path[0..1]);
            //     console_log(&format!("{:?}", f.empty_value()));
            // }

            // on:click = move |_| {
            //     let path = vec!["stuff","backup","where"];
            //     delete_data(&path[0..2]);
            // }


        >
            CLICK ME
        </button>
    }
}

// ---

#[component]
fn Fields(fields: Vec<FieldA>, path: String, data: Memo<Value>) -> impl IntoView {
    view! {
        <For
            each = move || {
                fields.clone().into_iter().filter(|f| is_show(&f)).collect::<Vec<_>>()
            }
            key = move | f | f.name.clone()
            let(field)
        >
            {
                let name = field.name.clone();
                let default = field.default.clone();

                let path = if path.is_empty() {name.clone()} else {format!("{}--{}", path, name)};

                let path2 = path.clone();   //??????
                let fd = Memo::new(
                    move |_|  {
                        data.with(|d| {
                            // console_log(&name);
                                match d.get(name.clone()) {
                                    Some(v) => v.clone(),
                                    None => {
                                        if !default.is_null() {
                                            update_data(path2.clone(), default.clone());
                                        }
                                        Value::Null
                                    }
                                }
                            }
                        )
                    }
                );
                // console_log(&format!("{:?}", fd.get()));

               view! {
                   <Field field path  data=fd/>
               }
            }
        </For>
    }
}

// ---

#[component]
fn Field(field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {

    view! {
            <div>
        {
            match &field.specific {
                SpecificFields::Text => view! {
                    <FText field = field.clone() path data />
                }.into_any(),
                SpecificFields::Group(g) => view! {
                    <FGroup field=field.clone() specific=g.clone() path data/>
                }.into_any(),

                SpecificFields::Select(c) => view! {
                    <FSelect field=field.clone() specific=c.clone() path data/>
                }.into_any(),

                SpecificFields::Radio(c) => view! {
                    <FRadio field=field.clone() specific=c.clone() path data/>
                }.into_any(),

                SpecificFields::Textarea(t) => view! {
                    <FTextarea field=field.clone() specific=t.clone() path  data/>
                }.into_any(),

                SpecificFields::TrueFalse => view! {
                    <FTrueFalse field=field.clone() path  data/>
                }.into_any(),

                SpecificFields::CheckBox(c) => view! {
                    <FCheckBox field=field.clone() specific=c.clone() path  data/>
                }.into_any(),

                SpecificFields::Repeater(r) => view! {
                    <FRepeater field=field.clone() specific=r.clone() path data/>
                }.into_any(),


                _ => view! {
                    <div>Not implemented yet</div>
                }.into_any()
            }

        }
            </div>
    }
}


#[component]
fn FText(field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let clean_val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            <input
                id={id}
                type="text"
                value=clean_val
                on:input=move |evt| update_data(path.clone().into(), event_target_value(&evt).into())
            />
        </div>
    }
}

// ---

#[component]
fn FSelect(field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            <select
                id={id}
                prop:value={val}
                on:change = move |evt|  update_data(path.clone().into(), event_target_value(&evt).into())
            >
                {
                    specific.options.into_iter().map(| o | view! {
                        <option value={o.value} >
                            {o.label}
                        </option>
                    }).collect_view()
                }
            </select>
        </div>
    }
}

// ---

#[component]
fn FRadio(field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            {
                specific.options.into_iter().map(| e | {
                    let spare_value = e.value.clone();
                    let spare_path = path.clone();
                    view! {
                    <label>{e.label}
                        <input
                            type="radio"
                            value= {e.value.clone()}
                            checked = move || data.get() == e.value.clone()
                            on:change= move |_| update_data( spare_path.clone(), spare_value.clone().into())
                        />
                    </label>
                }}).collect_view()

            }
        </div>
    }
}

// ---

#[component]
fn FTextarea(field: FieldA, specific: TextareaLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let clean_val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            <textarea
                id={id}
                rows = move | | (specific.rows != 0).then_some(specific.rows)

                on:input=move |evt| update_data(path.clone().into(), event_target_value(&evt).into())
            >
                {clean_val}
            </textarea>
        </div>
    }
}

// ---

#[component]
fn FTrueFalse(field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);

    let toggle = move | ev | {
        let checked = event_target_checked(&ev);
        update_data(path.clone(), checked.into());
    };

    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            <input
                type ="checkbox"
                checked = move | | data.get().as_bool()
                on:change= toggle
                />

        </div>
    }
}

// ---

#[component]
fn FGroup(field: FieldA, specific: ObjectLike, path: String, data: Memo<Value>) -> impl IntoView {
    view! {
        <div class={format!("group {}", field.classes.join(" "))}>
            <label for={path.clone()}>{field.label}</label>
            <Fields fields= specific.fields path data/>
        </div>
    }
}

// ---

#[component]
fn FCheckBox(field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    view! {
        <div class={format!("checkbox {}", field.classes.join(" "))}>
            <label for={id.clone()}>{field.label}</label>
            {
                specific.options.into_iter().map(| e | {
                    let spare_value = e.value.clone();
                    let spare_path = path.clone();
                    view! {
                    <label>{e.label}
                        <input
                            type="checkbox"
                            value= {e.value.clone()}
                            checked = move || data.get() == e.value.clone()
                            on:change= move |_| update_data( spare_path.clone(), spare_value.clone().into())
                        />
                    </label>
                }}).collect_view()
            }
        </div>
    }
}

// ---

// #[component]
// fn FRepeater(field: FieldA, specific: ObjectLike, path: String, data: Memo<Value>) -> impl IntoView {
//     let each = move || serde_json::from_value::<Vec<Value>>(data.get()).unwrap_or(vec![]);
//     view! {
//         <div class={format!("field-container {}", field.classes.join(" "))}>
//             <label for={path.clone()}>{field.label}</label>
//             <ForEnumerate
//                 each = move || each().clone()
//                 key = | e | e.clone()
//                 children = move |idx, row | {
//                     let fp = format!("{}--{}", path.clone(), field.name);
//                     let fd = Memo::new(move |_|  row.clone() );
//                     let data_path = format!("{}--{}", path, idx.get());

//                     view! {

//                         <div>
//                             <Fields fields= specific.fields.clone() path = fp  data = fd />
//                         </div>
//                         <div class="controls">
//                             <span
//                                 on:click = move |_| delete_data(data_path.clone())
//                             >
//                             x  {idx.get()}
//                             </span>
//                         </div>
//                     }
//                 }
//             />
//         </div>
//     }
// }


#[component]
fn FRepeater(field: FieldA, specific: ObjectLike, path: String, data: Memo<Value>) -> impl IntoView {
    let each = move || serde_json::from_value::<Vec<Value>>(data.get()).unwrap_or(vec![]);
    view! {
        <div class={format!("field-container {}", field.classes.join(" "))}>
            <label for={path.clone()}>{field.label}</label>

            {
                move || each().into_iter().enumerate().map(| (idx, row) | {
                    let fp = format!("{}--{}", path.clone(), idx);
                    let fd = Memo::new(move |_|  row.clone() );
                    let data_path = format!("{}--{}", path, idx);
                    view! {
                        <div>
                            <Fields fields= specific.fields.clone() path = fp  data = fd />
                        </div>
                        <div class="controls">
                            <span
                                on:click = move |_| delete_data(data_path.clone())
                            >
                            x  {idx}
                            </span>
                        </div>

                    }
                }).collect_view()
            }
        </div>
    }
}
