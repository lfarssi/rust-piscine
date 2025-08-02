use json::{object,JsonValue};

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub calories: (String,String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

fn round(value: f64) -> f64 {
    let rounded = (value * 100.0).round() / 100.0;
    if (rounded * 10.0) % 1.0 == 0.0 {
        ((rounded * 10.0).round()) / 10.0
    } else {
        rounded
    }
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {

    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods.iter(){
        let cal= food.calories.1.trim_end_matches("kcal");
        let kcal =cal.parse::<f64>().unwrap();
        total_cals += kcal*food.nbr_of_portions;
        total_carbs+=food.carbs*food.nbr_of_portions;
        total_proteins+=food.proteins*food.nbr_of_portions;
        total_fats+=food.fats*food.nbr_of_portions;

    }

     object! {
       cals : round(total_cals),
       carbs: round(total_carbs),
       proteins : round(total_proteins),
       fats: round(total_fats),
    }

}