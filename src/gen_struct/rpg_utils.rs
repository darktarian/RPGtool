use std::{collections::HashSet, rc::Rc};
use crate::{
    AppContext, gen_struct::cthulhu_struct::{
        Archetype, AtoutGenerique, Character, CharacterName, HackDice, random_distribution
    }
};
use chrono::{Local};
use dioxus::{logger::tracing::info, prelude::*};
use rand::{Rng, SeedableRng};
use rusqlite::{Connection, Result};

///Genrateur des valeurs de caracteristiques (entre 7 et 18)
pub(crate) fn get_random_carac() -> i32 {
    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    rng.random_range(7..18)
}

///Genrateur des valeurs de caracteristiques (entre 1 et 22)
pub(crate) fn get_random_tarot_card() -> i32 {
    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    rng.random_range(1..22)
}

///Genrateur des valeurs de caracteristiques (entre 1 et 4)
pub(crate) fn get_random_d4() -> i32 {
    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    rng.random_range(1..4)
}

pub(crate) fn set_ressources(mut perso: Character) -> Character {
    let distrib = random_distribution(5, 4, 6);
    info!("{:?}", distrib);

    if let Some(distribution) = distrib {
        perso.bagou = HackDice::from_level(distribution[0]).to_string();
        perso.torche = HackDice::from_level(distribution[1]).to_string();
        perso.de_sm = HackDice::from_level(distribution[2]).to_string();
        perso.de_vie = HackDice::from_level(distribution[3]).to_string();
        perso.degat_armed = HackDice::from_level(distribution[4] + 1).to_string();
        perso.degat_unarmed = HackDice::from_level(distribution[5] + 1).to_string();
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
/// TODO -> passeg d"argument du nombre d"atout à renvoyer de maniere random.
pub(crate) fn get_atout_generique(nb: Option<u8>) -> Vec<AtoutGenerique> {
    let ctx = use_context::<AppContext>();
    let conn: Rc<Connection> = ctx.connect;
    let mut rqst_atout = conn.prepare("select * from atout_generique where is_advanced = 0").unwrap();

    let atouts:Vec<AtoutGenerique> = rqst_atout
        .query_map([], |r| {
            Ok(AtoutGenerique {
                index: r.get(0).unwrap(),
                name: r.get(1).unwrap(),
                atout_desc: r.get(2).unwrap(),
            })
        })
        .unwrap()
        .collect::<Result<Vec<AtoutGenerique>, _>>()
        .unwrap();

    match nb {
        Some(nb) => {
            let mut indexes = HashSet::new();
            let mut selected = Vec::new();
            let val = getrandom::u64().unwrap();
            let mut rng = rand::rngs::SmallRng::seed_from_u64(val);

            // on essaie de ne pas prendre deux fois le meme atout
            while indexes.len() < nb.into() {
                let index = rng.random_range(0..atouts.len());
                if !indexes.contains(&index) {
                    indexes.insert(index);
                }
            }

            //on prend les atouts
            for index in indexes {
                if let Some(atout) = atouts.get(index) {
                    selected.push(atout.clone());
                }
            }

            selected
        }
        None => atouts,
    }
}

///Requete vers la base sqlite pour obtenir les données d"archetypes.
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

pub(crate) fn get_a_name() -> CharacterName {
        let ctx = use_context::<AppContext>();
    let conn: Rc<Connection> = ctx.connect;
    let mut rqst = conn.prepare("select Title, GivenName, Surname from FakeNameGenerator where NameSet != \"Hobbit\"").unwrap();

   let name_vec =  rqst.query_map([], |r| {
        Ok(
            CharacterName{
                name: r.get(1).unwrap(),
                surname:r.get(2).unwrap(),
                title: r.get(0).unwrap(),
            }
        )
    }).unwrap()
    .collect::<Result<Vec<_>, _>>()
    .unwrap();

    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);
    let i = rng.random_range(0..name_vec.len());

    if let Some(name) = name_vec.get(i){
        name.clone()
    }else{
        CharacterName::default()
    }


}

pub fn horodate_filename(filename: &str) -> String {
    let now = Local::now();
    let timestamp = now.format("%Y%m%d_%H%M%S").to_string();

    // Sépare le nom et l"extension s’il y en a une
    match filename.rsplit_once(".") {
        Some((base, ext)) => format!("{}_{}.{}", base, timestamp, ext),
        None => format!("{}_{}", filename, timestamp),
    }
}


