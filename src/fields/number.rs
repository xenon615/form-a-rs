use leptos::prelude::*;
use serde_json::Value;
use super::FieldA;
use crate::data::update;

#[component]
pub fn Field(_field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let val = move || data.get().as_i64().unwrap_or_default();
    view! {
        <input
            id={id}
            class="field-input"
            type="number"
            prop:value=val
            on:input=move |evt| update(path.clone().into(), event_target_value(&evt).into())
        />
    }
}
