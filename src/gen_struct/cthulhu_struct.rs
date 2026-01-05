use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::gen_struct::{generate_tarot::get_tarot_card_effect, rpg_utils::{
    get_a_name, get_atout_generique, get_bonus, get_random_carac, get_random_tarot_card, set_ressources
}};

/// Permet de distribuer 10pt pour les ressources
/// on part d'une base fixe qui intègre que les deux derniers (degâts armé et non armés comme à la valeurs de 1).
/// je fixe bagout, dés de vie, san  et torche avec un min de 1 ou 2 et je ne distribue que 4pt au lieu de 10.
pub(crate) fn random_distribution(objects: usize, points: i32, max: i32) -> Option<Vec<i32>> {
    let base = vec![2, 2, 2, 2, 0, 0];

    let val = getrandom::u64().unwrap();
    let mut rng = rand::rngs::SmallRng::seed_from_u64(val);

    let mut result = base.clone();
    let mut remaining = points;

    // Distribuer aléatoirement les points restants
    while remaining > 0 {
        let i = rng.random_range(0..objects);
        if result[i] < max {
            result[i] += 1;
            remaining -= 1;
        }
    }

    Some(result)
}

pub(crate) enum HackDice {
    D1,
    D4,
    D6,
    D8,
    D10,
    D12,
}

impl HackDice {
    #[allow(dead_code)]
    pub(crate) fn get_dice_level(self) -> i32 {
        match self {
            HackDice::D1 => 1,
            HackDice::D4 => 2,
            HackDice::D6 => 3,
            HackDice::D8 => 4,
            HackDice::D10 => 5,
            HackDice::D12 => 6,
        }
    }
    pub(crate) fn from_level(value: i32) -> HackDice {
        match value {
            1 => HackDice::D1,
            2 => HackDice::D4,
            3 => HackDice::D6,
            4 => HackDice::D8,
            5 => HackDice::D10,
            6 => HackDice::D12,
            _ => HackDice::D1, // valeur par défaut (au choix)
        }
    }

    pub(crate) fn sub_one(self)-> HackDice{
        match self{
            HackDice::D1 => HackDice::D1,
            HackDice::D4 => HackDice::D1,
            HackDice::D6 => HackDice::D4,
            HackDice::D8 => HackDice::D6,
            HackDice::D10 => HackDice::D8,
            HackDice::D12 => HackDice::D12
        }
    }

    pub(crate) fn add_one(self)-> HackDice{
        match self {
            HackDice::D1 => HackDice::D4,
            HackDice::D4 => HackDice::D6,
            HackDice::D6 => HackDice::D8,
            HackDice::D8 => HackDice::D10,
            HackDice::D10 => HackDice::D12,
            HackDice::D12 => HackDice::D12
        }
    }

    pub(crate) fn max(self)-> u8{
        match self {
            HackDice::D1 => 1,
            HackDice::D4 => 4,
            HackDice::D6 => 6,
            HackDice::D8 => 8,
            HackDice::D10 => 10,
            HackDice::D12 => 12
        }
    }

}

// Implémentation de Display
impl fmt::Display for HackDice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HackDice::D1 => "1",
            HackDice::D4 => "D4",
            HackDice::D6 => "D6",
            HackDice::D8 => "D8",
            HackDice::D10 => "D10",
            HackDice::D12 => "D12",
        };
        write!(f, "{}", s)
    }
}

// Implémentation de From<i32>
impl From<i32> for HackDice {
    fn from(value: i32) -> Self {
        match value {
            1 => HackDice::D1,
            4 => HackDice::D4,
            6 => HackDice::D6,
            8 => HackDice::D8,
            10 => HackDice::D10,
            12 => HackDice::D12,
            _ => HackDice::D6, // valeur par défaut (au choix)
        }
    }

}

impl From<&str> for HackDice{
    fn from(value: &str) -> Self {
        match value {
            "1" => HackDice::D1,
            "D4" => HackDice::D4,
            "D6" => HackDice::D6,
            "D8" => HackDice::D8,
            "D10" => HackDice::D10,
            "D12" => HackDice::D12,
            _ => HackDice::D6, // valeur par défaut (au choix)
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct Caracterisques {
    pub(crate) fo: i32,
    pub(crate) fo_bonus: String,
    pub(crate) con: i32,
    pub(crate) co_bonus: String,
    pub(crate) dex: i32,
    pub(crate) dex_bonus: String,
    pub(crate) sag: i32,
    pub(crate) sage_bonus: String,
    pub(crate) int: i32,
    pub(crate) int_bonus: String,
    pub(crate) cha: i32,
    pub(crate) cha_bonus: String,
}

impl Display for Caracterisques {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FOR: {} ({}), CON: {} ({}), DEX: {} ({}), SAG: {} ({}), INT: {} ({}), CHA: {} ({}),",
            self.fo,
            self.fo_bonus,
            self.con,
            self.co_bonus,
            self.dex,
            self.dex_bonus,
            self.sag,
            self.sage_bonus,
            self.int,
            self.int_bonus,
            self.cha,
            self.cha_bonus
        )?;

        /*writeln!(f, "FOR : {:>2} ({})", self.fo, self.fo_bonus)?;
        writeln!(f, "CON : {:>2} ({})", self.con, self.co_bonus)?;
        writeln!(f, "DEX : {:>2} ({})", self.dex, self.dex_bonus)?;
        writeln!(f, "SAG : {:>2} ({})", self.sag, self.sage_bonus)?;
        writeln!(f, "INT : {:>2} ({})", self.int, self.int_bonus)?;
        writeln!(f, "CHA : {:>2} ({})", self.cha, self.cha_bonus)?;*/
        Ok(())
    }
}

/// pour le retour de la bdd et permettre par la suite de faire un choix (select) puis 3 case à cocher
#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct MetierFmBdd {
    pub(crate) name: String,
    pub(crate) skill: String,
    pub(crate) skill2: String,
    pub(crate) skill3: String,
}

