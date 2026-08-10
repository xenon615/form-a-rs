use super::{OptionsLike, FieldA};
use leptos::prelude::*;
use crate::data::update;
use serde_json::Value;

#[component]
pub fn Field(_field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
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
                                on:change= move |_| update( spare_path.clone(), spare_value.clone().into())
                            />
                        </div>
                }}).collect_view()

            }
            </div>
    }
}
