use std::clone;

use leptos::{attr::value, html::select, leptos_dom::logging::console_log, prelude::*, svg::view, tachys::view::Position};
use serde::Deserialize;
use serde_json::{Value, json};
// use uuid::uuid;
use wasm_bindgen::JsCast;
use gloo_net::http::Request;
mod functions;
use functions::{update_data, delete_data, is_show};

use crate::functions::{get_data, set_ids};

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

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(untagged)]
enum Label {
     #[default]
    Empty,
    Scalar(String),
    Object {
        text: String,
        position: String
    }
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

    #[serde(rename = "email")]
    Email,

    #[serde(rename = "date")]
    Date,
    #[serde(rename = "number")]
    Number,

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
    compare: Compare,
    relation: Relation
}

#[derive(Deserialize, Clone, Debug)]
pub struct FieldA {
        name: String,
        #[serde(default)]
        label: Label,
        #[serde(default)]
        classes: Vec<String>,
        #[serde(rename = "breakAfter")]
        #[serde(default)]
        break_after: bool,
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
            SpecificFields::CheckBox(_) => json!([]),
            _ => json!(())
        }
    }
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

// ---

#[component]
fn AForm(form: FormA) -> impl IntoView {
    let (data, set_data) = signal(form.data);

    let memo = Memo::new( move | _ | data.get());
    provide_context(set_data);
    set_ids();
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

            <Fields fields = form.def.fields path="".to_string() data=memo/>


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
            on:click= move |_| {
                console_log("clicked");
                // leptos::logging::log!("{:?}", "aaaaaaaaaaaaaaaa");
                update_data("hobbies".into(), json!(["reading"]))
                // console_log("here");
                // let tr = ArcTrigger::new();
                // tr.notify();
            }
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
        <div class="fields-wrap">
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
                                    match d.get(name.clone()) {
                                        Some(v) => v.clone(),
                                        None => {
                                            if !default.is_null() {
                                                update_data(path2.clone(), default.clone());
                                            }
                                            // Value::Null
                                            default.clone()
                                        }
                                    }
                                }
                            )
                        }
                    );

                    view! {
                        <Field field path  data=fd/>
                    }
                }
            </For>
        </div>
    }
}

// ---

#[component]
fn Field(field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let (classes, set_classes) = signal(format!("field-wrap col {} ", field.classes.join(" ")));
    let id = format!("_{}",path);

    let (label_text, label_position) = match field.clone().label {
        Label::Scalar(l) => (l, "before".to_string()),
        Label::Object { text, position } => (text, position),
        Label::Empty => (String::new(), String::new())
    };

    view! {
            <div class= move || classes.get() >

                {
                    move ||
                    if !label_text.is_empty() {
                        view! {
                                <label
                                    class= {label_position.clone()}
                                    for={id.clone()}>{label_text.clone()}

                                </label>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }

                // .line-break(v-if='field.breakAfter')

        {
            match &field.specific {
                SpecificFields::Text => view! {<FText _field = field.clone() subtype="text" path data />}.into_any(),
                SpecificFields::Email => view! {<FText _field = field.clone() subtype="email" path data />}.into_any(),
                SpecificFields::Date => view! {<FText _field = field.clone() subtype="date" path data />}.into_any(),
                SpecificFields::Number => view! {<FNumber _field = field.clone() path data />}.into_any(),

                SpecificFields::Select(c) => view! {<FSelect _field=field.clone() specific=c.clone() path data/>}.into_any(),
                SpecificFields::Radio(c) => view! {<FRadio _field=field.clone() specific=c.clone() path data/>}.into_any(),
                SpecificFields::Textarea(t) => view! {<FTextarea _field=field.clone() specific=t.clone() path  data/>}.into_any(),
                SpecificFields::TrueFalse => view! {<FTrueFalse _field=field.clone() path  data/>}.into_any(),

                SpecificFields::CheckBox(c) => view! {
                    <FCheckBox _field=field.clone() specific=c.clone() path  data/>
                }.into_any(),

                SpecificFields::Group(g) => {
                    set_classes.update(|c|  c.push_str("group") );
                    view! {
                        <FGroup _field=field.clone() specific=g.clone() path data/>
                    }.into_any()
                },

                SpecificFields::Repeater(r) => {
                    set_classes.update(|c|  c.push_str("repeater") );
                    view! {<FRepeater _field=field.clone() specific=r.clone() path data/>}.into_any()
                },
                _ => view! {
                    <div>Not implemented yet</div>
                }.into_any()
            }

        }
            </div>
            {
                move || if field.break_after {
                    view! {<div class="line-break"/>}.into_any()
                } else {
                    view! {}.into_any()
                }
            }
    }
}


#[component]
fn FText(_field: FieldA, subtype: &'static str ,path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let clean_val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <input
            id={id}
            class="field-input"
            type={subtype}
            value=clean_val
            on:input=move |evt| update_data(path.clone().into(), event_target_value(&evt).into())
        />
    }
}

#[component]
fn FNumber(_field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let val = move || data.get().as_i64().unwrap_or_default();
    view! {
        <input
            id={id}
            class="field-input"
            type="number"
            prop:value=val
            on:input=move |evt| update_data(path.clone().into(), event_target_value(&evt).into())
        />
    }
}


// ---

#[component]
fn FSelect(_field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
            <select
                class="field-input"
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
    }
}

