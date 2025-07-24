use std::collections::HashSet;

use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub(crate) fn DiceBoard() -> Element {
    info!("page 2 ");
    
    let mut selected_options = use_signal(HashSet::<String>::new);


    rsx! {
        div { class:"row mb-2",
            div { class:"col",

                // les checkbox de choix explode ou autre
                input {
                class: "form-check-input",
                r#type: "checkbox",
                id: "check_explode",
                checked: selected_options().contains("explode"),
                oninput: move |evt| {
                    //println!("1 atout : {atout}");
                    if evt.checked(){
                        //println!("2 checked");
                        //println!("4 add");
                        let mut sc = selected_options();
                        sc.insert("explode".to_string());
                        selected_options.set(sc);
                        //println!(" post add {:?}", selected_atout);
                    }else {
                        //println!("3 remove");
                        let mut sc = selected_options();
                        sc.remove("explode");
                        selected_options.set(sc);
                    }
                    //println!("---> {selected_atout:?}")
                },
                }
                label {
                    class:"form-check-label",
                    //r#for: "{atout.clone()}",
                    " Expolding dice"
                }
            }
            div { class:"col"}
        }
        div{
            div { class:"btn btn-cth-eldritch w-25", id:"btn7", "dashboard" }
        }
    }
}
