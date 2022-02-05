mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod sarving {
        fn take_order() {}
        fn take_payment() {}
    }
}

fn eat_at_restaurant() {
    //absolute
    crate::front_of_house::hosting::add_to_waitlist();

    //relative
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {
    println!("serve_order");
}

mod back_of_house {

    // struct has pub / private member
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer_menu(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // pub enum is allowed all elements in pub
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {
        println!("cook_order");
    }
}

fn eat_at_restaurant2() {
    let mut meal = back_of_house::Breakfast::summer_menu("Rye");
    println!("1: I'd like {} toast please.", meal.toast);

    meal.toast = String::from("Wheat");
    println!("2: I'd like {} toast please.", meal.toast);

    // this is not allowed(private)
    // meal.seasonal_fruit = String::From("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        eat_at_restaurant();
        serve_order();
    }

    #[test]
    fn it_works2() {
        eat_at_restaurant2();
    }
}
