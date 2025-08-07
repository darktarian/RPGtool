use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use serde::{Deserialize, Serialize};

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
            "FOR: {} ({}), CON: {} ({}), DEX: {} ({}), SAG: {} ({}), INT: {} ({}), CHA: {} ({})",
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
