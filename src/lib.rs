//! # Art
//! 
//! 一个用以建立艺术概念的代码库
//！
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    ///RYB颜色模型的三原色
    pub enum PrimaryColor {
        Red, Yellow, Blue
    }

    ///RYB颜色模型的调和色
    pub enum SecondaryColor {
        Orange, Green, Purple
    }
}
pub mod utils {
    use crate::kinds::*;
    ///将两种等量的原色混合生成调和色
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {

    }
}
/* mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn take_payment() {}
    }
}
fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::back_of_house::Appetizer;
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //meal.seasonal_fruit = String::from("blueberries");
    let order1 = Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
use std::fmt;
use std::io::Result as IOResult;
use std::io::{self, Write};
fn function1() -> fmt::Result {
    return;
}
fn function2() -> IOResult<()> {
    return;
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
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter(); //生成的是不可变引用的迭代器，next取得的值实际上是指向动态数组中各个元素的不可变引用
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter(); //调用next的方法也被称为消耗适配器，因为它们同时消耗了迭代器本身
        let total: i32 = v1_iter.sum(); //sum方法会获取迭代器的所有权并反复调用next来遍历元素，sum
                                        //对所有元素进行求和，并在迭代结束后将结果作为结果返回
        assert_eq!(total, 6); //由于sum获取了迭代器的所有权，该迭代器无法继续被随后的代码使用
    }
} */
/* #[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() //调用filter将into_iter()所创建的迭代器适配成一个新的迭代器
}
#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
} */
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
#[cfg(test)]
#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a*b).filter(|x| x%3 == 0).sum();
    //zip方法只会产生4对值，在两个迭代器中的任意一个返回None时结束迭代
    assert_eq!(18, sum);
}

/// 将传入的数字加1
/// 
/// # Example
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(answer, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
//cargo doc文档场景
//Panics：函数可能引发panic的场景
//Errors：返回Result作为结果时，指出可能出现的错误及具体原因
//Safety：函数使用了unsafe关键字，指出当前函数不安全的原因
