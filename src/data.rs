use leptos::{prelude::*, leptos_dom::logging::console_log};
use serde_json::{Value, json};
use crate::{
    fields::get_field
};

pub fn update(path_str: String, value: Value) {
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
        *f = value
    })
}

// ---

pub fn get(path: String) -> Value{
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

pub fn delete(path_str: String) {
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
                if path_len == idx + 1 {
                    console_log(&format!("{}  / {:?}", pe, &f));
                    f.as_array_mut().unwrap().retain(| e| e["___id"].to_string() != *pe);
                    break;
                }
                f.get_mut(pe).unwrap()
            }

        }
    })
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
