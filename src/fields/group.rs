use super::{ObjectLike, Fields};
use leptos::prelude::*;
use serde_json::Value;

#[component]
pub fn Field(specific: ObjectLike, path: String, data: Memo<Value>) -> impl IntoView {
    view! {
        <Fields fields= specific.fields path data/>
    }
}
