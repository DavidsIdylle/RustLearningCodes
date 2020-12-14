/*mod front_of_house{
    pub mod hosting{
            pub fn add_to_waitlist(){}
            fn seat_at_table(){}
    }
    mod serving{
            fn take_order(){}
            
            fn take_payment(){}
    }
}
fn serve_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
            cook_order();
            super::serve_order();
    }
    fn cook_order(){}
    pub struct Breakfast{
            pub toast: String, 
            seasonal_fruit: String,
    }
    impl Breakfast{
            pub fn summer(toast: &str) -> Breakfast{
                    Breakfast{
                            toast: String::from(toast),
                            seasonal_fruit: String::from("peaches"),
                    }
            }
    }
    pub enum Appetizer{
            Soup,
            Salad,
    }
}

use crate::front_of_house::hosting;
use crate::back_of_house::Appetizer;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //meal.seasonal_fruit = String::from("blueberries");
    let order1 = Appetizer::Soup;
    let order2  = back_of_house::Appetizer::Salad;
}
use std::fmt;
use std::io::Result as IOResult;
use std::io::{self, Write};
fn function1() -> fmt::Result {
        return 
}
fn function2() -> IOResult<()>{
        return
} */
pub trait Summary {
        fn summarize(&self) -> String;
}
pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub contents: String,
}
impl Summary for NewsArticle {
        fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
}
pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
}
impl Summary for Tweet {
        fn summarize(&self) -> String { 
                format!("{}: {}", self.username, self.content)
        }
}