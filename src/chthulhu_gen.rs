use std::{collections::{HashMap, HashSet}, rc::Rc};

use crate::{
    gen_struct::cthulhu_struct::{Archetype, AtoutGenerique, Caracterisques, Character, HackDice},
    AppContext,
};
use dioxus::{logger::tracing::info, prelude::*};
use rand::{Rng, SeedableRng};
use rusqlite::Connection;

///Genrateur des valeurs de caracteristiques (entre 4 et 18)
pub(crate) fn get_random_carac() -> i32 {
    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    rng.random_range(4..18)
}

pub(crate) fn set_ressources(mut perso: Character) -> Character  {
    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    let pool = 10;
    
    perso.bagou = HackDice::from(rng.random_range(1..6)).to_string();
    perso.torche = HackDice::from(rng.random_range(1..6)).to_string();
    perso.richesse = HackDice::from(rng.random_range(1..6)).to_string();
    perso.de_sm =HackDice::from(rng.random_range(1..6)).to_string();
    perso.de_vie = HackDice::from(rng.random_range(1..6)).to_string();


    perso
}



///
/// Petite fonction pour attribuer les bonus au caracteristiques.
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

/// Requet vers la base sqlite pour obtenir les atout
/// TODO : check des atouts avancés ?
fn get_atout_generique() -> Vec<AtoutGenerique> {
    let ctx = use_context::<AppContext>();
    let conn:Rc<Connection>  = ctx.connect;
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

///Requete vers la base sqlite pour obtenir les données d'archetypes.
///
fn get_archetype() -> Vec<Archetype> {
    let ctx = use_context::<AppContext>();
    let conn: Rc<Connection> = ctx.connect;
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

///On assemble les parties de la vue ici.
#[component]
pub(crate) fn CthulhuGenAll() -> Element {
    rsx! {
        ChackGenerate {  },
        Get_atout {  }
    }
}

///
/// Ou l'on genere la partie haute de cthulhu genrator avec les caracteristique.
///
#[component]
pub(crate) fn ChackGenerate() -> Element {
    let mut sig_fo = use_signal(|| 0);
    let mut sig_dex = use_signal(|| 0);
    let mut sig_co = use_signal(|| 0);
    let mut sig_int = use_signal(|| 0);
    let mut sig_sag = use_signal(|| 0);
    let mut sig_cha = use_signal(|| 0);

    rsx! {
        div {  class:"row mb-2 align-items-start",
            div {  class:"col",
                button { type:"button", class:"btn btn-md btn-outline-light m-1", id:"btnGen", onclick:
                    move |_| {info!("on génére click");
                    sig_fo.set(get_random_carac());
                    sig_dex.set(get_random_carac());
                    sig_co.set(get_random_carac());
                    sig_int.set(get_random_carac());
                    sig_sag.set(get_random_carac());
                    sig_cha.set(get_random_carac());

                    let ctx: AppContext = use_context();
                    let mut perso: Signal<Character> = ctx.cthulhu_char;
                    info!(" av {}", perso);
                    
                    let carac = Caracterisques{
                        fo:sig_fo(),
                        con:sig_co(),
                        dex:sig_dex(),
                        sag:sig_sag(),
                        int:sig_int(),
                        cha:sig_cha(),
                        fo_bonus:get_bonus(sig_fo()),
                        co_bonus: get_bonus(sig_co()),
                        dex_bonus: get_bonus(sig_dex()),
                        sage_bonus: get_bonus(sig_sag()),
                        int_bonus: get_bonus(sig_int()),
                        cha_bonus: get_bonus(sig_cha())
                    };
                   perso.write().carac = carac;
                    info!(" ap: {}", perso);

                },"Generate Value" }
            }
            div {  class:"col"}
        }
        div { class:"row",
            div { class: "col-4", id:"colCarac",
                div { class:"col",
                    div { class: "row mb-1",
                         div { class: "col-3", div { class:"btn btn-warning", style:"width:60px", id:"btn1", "FOR" }}
                        div { class: "col-2", div {class:"btn btn-outline-warning", style:"width:45px", id:"Fo","{sig_fo}" }}
                        div { class: "col-2",div { class:"btn btn-outline-warning", style:"width:45px", id:"btn11",  if *sig_fo.read() !=0 { {get_bonus(*sig_fo.read())} }else{ "0" } } }
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-3", div { class:"btn btn-warning", style:"width:60px", id:"btn2", "CONS" } }
                        div { class:"col-2", div { class:"btn btn-outline-warning", style:"width:45px", id:"Co", "{sig_co}" } }
                        div {class:"col-2",  div { class:"btn btn-outline-warning", style:"width:45px", id:"btn11",  if *sig_co.read() !=0 { {get_bonus(*sig_co.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-3", div { class:"btn btn-warning", style:"width:60px", id:"btn2", "DEX" } }
                        div { class:"col-2", div { class:"btn btn-outline-warning", style:"width:45px", id:"dex", "{sig_dex}" } }
                        div {class:"col-2",  div { class:"btn btn-outline-warning", style:"width:45px", id:"btn11",  if *sig_dex.read() !=0 { {get_bonus(*sig_dex.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-3", div { class:"btn btn-warning", style:"width:60px", id:"btn2", "INT" } }
                        div { class:"col-2", div { class:"btn btn-outline-warning", style:"width:45px", id:"int", "{sig_int}" } }
                        div {class:"col-2",  div { class:"btn btn-outline-warning", style:"width:45px", id:"btn11",  if *sig_int.read() !=0 { {get_bonus(*sig_int.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-3", div { class:"btn btn-warning", style:"width:60px", id:"btn2", "SAG" } }
                        div { class:"col-2", div { class:"btn btn-outline-warning", style:"width:45px", id:"sag", "{sig_sag}" } }
                        div {class:"col-2",  div { class:"btn btn-outline-warning", style:"width:45px", id:"btn11",  if *sig_sag.read() !=0 { {get_bonus(*sig_sag.read())} }else{ "0" } }}
                    }
                }
                div { class:"col",
                    div {class:"row mb-1",
                        div {class:"col-3", div { class:"btn btn-warning", style:"width:60px", id:"btn2", "CHA" } }
                        div { class:"col-2", div { class:"btn btn-outline-warning", style:"width:45px", id:"cha", "{sig_cha}" } }
                        div {class:"col-2",  div { class:"btn btn-outline-warning", style:"width:45px", id:"btn11",  if *sig_cha.read() !=0 { {get_bonus(*sig_cha.read())} }else{ "0" } }}
                    }
                }
            }
            div { class: "col-3", id: "colAutre",
                    div { class: "col", id:"col2",
                        div {  class:"btn btn-warning",style:"width:80px", id:"btn4", "Bagout" }
                        div {  class:"btn btn-outline-warning m-1", id:"btn5", "E" }
                    }
                    div{
                        div { class: "col", id:"col3",
                            div { class:"btn btn-warning",style:"width:80px", id:"btn5", "Torche" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                        }
                    }
                    div{
                        div { class: "col", id:"col4",
                            div { class:"btn btn-warning",style:"width:80px", id:"btn6", "SAN" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                        }
                    }
                    div{
                        div { class: "col", id:"col5",
                            div { class:"btn btn-warning", style:"width:80px", id:"btn7", "Richesse" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                        }
                    }
            }
            div { class:"col-3", id:"dvie",
                    div{ class:"row",
                        div { class: "col", id:"col5",
                            div { class:"btn btn-warning", style:"width:70px", id:"btn8", "DV" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                        }
                    }
                    div{ class:"row",
                        div { class: "col", id:"col5",
                            div { class:"btn btn-warning",style:"width:70px", id:"btn9", "PdV" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                        }
                    }
          }
        }
    }
}

#[component]
pub(crate) fn Get_atout() -> Element {
    let archetypes = get_archetype();
    let mut name_vec = Vec::new();
    for arch in &archetypes {
        name_vec.push(arch.name.clone());
        println!("result base : {arch:?}");
    }

    let atouts_gen = get_atout_generique();
    let mut atout_names = Vec::new();

    for atout in atouts_gen {
        atout_names.push(atout.name);
    }

    let mut sig_name = use_signal(String::new);
    let txt_base = get_archetype_base(archetypes, &sig_name.read());
    let mut sig_atout_name = use_signal(String::new);
    let mut selected_atout = use_signal(HashSet::<Rc<String>>::new);

    rsx! {
        div { class: "row mt-3",
            div { class:"col",
                div {  class:"row",
                    div {  class:"col-5",
                        ///////
                        label { "Choisissez vos 2 atouts : " }
                        br {  }
                        select {
                                class: "btn",
                                value: "{sig_atout_name}",
                                onchange: move |evt| {
                                    sig_atout_name.set(evt.value());
                                    let mut sc = selected_atout();
                                    info!("{sig_atout_name}");
                                    //on limite à 4 le nombre d'atout à choisir
                                    //todo -> utiliser un signal pour décompter.
                                    if sc.len()<3{
                                        sc.insert(sig_atout_name().into());
                                        selected_atout.set(sc);
                                    }
                                },
                                option { value: "...", "..." }
                                for atout in atout_names  {
                                    option { value: "{atout}", "{atout}" }
                                }
                        }
                        ///////
                        div { class:"row mt-2",
                            div { class: "col",
                                label { "les selectionnés:" }
                                div {
                                    for select in selected_atout().iter().cloned(){
                                            "{select} "
                                        p{ display:"inline",
                                            button {class: "btn btn-sm btn-close btn-danger",
                                                    type:"button",
                                                    onclick: move |_| {
                                                        let mut sc = selected_atout();
                                                        sc.remove(&select);
                                                        selected_atout.set(sc);
                                                    },
                                            }
                                        }
                                    }
                                }
                            }//end des atouts selectionnés
                        }
                        //
                    }
                    div{class:"col-1",
                        div{class:"badge text-bg-danger fs-6", "OU" }
                    }
                    div{class:"col-5",
                        //mettre ici le choix des archétypes.
                        div { class: "row",
                            div { class:"col ml-3",
                                label { "Choisissez votre classe : " }
                                select {
                                        class: "btn",
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
                    }
                }

            }//fin col1
            // besoin d'ajouter une ligne en plus pour mettre en vis à vis les choix d'archetypes.
        }
        div { class:"row mt-5",
            div { class:"col",
                button { class:"btn btn-warning", 
                onclick: move |_ |{
                    let ctx: AppContext = use_context();
                    let mut perso: Signal<Character> = ctx.cthulhu_char;
                    let mut capacite: HashMap<String, String> = HashMap::new();
                    for cap in selected_atout().iter().cloned(){
                        capacite.insert(cap.to_string(), "test".to_string());
                    }
                    
                    perso.write().capacite = capacite;
                    info!(" le perso ----> {}", perso());




                } ,"Generer PDF" }
            }
        }
    }
}

#[component]
pub(crate) fn Get_metier() -> Element {
    rsx! {}
}
