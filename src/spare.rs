// fn Fields(fields: Vec<FieldA>, path: String, data:Memo<Value>) -> impl IntoView {
//     view! {
//         { move  ||
//             fields.clone().into_iter()
//                 // .filter(| f|  is_show(f))
//                 .map(| f | {
//                     if is_show(&f) {
//                         let name = f.name.clone();
//                         let path = if path.is_empty() {name.clone()} else {format!("{}--{}", path, name)};

//                         let fd = Memo::new( move |_| data.get()[name.clone()].clone() );
//                         view! {
//                             <Field field = f path data = fd/>
//                         }.into_any()
//                     } else {
//                         view! {}.into_any()
//                     }
//             }).collect_view()
//         }
//     }
// }

// #[component]
// fn Fields(fields: Vec<FieldA>, path: String, data:Memo<Value>) -> impl IntoView {
//     fields.into_iter().map(| f | {
//         let name = f.name.clone();
//         let path = if path.is_empty() {name.clone()} else {format!("{}--{}", path, name)};

//         let fd = Memo::new( move |_| data.get()[name.clone()].clone() );
//         view! {
//             <Field field = f path data = fd/>
//         }
//     }).collect_view()
// }
