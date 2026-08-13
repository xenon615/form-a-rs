use super::TextareaLike;
use leptos::prelude::*;
use crate::data::update;
use serde_json::Value;

#[component]
pub fn Field(specific: TextareaLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let clean_val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
        <textarea
            id={id}
            rows = move | | (specific.rows != 0).then_some(specific.rows)

            on:input=move |evt| update(path.clone().into(), event_target_value(&evt).into())
        >
            {clean_val}
        </textarea>
    }
}
