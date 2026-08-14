use leptos::{
    // leptos_dom::logging::console_log,
    prelude::*
};
use crate::data::*;
use serde::Deserialize;
use serde_json::{Value, json};

mod text;
mod number;
mod radio;
mod select;
mod textarea;
mod true_false;
mod group;
mod repeater;
mod checkbox;

#[derive(Deserialize, Clone)]
pub struct Button {
    pub text: String,
    pub classes: Vec<String>,
    #[serde(rename = "type")]
    pub btype: String,
    #[serde(default)]
    pub action: String
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
    // Table(ObjectLike)
    Table

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
    Less,
    #[serde(rename = "in")]
    In
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
    pub fn empty_value(&self) -> Value {
        match self.specific {
            SpecificFields::Group(_) => json!({}),
            SpecificFields::CheckBox(_) => json!([]),
            _ => json!(())
        }
    }
}


#[component]
pub fn Fields(fields: Vec<FieldA>, path: String, data: Memo<Value>) -> impl IntoView {
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
                                                update(path2.clone(), default.clone());
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
                SpecificFields::Text => view! {<text::Field subtype="text" path data />}.into_any(),
                SpecificFields::Email => view! {<text::Field subtype="email" path data />}.into_any(),
                SpecificFields::Date => view! {<text::Field subtype="date" path data />}.into_any(),
                SpecificFields::Number => view! {<number::Field  path data />}.into_any(),

                SpecificFields::Select(c) => view! {<select::Field specific=c.clone() path data/>}.into_any(),
                SpecificFields::Radio(c) => view! {<radio::Field  specific=c.clone() path data/>}.into_any(),
                SpecificFields::Textarea(t) => view! {<textarea::Field  specific=t.clone() path  data/>}.into_any(),
                SpecificFields::TrueFalse => view! {<true_false::Field  path  data/>}.into_any(),

                SpecificFields::CheckBox(c) => view! { <checkbox::Field specific=c.clone() path  data/>}.into_any(),

                SpecificFields::Group(g) => {
                    set_classes.update(|c|  c.push_str("group"));
                    view! {
                        <group::Field  specific=g.clone() path data/>
                    }.into_any()
                },

                SpecificFields::Repeater(r) => {
                    set_classes.update(|c|  c.push_str("repeater") );
                    view! {<repeater::Field  specific=r.clone() path data/>}.into_any()
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

// ---

pub fn get_field(path: &[&str]) -> FieldA {
    let r = use_context::<Vec<FieldA>>().unwrap();
    // console_log(&format!("{:?}", path.join("-")));
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

pub fn is_show(field: &FieldA) -> bool {
    if field.c_logic.is_empty() {
        return true;
    }
    let mut result = false;
    for l in &field.c_logic {
        let test = get(l.path.clone());
        // console_log(&format!("{} {}", &field.name,test.as_str().unwrap_or_default()));

        let l0 = match l.compare {
            Compare::Eq => l.value == test,
            Compare::MotEq => l.value != test,
            Compare::In =>  test.as_array().unwrap_or(&vec![]).contains(&l.value),
            _ => true
        };

        result = match l.relation {
            Relation::And => {
                // console_log(&format!("{} && {}", l0, result));
                l0 && result
            },
            Relation::Or => {
                // console_log(&format!("{} || {}", l0, result));
                l0 || result
            }
        };
        // console_log(&format!("{}", result));
        // console_log("-------------------");
    }
    result
}
