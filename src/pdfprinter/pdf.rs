use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use crate::gen_struct::cthulhu_struct::Character;



pub fn hack_to_pdf(perso: Character){
    println!("{}", perso);

    //let item_perso = TextItem::Text(converted_perso.to);

        let mut head_name = Vec::new();
        if perso.name.is_empty(){
            head_name.push(TextItem::Text("Name : ___________________________".to_string())); 
        }else{
            head_name.push(TextItem::Text(perso.name.clone())); 
        }
        //head_name.push(TextItem::Text(format!("Caracteristique:")));

        let mut item_carac1 = Vec::new();
        item_carac1.push(TextItem::Text(format!("FO:{} ({})", perso.carac.fo, perso.carac.fo_bonus)));
        item_carac1.push(TextItem::Text(format!("DEX:{} ({})", perso.carac.dex, perso.carac.dex_bonus)));
        item_carac1.push(TextItem::Text(format!("CON:{} ({})", perso.carac.con, perso.carac.co_bonus)));
        let mut item_carac2 = Vec::new();
        item_carac2.push(TextItem::Text(format!("INT:{} ({})", perso.carac.int, perso.carac.int_bonus)));
        item_carac2.push(TextItem::Text(format!("SAG:{} ({})", perso.carac.sag, perso.carac.sage_bonus)));
        item_carac2.push(TextItem::Text(format!("CHA:{} ({})", perso.carac.cha, perso.carac.cha_bonus)));




    let mut doc = PdfDocument::new(&perso.name);
    let page1_contents = vec![
            Op::Marker { id: "debugging-marker".to_string() }, 
            Op::StartTextSection,
            Op::SetTextCursor {pos: Point::new(Mm(20.0), Mm(270.0)), },
            Op::SetLineHeight { lh: Pt(15.0) },
            Op::SetFontSizeBuiltinFont{ size: Pt(13.0), font: BuiltinFont::TimesRoman },
            Op::WriteTextBuiltinFont { items: head_name, font: BuiltinFont::TimesRoman },
            Op::AddLineBreak,
            Op::WriteTextBuiltinFont { items: vec![TextItem::Text(format!("Caracteristiques:"))], font: BuiltinFont::TimesRoman },
            Op::AddLineBreak,
            //Op::SetTextCursor {pos: Point::new(Mm(20.0), Mm(200.0))},
            Op::WriteTextBuiltinFont { items: item_carac1, font: BuiltinFont::TimesRoman },
            Op::AddLineBreak,
            //Op::SetTextCursor {pos: Point::new(Mm(20.0), Mm(250.0))},
            Op::WriteTextBuiltinFont { items: item_carac2, font: BuiltinFont::TimesRoman },
            //Op::AddLineBreak,
            Op::EndTextSection,
        ];
    let page1 = PdfPage::new(Mm(200.0), Mm(280.0), page1_contents);

    let bytes = doc
        .with_pages(vec![page1])
        .save(&PdfSaveOptions::default(), &mut Vec::new());

    std::fs::write("./perso.pdf", bytes).unwrap();



}

