use crate::gen_struct::{cthulhu_struct::{Character, HackDice}, rpg_utils::get_random_d4};




pub(crate) fn get_tarot_card_effect(card: i32, mut character:  Character)-> Character{
    println!("carte appliquée: {}", card);

    match card {
        1 =>{
            character.carac.int +=1;
            character.carac.cha-=1;
            character.carac.sag+=2;
            let dice = HackDice::from(character.de_sm.as_str()).sub_one();
            character.de_sm = dice.to_string();
            character.metier.name = "Archéologue / Archiviste / Bibliothécaire / Medium".to_string();
        },
        2 =>{
            character.carac.sag+=2;
            let dice = HackDice::from(character.bagou.as_str()).add_one();
            character.bagou = dice.to_string();
            character.metier.name = "Acteur / Athlète / Négociateur / Pasteur".to_string()
        },
        3 => {
            character.carac.fo+=2;
            character.carac.dex+=2;
            character.carac.con-=2;
            character.carac.int-=2;
            character.metier.name = "Aviateur / Pompier / Soldat".to_string();
        },
        4 => {
            character.carac.con-=1;
            character.carac.cha+=2;
            character.carac.sag+=1;
            let dice = HackDice::from(character.torche.as_str()).sub_one();
            character.torche = dice.to_string();
            character.metier.name = "Dilettante / Entrepreneur / Occultiste".to_string()
        },
        5 => {
            character.carac.sag-=2;
            let dice = HackDice::from(character.degat_armed.as_str()).add_one();
            character.degat_armed = dice.to_string();
            character.metier.name = "Espion / Criminel / Soldat".to_string();
        },
        6 => {
            character.carac.dex-=1;
            character.carac.con-=1;
            character.carac.int+=1;
            character.carac.cha+=1;
            character.metier.name = "Médium / Missionnaire / Occultiste".to_string();
        },
        7 => {
            character.carac.dex-=2;
            character.carac.cha+=1;
            character.carac.sag+=1;
            let dice = HackDice::from(character.bagou.as_str()).add_one();
            character.bagou = dice.to_string();
            character.metier.name = "Garde forestier / guide touristique / Négociateur".to_string();
        },
        8 => {
            character.carac.dex+=2;
            character.carac.con+=2;
            character.carac.int-=1;
            let dice = HackDice::from(character.de_vie.as_str()).sub_one();
            character.de_vie = dice.to_string();
            character.metier.name = "Barman / Criminel / Soldat".to_string();
        },
        9 => {
            character.metier.name = "Académicien / Détective / Vagabond".to_string();
            character.carac.dex+=1;
            character.carac.int+=1;
            character.carac.cha-=1;
            character.carac.sag+=1;
            let dice = HackDice::from(character.torche.as_str()).add_one();
            character.torche = dice.to_string();
            let dice2 = HackDice::from(character.bagou.as_str()).sub_one();
            character.bagou = dice2.to_string();
            let dice3 = HackDice::from(character.degat_armed.as_str()).sub_one();
            character.degat_armed = dice3.to_string();
        },
        10 => {
            character.carac.con+=2;
            character.carac.cha-=2;
            let dice = HackDice::from(character.degat_armed.as_str()).add_one();
            character.degat_armed = dice.to_string();

            //on doit choisir au hasard une ressource à minorer
            let ressource = get_random_d4();
            match ressource {
                1 => {
                    let dice = HackDice::from(character.de_sm.as_str()).sub_one();
                    character.de_sm = dice.to_string();
                },
                2 => {
                    let dice = HackDice::from(character.bagou.as_str()).sub_one();
                    character.bagou = dice.to_string();
                },
                3 => {
                    let dice = HackDice::from(character.de_sm.as_str()).sub_one();
                    character.de_sm = dice.to_string();
                },
                4 => {
                    let dice = HackDice::from(character.torche.as_str()).sub_one();
                    character.torche = dice.to_string();
                },
                _ => {}
            }
            character.metier.name = "Magicien / Musicien".to_string()
        },
        11 => {
            character.carac.con+=2;
            character.carac.cha-=2;
            let dice = HackDice::from(character.de_sm.as_str()).sub_one();
            character.de_sm = dice.to_string();
            character.metier.name = "Garde forestier / Escroc / Ouvrier".to_string();
        },
        12 => {
            character.carac.fo-=1;
            character.carac.dex-=1;
            character.carac.con-=1;
            character.carac.sag+=1;
            let dice = HackDice::from(character.de_sm.as_str()).add_one();
            character.de_sm = dice.to_string();
            character.metier.name = "Game designer / Musicien / Vagabond".to_string()
        },
        13 => {
            character.metier.name = "Artiste de cirque / Astronome / Docteur".to_string();
            character.carac.int +=3;
            character.carac.sag-=1;
            let dice = HackDice::from(character.bagou.as_str()).sub_one();
            character.bagou = dice.to_string();
        },
        14 => {
            character.carac.int-=1;
            character.carac.cha-=1;
            let dice = HackDice::from(character.de_vie.as_str()).add_one();
            character.de_vie = dice.to_string();
            character.metier.name = "Docteur / Missionnaire / Pasteur".to_string()
        },
        15 => {
            character.metier.name = "Archéologue / Journaliste / Missionnaire".to_string();
            character.carac.fo-=1;
            character.carac.int-=1;
            let dice = HackDice::from(character.torche.as_str()).add_one();
            character.torche = dice.to_string();
        },
        16 => {
            character.carac.dex+=1;
            character.carac.con-=1;
            character.carac.int-=1;
            character.carac.cha-=1;
            character.metier.name = "Archéologue / Garde forestier / Missionnaire".to_string();
        },
        17 => {
            character.metier.name = "Astronome / Dilettante / Représentant de commerce".to_string();
            character.carac.int+=2;
            let dice = HackDice::from(character.degat_armed.as_str()).sub_one();
            character.degat_armed = dice.to_string();
        },
        18 => {
            character.carac.fo-=1;
            character.carac.dex+=1;
            character.carac.sag+=2;
            let dice = HackDice::from(character.de_vie.as_str()).sub_one();
            character.de_vie = dice.to_string();
            character.metier.name = "Criminel / Escroc / Journaliste".to_string();
        },
        19 => {
            character.metier.name = "Artisan / Ouvrier / Technicien de Police".to_string();
            character.carac.dex-=2;
            character.carac.dex+=1;
            character.carac.cha+=1;
        },
        20 => {
            character.carac.dex+=1;
            character.carac.int-=1;
            let dice = HackDice::from(character.torche.as_str()).sub_one();
            character.torche = dice.to_string();
            let dice2 = HackDice::from(character.degat_unarmed.as_str()).add_one();
            character.degat_unarmed = dice2.to_string();
            character.metier.name = "Athlète / Criminel / Pompier".to_string();
        },
        21 => {
            character.metier.name = "Archéologue / Journaliste / Scientifique".to_string();
            character.carac.dex-=2;
            let dice = HackDice::from(character.de_vie.as_str()).add_one();
            character.de_vie = dice.to_string();
        },
        22 => {
            character.carac.sag-=2;
            let dice = HackDice::from(character.de_vie.as_str()).add_one();
            character.de_vie = dice.to_string();
            character.metier.name = "Barman / Dilettante / Vagabond".to_string();
        },
        _ =>{}
    }

    character

}