use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct Caracterisques{
    pub(crate) fo : i32,
    pub(crate) con: i32,
    pub(crate) dex: i32,
    pub(crate) sag: i32,
    pub(crate) int: i32,
    pub(crate) cha: i32
}

/// pour le retour de la bdd et permettre par la suite de faire un choix (select) puis 3 case à cocher
#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct MetierFmBdd{
    pub(crate) name: String,
    pub(crate) skill: String,
    pub(crate) skill2: String,
    pub(crate) skill3: String
}

/// la struct pour completer le personnage
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct Metier{
    pub(crate) name: String,
    pub(crate) skill: String,
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
    pub(crate) capacite: HashMap<String,String>,
    pub(crate) metier: Metier,
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
/// 
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub(crate) struct AtoutGenerique {
    pub(crate) index: u32,
    pub(crate) name: String,
    pub(crate) atout_desc: String,
}
