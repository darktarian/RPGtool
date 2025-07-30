use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub(crate) fn get_spe_capacite() -> HashMap<&'static str, &'static str> {
    let mut capa: HashMap<&str, &str> = HashMap::new();

    capa.insert("Agile", "le personnage bénéficie d’un Avantage pour les Sauvegardes nécessitant des capacités athlétiques, comme grimper, sauter ou se balancer.");
    capa.insert("Ambitieux", "le personnage a un bonus de 2 points aux Sauvegardes associées à un ou plusieurs Désavantage(s).");
    capa.insert("Artiste", "le personnage bénéficie d’un Avantage lorsqu’il divertit un auditoire ou joue un rôle afin d’éviter des dégâts ou des répercussions de ses actes.");
    capa.insert("Attaque surprise", "si le personnage est armé lorsqu’un combat commence, il inflige deux fois les dégâts habituels avec l’arme qu’il a en main lors de sa première action si elle vise à blesser quelqu’un." );
    capa.insert("Autorité", "une fois par séance, le personnage peut intimer un ordre simple à un PNJ (« Lâche ton arme », « Ferme les yeux pendant dix secondes », « Ouvre la porte »), qui le réalise si cela ne met pas directement sa vie en danger ni celle d’autrui.");
    capa.insert("Ce ne sont pas vos affaires", "une fois par session, vous pouvez baisser d’un niveau le Bagou de votre personnage pour mettre hors jeu un adversaire qui n’est pas une créature du Mythe et qui a actuellement 15 Points de Vie ou moins.");
    capa.insert("Chanceux", "une fois par session vous pouvez supprimer un jeton de Fortune de votre personnage ou d’un allié que votre personnage voit.");
    capa.insert("Chef d’équipe", "lorsqu’un allié épuise son score de Torche ou de Bagou et que votre personnage est avec lui, vous pouvez décider de perdre un niveau en (respectivement) Torche ou Bagou à sa place.");
    capa.insert("Château de cartes", "une fois par partie, le personnage peut trahir (vous devez expliquer ce qui se passe) un allié, PJ ou non, afin d’éviter les conséquences d’une Sauvegarde de Sagesse ou Charisme ratée.");
    capa.insert("Comme un roc", "tant que le personnage est debout, il bénéficie temporairement de 2 Points d’Armure et d’un bonus de + 2 aux dégâts de mêlée. Les Sauvegardes de Dextérité subissent en revanche un malus de -2.");
    capa.insert("Crochetage", "le personnage peut déverrouiller en quelques secondes n’importe quelle serrure ou cadenas grâce à un test de Torche.");
    capa.insert("Crochetage amélioré", "Nécessite d’avoir la Capacité spéciale « Crochetage ». Lorsque vous effectuez un test de Torche afin que votre personnage déverrouille une serrure ou un cadenas, vous bénéficiez d’un Avantage.");
    capa.insert("Dans le mille", "une fois par session, au lieu de lancer les dés pour les dégâts, occasionnez le nombre maximal du dé de dégâts + 2.");
    capa.insert("Débrouillard", "une fois par session de jeu, le personnage peut improviser une solution à un problème avec ce qu’il a sous la main (objets, outils, machines).");
    capa.insert("Déduction", "une fois par session de jeu, cette capacité peut être revendiquée afin que le MJ vous donne un indice à propos de la prochaine étape de l’enquête.");
    capa.insert("Dites-moi tout", "le personnage bénéficie d’un Avantage aux tests visant à obtenir des aveux ou des confidences.");
    capa.insert("Double-vue", "le personnage perçoit une manifestation de l’Outre-monde, parmi les suivantes. Fantôme permet de percevoir l’âme d’un humain décédé dans une zone Au contact, sous la forme d’un fantôme éthéré. Il ne peut pas interagir avec lui sauf si le fantôme initie le contact. Entités permet de détecter la présence, dans les environs, d’une des multiples entités surnaturelles qui peuplent l’Outre-monde. À proximité, il perçoit même celles qui sont invisibles ou dissimulées. Les entités apparaissent nimbées d’une aura colorée. Magie permet de percevoir toute forme de magie active, sous la forme d’une aura colorée, autour de l’individu ou du lieu concerné. Ouïe spectrale permet d’entendre une phrase (au gré du MJ) prononcée en Outre-monde par l’esprit d’un mort dans une zone Proche.");
    capa.insert(
        "Dressage",
        "le personnage bénéficie d’un Avantage lorsqu’il s’occupe, monte ou affronte un animal.",
    );
    capa.insert("Échappée belle", " une fois par session de jeu, vous pouvez décider que votre personnage casse un équipement (vous devez raconter comment cela se produit) afin d’éviter des dégâts suite à une Sauvegarde de Force ou de Dextérité ratée.");
    capa.insert("Érudit", "une fois par heure de jeu, vous réussissez automatiquement une Sauvegarde d’Intelligence ou de Sagesse.");
    capa.insert("Foi", "votre personnage a une foi indéfectible qui lui permet de résister aux affres de la folie et rayonne sur ses alliés. Cette foi peut être tournée vers une divinité, ou un concept comme la loyauté, l’héroïsme, la science. Une fois par scénario, vous pouvez conférer à votre personnage et tous ses alliés un Avantage à un test de Santé mentale (par exemple lorsque tout le groupe rencontre une créature horrible, ou découvre un massacre qui peut faire vaciller l’esprit).");
    capa.insert("Force de caractère ", "une fois par session de jeu, vous pouvez relancer les dés pour un test de Hors Jeu ou de Santé mentale / Choc. Choisissez le résultat de votre choix.");
    capa.insert("Fuyant comme une anguille ", "une fois par session le personnage peut répercuter les conséquences d’un échec de Sauvegarde d’Intelligence ou Sagesse sur quelqu’un d’autre.");
    capa.insert(" Goût du risque", "une fois par session, lorsque vous devez faire un test avec un Désavantage, vous pouvez le faire avec Avantage (si vous avez la Capacité spéciale Ambitieux, votre test ne se fait pas avec un bonus de 2 points).");
    capa.insert("Gros bras", "augmentez les dégâts de mêlée du personnage d’un niveau (si les dégâts sont actuellement à 1, augmentez-les à d4).");
    capa.insert("Indomptable", "le personnage bénéficie d’un Avantage pour les Sauvegardes contre la tromperie, le contrôle mental ou lorsqu’il tente de tromper son interlocuteur.");
    capa.insert("Inspiration", "une fois par session, vous pouvez effectuer une Sauvegarde en utilisant le score de Sauvegarde d’un de vos alliés au lieu du vôtre.");
    capa.insert("Jauger l’ennemi ", "le personnage sait repérer les faiblesses. Il a un bonus de + 1 pour toucher et pour calculer ses dégâts contre un adversaire à condition d’effectuer un test de Torche après l’attaque.");
    capa.insert("Jauger l’ennemi amélioré", "Nécessite d’avoir la Capacité spéciale « Jauger l’ennemi ». Le personnage peut effectuer une Sauvegarde de Sagesse. En cas de succès il détermine le nombre de Points de Vie actuels de son adversaire. En cas d’échec il a un Désavantage à son prochain test de Sauvegarde.");
    capa.insert("Joker", "au contact de votre personnage, les autres se voient pousser des ailes. Une fois par session, vous permettez à un autre personnage de réutiliser une Capacité spéciale qu’il ne devrait plus pouvoir utiliser.");
    capa.insert("Loup solitaire", "lorsque le personnage combat sans allié (PJ ou PNJ), il gagne temporairement 2 Points d’Armure et inflige + 2 dégâts armé avec les armes de mêlée.");
    capa.insert(
        "Main sûre",
        "une fois par heure de jeu, vous réussissez automatiquement un test d’attaque à distance.",
    );
    capa.insert("Main sûre améliorée", "Nécessite d’avoir la capacité spéciale « Main sûre ». Vous n’avez pas besoin de faire une Sauvegarde de Dextérité pour toucher avec une arme à distance (à moins d’avoir un Désavantage), mais vous devez faire un test de Matériel immédiatement.");
    capa.insert("Mécanicien", "le personnage bénéficie d’un Avantage lorsqu’il répare des machines ou des véhicules endommagés.");
    capa.insert("Médecin de terrain", "une fois par heure de jeu, en dehors d’un combat vous pouvez soigner la perte de 1d6 Points de Vie perdus.");
    capa.insert("Médecin de terrain amélioré", "Nécessite d’avoir la capacité spéciale « Médecin de terrain ». une fois par heure de jeu, en dehors d’un combat vous pouvez soigner la perte de 2d4 Points de Vie perdus.");
    capa.insert("Mental d’acier", "le personnage bénéficie d’un Avantage pour les Sauvegardes visant à résister à la magie, aux rituels occultes, aux malédictions ou à la possession.");
    capa.insert("Musclé", "le personnage bénéficie d’un Avantage pour les Sauvegardes lorsqu’il utilise sa force pour soulever, déplacer ou lutter.");
    capa.insert("Œil de lynx", "augmentez les dégâts à distance d’un niveau (si les dégâts sont actuellement à 1, augmentez-les à d4).");
    capa.insert("Œil de lynx amélioré", "Nécessite d’avoir la Capacité spéciale « Œil de lynx ». Si le personnage obtient un score inférieur au score Dé de Vie de l’adversaire, ce-dernier tombe au sol.");
    capa.insert(
        "Physique banal",
        "le personnage bénéficie d’un Avantage pour tous les tests visant à passer inaperçu.",
    );
    capa.insert("Planification", "une fois par session, lorsque vous établissez un plan avec d’autres personnages, lancez 1d6. Le résultat est un nombre de points à distribuer aux autres personnages, en bonus à des tests de Sauvegardes. Vous devez indiquer que vous donnez un bonus à un autre personnage avant que son joueur lance les dés. Si le plan dévie sensiblement de ce qui était prévu, le MJ peut décider que les points à distribuer ne sont plus utilisables, ils sont perdus.");
    capa.insert("Polyglotte", "votre personnage sait s’adapter rapidement à des langues inconnues, ce qui lui permet de tenir une conversation simple avec n’importe quel individu dans sa langue maternelle.");
    capa.insert(
        "Polyvalent",
        "une fois par session vous pouvez relancer un test de Sauvegarde que vous avez échoué.",
    );
    capa.insert("Poussée d’adrénaline ", " une fois par heure de jeu, en combat, lancez votre Dé de Vie et ajoutez la valeur obtenue aux Points de Vie actuels du personnage (cela ne peut pas vous permettre de dépasser vos Points de Vie maximum).");
    capa.insert("Prestidigitation", "le personnage bénéficie d’un Avantage pour les tâches nécessitant une bonne coordination entre la main et l’œil, comme les tours de passe-passe, le vol à la tire et la manipulation de petits objets fragiles.");
    capa.insert("Profiteur", "vous pouvez annuler la perte d’un équipement suite à un test de Matériel raté en perdant à la place un niveau dans une Ressource d’Investigation.");
    capa.insert("Profiteur amélioré ", "Nécessite d’avoir la capacité spéciale « Profiteur ». Lorsque vous faites un test de Matériel pour vérifier si votre personnage est à court de munitions, vous bénéficiez d’un Avantage.");
    capa.insert("Psychologue", "une fois par session de jeu, vous pouvez passer une demi-journée à apaiser les craintes d’un autre personnage qui a perdu en Santé mentale. Il jette son dé de Santé mentale : en cas de 1 ou de 2, il regagne un niveau.");
    capa.insert("Pugiliste", "augmentez les Dégât(s) non armé d’un niveau (si les dégâts sont actuellement à 1, augmentez-les à d4).");
    capa.insert("Pugiliste amélioré", "Nécessite d’avoir la capacité spéciale « Pugiliste ». Si le personnage obtient un score inférieur au score Dé de Vie de l’adversaire, un membre de l’adversaire est paralysé (brisé ou bloqué par exemple).");
    capa.insert("Réseau", "une fois par session de jeu, vous pouvez éviter les conséquences d’une Sauvegarde de Charisme ou de Sagesse ratée, en tirant parti de votre réseau d’amis, d’associés ou de faveurs que l’on vous doit.");
    capa.insert("Ressources insoupçonnées", "une fois par session, lorsque vous échouez à un test de Ressource qui est à d4, au lieu d’épuiser la Ressource, considérez que le test est réussi, la Ressource reste à d4.");
    capa.insert("Retour de flamme", "une fois par heure de jeu, vous réussissez automatiquement une Sauvegarde défensive et retournez les dégâts de l’attaquant contre lui (il subit les dégâts desa propre arme).");
    capa.insert("Robuste", "le personnage bénéficie d’une protection naturelle dans les combats de mêlée, réduisant les dégâts des armes de mêlée contre lui de 1 point. Vous pouvez choisir cette Capacité spéciale plusieurs fois");
    capa.insert("Sens du danger", "le personnage bénéficie d’un Avantage lors des Sauvegardes visant à éviter les pièges ou les machines pouvant le blesser ou l’entraver.");
    capa.insert("Sincérité", "le personnage peut une fois par session énoncer une vérité et persuader son interlocuteur qu’il est sincère à ce propos.");
    capa.insert("Soigneux", " le personnage sait utiliser ses possessions jusqu’à leur dernier souffle. Il bénéficie d’un Avantage lors des tests de Matériel lorsque le dé de Matériel est à d4.");
    capa.insert("Tactique", "une fois par combat, lorsqu’il effectue une action, le personnage peut faire une attaque supplémentaire contre la même cible ou une autre. Vous pouvez choisir cette Capacité spéciale plusieurs fois.");
    capa.insert("Verbe blessant", "vous pouvez utiliser Charisme pour les attaques (au lieu de Force ou Dextérité), du moment que les adversaires peuvent entendre votre personnage et le comprendre. Lancez les dégâts comme si le personnage était armé. Si la victime tombe à 0 Point de Vie, elle est Hors Jeu mais n’est pas tuée.");
    capa.insert("Vigoureux", "le personnage bénéficie d’un Avantage lors des Sauvegardes visant à éviter les effets de poisons, de drogues ou de l’alcools.");
    capa.insert("Voyage onirique", "chaque nouveau voyage dans les Contrées du rêve (voir p. 92) fait baisser le malus du test suivant de deux points au lieu d’un lorsque le personnage tente délibérément d’entreprendre un voyage onirique.");

    capa
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Archetype {
    save_primaire: String,
    save_secondaires: Vec<String>,
    torche: String,
    bagou: String,
    de_vie: String,
    pdv: String,
    richesse: String,
    de_sm: String,
    degat: (String, String),
}

pub(crate) fn get_archetype() -> HashMap<&'static str, Archetype> {
    let mut arch = HashMap::new();

    arch.insert(
        "Artiste",
        Archetype {
            save_primaire: "Dextérité".to_string(),
            save_secondaires: vec!["Sagesse".to_string(), "Charisme".to_string()],
            torche: "d8".to_string(),
            bagou: "d10".to_string(),
            de_vie: "d6".to_string(),
            pdv: "12".to_string(),
            richesse: "d6".to_string(),
            de_sm: "d8".to_string(),
            degat: ("d4".to_string(), "d4".to_string()),
        },
    );

    arch
}
