use super::FieldA;
use leptos::prelude::*;
use crate::data::update;
use serde_json::Value;

#[component]
pub fn Field(_field: FieldA, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);

    let toggle = move | ev | {
        let checked = event_target_checked(&ev);
        update(path.clone(), checked.into());
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
