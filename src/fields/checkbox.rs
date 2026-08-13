
use super::{OptionsLike, get_field};
// use crate::data::update;
use leptos::{
    // leptos_dom::logging::console_log,
    prelude::*
};

use serde_json::{Value, json};


#[component]
pub fn Field(specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);
    view! {
            <div class="options">
            {
                specific.options.into_iter().enumerate().map(| (idx, e) | {
                    let spare_path = path.clone();
                    let opt_id = format!("{}-{}",id ,idx);

                    view! {
                        <div class="option-wrap">
                            <label for={opt_id.clone()}>{e.label}</label>
                            <input
                                id = {opt_id.clone()}
                                name = {format!("{}", path.clone())}
                                type="checkbox"
                                value= {e.value.clone()}
                                prop:checked = move | | match data.get().as_array() {
                                    Some(v) => v.iter().map(| e | e.as_str().unwrap_or("").to_string()).collect::<Vec<_>>(),
                                    None => vec![]
                                }.contains(&e.value)
                                on:change =  move | evt | update( spare_path.clone(), event_target_value(&evt).into(), event_target_checked(&evt))
                            />
                        </div>
                }}).collect_view()
            }
            </div>
    }
}

pub fn update(path_str: String, value: Value, is_set: bool) {
    // console_log(&format!("{}  {:?}", path_str, value));
    let w = use_context::<WriteSignal<Value>>().unwrap();
    let path = path_str.split("--").collect::<Vec<_>>();
    w.update(| p |  {
        let mut f = p;
        for (idx, key) in path.iter().enumerate() {
            // console_log(&key);
            if f.is_object() {
                f = f.as_object_mut().unwrap().entry(*key).or_insert(
                    get_field(&path[0 .. idx + 1]).empty_value()
                );
            } else if f.is_array() {
                let t = f.as_array().unwrap();
                let i = key.parse::<usize>().unwrap();

                f = if t.get(i).is_none()  {
                    f.as_array_mut().unwrap().push_mut(json!({}))
                } else {
                    f.as_array_mut().unwrap().get_mut(i).unwrap()
                };
            }
        }

        if is_set {
            f.as_array_mut().unwrap().push(value);
        } else {
            f.as_array_mut().unwrap().retain(| e  |   *e != value);
        }

    })
}
