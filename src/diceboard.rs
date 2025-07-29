use std::collections::HashSet;

use ndm::Dice;
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub(crate) fn DiceBoard() -> Element {
    info!("diceboard");
    
    let mut selected_options = use_signal(HashSet::<String>::new);
    let mut sig_result = use_signal(|| 0);
    let mut sig_custom_dice= use_signal(String::new);
    let mut sig_all_dice= use_signal(String::new);

    rsx! {
        //finalement c'est inutile
        /*
        div {  class:"row mb-4 pb-1 border-bottom border-warning-subtle", div { class:"col", div {class:"text-warning",  "Options" } }}
        div { class:"row mb-4 pb-1",
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
                    span{class: "p-2 text-warning", "Expolding dice"}
                }
            }
            div { class:"col-6"}
        }
        */

        div {  class:"row pb-1 border-bottom border-warning-subtle", div { class:"col", div {class:"text-warning",  "Set classique " } }}
        div{ class: "row mt-4 mb-4",
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn4", onclick: move |event| {
                        let d4 = "1d4".parse::<Dice>().unwrap();
                        sig_result.set(d4.total());
                }, "1D4" }
            }
            div { class:"col",
                div { class:"btn btn-warning", id:"btn6", onclick: move |event| {
                        let d6 = "1d6".parse::<Dice>().unwrap(); //Equation::new("1d6").unwrap();
                        sig_result.set(d6.total());
                }, "1D6" }
            }
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn8", onclick: move |event| {
                        let d8 = "1d8".parse::<Dice>().unwrap();
                        sig_result.set(d8.total());
                }, "1D8" }
            }
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn10", onclick: move |event| {
                        let d10 = "1d10".parse::<Dice>().unwrap();
                        sig_result.set(d10.total());
                }, "1D10" }
            }
            div {  class:"col",
                div { class:"btn btn-warning", id:"btn12", onclick: move |event| {
                        let d12 = "1d12".parse::<Dice>().unwrap();
                        sig_result.set(d12.total());
                }, "1D12" }
            }
            div {  class:"col",
                div { class:"btn btn-warning w", id:"btn20", onclick: move |event| {
                        let d20 = "1d20".parse::<Dice>().unwrap();
                        sig_result.set(d20.total());
                }, "1D20" }
            }
             div {  class:"col",
                div { class:"btn btn-warning", id:"btn30", onclick: move |event| {
                        let d30 = "1d30".parse::<Dice>().unwrap();
                        sig_result.set(d30.total());
                }, "1D30" }
            }
        }
        div {  class:"row mb-4 pb-1 border-bottom border-warning-subtle", div { class:"col", div {class:"text-warning",  "Dés custom " } }}
        div { class:"row mb-2 mt-2", 
            div { class:"col",  
                input { class:"mr-2 form-control form-control-sm", 
                    type:"text", 
                    id:"custom_dice", 
                    placeholder:"dice equation ... ",
                    value: "{sig_custom_dice}",
                    oninput: move |event| {sig_custom_dice.set(event.value());}
                 }
                
            } 
            div { class:"col", div { class:"btn btn-warning", id:"custom_dice_val", onclick: move |event|{
                let dice_txt = sig_custom_dice.read();
                let dice = dice_txt.parse::<Dice>().unwrap();
                let all = dice.all_rolls();
                sig_all_dice.set(vec_u16_to_string(all));
                sig_result.set(dice.total());
            }, "Roll" } }
        }
        
        div { class:"row mt-5", 
            div { class:"container",
                div { class: "border border-warning-subtle p-3 w-75", id: "result", 
                    " Résultat : {sig_result}" 
                 }
            }
        }
        div { class:"row mt-3", div { class:"text-start fst-italic text-light", " Tous les dés : {sig_all_dice}" } }
    }
}


fn vec_u16_to_string(all_dices: &Vec<u16>) -> String {
    println!("les u16: {:?}",  all_dices);

    let st = all_dices.iter()
    .map(|n| n.to_string())
    .collect::<Vec<_>>()
    .join(", ");

    println!("les dés : {st}");
    st 
}