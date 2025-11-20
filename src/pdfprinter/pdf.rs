use printpdf::*;
use unaccent::unaccent;
use hyphenation::{Language, Load, Standard};
use textwrap::{wrap, Options, WordSplitter};
use crate::gen_struct::cthulhu_struct::Character;
use crate::gen_struct::rpg_utils::{horodate_filename};

pub fn hack_to_pdf(perso: Character) {
    println!("{}", perso);
    let police_file = include_bytes!("../../assets/dejavu-sans.condensed.ttf");
    //println!("police size byte: {}", police_file.len());
    let mut doc = PdfDocument::new(&perso.name);
    let font_slice = ParsedFont::from_bytes(police_file, 0, &mut Vec::new()).unwrap();
    let police_id = doc.add_font(&font_slice);



    //let item_perso = TextItem::Text(converted_perso.to);

    let mut head_name = Vec::new();
    if perso.name.is_empty() {
        head_name.push(TextItem::Text(
            "Name : ___________________________".to_string(),
        ));
    } else {
        head_name.push(TextItem::Text(perso.name.clone()));
    }
    //head_name.push(TextItem::Text(format!("Caracteristique:")));

    let mut item_carac1 = Vec::new();
    item_carac1.push(TextItem::Text(format!(
        "FO:{} ({})",
        perso.carac.fo, perso.carac.fo_bonus
    )));
    item_carac1.push(TextItem::Offset(-500.0));
    item_carac1.push(TextItem::Text(format!(
        "DEX:{} ({})",
        perso.carac.dex, perso.carac.dex_bonus
    )));
    item_carac1.push(TextItem::Offset(-500.0));
    item_carac1.push(TextItem::Text(format!(
        "CON:{} ({})",
        perso.carac.con, perso.carac.co_bonus
    )));
    item_carac1.push(TextItem::Offset(-500.0));
    let mut item_carac2 = Vec::new();
    item_carac2.push(TextItem::Text(format!(
        "INT:{} ({}) ",
        perso.carac.int, perso.carac.int_bonus
    )));
    item_carac2.push(TextItem::Offset(-500.0));
    item_carac2.push(TextItem::Text(format!(
        "SAG:{} ({}) ",
        perso.carac.sag, perso.carac.sage_bonus
    )));
    item_carac2.push(TextItem::Offset(-500.0));
    item_carac2.push(TextItem::Text(format!(
        "CHA:{} ({}) ",
        perso.carac.cha, perso.carac.cha_bonus
    )));
    let mut item_secondaire1 = Vec::new();
    item_secondaire1.push(TextItem::Text(format!("Bagou:{} ", perso.bagou)));
    item_secondaire1.push(TextItem::Offset(-500.0));
    item_secondaire1.push(TextItem::Text(format!("Torche:{} ", perso.torche)));
    item_secondaire1.push(TextItem::Offset(-500.0));
    item_secondaire1.push(TextItem::Text(format!("SAN:{} ", perso.de_sm)));
    let mut item_secondaire2 = Vec::new();
    item_secondaire2.push(TextItem::Text(format!("De de vie:{} ", perso.de_vie)));
    item_secondaire2.push(TextItem::Offset(-500.0));
    item_secondaire2.push(TextItem::Text(format!("Point de Vie:{} ", perso.pdv)));
    item_secondaire2.push(TextItem::Offset(-500.0));
    item_secondaire2.push(TextItem::Text(format!("Richesse:{} ", perso.richesse)));
    let mut item_secondaire3 = Vec::new();
    item_secondaire3.push(TextItem::Text(format!(
        "Combat armes:{} ",
        perso.degat_armed
    )));
    item_secondaire3.push(TextItem::Offset(-500.0));
    item_secondaire3.push(TextItem::Text(format!(
        "Combat non armes:{} ",
        perso.degat_unarmed
    )));
    let capacites = perso.capacite;
    //let mut item_capacite1 = Vec::new();
    //let names: Vec<String> = capacites.clone().into_keys().collect();
    //let name1 = names.get(0).unwrap();
    //let item1_desc = capacites.get(name1).unwrap().to_string();
    //item_capacite1.push(TextItem::Text(format!("{name1}:")));
    //let item1_desc = vec![TextItem::Text(item1_desc)];


    let mut page1_contents = vec![
        Op::Marker {
            id: "debugging-marker".to_string(),
        },
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(20.0), Mm(270.0)),
        },
        Op::SetLineHeight { lh: Pt(15.0) },
        Op::SetFontSizeBuiltinFont {
            size: Pt(13.0),
            font: BuiltinFont::TimesRoman,
        },
        Op::WriteTextBuiltinFont {
            items: head_name,
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::WriteTextBuiltinFont {
            items: vec![TextItem::Text("Caracteristiques:".to_string())],
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::WriteTextBuiltinFont {
            items: item_carac1,
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::WriteTextBuiltinFont {
            items: item_carac2,
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::WriteTextBuiltinFont {
            items: item_secondaire1,
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::WriteTextBuiltinFont {
            items: item_secondaire2,
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::WriteTextBuiltinFont {
            items: item_secondaire3,
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::AddLineBreak,
        Op::WriteTextBuiltinFont {
            items: vec![TextItem::Text("Atouts:".to_string())],
            font: BuiltinFont::TimesRoman,
        },
        Op::AddLineBreak,
        Op::SetLineHeight { lh: Pt(12.0) },
        Op::SetFontSizeBuiltinFont {
            size: Pt(11.0),
            font: BuiltinFont::TimesRoman,
        },
        Op::SetFontSize { size: Pt(11.0), font: police_id.clone() },
    ];

    //on genere ici les capacite/atout du personnage
    for (name, desc) in capacites.iter() {
        println!("{name}");

        page1_contents.push(
            Op::WriteText {
            items: vec![TextItem::Text(unaccent(name))],
            font: police_id.clone(),
        });
        page1_contents.push(Op::AddLineBreak);
        
        let dictionary = Standard::from_embedded(Language::French).unwrap();
        let options = Options::new(100).word_splitter(WordSplitter::Hyphenation(dictionary));
        let resized = wrap(desc, &options);

        for atout_line in resized {
             
            page1_contents.push(Op::WriteText{
                items: vec![TextItem::Offset(-500.0), TextItem::Text(atout_line.to_string())],
                font: police_id.clone(),
            });
            page1_contents.push(Op::AddLineBreak);
        }
        

        page1_contents.push(Op::AddLineBreak);
    }
    page1_contents.push(Op::EndTextSection);

    let page1 = PdfPage::new(Mm(200.0), Mm(280.0), page1_contents);

    let bytes = doc
        .with_pages(vec![page1])
        .save(&PdfSaveOptions::default(), &mut Vec::new());


    let fichier = horodate_filename(&perso.name);
    std::fs::write(format!("./{}.pdf", fichier), bytes).unwrap();
}

fn special_fr_char(s: String)-> String{

    let s2 = s.replace("’", " ");
    let s3 = s2.replace("œ", "oe");
    let s4 = s3.replace(" « ", " \"");
     s4.replace(" »", "\"")

    
}

fn get_UTF8(input: String)->String{

    let s_vec = input.clone().into_bytes();

    let s = String::from_utf8(s_vec);
    if let Ok(s_utf) = s {
        s_utf
    }else{
        println!("erreur utf-8");
        input
    }

}
