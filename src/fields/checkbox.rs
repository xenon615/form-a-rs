
use super::{OptionsLike, FieldA};
use leptos::{prelude::*, leptos_dom::logging::console_log};
use crate::data::update;
use serde_json::{Value, json};

// #[component]
// fn FCheckBox(_field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
//     let id = format!("_{}",path);
//     view! {
//             <div class="options">
//             {
//                 specific.options.into_iter().enumerate().map(| (idx, e) | {
//                     let spare_value = e.value.clone();
//                     // let spare_path = path.clone();
//                     let data_path = format!("{}--{}",path.clone() ,idx);
//                     let opt_id = format!("{}-{}",id ,idx);
//                     view! {
//                         <div class="option-wrap">
//                             <label for={opt_id.clone()}>{e.label}</label>
//                             <input
//                                 id = {opt_id.clone()}
//                                 name = {format!("{}", path.clone())}
//                                 type="checkbox"
//                                 value= {e.value.clone()}
//                                 checked = move || data.get() == e.value.clone()
//                                 on:change= move | evt | {
//                                     if event_target_checked(&evt) {
//                                         update_data( data_path.clone(), spare_value.clone().into())
//                                     } else {
//                                         delete_data( data_path.clone())
//                                     }
//                                     console_log(&format!("{:?}", event_target_checked(&evt)));

//                                 }
//                             />
//                         </div>
//                 }}).collect_view()
//             }
//             </div>
//     }
// }

#[component]
pub fn Field(_field: FieldA, specific: OptionsLike, path: String, data: Memo<Value>) -> impl IntoView {
    let id = format!("_{}",path);


    view! {
            <div class="options">
            {

                // let sarray  = Memo::new(move |_| {
                //     match  data.get().as_array() {
                //         Some(v) => v.iter().map(| e | e.as_str().unwrap_or("").to_string()).collect::<Vec<_>>(),
                //         None => vec![]
                //     }
                // });
                // // let (sarray_s, set_sarray_s) = signal(sarray);

                // let in_array = move |v| {
                //     match data.get().as_array() {
                //         Some(v) => v.iter().map(| e | e.as_str().unwrap_or("").to_string()).collect::<Vec<_>>(),
                //         None => vec![]
                //     }.contains(&v)
                // };
                // let spare_spare_path = path.clone();
                // let (sarray_s, set_sarray_s) = signal(vec!["reading".to_string()]);
                specific.options.into_iter().enumerate().map(| (idx, e) | {
                    let spare_value = e.value.clone();
                    let spare_path = path.clone();
                    let data_path = format!("{}--{}",path.clone() ,idx);
                    let opt_id = format!("{}-{}",id ,idx);

                    view! {
                        <div class="option-wrap">
                            <label for={opt_id.clone()}>{e.label}</label>
                            <input
                                id = {opt_id.clone()}
                                name = {format!("{}", path.clone())}
                                type="checkbox"
                                value= {e.value.clone()}
                                // prop:checked = move | | in_array(e.value.clone())
                                prop:checked = move | | match data.get().as_array() {
                                    Some(v) => v.iter().map(| e | e.as_str().unwrap_or("").to_string()).collect::<Vec<_>>(),
                                    None => vec![]
                                }.contains(&e.value)

                                on:change= move | evt |  {
                                    if event_target_checked(&evt) {
                                        console_log(&format!("{:?}", data.get()));

                                        let new_data = data.get().as_array().unwrap().iter();
                                        update( spare_path.clone(), json!(["reading", "kayaking"]));
                                    } else {
                                        update( spare_path.clone(), json!([]));
                                    }
                                    // console_log(&format!("{} {} ", checked, value));

                                }

                                // set_data( event_target_checked(&evt), spare_value.clone())
                                // prop:checked = move | | sarray.get().contains(&e.value)
                                // prop:checked = move | | sarray_s.get().contains(&e.value)
                                // on:change= move | evt | {
                                //     if event_target_checked(&evt) {
                                //         update_data( data_path.clone(), spare_value.clone().into())
                                //     } else {
                                //         // console_log(&format!("{:?}", sarray.get()));


                                //         // update_data( spare_path.clone(), data.get().into())
                                //         // delete_data( data_path.clone())
                                //     }
                                //     console_log(&format!("{:?}", event_target_checked(&evt)));

                                // }
                            />
                        </div>
                }}).collect_view()
            }
            </div>
    }
}