// ---

#[component]
fn FRadio(_field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    view! {
            <div class="options">
            {
                specific.options.into_iter().enumerate().map(| (idx, e) | {
                    let spare_value = e.value.clone();
                    let spare_path = path.clone();
                    let opt_id = format!("{}-{}",id ,idx);
                    view! {
                        <div class="option-wrap">
                            <label for={opt_id.clone()}>{e.label}</label>
                            <input
                                id = {opt_id}
                                name = path.clone()
                                class="field-input"
                                type="radio"
                                value= {e.value.clone()}
                                checked = move || data.get() == e.value.clone()
                                on:change= move |_| update_data( spare_path.clone(), spare_value.clone().into())
                            />
                        </div>
                }}).collect_view()

            }
            </div>
    }
}

// ---

// #[component]
// fn FCheckBox(_field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
//     let id = format!("_{}",path);
//     view! {
//             <div class="options">
//             {
//                 specific.options.into_iter().enumerate().map(| (idx, e) | {
//                     let spare_value = e.value.clone();
//                     // let spare_path = path.clone();
//                     let data_path = format!("{}--{}",path.clone() ,idx);
//                     let opt_id = format!("{}-{}",id ,idx);
//                     view! {
//                         <div class="option-wrap">
//                             <label for={opt_id.clone()}>{e.label}</label>
//                             <input
//                                 id = {opt_id.clone()}
//                                 name = {format!("{}", path.clone())}
//                                 type="checkbox"
//                                 value= {e.value.clone()}
//                                 checked = move || data.get() == e.value.clone()
//                                 on:change= move | evt | {
//                                     if event_target_checked(&evt) {
//                                         update_data( data_path.clone(), spare_value.clone().into())
//                                     } else {
//                                         delete_data( data_path.clone())
//                                     }
//                                     console_log(&format!("{:?}", event_target_checked(&evt)));

//                                 }
//                             />
//                         </div>
//                 }}).collect_view()
//             }
//             </div>
//     }
// }

