use super::OptionsLike;
use leptos::prelude::*;
use crate::data::update;
use serde_json::Value;

#[component]
pub fn Field(specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    let val = move || data.get().as_str().unwrap_or("").to_string();
    view! {
            <select
                class="field-input"
                id={id}
                prop:value={val}
                on:change = move |evt|  update(path.clone().into(), event_target_value(&evt).into())
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
