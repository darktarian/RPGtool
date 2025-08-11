use dioxus::prelude::*;
use ndm::{Dice, DiceParseError, RollSet};

/*use tyche::{
	dice::modifier::{Condition, Modifier},
	Dice, Expr,
}*/

fn get_a_dice(dice: &str) -> String {
    match dice.parse::<Dice>() {
        Ok(de) => format!("{} => {}", dice, de.total()),
        Err(_) => "".to_string(),
    }
}

#[component]
pub(crate) fn DiceBoard() -> Element {
    let mut sig_result = use_signal(String::new);
    let mut sig_custom_dice = use_signal(String::new);
    let mut sig_all_dice = use_signal(String::new);

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
            div{ pre {class:"text-light",  
            "[count]d<sides>[/<H|L><keep>][![fuse]]
            ! for exploding dice
            Keeping the highest dice : H + nb dice
            Keeping the lowest dice : L + nb dice"}
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