#[component]
fn FCheckBox(_field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);


    view! {
            <div class="options">
            {

                // let sarray  = Memo::new(move |_| {
                //     match  data.get().as_array() {
                //         Some(v) => v.iter().map(| e | e.as_str().unwrap_or("").to_string()).collect::<Vec<_>>(),
                //         None => vec![]
                //     }
                // });
                // // let (sarray_s, set_sarray_s) = signal(sarray);

                // let in_array = move |v| {
                //     match data.get().as_array() {
                //         Some(v) => v.iter().map(| e | e.as_str().unwrap_or("").to_string()).collect::<Vec<_>>(),
                //         None => vec![]
                //     }.contains(&v)
                // };
                // let spare_spare_path = path.clone();
                // let (sarray_s, set_sarray_s) = signal(vec!["reading".to_string()]);
                specific.options.into_iter().enumerate().map(| (idx, e) | {
                    let spare_value = e.value.clone();
                    let spare_path = path.clone();
                    let data_path = format!("{}--{}",path.clone() ,idx);
                    let opt_id = format!("{}-{}",id ,idx);

                    view! {
                        <div class="option-wrap">
                            <label for={opt_id.clone()}>{e.label}</label>
                            <input
                                id = {opt_id.clone()}
                                name = {format!("{}", path.clone())}
                                type="checkbox"
                                value= {e.value.clone()}
                                // prop:checked = move | | in_array(e.value.clone())
                                prop:checked = move | | match data.get().as_array() {
                                    Some(v) => v.iter().map(| e | e.as_str().unwrap_or("").to_string()).collect::<Vec<_>>(),
                                    None => vec![]
                                }.contains(&e.value)

                                on:change= move | evt |  {
                                    if event_target_checked(&evt) {
                                        console_log(&format!("{:?}", data.get()));

                                        let new_data = data.get().as_array().unwrap().iter();
                                        update_data( spare_path.clone(), json!(["reading", "kayaking"]));
                                    } else {
                                        update_data( spare_path.clone(), json!([]));
                                    }
                                    // console_log(&format!("{} {} ", checked, value));

                                }

                                // set_data( event_target_checked(&evt), spare_value.clone())
                                // prop:checked = move | | sarray.get().contains(&e.value)
                                // prop:checked = move | | sarray_s.get().contains(&e.value)
                                // on:change= move | evt | {
                                //     if event_target_checked(&evt) {
                                //         update_data( data_path.clone(), spare_value.clone().into())
                                //     } else {
                                //         // console_log(&format!("{:?}", sarray.get()));


                                //         // update_data( spare_path.clone(), data.get().into())
                                //         // delete_data( data_path.clone())
                                //     }
                                //     console_log(&format!("{:?}", event_target_checked(&evt)));

                                // }
                            />
                        </div>
                }}).collect_view()
            }
            </div>
    }
}

#[component]
fn FTextarea(_field: FieldA, specific: TextareaLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let clean_val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <textarea
            id={id}
            rows = move | | (specific.rows != 0).then_some(specific.rows)

            on:input=move |evt| update_data(path.clone().into(), event_target_value(&evt).into())
        >
            {clean_val}
        </textarea>
    }
}

// ---

#[component]
fn FTrueFalse(_field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);

    let toggle = move | ev | {
        let checked = event_target_checked(&ev);
        update_data(path.clone(), checked.into());
    };

    view! {
        <input
            type ="checkbox"
            id = {id.clone()}
            checked = move | | data.get().as_bool()
            on:change= toggle
        />
    }
}

// ---

#[component]
fn FGroup(_field: FieldA, specific: ObjectLike, path: String, data: Memo<Value>) -> impl IntoView {
    view! {
        <Fields fields= specific.fields path data/>
    }
}

// ---

#[component]
fn FRepeater(_field: FieldA, specific: ObjectLike, path: String, data: Memo<Value>) -> impl IntoView {
    let each = move || serde_json::from_value::<Vec<Value>>(data.get()).unwrap_or(vec![]);
    let cc = Memo::new(  move |_| each().iter().len());

    let path_cloned = path.clone();
    view! {

            <ForEnumerate
                each = move || each().clone()
                key = | e | e["___id"].clone()
                children = move |idx, row | {
                    let row_spare = row.clone();
                    let fp = format!("{}--{}", path.clone(), idx.get());
                    let fd = Memo::new(move |_|  row.clone() );
                    let delete_path = format!("{}--{}", path.clone(), row_spare["___id"].clone());
                    view! {
                        <div class="row">
                            <Fields fields= specific.fields.clone() path = fp  data = fd />
                            <div class="controls">
                                <span
                                    on:click = move |_| delete_data(delete_path.clone())
                                    inner_html = "&#9747;"
                                />
                            </div>
                        </div>
                    }
                }
            />
            <button
                on:click = move |_| {
                    let c = cc.get();
                    update_data(format!("{}--{}", path_cloned, c),json!({"___id": c}))
                }
                inner_html="&#43;"
            />


    }
}
