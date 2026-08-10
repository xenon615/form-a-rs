use leptos::prelude::*;
use serde_json::Value;
use super::FieldA;
use crate::data::update;

#[component]
pub fn Field(_field: FieldA, subtype: &'static str ,path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let clean_val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <input
            id={id}
            class="field-input"
            type={subtype}
            value=clean_val
            on:input=move |evt| update(path.clone().into(), event_target_value(&evt).into())
        />
    }
}
