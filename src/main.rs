use leptos::{leptos_dom::logging::console_log, prelude::*, svg::view};
use serde::Deserialize;
use serde_json::{Value, json};
use wasm_bindgen::JsCast;
use gloo_net::http::Request;


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
struct Opt {
    value: String,
    label: String
}

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

#[derive(Deserialize, Debug, Clone)]
enum Relation {
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "and")]
    And
}

#[derive(Deserialize, Debug, Clone)]
enum Compare {
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
struct Logic {
    path: String,
    value: Value,
    // compare: String,
    compare: Compare,
    relation: Relation
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
                <Fields fields = form.def.fields path="form".to_string() data=memo/>
            </div>

            <div class="buttons">
                {
                    form.def.buttons.into_iter().map(| b | view! {
                        <button type = {b.btype} class = {b.classes.join(" ")}  >{b.text}</button>
                    }).collect_view()
                }
            </div>
            <Jachc/>
            <Pretty data/>

        </div>
    }
}

// ---

fn update_data(path: String, value: Value) {
    let w = use_context::<WriteSignal<Value>>().unwrap();
    let path_arr = path.split("--").skip(1).collect::<Vec<_>>();
    w.update(| p |  {
        let mut f = p;
        for (idx, pe) in path_arr.iter().enumerate() {
            if f.is_object() {
                f = f.as_object_mut().unwrap().entry(*pe).or_insert(
                    get_field(&path_arr[0 .. idx + 1]).empty_value()
                );
            }
        }
        *f = value
    })
}

// ---

fn get_field(path: &[&str]) -> FieldA {
    let r = use_context::<Vec<FieldA>>().unwrap();
    console_log(&format!("{:?}", path.join("-")));

    let mut p = r;
    let mut idx = 0;
    loop {
        let f = p.into_iter().find(| e| e.name == path[idx]).unwrap();
        match f.specific  {
            SpecificFields::Group(sf) if idx < path.len() - 1  => p = sf.fields,
            _ => {break f}
        }
        idx += 1;
    }
}

// ---

fn get_data(path: String) -> Value{
    let r = use_context::<ReadSignal<Value>>().unwrap();
    let path_arr = path.split("--").collect::<Vec<_>>();
    let mut p = r.get();
    for pe in path_arr.iter() {
        if p.is_null() {
           return p;
        }
        if p.is_object() {
            p = p[pe].clone()
        } else {
            return p;
        }
    }
    p
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
            // on:click= move |_| update_data("form--stuff--name".into(), "ASD!!".into())
            on:click = move |_| {
                let path = vec!["stuff","backup","where"];
                let f = get_field(&path[0..1]);
                // let f = get_field(vec![
                //     "stuff".to_string(),
                //     "backup".to_string(),
                //     // "where".to_string()
                // ]);

                console_log(&format!("{:?}", f.empty_value()));
            }
        >
            CLICK ME
        </button>
    }
}

// ---

// #[component]
// fn Fields(fields: Vec<FieldA>, path: String, data:Memo<Value>) -> impl IntoView {
//     fields.into_iter().map(| f | {
//         let name = f.name.clone();
//         let path = format!("{}--{}", path, name);
//         let fd = Memo::new( move |_| data.get()[name.clone()].clone() );
//         view! {
//             <Field field = f path data = fd/>
//         }
//     }).collect_view()
// }

// ---

fn is_show(field: &FieldA) -> bool {
    if field.c_logic.is_empty() {
        return true;
    }
    let mut result = false;
    for l in &field.c_logic {
        let test = get_data(l.path.clone());
        let l0 = match l.compare {
            Compare::Eq => l.value == test,
            Compare::MotEq => l.value != test,
            _ => true
        };
        result = match l.relation {
            Relation::And => l0 && result,
            Relation::Or => l0 || result
        }
    }
    result
}

// ---

#[component]
fn Fields(fields: Vec<FieldA>, path: String, data: Memo<Value>) -> impl IntoView {
    view! {
        <For
            // each = move || fields.clone()
            each = move || {
                fields.clone().into_iter().filter(|f| is_show(&f)).collect::<Vec<_>>()
            }
            key = move | f | f.name.clone()
            let(field)
        >
            {
                let name = field.name.clone();
                let default = field.default.clone();

                let path = format!("{}--{}", path, name);
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
                SpecificFields::Group(s) => view! {
                    <FGroup field=field.clone() specific=s.clone() path data/>
                }.into_any(),

                SpecificFields::Select(s) => view! {
                    <FSelect field=field.clone() specific=s.clone() path data/>
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
fn FSelect(field: FieldA, specific: ChoiceLike, path: String, data: Memo<Value>) -> impl IntoView {
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
fn FGroup(field: FieldA, specific: ContainerLike, path: String, data: Memo<Value>) -> impl IntoView {
    view! {
        <div class={format!("group {}", field.classes.join(" "))}>
            <label for={path.clone()}>{field.label}</label>
            <Fields fields= specific.fields path data/>
        </div>
    }
}

// ---
