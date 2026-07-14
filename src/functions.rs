use leptos::{prelude::*, leptos_dom::logging::console_log};
use serde_json::{Value, json};
use crate::{SpecificFields, FieldA, Relation, Compare};

pub fn update_data1(path: String, value: Value) {
    let w = use_context::<WriteSignal<Value>>().unwrap();
    let path_arr = path.split("--").collect::<Vec<_>>();
    w.update(| p |  {
        let mut f = p;
        for (idx, pe) in path_arr.iter().enumerate() {
            if f.is_object() {
                f = f.as_object_mut().unwrap().entry(*pe).or_insert(
                    if idx == 0 {Value::Null} else {
                        get_field(&path_arr[0 .. idx + 1]).empty_value()
                    }
                );
            }
        }
        *f = value
    })
}

pub fn update_data(path_str: String, value: Value) {
    console_log(&format!("{}  {:?}", path_str, value));
    let w = use_context::<WriteSignal<Value>>().unwrap();
    let path = path_str.split("--").collect::<Vec<_>>();
    w.update(| p |  {
        let mut f = p;
        for (idx, key) in path.iter().enumerate() {
            // console_log(&key);
            if f.is_object() {
                f = f.as_object_mut().unwrap().entry(*key).or_insert(
                    if idx == 0 {Value::Null} else {
                        get_field(&path[0 .. idx + 1]).empty_value()
                    }
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
        *f = value
    })
}

// pub fn update_data(path_str: String, value: Value) {
//     console_log(&path_str);
//     let w = use_context::<WriteSignal<Value>>().unwrap();
//     let path = path_str.split("--").collect::<Vec<_>>();
//     w.update(| p |  {
//         let mut f = p;
//         for key in path.into_iter() {
//             if f.is_object() {
//                 f = f.as_object_mut().unwrap().entry(key).or_insert(
//                     Value::Null
//                 );
//             } else if f.is_array() {
//                 let t = f.as_array().unwrap();
//                 let i = key.parse::<usize>().unwrap();

//                 f = if t.get(i).is_none()  {
//                     f.as_array_mut().unwrap().push_mut(json!({}))
//                 } else {
//                     f.as_array_mut().unwrap().get_mut(i).unwrap()
//                 };
//             }
//         }
//         *f = value
//     })
// }

// ---

fn get_field(path: &[&str]) -> FieldA {
    let r = use_context::<Vec<FieldA>>().unwrap();
    // console_log(&format!("{:?}", path.join("-")));
    let mut p = r;
    let mut idx = 0;
    loop {
        let f = p.into_iter().find(| e| e.name == path[idx]).unwrap();
        match f.specific  {
            SpecificFields::Group(sf) if idx < path.len() - 1  => p = sf.fields,
            _ => {break f}
        }
        idx += 1;
    }
}

// ---

pub fn get_data(path: String) -> Value{
    let r = use_context::<ReadSignal<Value>>().unwrap();
    let path_arr = path.split("--").collect::<Vec<_>>();
    let mut p = r.get();
    for pe in path_arr.iter() {
        if p.is_null() {
           return p;
        }
        if p.is_object() {
            p = p[pe].clone()
        } else {
            return p;
        }
    }
    p
}

// ---

// pub fn delete_data(path_str: String) {

//     let path = path_str.split("--").collect::<Vec<_>>();
//     console_log(&format!("{:?}", path));
//     let w = use_context::<WriteSignal<Value>>().unwrap();
//     let path_len = path.len();
//     w.update (| p |  {
//         let mut f = p;
//         for (idx, pe) in path.iter().enumerate() {
//             f = if f.is_object() {
//                 if path_len == idx + 1 {
//                     f.as_object_mut().unwrap().retain(|k, _v| k != pe);
//                     break;
//                 }
//                 f.get_mut(pe).unwrap()
//             } else {
//                 let i = pe.parse::<usize>().unwrap();
//                 if path_len == idx + 1 {
//                     f.as_array_mut().unwrap().remove(i);
//                     break;
//                 }
//                 f.get_mut(i).unwrap()
//             }

//         }
//     })
// }

pub fn delete_data(path_str: String) {

    let path = path_str.split("--").collect::<Vec<_>>();
    console_log(&format!("{:?}", path));
    let w = use_context::<WriteSignal<Value>>().unwrap();
    let path_len = path.len();
    w.update (| p |  {
        let mut f = p;
        for (idx, pe) in path.iter().enumerate() {
            f = if f.is_object() {
                if path_len == idx + 1 {
                    f.as_object_mut().unwrap().retain(|k, _v| k != pe);
                    break;
                }
                f.get_mut(pe).unwrap()
            } else {
                // let i = pe.parse::<usize>().unwrap();
                if path_len == idx + 1 {
                    // f.as_array_mut().unwrap().remove(i);
                    console_log(&format!("{}  / {:?}", pe, &f));
                    f.as_array_mut().unwrap().retain(| e| e["___id"].to_string() != *pe);
                    break;
                }
                // f.get_mut(i).unwrap()
                f.get_mut(pe).unwrap()
            }

        }
    })
}

// pub fn delete_data(path: &[&str]) {
//     console_log(&format!("{:?}", path));
//     let w = use_context::<WriteSignal<Value>>().unwrap();
//     let path_len = path.len();
//     w.update (| p |  {
//         let mut f = p;
//         for (idx, pe) in path.iter().enumerate() {
//             if path_len == idx + 1 {
//                 f.as_object_mut().unwrap().retain(|k, _v| k != pe);
//                 break;
//             }
//             f = f.get_mut(pe).unwrap();
//         }
//     })
// }

// ---

pub fn is_show(field: &FieldA) -> bool {
    if field.c_logic.is_empty() {
        return true;
    }
    let mut result = false;
    for l in &field.c_logic {
        let test = get_data(l.path.clone());
        let l0 = match l.compare {
            Compare::Eq => l.value == test,
            Compare::MotEq => l.value != test,
            _ => true
        };
        result = match l.relation {
            Relation::And => l0 && result,
            Relation::Or => l0 || result
        }
    }
    result
}

// ---

pub fn set_ids() {
    let w = use_context::<WriteSignal<Value>>().unwrap();
    w.update (| p |  {
        _set_ids(p);
    });
}

pub fn _set_ids(v: &mut Value){
    if v.is_object() {
        for (_, val) in v.as_object_mut().unwrap() {
            _set_ids(val);
        }
    } else if v.is_array() {
        for (i, ae) in v.as_array_mut().unwrap().iter_mut().enumerate() {
             let Some(row) = ae.as_object_mut() else {
                 continue;
             };
             row.insert("___id".to_string(), i.into());
             _set_ids(ae);
        }
    }
}


// pub fn set_ids(v: &mut Value){
//     if v.is_object() {
//         for (_, val) in v.as_object_mut().unwrap() {
//             set_ids(val);
//         }
//     } else if v.is_array() {
//         for (i, ae) in v.as_array_mut().unwrap().iter_mut().enumerate() {
//              let Some(row) = ae.as_object_mut() else {
//                  continue;
//              };
//              row.insert("___id".to_string(), i.into());
//              set_ids(ae);
//         }
//     }
// }
