use super::{ObjectLike, FieldA, Fields};
use leptos::prelude::*;
use serde_json::{Value, json};
use crate::data::{delete, update};

#[component]
pub fn Field(_field: FieldA, specific: ObjectLike, path: String, data: Memo<Value>) -> impl IntoView {
    let each = move || serde_json::from_value::<Vec<Value>>(data.get()).unwrap_or(vec![]);
    let cc = Memo::new(  move |_| each().iter().len());

    let path_cloned = path.clone();
    view! {

            <ForEnumerate
                each = move || each().clone()
                key = | e | e["___id"].clone()
                children = move |idx, row | {
                    let row_spare = row.clone();
                    let fp = format!("{}--{}", path.clone(), idx.get());
                    let fd = Memo::new(move |_|  row.clone() );
                    let delete_path = format!("{}--{}", path.clone(), row_spare["___id"].clone());
                    view! {
                        <div class="row">
                            <Fields fields= specific.fields.clone() path = fp  data = fd />
                            <div class="controls">
                                <span
                                    on:click = move |_| delete(delete_path.clone())
                                    inner_html = "&#9747;"
                                />
                            </div>
                        </div>
                    }
                }
            />
            <button
                on:click = move |_| {
                    let c = cc.get();
                    update(format!("{}--{}", path_cloned, c),json!({"___id": c}))
                }
                inner_html="&#43;"
            />


    }
}
