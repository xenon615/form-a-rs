use serde::Deserialize;
use serde_json::{Value, json};
use leptos::{
    prelude::*,
    leptos_dom::logging::console_log
};

use crate::fields::*;
use crate::data::*;


#[derive(Deserialize, Clone)]
pub struct FormA {
    def: FormDef,
    pub data: Value
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

#[component]
pub fn AForm(form: FormA) -> impl IntoView {
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
            // <TestAction/>


        </div>
        <Pretty data/>
    }
}

// ---

fn submit(action:&str) {
    update("__action".to_string(), action.into());
    console_log(action);
}

// ---

#[component]
fn Pretty (data: ReadSignal<Value>) -> impl IntoView {
    view! {
        <div class = "pretty">
            <pre>
                { move || data.with(| d |  serde_json::to_string_pretty(d).unwrap_or_default()) }
            </pre>
        </div>
    }
}

// ---

#[component]
fn TestAction () -> impl IntoView {
    view! {
        <button
            class="primary"
            on:click= move |_| {
                console_log("clicked");
                update("hobbies".into(), json!(["reading"]))
            }
        >
            CLICK ME
        </button>
    }
}