/// la struct pour completer le personnage
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct Metier {
    pub(crate) name: String,
    pub(crate) skill: String,
}

impl Display for Metier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.skill)
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct Character {
    pub(crate) carac: Caracterisques,
    pub(crate) name: String,
    pub(crate) save_primary: String,
    pub(crate) save_secondary: String,
    pub(crate) torche: String,
    pub(crate) bagou: String,
    pub(crate) de_vie: String,
    pub(crate) pdv: u8,
    pub(crate) richesse: String,
    pub(crate) de_sm: String,
    pub(crate) degat_unarmed: String,
    pub(crate) degat_armed: String,
    pub(crate) capacite: HashMap<String, String>,
    pub(crate) metier: Metier,
}

impl Character {

    pub(crate) fn generate_pj_tarot()->Character{
        let name = get_a_name();
        let mut character = Character{
            carac: Caracterisques { 
                fo: 11, fo_bonus:"".to_string(), 
                con: 11, co_bonus: "".to_string(), 
                dex: 11, dex_bonus: "".to_string(), 
                sag: 11, sage_bonus: "".to_string(), 
                int: 11, int_bonus: "".to_string(), 
                cha: 11, cha_bonus: "".to_string() },
            name: format!("{} {} {}", name.title.replace(".", ""), name.name, name.surname),
            torche: "D8".to_string(),
            bagou: "D8".to_string(),
            de_vie: "D8".to_string(),
            de_sm: "D8".to_string(),
            degat_unarmed: "1".to_string(),
            degat_armed: "D8".to_string(),
            ..Default::default()
        };

        let mut previous_card = Vec::new();
        for _x in 0..4{
            let mut nb = get_random_tarot_card();
            while  previous_card.contains(&nb){
                nb = get_random_tarot_card();
            }
            previous_card.push(nb);

            character = get_tarot_card_effect(nb, character);
            println!("carac after : {:?}", character.carac);
        }

        character.carac.fo_bonus = get_bonus(character.carac.fo);
        character.carac.dex_bonus = get_bonus(character.carac.dex);
        character.carac.co_bonus = get_bonus(character.carac.con);
        character.carac.int_bonus = get_bonus(character.carac.int);
        character.carac.sage_bonus = get_bonus(character.carac.sag);
        character.carac.cha_bonus = get_bonus(character.carac.cha);

        //on pourrait passer par une ref ici non ...
        let mut character = set_ressources(character);
        let atouts = get_atout_generique(Some(3));

        for at in atouts {
            character.capacite.insert(at.name, at.atout_desc);
        }

        character
    }


    pub(crate) fn generate_pj() -> Character {
        let name = get_a_name();
        let mut character = Character {
            name : format!("{} {} {}", name.title.replace(".", ""), name.name, name.surname),
            carac: Caracterisques {
                fo: get_random_carac(),
                fo_bonus: "".to_string(),
                dex: get_random_carac(),
                dex_bonus: "".to_string(),
                con: get_random_carac(),
                co_bonus: "".to_string(),
                int: get_random_carac(),
                int_bonus: "".to_string(),
                sag: get_random_carac(),
                sage_bonus: "".to_string(),
                cha: get_random_carac(),
                cha_bonus: "".to_string(),
            },
            ..Default::default()
        };

        character.carac.fo_bonus = get_bonus(character.carac.fo);
        character.carac.dex_bonus = get_bonus(character.carac.dex);
        character.carac.co_bonus = get_bonus(character.carac.con);
        character.carac.int_bonus = get_bonus(character.carac.int);
        character.carac.sage_bonus = get_bonus(character.carac.sag);
        character.carac.cha_bonus = get_bonus(character.carac.cha);

        //on pourrait passer par une ref ici non ...
        let mut character = set_ressources(character);
        let atouts = get_atout_generique(Some(3));

        for at in atouts {
            character.capacite.insert(at.name, at.atout_desc);
        }

        character
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Nom : {}", self.name)?;
        writeln!(f, "Caractéristiques : {}", self.carac)?;
        writeln!(f, "Métier : {}", self.metier)?;
        writeln!(f, "Sauvegarde primaire : {}", self.save_primary)?;
        writeln!(f, "Sauvegarde secondaire : {}", self.save_secondary)?;
        writeln!(f, "Torche : {}", self.torche)?;
        writeln!(f, "Bagou : {}", self.bagou)?;
        writeln!(f, "Dé de vie : {}", self.de_vie)?;
        writeln!(f, "PDV : {}", self.pdv)?;
        writeln!(f, "Richesse : {}", self.richesse)?;
        writeln!(f, "Dé de santé mentale : {}", self.de_sm)?;
        writeln!(f, "Dégâts (non armé) : {}", self.degat_unarmed)?;
        writeln!(f, "Dégâts (armé) : {}", self.degat_armed)?;

        // un peu de mise en forme ....
        writeln!(f, "Capacités :")?;
        for (key, value) in &self.capacite {
            writeln!(f, "  - {}: {}", key, value)?;
        }

        Ok(())
    }
}

/// Structure de retour de la requete SQLITE pour les archetype.
///
#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Archetype {
    pub(crate) index: u32,
    pub(crate) name: String,
    pub(crate) capacite_spe_base: String,
}

/// Structure de retour de la requete SQLITE pour les atouts.
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct AtoutGenerique {
    pub(crate) index: u32,
    pub(crate) name: String,
    pub(crate) atout_desc: String,
}



#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct CharacterName{
    pub(crate) title: String,
    pub(crate) name: String,
    pub(crate) surname: String
}
