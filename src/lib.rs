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
/* pub trait Summary {
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
} */
/* #[cfg(test)]
mod tests {
        #[test]
        fn it_works() {
                assert_eq!(2+2, 4);
        }
        #[test]
        fn another(){
                panic!("Make this test fail! "); //failed test
        }
} */
/* #[derive(Debug)]
pub struct Rectangle {
        length: u32, 
        width: u32,
}
impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool{
                self.length > other.length && self.width > other.width
        }
}
#[cfg(test)]
mod tests {
        use super::*;
        #[test]
        fn larger_can_hold_smaller() {
                let larger = Rectangle {length: 8, width: 7};
                let smaller = Rectangle {length: 7, width: 5};
                assert!(larger.can_hold(&smaller));
        }
        #[test]
        fn smaller_cannot_hold_larger() {
                let larger = Rectangle {length: 8, width: 7};
                let smaller = Rectangle {length: 7, width: 5};
                assert!(!smaller.can_hold(&larger));
        }
} */
/* pub fn greeting(name: &str) -> String {
        String::from("Hello! ")
        //format!("Hello {}!", name)
}
#[cfg(test)]
mod tests{
        use super::*;
        #[test]
        fn greeting_contains_name() {
                let result = greeting("Carol");
                assert!(result.contains("Carol"), "Greeting did not contain a name, value is '{}'", result);
        }
} */
/* pub struct Guess {
        value: u32,
}
impl Guess {
        pub fn new(value: u32) -> Guess {
                if value < 1 /* || value > 100 */ {
                        panic!("Guess value must be greater than or equal to 1, got {}.", value);
                } else if value > 100 {
                        panic!("Guess value must be less than or equal to 100, got {}.", value);
                }
                Guess {
                        value
                } 
        }
}
#[cfg(test)]
mod tests{
        use super::*;
        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn greater_than_100(){
                Guess:: new(200);
        }
        #[test]
        fn it_works() -> Result<(), String> {
                if 2+2 == 4 {
                        Ok(())
                }else{
                        Err(String::from("Two plus two does not equal to four"))
                }
        }
} */
