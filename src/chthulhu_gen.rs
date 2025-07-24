use std::{collections::HashSet, rc::Rc};

use dioxus::{logger::tracing::info, prelude::*};
use rand::{Rng, SeedableRng};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

//use crate::DB;

pub(crate) fn get_random_carac() -> i32 {
    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    rng.random_range(4..18)
}

pub(crate) fn get_a_dice(max: u32) -> u32 {
    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    rng.random_range(1..max)
}

pub(crate) fn get_bonus(val: i32) -> String {
    match val {
        4..=5 => "-3".to_string(),
        6..=7 => "-2".to_string(),
        8..=9 => "-1".to_string(),
        10..=11 => "+0".to_string(),
        12..=13 => "+1".to_string(),
        14..=15 => "+2".to_string(),
        16..=17 => "+3".to_string(),
        18..=19 => "+4".to_string(),
        _ => "N.o.N".to_string(),
    }
}

fn get_archetype_base(arch: Vec<Archetype>, target: &str) -> String {
    info!("target: {}", target);
    let mut result = String::new();
    for archetype in arch {
        if archetype.name == target {
            result = archetype.capacite_spe_base;
            continue;
        }
    }
    result
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ArchetypeJson {
    datas: Vec<Archetype>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Archetype {
    index: u32,
    name: String,
    capacite_spe_base: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
struct AtoutGenerique {
    index: u32,
    name: String,
    atout_desc: String,
}

fn get_atout_generique() -> Vec<AtoutGenerique> {
    let conn: Rc<Connection> = use_context();
    let mut rqst_atout = conn.prepare("select * from atout_generique").unwrap();

    rqst_atout
        .query_map([], |r| {
            Ok(AtoutGenerique {
                index: r.get(0)?,
                name: r.get(1)?,
                atout_desc: r.get(2)?,
            })
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn get_archetype() -> Vec<Archetype> {
    let conn: Rc<Connection> = use_context();
    let mut rqst = conn.prepare("select * from archetype").unwrap();

    rqst.query_map([], |r| {
        Ok(Archetype {
            index: r.get(0)?,
            name: r.get(1)?,
            capacite_spe_base: r.get(2)?,
        })
    })
    .unwrap()
    .collect::<Result<Vec<_>, _>>()
    .unwrap()
}

#[component]
pub(crate) fn ChackGenerate() -> Element {
    let archetypes = get_archetype();
    let atouts_gen = get_atout_generique();
    let mut atout_names = Vec::new();

    for atout in atouts_gen {
        atout_names.push(atout.name);
    }

    let mut name_vec = Vec::new();

    for arch in &archetypes {
        name_vec.push(arch.name.clone());
        println!("result base : {arch:?}");
    }

    println!("{name_vec:?}");

    let mut sig_fo = use_signal(|| 0);
    let mut sig_dex = use_signal(|| 0);
    let mut sig_co = use_signal(|| 0);
    let mut sig_int = use_signal(|| 0);
    let mut sig_sag = use_signal(|| 0);
    let mut sig_cha = use_signal(|| 0);

    let mut sig_name = use_signal(String::new);
    let mut selected_atout = use_signal(HashSet::<String>::new);

    let txt_base = get_archetype_base(archetypes, &sig_name.read());
    info!("txt base : {txt_base}");

    info!("{:?}", selected_atout);

    rsx! {
        div {  class:"row mb-2",
            div {  class:"col",
                button { type:"button", class:"btn btn-md btn-outline-light m-1", id:"btnGen", onclick:
                    move |_| {info!("on génére click");
                    sig_fo.set(get_random_carac());
                    sig_dex.set(get_random_carac());
                    sig_co.set(get_random_carac());
                    sig_int.set(get_random_carac());
                    sig_sag.set(get_random_carac());
                    sig_cha.set(get_random_carac());
                },"Generate Value" }
            }
            div {  class:"col"}
        }
        div { class:"row",
            div { class: "col", id:"colCarac",
                div { class:"col",
                    div { class: "row mb-1",
                         div { class: "col-4", div { class:"btn btn-warning w-100", id:"btn1", "FOR" }}
                        div { class: "col-3", div {class:"btn btn-outline-warning w-100", id:"Fo","{sig_fo}" }}
                        div { class: "col-3",div { class:"btn btn-warning w-100", id:"btn11",  if *sig_fo.read() !=0 { {get_bonus(*sig_fo.read())} }else{ "0" } } }
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-4", div { class:"btn btn-warning w-100", id:"btn2", "CONS" } }
                        div { class:"col-3", div { class:"btn btn-outline-warning w-100", id:"Co", "{sig_co}" } }
                        div {class:"col-3",  div { class:"btn btn-warning w-100", id:"btn11",  if *sig_co.read() !=0 { {get_bonus(*sig_co.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-4", div { class:"btn btn-warning w-100", id:"btn2", "DEX" } }
                        div { class:"col-3", div { class:"btn btn-outline-warning w-100", id:"dex", "{sig_dex}" } }
                        div {class:"col-3",  div { class:"btn btn-warning w-100", id:"btn11",  if *sig_dex.read() !=0 { {get_bonus(*sig_dex.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-4", div { class:"btn btn-warning w-100", id:"btn2", "INT" } }
                        div { class:"col-3", div { class:"btn btn-outline-warning w-100", id:"int", "{sig_int}" } }
                        div {class:"col-3",  div { class:"btn btn-warning w-100", id:"btn11",  if *sig_int.read() !=0 { {get_bonus(*sig_int.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-4", div { class:"btn btn-warning w-100", id:"btn2", "SAG" } }
                        div { class:"col-3", div { class:"btn btn-outline-warning w-100", id:"sag", "{sig_sag}" } }
                        div {class:"col-3",  div { class:"btn btn-warning w-100", id:"btn11",  if *sig_sag.read() !=0 { {get_bonus(*sig_sag.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-4", div { class:"btn btn-warning w-100", id:"btn2", "CHA" } }
                        div { class:"col-3", div { class:"btn btn-outline-warning w-100", id:"cha", "{sig_cha}" } }
                        div {class:"col-3",  div { class:"btn btn-warning w-100", id:"btn11",  if *sig_cha.read() !=0 { {get_bonus(*sig_cha.read())} }else{ "0" } }}
                    }
                }

            }
            div { class: "col", id: "colAutre",
                div { class:"row",
                    div { class: "col", id:"col2",
                        div {  class:"btn btn-warning w-25", id:"btn4", "Bagout" }
                        div {  class:"btn btn-outline-warning m-1", id:"btn5", "E" }
                        div {  class:"btn btn-outline-warning m-1", id:"btn6", "F" }
                    }
                    div{
                        div { class: "col", id:"col3",
                            div { class:"btn btn-warning w-25", id:"btn7", "Torche" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                            div { class:"btn btn-outline-warning m-1", id:"btn9", "I" }
                        }
                    }
                    div{
                        div { class: "col", id:"col4",
                            div { class:"btn btn-warning w-25", id:"btn7", "SAN" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                            div { class:"btn btn-outline-warning m-1", id:"btn9", "I" }
                        }
                    }
                    div{
                        div { class: "col", id:"col5",
                            div { class:"btn btn-warning w-25", id:"btn7", "Richesse" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                            div { class:"btn btn-outline-warning m-1", id:"btn9", "I" }
                        }
                    }
                }
                div { class: "row mt-3",
                    div { class:"col",
                        label { "Choisissez votre classe : " }
                        select {
                                class: "btn btn-cth-eldritch",
                                value: "{sig_name}",
                                onchange: move |evt| {
                                    sig_name.set(evt.value());
                                },
                                option { value: "...", "..." }
                               for name in name_vec  {
                                    option { value: "{name}", "{name}" }
                                }
                        }

                    }
                }
                div { class:"row",
                    div { class:"col mt-3",
                        if (sig_name.read().as_str() != String::new()) && (sig_name.read().as_str() != "...") {
                            div { id:"text-archetype", class:"border text-bg-light p-3 rounded-2",
                                    {txt_base}
                            }
                        }
                    }
                }
                div {  class:"row",

                }

            }
        }
        div { class:"row my-3 border-top", id:"ligne2",
            div{ class:"col mt-2",
                        for atout in atout_names{
                                div {
                                    input {
                                        class: "form-check-input",
                                        r#type: "checkbox",
                                        id: "{atout}",
                                        checked: selected_atout().contains(&atout),
                                        oninput: move |evt| {
                                            //println!("1 atout : {atout}");
                                            if evt.checked(){
                                                //println!("2 checked");
                                                //println!("4 add");
                                                let mut sc = selected_atout();
                                                sc.insert(atout.clone());
                                                selected_atout.set(sc);
                                                //println!(" post add {:?}", selected_atout);
                                            }else {
                                                //println!("3 remove");
                                                let mut sc = selected_atout();
                                                sc.remove(&atout);
                                                selected_atout.set(sc);
                                            }
                                            //println!("---> {selected_atout:?}")
                                        },
                                    }
                                    label {
                                        class:"form-check-label",
                                        //r#for: "{atout.clone()}",
                                        " {atout}"
                                    }
                                }

                        } // end for
                }
            div { class:"col mt-2",
                div {
                    label { "les selectionnés:" }
                    div {
                        ul {
                            for select in selected_atout().iter(){
                                li { "{select:?}" }
                            }
                        }
                     }
                }//end des atouts selectionnés
             }
         }
    } //end rsx
}

/*


use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let mut selected_value = use_signal(|| String::new());
    let options = vec![
        ("fr", "Français"),
        ("en", "English"),
        ("es", "Español"),
    ];

    rsx! {
        select {
            value: "{selected_value}",
            onchange: move |evt| {
                selected_value.set(evt.value());
            },
            option { value: "", "Choisissez une langue..." }
            for (value, label) in options {
                option { value: "{value}", "{label}" }
            }
        }
    }
}





use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
}

#[component]
fn UserSelector() -> Element {
    let mut selected_user_id = use_signal(|| 0u32);
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
        User { id: 3, name: "Charlie".to_string() },
    ];

    rsx! {
        select {
            value: "{selected_user_id}",
            onchange: move |evt| {
                if let Ok(id) = evt.value().parse::<u32>() {
                    selected_user_id.set(id);
                }
            },
            option { value: "0", "Sélectionnez un utilisateur" }
            for user in users {
                option {
                    value: "{user.id}",
                    selected: selected_user_id() == user.id,
                    "{user.name}"
                }
            }
        }

        if selected_user_id() > 0 {
            p { "Utilisateur sélectionné : ID {selected_user_id}" }
        }
    }
}



*/
