use std::collections::HashSet;
use dice_forge::Equation;
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub(crate) fn DiceBoard() -> Element {
    info!("page 2 ");
    
    let mut selected_options = use_signal(HashSet::<String>::new);
    let mut sig_result = use_signal(|| 0);

    rsx! {
        div { class:"row mb-5",
            div { class:"col-6",

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
            div { class:"col-6"}
        }
        div{ class: "row",
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn4", onclick: move |event| {
                        let d4 = Equation::new("1d4").unwrap();
                        sig_result.set(d4.roll().unwrap());
                }, "1D4" }
            }
            div { class:"col",
                div { class:"btn btn-warning", id:"btn6", onclick: move |event| {
                        let d6 = Equation::new("1d6").unwrap();
                        sig_result.set(d6.roll().unwrap());
                }, "1D6" }
            }
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn8", onclick: move |event| {
                        let d8 = Equation::new("1d8").unwrap();
                        sig_result.set(d8.roll().unwrap());
                }, "1D8" }
            }
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn10", onclick: move |event| {
                        let d10 = Equation::new("1d10").unwrap();
                        sig_result.set(d10.roll().unwrap());
                }, "1D10" }
            }
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn12", onclick: move |event| {
                        let d12 = Equation::new("1d12").unwrap();
                        sig_result.set(d12.roll().unwrap());
                }, "1D12" }
            }
            div {  class:"col",
                div { class:"btn btn-warning w", id:"btn20", onclick: move |event| {
                        let d20 = Equation::new("1d20").unwrap();
                        sig_result.set(d20.roll().unwrap());
                }, "1D20" }
            }
             div {  class:"col",
                div { class:"btn btn-warning", id:"btn30", onclick: move |event| {
                        let d30 = Equation::new("1d30").unwrap();
                        sig_result.set(d30.roll().unwrap());
                }, "1D30" }
            }
        }
        div { class:"row mt-5", 
            div { class:"col",
                div { class: "btn btn-outline-secondary", id: "result", 
                    " Résultat : {sig_result}" 
                 }
            }
        }
    }
}
