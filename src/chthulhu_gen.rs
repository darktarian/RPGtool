use std::{collections::{HashMap, HashSet}, rc::Rc};

use crate::{
    gen_struct::{cthulhu_struct::{Character}, 
    rpg_utils::{get_archetype, get_archetype_base, get_atout_generique, get_bonus}},
    AppContext,
};
use dioxus::{logger::tracing::info, prelude::*};

use crate::pdfprinter::pdf::hack_to_pdf;

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

    let mut sig_bagout = use_signal(String::new);
    let mut sig_torche = use_signal(String::new);
    let mut sig_san = use_signal(String::new);
    let mut sig_dv = use_signal(String::new);
    let mut sig_arme = use_signal(String::new);
    let mut sig_unarmed = use_signal(String::new);



    rsx! {
        div {  class:"row mb-2 align-items-start",
            div {  class:"col",
                button { type:"button", class:"btn btn-md btn-outline-light m-1", id:"btnGen", onclick:
                    move |_| {info!("on génére click");
                    let mut ctx: AppContext = use_context();
                    let generated_pj = Character::generate_pj();
                    //info!("TEST -----> {generated_pj}");
                    
                    sig_fo.set(generated_pj.carac.fo);
                    sig_dex.set(generated_pj.carac.dex);
                    sig_co.set(generated_pj.carac.con);
                    sig_int.set(generated_pj.carac.int);
                    sig_sag.set(generated_pj.carac.sag);
                    sig_cha.set(generated_pj.carac.cha);
                    sig_bagout.set(generated_pj.bagou.clone());
                    sig_torche.set(generated_pj.torche.clone());
                    sig_san.set(generated_pj.de_sm.clone());
                    sig_dv.set(generated_pj.de_vie.clone());
                    sig_arme.set(generated_pj.degat_armed.clone());
                    sig_unarmed.set(generated_pj.degat_unarmed.clone());

                    ctx.cthulhu_char.set(generated_pj);
                    

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
                        div {  class:"btn btn-outline-warning m-1", id:"btn5", if sig_bagout().is_empty() {" "}else{ {sig_bagout} }}
                    }
                    div{
                        div { class: "col", id:"col3",
                            div { class:"btn btn-warning",style:"width:80px", id:"btn5", "Torche" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", {sig_torche}  }
                        }
                    }
                    div{
                        div { class: "col", id:"col4",
                            div { class:"btn btn-warning",style:"width:80px", id:"btn6", "SAN" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", {sig_san} }
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
                            div { class:"btn btn-outline-warning m-1", id:"btn8", {sig_dv} }
                        }
                    }
                    div{ class:"row",
                        div { class: "col", id:"col5",
                            div { class:"btn btn-warning",style:"width:70px", id:"btn9", "PdV" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", "H" }
                        }
                    }
                    div{ class:"row",
                        div { class: "col", id:"col5",
                            div { class:"btn btn-warning", style:"width:70px", id:"btn10", "Armé" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", {sig_arme} }
                        }
                    }
                    div{ class:"row",
                        div { class: "col", id:"col5",
                            div { class:"btn btn-warning",style:"width:70px", id:"btn11", "Non Armé" }
                            div { class:"btn btn-outline-warning m-1", id:"btn8", {sig_unarmed} }
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

    let atouts_gen = get_atout_generique(None);
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
                        /* 
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
                        */
                    }
                    div{class:"col-1",
                        //div{class:"badge text-bg-danger fs-6", "OU" }
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
                    //info!("\n\nle perso ------------> {}", perso());
                    hack_to_pdf(perso());



                } ,"Generer PDF" }
            }
        }
    }
}

#[component]
pub(crate) fn Get_metier() -> Element {
    rsx! {}
}
