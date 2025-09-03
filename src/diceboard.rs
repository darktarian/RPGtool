use dioxus::prelude::*;
use ndm::{Dice, DiceParseError};

use crate::dice_custom::get_custom_dice;

fn get_a_dice(dice: &str) -> String {
    match dice.parse::<Dice>() {
        Ok(de) => format!("{} => {}", dice, de.total()),
        Err(_) => DiceParseError::Unparseable.to_string(),
    }
}

#[component]
pub(crate) fn DiceBoard() -> Element {
    let mut sig_result = use_signal(String::new);
    let mut sig_custom_dice = use_signal(String::new);
    let _sig_all_dice = use_signal(String::new);

    rsx! {
        div {  class:"pb-1 border-bottom border-warning-subtle",
         div {class:"text-warning",  "Set classique " }
        }
        div{ class: "container mt-2 mb-2",
                div { class:"btn-dice", id:"btn3", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d3"));
                }, "1D3" }
                div { class:"btn-dice", id:"btn4", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d4"));
                }, "1D4" }
                div { class:"btn-dice", id:"btn4", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d5"));
                }, "1D5" }
                div { class:"btn-dice", id:"btn6", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d6"));
                }, "1D6" }
                div { class:"btn-dice", id:"btn7", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d7"));
                }, "1D7" }
                div { class:"btn-dice", id:"btn8", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d8"));
                }, "1D8" }
                div { class:"btn-dice", id:"btn10", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d10"));
                }, "1D10" }
        }
        div{ class:"container mt-1 mb-1",
                div { class:"btn-dice", id:"btn12", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d12"));
                }, "1D12" }
                div { class:"btn-dice", id:"btn14", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d14"));
                }, "1D14" }
                div { class:"btn-dice", id:"btn16", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d16"));
                }, "1D16" }
                div { class:"btn-dice", id:"btn20", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d20"));
                }, "1D20" }
                div { class:"btn-dice", id:"btn24", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d24"));
                }, "1D24" }
                div { class:"btn-dice", id:"btn30", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d30"));
                }, "1D30" }
                div { class:"btn-dice", id:"btn100", onclick: move |_event| {
                    sig_result.set(get_a_dice("1d100"));
                }, "1D100" }
        }
        div {  class:"row mt-2 mb-4 pb-1 border-bottom border-warning-subtle text-warning","Dés custom"}
        div { class:"row mb-1 mt-4",
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
                    //info!("->>>{}",get_custom_dice(&dice_txt));
                    //sig_all_dice.set(all);
                    sig_result.set(get_custom_dice(&dice_txt));
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
        }
        div { class:"row mt-3", div {class:"col",
            div{
                button {
                    class:"btn-help",
                    type:"button",
                    "data-bs-toggle":"collapse",
                    "data-bs-target":"#collapseExample",
                    "aria-expanded":"false",
                    "aria-controls":"collapseExample",
                    "click -> Help"
                }

                div{id:"collapseExample", class:"collapse",
                    div {
                        style: "font-family: monospace; font-size: 0.9rem; line-height: 1.4;",
                        div { style: "font-weight: bold; margin-top: 0.5rem;", "Arithmetic" }
                        div { "Addition: 2d6 + 2" }
                        div { "Subtraction: 2d6 - 2" }
                        div { "Multiplication: 2d6 * 2" }
                        div { "Division (integer, rounded down): 2d6 / 2" }
                        div { "Division (integer, rounded up): 2d6 \\ 2" }
                        div { "Standard mathematical order of operations" }
                        div { "Parenthetical grouping: (2d6 + 2) * 2" }br {  }
                        div { style: "font-weight: bold; margin-top: 0.5rem;", "Dice modifiers" }
                        div { "Keep highest (advantage): 2d20kh, 2d20k" }
                        div { style: "margin-left: 1rem;", "With specific amount to keep: 3d20kh2" }
                        div { "Keep lowest (disadvantage): 2d20kl" }
                        div { style: "margin-left: 1rem;", "With specific amount to keep: 3d20kl2" }
                        div { "Reroll (once): 4d6r" }
                        div { style: "margin-left: 1rem;", "With specific condition: 4d6r>4, 4d6r>=5, 4d6r<3, 4d6r<=2, 4d6r4" }
                        div { "Reroll (recursive): 4d6rr" }
                        div { style: "margin-left: 1rem;", "With specific condition: 4d6rr>4, 4d6rr>=5, 4d6rr<3, 4d6rr<=2, 4d6rr4" }
                        div { "Explode (recursive): 4d6x" }
                        div { style: "margin-left: 1rem;", "With specific condition: 4d6x>4, 4d6x>=5, 4d6x<3, 4d6x<=2, 4d6x4" }
                        div { "Explode (once): 4d6xo" }
                        div { style: "margin-left: 1rem;", "With specific condition: 4d6xo>4, 4d6xo>=5, 4d6xo<3, 4d6xo<=2, 4d6xo4" }
                        div { "Minimum: 4d8min3" }
                        div { "Maximum: 4d8max6" }br {  }
                        div { style: "font-weight: bold; margin-top: 0.5rem;", "Dice modifier chaining" }
                        div { "Applied in the order they're specified: 4d6rr1x>4, 8d8min2kh4xo" }
                    }
                }
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
