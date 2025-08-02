mod mall;
pub use mall::*;
use std::collections::HashMap;
pub fn biggest_store(mall: &Mall)-> (String, Store){
    let floors=mall.floors.values();
    let mut biggest = 0;
    let  mut res = Store::new(HashMap::<String,Employee>::new(),0);
    let mut n = String::new();
    for floor in floors {
        for (name,store) in &floor.stores{
            if biggest < store.square_meters{
                biggest=store.square_meters;
                n=name.to_string();
                res = store.clone();
            }
        }
    }
    (n, res)

}


pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut highest_salary = 0.0;
    let mut highest_paid = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push((name, employee));
                } else if (employee.salary - highest_salary).abs() < std::f64::EPSILON {
                    highest_paid.push((name, employee));
                }
            }
        }
    }

    highest_paid
}


pub fn nbr_of_employees(mall: &Mall)->usize{
    let guards = mall.guards.len();
    let floors=mall.floors.values();
    let mut nbr_employees = 0;
    for floor in floors {
        for store in floor.stores.values(){
            nbr_employees += store.employees.len();
        }
    }
    nbr_employees+guards
}

pub fn check_for_securities(mall: &mut Mall, mut guards:HashMap<String,Guard>){
     let mut total = 0;
        let floors=mall.floors.values();
        for floor in floors{
            total+=floor.size_limit;
            
        }

       let guards_ln = mall.guards.len();

       total  -= guards_ln as u64 * 200 ; 

        while total  >= 200 {
            if let Some((key, value)) = guards.iter().next() {

                mall.guards.insert(key.to_string() , *value);

                let key = key.clone();

                guards.remove(&key);

            }
            total-=200;
        };
}

pub fn cut_or_raise(mall: &mut Mall){
    for floor in mall.floors.values_mut(){
        for store in floor.stores.values_mut(){
            for  employee in store.employees.values_mut(){
                let working = employee.working_hours.1-employee.working_hours.0;
                let amount = employee.salary/10.0;
                if working >=10{
                    employee.raise(amount);
                }else {
                    employee.cut(amount);
                }
                // println!("{:?}", working);
            }
        }
    }
}