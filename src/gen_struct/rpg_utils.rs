use std::rc::Rc;

use crate::{
    gen_struct::cthulhu_struct::{random_distribution, Archetype, AtoutGenerique, Character, HackDice},
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
    let distrib = random_distribution(5,4,6);
    info!("{:?}", distrib);

    if let Some(distribution) = distrib{
        perso.bagou = HackDice::from_level(distribution[0]).to_string();
        perso.torche = HackDice::from_level(distribution[1]).to_string();
        perso.de_sm =HackDice::from_level(distribution[2]).to_string();
        perso.de_vie = HackDice::from_level(distribution[3]).to_string();
        perso.degat_armed = HackDice::from_level(distribution[4]+1).to_string();
        perso.degat_unarmed = HackDice::from_level(distribution[5]+1).to_string();
    }
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

pub(crate) fn get_archetype_base(arch: Vec<Archetype>, target: &str) -> String {
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
pub(crate) fn get_atout_generique() -> Vec<AtoutGenerique> {
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
pub(crate) fn get_archetype() -> Vec<Archetype> {
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
