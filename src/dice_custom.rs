

use tyche::{
Expr
};

use tyche::dice::roller::FastRand as FastRandRoller;


pub(crate) fn get_custom_dice(dice: &str) -> String {
    let mut roller = FastRandRoller::default();
    match dice.parse::<Expr>() {
        Ok(dice_custom) => {
            let resultat = dice_custom.eval(&mut roller).unwrap();
            let desc = resultat.to_string();
            let t = resultat.calc().unwrap_or_default();

            format!("Dés: {} total {}", desc, t)
        }
        Err(e) => e.to_string()
    }
}
