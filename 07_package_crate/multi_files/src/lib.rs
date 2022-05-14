mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod front_of_house2;
pub use crate::front_of_house2::hosting2;
pub fn eat_at_restaurant2() {
    hosting2::add_to_waitlist();
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        eat_at_restaurant();
    }

    #[test]
    fn it_works2() {
        eat_at_restaurant2();
    }

}

