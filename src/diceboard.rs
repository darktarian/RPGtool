use dioxus::prelude::*;
use ndm::{Dice, RollSet};

#[component]
pub(crate) fn DiceBoard() -> Element {
    //let mut selected_options = use_signal(HashSet::<String>::new);
    let mut sig_result = use_signal(String::new);
    //let mut sig_result2 = use_signal(String::new);
    let mut sig_custom_dice = use_signal(String::new);
    let mut sig_all_dice = use_signal(String::new);

    rsx! {

        div {  class:"row pb-1 border-bottom border-warning-subtle", div { class:"col", div {class:"text-warning",  "Set classique " } }}
        div{ class: "container mt-4 mb-1",
                div { class:"btn-dice", id:"btn3", onclick: move |_event| {
                        let d3 = "1d3".parse::<Dice>().unwrap();
                        sig_result.set(d3.total().to_string());
                }, "1D3" }
                div { class:"btn-dice", id:"btn4", onclick: move |_event| {
                        let d4 = "1d4".parse::<Dice>().unwrap();
                        sig_result.set(d4.total().to_string());
                }, "1D4" }

                div { class:"btn-dice", id:"btn4", onclick: move |_event| {
                        let d5 = "1d5".parse::<Dice>().unwrap();
                        sig_result.set(d5.total().to_string());
                }, "1D5" }
                div { class:"btn-dice", id:"btn6", onclick: move |_event| {
                        let d6 = "1d6".parse::<Dice>().unwrap();
                        sig_result.set(d6.total().to_string());
                }, "1D6" }
                div { class:"btn-dice", id:"btn7", onclick: move |_event| {
                        let d7 = "1d7".parse::<Dice>().unwrap();
                        sig_result.set(d7.total().to_string());
                }, "1D6" }
                div { class:"btn-dice", id:"btn8", onclick: move |_event| {
                        let d8 = "1d8".parse::<Dice>().unwrap();
                        sig_result.set(d8.total().to_string());
                }, "1D8" }
                div { class:"btn-dice", id:"btn10", onclick: move |_event| {
                        let d10 = "1d10".parse::<Dice>().unwrap();
                        sig_result.set(d10.total().to_string());
                }, "1D10" }
        }
        div{ class:"container mt-1 mb-1",
                div { class:"btn-dice", id:"btn12", onclick: move |_event| {
                        let d12 = "1d12".parse::<Dice>().unwrap();
                        sig_result.set(d12.total().to_string());
                }, "1D12" }
                div { class:"btn-dice", id:"btn14", onclick: move |_event| {
                        let d14 = "1d14".parse::<Dice>().unwrap();
                        sig_result.set(d14.total().to_string());
                }, "1D14" }
                div { class:"btn-dice", id:"btn16", onclick: move |_event| {
                        let d12 = "1d16".parse::<Dice>().unwrap();
                        sig_result.set(d12.total().to_string());
                }, "1D16" }
                div { class:"btn-dice", id:"btn20", onclick: move |_event| {
                        let d20 = "1d20".parse::<Dice>().unwrap();
                        sig_result.set(d20.total().to_string());
                }, "1D20" }
                div { class:"btn-dice", id:"btn24", onclick: move |_event| {
                        let d24 = "1d24".parse::<Dice>().unwrap();
                        sig_result.set(d24.total().to_string());
                }, "1D24" }
                div { class:"btn-dice", id:"btn30", onclick: move |_event| {
                        let d30 = "1d30".parse::<Dice>().unwrap();
                        sig_result.set(d30.total().to_string());
                }, "1D30" }
                div { class:"btn-dice", id:"btn100", onclick: move |_event| {
                        let d100 = "1d100".parse::<Dice>().unwrap();
                        sig_result.set(d100.total().to_string());
                }, "1D100" }
        }
        div {  class:"row mb-4 pb-1 border-bottom border-warning-subtle", div { class:"col", div {class:"text-warning",  "Dés custom " } }}
        div { class:"row mb-2 mt-4",
            div { class:"col-5",
                input { class:"mr-2 form-control form-control-sm",
                    type:"text",
                    id:"custom_dice",
                    placeholder:"dice equation ... ",
                    value: "{sig_custom_dice}",
                    oninput: move |event| {sig_custom_dice.set(event.value());}
                 }

            }
            //dés custom
            div { class:"col-1",
                    div { class:"btn btn-sm btn-warning", id:"custom_dice_val", onclick: move |_event|{
                    let dice_txt = sig_custom_dice.read();

                    let (all, total) = match dice_txt.parse::<Dice>(){
                        Ok(d) => (vec_u16_to_string(d.all_rolls()),d.total().to_string()),
                        Err(_) => {
                            let roll = dice_txt.parse::<RollSet>().ok();

                            if let Some(rolled) = roll {
                                ("".to_string(), format!("{}", rolled))
                            }  else {
                                ("Error : Format non pris en charge.".to_string(), "".to_string())
                            }
                        }
                    };

                    sig_all_dice.set(all);
                    sig_result.set(total);
                }, "Roll" }
            }
            div { class:"col-1",
                button {class: "btn btn-sm btn-outline-success",
                    type:"button",
                    onclick: move |_| {
                            sig_custom_dice.set(String::new());
                    },
                "⭐"
            }
        }
            div { class:"col-1",
                                button {class: "btn btn-sm btn-outline-danger",
                    type:"button",
                    onclick: move |_| {
                            sig_custom_dice.set(String::new());
                    },
                "❌"
            }
        }
        }
        div {class:"text-warning border-bottom border-warning-subtle",  "Dices total value: " }
        //zone des résultats.
        div { class:"row mt-2",
            div { class:"col",
                div { class: "bg-dark round rounded-2 p-3", id: "result",
                    "{sig_result}"
                 }
            }
            //un affichage de tous les dés générés pour le jet
            if !sig_custom_dice().is_empty(){
                div { class:"col ", div { class:"border border-warning-subtle p-3 text-start", " Tous les dés : {sig_all_dice}" } }
            }
        }
        div { class:"row mt-5", div {class:"col",
            div{ p {class:"text-light",  "[count]d<sides>[/<H|L><keep>][![fuse]]"}
                p {class:"text-light",  "! for exploding dice"}
                p {class:"text-light",  "Keeping the highest dice : H + nb dice"}
                p {class:"text-light",  "Keeping the lowest dice : L + nb dice"}
            }
          }
        }

    }
}

fn vec_u16_to_string(all_dices: &Vec<u16>) -> String {
    println!("les u16: {:?}", all_dices);

    let st = all_dices
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    println!("les dés : {st}");
    st
}
