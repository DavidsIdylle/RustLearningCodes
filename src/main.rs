/*fn main() {
   /*println!("Hello World!");
    println!("{} days", 31);
    println!("{0}, this is {1}, {1}, this is {0}","Alice","Bob");
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jump over");

        println!("{} of {:b} people know binary, the other half dosen't",1,2);
    println!("{number:>width$}",number=1, width=6);
    println!("{number:>0width$}",number=1, width=6);*/

    /*#[derive(Debug)]
    struct Structure(i32);
    #[derive(Debug)]
    struct Deep(Structure);
    println!("This struc {:?} will print! ",Structure(3));
    println!("{:?} will print! ", Deep(Structure(7)));*/
}
#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age:u8
}
fn main(){
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};
    println!("{:#?}", peter);
}*/
/*use std::fmt;
#[derive(Debug)]
struct Vector2D{
    x:isize,
    y:isize,
}
impl fmt::Display for Vector2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl fmt::Binary for Vector2D{
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result{
        let magnitude = (self.x * self.x + self.y * self.y) as f64;
        let magnitude = magnitude.sqrt();

        let decimals = f.precision().unwrap_or(3);
        let string = format!("{:.*}", decimals, magnitude);
        f.pad_integral(true, "", &string)
    }
}
fn main() {
    let myvector = Vector2D{x:3, y:4};
    println!("{}", myvector);
    println!("{:?}", myvector);
    println!("{:10.3b}", myvector);
}*/
/*use std::fmt;
#[derive(Debug)]
struct MinMax(i64, i64);
impl fmt::Display for MinMax{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
#[derive(Debug)]
struct Point2D{
    x:f64,
    y:f64,
}
impl fmt::Display for Point2D{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}
#[derive(Debug)]
struct Complex{
    real:f64,
    imag:f64,
}
impl fmt::Display for Complex{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} + {}i", self.real, self.imag)
    }
}
fn main() {
    let minmax = MinMax(0,14);

    println!("Compare Structures:");
    println!("Display:{}",minmax);
    println!("Debug:{:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
    small = small_range,
    big = big_range);

    let point = Point2D{x:3.3, y:7.2};
    println!("Compare points:");
    println!("Display:{}", point);
    println!("Debug:{:?}", point);

    let complex = Complex{real:3.3, imag:7.2};
    println!("Compare complex:");
    println!("Display:{}", complex);
    println!("Debug:{:?}", complex);
}*/
/* fn main() {
    /*pub fn answer() ->(){
        let a = 4;
        let b = 2;
        assert_eq!(sum(a,b),6);
    }
    pub fn sum(a:i32, b:i32)->i32{
        a + b
    }
    answer();*/

    /* let a = [1,2,3];
    let b = &a;
    println!("{:p}",b);
    let mut c = vec![1,2,3];
    let d = &mut c;
    d.push(4);
    println!("{:?}",d);
    let e = &42;
    assert_eq!(42, *e); */

    let v = "hello world!";
    assert_eq!(v, "hello world!");
    let v = "hello Rust!";
    assert_eq!(v, "hello Rust!");
    {
        let v = "hello world!";
        assert_eq!(v, "hello world!");
    }
    assert_eq!(v, "hello Rust!");
} */
/* pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}
fn main() {
    let a = 3;
    let b = 2;
    assert_eq!(math(sum, a, b), 5);
    assert_eq!(math(product, a, b), 6);
} */
/*fn is_true() -> bool{ true }
fn true_maker() -> fn() -> bool{is_true}
fn main() {
    assert_eq!(true_maker()(), true);
}*/
/*fn math<F:Fn() -> i32>(op:F) ->i32{
    op()
}
fn main(){
    let a = 2;
    let b = 3;
    assert_eq!(math(||a+b), 5);
    assert_eq!(math(||a*b), 6);
}*/
/*fn two_times_impl() -> impl Fn(i32) ->i32{
    let i = 2;
    move |j| j*i
}
fn main() {
    let result = two_times_impl();
    assert_eq!(result(2), 4);
}*/
/*fn main(){
        /*let n = 13;
        let big_n = if(n<10 && n>-10){
                10*n
        } else{
                n/2
        };
        assert_eq!(big_n, 6);*/

        for n in 1..101{
                if n%15 == 0{
                        println!("fizzbuzz");
                }
                else if n%3 == 0{
                        println!("fizz");
                }
                else if n%5 == 0{
                        println!("buzz");
                }
                else{
                        println!("{}",n);
                }
        }
}*/
/*fn while_true(x:i32)->i32{
        while true{
                return x+1;
        }
        x
}
fn main() {
        let y = while_true(5);
        assert_eq!(y, 6);
}*/
/*fn main(){
        /*let number = 42;
        match number{
                0 => println!("Origin"),
                1..=3 => println!("All"),
                |5|7|13 => println!("Bad luck!"),
                n @ 42 =>println!("Answer is {}", n),
                _ => println!("Common"),

        }
        let boolean = true;
        let mut binary = 0;
        if let true = boolean{
                binary = 1;
        }
        assert_eq!(binary,1);
        let mut v = vec![1,2,3,4,5];
        loop{
                match v.pop(){
                        Some(x) => println!("{}",x),
                        None => break,
                }
        }
        while let Some(x) = v.pop(){
                println!("{}",x);
        }
        assert_eq!((1..5), std::ops::Range{start:1, end:5});
        assert_eq!((1..=5), std::ops::RangeInclusive::new(1,5));
        assert_eq!(3+4+5, (3..6).sum());
        assert_eq!(3+4+5+6, (3..=6).sum());
        for i in 1..5{
                println!("{}", i);
        }
        for i in 1..=5{
                println!("{}",i);
        }*/
        /*let truth: &'static str = "Rust language";
        let ptr = truth.as_ptr();
        let len = truth.len();
        assert_eq!(13,len);
        let s = unsafe{
                let slice = std::slice::from_raw_parts(ptr, len);
                std::str::from_utf8(slice)
        };
        assert_eq!(s, Ok(truth));*/

        let mut x = 10;
        let ptr_x = &mut x as *mut i32;
        let y = Box::new(20);
        let ptr_y = &*y as *const i32;
        unsafe{
                *ptr_x += *ptr_y;
        }
        assert_eq!(x, 30);
}*/
/*fn move_coords(x:(i32, i32)) -> (i32, i32){
        (x.0+1, x.1+1)
}
fn main(){
        let tuple :(&'static str, i32, char) = ("hello", 5, 'c');
        assert_eq!(tuple.0, "hello");

        let coords = (0, 1);
        let result = move_coords(coords);
        assert_eq!(result, (1,2));
        let (x, y) = move_coords(coords);
        assert_eq!(x, 1);
        assert_eq!(y, 2);
}*/
/*#[derive(Debug, PartialEq)]
struct People {
        name: &'static str,
        gender:u32,
}
impl People{
        fn new(name: &'static str, gender: u32) ->Self{
                return People{
                        name: name, gender: gender
                };
        }
        fn name(&self){
                println!("name: {:?}", self.name);
        }
        fn set_name(&mut self, name: &'static str){
                self.name = name;
        }
        fn gender(&self){
                let gender = if(self.gender == 1){"boy"} else{"girl"};
                println!("gender:{:?}", gender);
        }
}
fn main(){
        let alex = People::new("Alex", 1);
        alex.name();
        alex.gender();
        assert_eq!(alex, People{ name:"Alex", gender: 1});
        let mut alice = People::new("Alice", 0);
        alice.name();
        alice.gender();
        assert_eq!(alice, People{ name:"Alice", gender: 0});
        alice.set_name("Rose");
        alice.name();
        assert_eq!(alice, People{ name:"Rose", gender: 0});
}*/
/*struct Integer(u32);
type Int = i32;
fn main(){
        let int = Integer(10);
        assert_eq!(int.0, 10);
        let int: Int = 10;
        assert_eq!(int, 10);
}*/
/*struct Empty;
fn main(){
        let x = Empty;
        println!("{:p}", &x);
        let y = x;
        println!("{:p}", &y);
        let z = Empty;
        println!("{:p}", &z);
        assert_eq!((..), std::ops::RangeFull);
}*/
/*fn main() {
        let s:&Option<String> = &Some("Hello".to_string());
        match s {
                Some(s) => println!("s is: {}", s),
                _ => (),
        };
}*/
/*n main(){
        let mut v1 = vec![];
        v1.push(1);
        v1.push(2);
        assert_eq!(v1,[1,2]);
        assert_eq!(v1[1],2);
        let v2 = vec![0;10];
        let mut v3 = Vec::new();
        v3.push(3);
        v3.push(4);
        println!("{:?}",v2);
}*/
//use std::collections::VecDeque;
//use std::collections::HashMap;
//use std::collections::BTreeMap;
/*use std::collections::BinaryHeap;
fn main() {
        /*let mut buf = VecDeque::new();
        buf.push_front(1);
        buf.push_front(2);
        assert_eq!(buf.get(0),Some(&2));
        assert_eq!(buf.get(1),Some(&1));
        buf.push_back(3);
        buf.push_back(4);
        assert_eq!(buf.get(2),Some(&3));
        assert_eq!(buf.get(3),Some(&4));
        println!("{:?}", buf);
        let mut buf1 = VecDeque::new();
        buf1.push_back(5);
        buf.append(&mut buf1);
        println!("{:?}", buf);
        let mut hmap = HashMap::new();
        let mut bmap = BTreeMap::new();
        hmap.insert(3, "c");
        hmap.insert(1, "a");
        hmap.insert(2, "b");
        bmap.insert(3, "c");
        bmap.insert(2, "b");
        bmap.insert(1, "a");
        println!("{:?}",hmap);
        println!("{:?}",bmap);*/
        let mut heap = BinaryHeap::new();
        assert_eq!(heap.peek(), None);
        let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45];
        for i in arr.iter().cloned(){
                heap.push(i);
        }
        assert_eq!(heap.peek(), Some(&93));
        println!("{:?}", heap);
}*/
/*use std::fmt::Debug;
fn match_option<T:Debug>(o :Option<T>){
        match o {
                Some(i) => println!("{:?}", i),
                None => println!("nothing"),
        }
}
fn main() {
        let a: Option<i32> = Some(3);
        let b: Option<&str> = Some("hello");
        let c: Option<char> = Some('A');
        let d: Option<u32> = None;
        match_option(a);
        match_option(b);
        match_option(c);
        match_option(d);
}*/
/*struct Duck;
struct Pig;
trait Fly{
        fn fly(&self) -> bool;
}
impl Fly for Duck{
        fn fly(&self) -> bool{ return true;}
}
impl Fly for Pig{
        fn fly(&self) -> bool{ return false;}
}
fn fly_static<T: Fly>(s:T) ->bool{ s.fly() }
fn fly_dyn(s: &dyn Fly) ->bool{ s.fly() }
fn main() {
        let pig = Pig;
        assert_eq!(fly_static::<Pig>(pig), false);
        let duck = Duck;
        assert_eq!(fly_static::<Duck>(duck), true);
        assert_eq!(fly_dyn(&Pig), false);
        assert_eq!(fly_dyn(&Duck), true);
}*/
/*#[derive(Debug)]
struct Point{
        x:i32, y:i32,
}
fn main() {
        let origin = Point{x:0, y:0};
        println!("The origin is: {:?}", origin);
}*/
/*fn main(){
        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_ok(), true);
        let x: Result<i32, &str> = Err("Some error message");
        assert_eq!(x.is_ok(), false);
}*/
/*use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main(){
        println!("Guess the number!");
        let secret_number =  rand::thread_rng().gen_range(1, 101);
        loop{
                println!("Please input your guess");
                let mut guess = String::new();
                io::stdin().read_line(&mut guess)
                        .expect("failed to read line.");
                let guess: u32 = match guess.trim().parse(){
                        Ok(num) => num,
                        Err(_) => continue,
                };
                println!("You guessed :{}", guess);
                match guess.cmp(&secret_number){
                        Ordering::Less => println!("Too small! "),
                        Ordering::Greater => println!("Too big! "),
                        Ordering::Equal =>{
                                println!("You win! ");
                                break;
                        }
                }
        }
}*/
/*fn main(){
        let x = plus_one(5);
        println!("{:?}", x);
}
fn plus_one(x:i32)->i32{
        x+1
}*/
/*fn main(){

        /*let number = 6;
        if number%4 == 0{
                println!("nummber is divisible by 4");
        }else if number%3 == 0{
                println!("nummber is divisible by 3");
        }else if number%2 == 0{
                println!("nummber is divisible by 2");
        }*/

        /*let mut counter = 0;
        let result = loop{
                counter += 1;
                if counter == 10{
                        break counter*2;
                }
        };
        println!("The result is {:?}", result);
        let a = [10,20,30,40,50];
        for element in a.iter(){
                println!("{:?}", element);
        }
        for number in (1..5).rev(){
                println!("{:?}!", number);
        }*/

        let mut s = String::from("Hello");
        s.push_str(",world! ");
        let s_moved = s.clone();
        println!("s = {:?}, cloned s = {:?}", s, s_moved);

}*/
/*fn main() {
        let s = String::from("Hello");//变量s进入作用域
        takes_ownership(s);//s值被移入函数中

        let x = 5;//变量x进入作用域
        makes_copy(x);//x被传递进函数中，但由于i32有copy trait，在这之后x依然可用
}//x值首先离开作用域，随后是s。但s值已经发生了移动

fn takes_ownership(some_string: String){
        println!("{:?}", some_string);
}//some_string在这里离开作用域，drop函数被自动调用

fn makes_copy(some_integer: i32){
        println!("{:?}", some_integer);
}*/
/*fn main(){
        let s1 = gives_ownership();
        let s2 = String::from("Hello!"); //s2进入作用域
        let s3 = take_and_gives_back(s2);//s2被移入函数
}//s3离开作用域，s1离开作用域
fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string//作为返回值被移动至调用函数
}
fn take_and_gives_back(a_string: String) -> String {
        a_string//作为返回值被移动至调用函数
}*/
/*fn main() {
        let s1 = String::from("Hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of {:?} is {:?}", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
}*/
/*fn main() {
        let s = String::from("Hello");
        let len = calculate_length(&s);
        println!("The length of {:?} is {:?}", s, len);
}
fn calculate_length(s: &String) -> usize {
        s.len()
}//s离开作用域，但并不持有所指值的所有权，故该值不会被销毁*/

/*fn main() {
        let mut s = String::from("Hello");
        change(&mut s);
        println!("{:?}", s);
}
fn change(some_string: &mut String){
        some_string.push_str(", world! ");
}*/
/*fn main() {
        let reference_to_nothing = dangle();
}
fn dangle() -> &String {
        let s = String::from("Hello");
        &s //将指向s的引用返回给调用者
}//s离开作用域并被销毁，指向的内存不再有效*/
/* fn main() {
        let mut s = String::from("Hello world");
        let word = first_word(&s);
        println!("The first word is {:?}",&s[..word]);
        println!("The second word is {:?}",&s[word+1..]);
        s.clear();
}
fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();
        for(i, &item) in bytes.iter().enumerate(){
                if item == b' '{
                        return i;
                }
        }
        s.len()
} */
/* struct User{
        username: String,
        email: String,
        sign_in_count: u64,
        active:bool,
}
fn main() {
        let user1 = build_user(String::from("secondone@example.com"), String::from("Seconduser"));
        let mut user2 = User{
                email: String::from("sonmeone@example.com"),
                username: String::from("someusername"),
                active: user1.active,
                sign_in_count: user1.sign_in_count+1,
        };
        user2.email = String::from("anotherone@example.com");
}
fn build_user(email: String, username: String) -> User {
        User{
                email,
                username,
                active:true,
                sign_in_count:1,
        }
} */
/*#[derive(Debug)]
struct Rectangle {
        width: u32,
        height: u32,
}
/* fn main() {
        let rec1 = Rectangle{ width:30, height: 20,};
        println!("rectangle1 is {:#?}", rec1);
        println!("The area of the rectangle is {:?}", area(&rec1));
}
fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width* rectangle.height
} */
impl Rectangle{
        fn area(&self) -> u32{
                return self.width* self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool{
                return self.width > other.width && self.height > other.height
        }
        fn square(size: u32) ->Rectangle{
                Rectangle { width: size, height: size }
        }
}
fn main() {
        let rec1 = Rectangle{ width:30, height: 20};
        println!("The area of the rectangle is {:?}", rec1.area());
        let rec2 = Rectangle{ width:20, height: 10};
        let rec3 = Rectangle{ width:40, height: 30};
        println!("Can rec1 can hold rec2? {:?}", rec1.can_hold(&rec2));
        println!("Can rec2 can hold rec3? {:?}", rec2.can_hold(&rec3));
} */
/* struct Ipv4Addr{

}
struct Ipv6Addr{

}
enum IpAddr{
        V4(Ipv4Addr),
        V6(Ipv6Addr),
}
fn main(){
        /* let home = IpAddr::V4(127,0,0,1);
        let loopback = IpAddr::V6(String::from("::1")); */
} */
/* enum Message{
        Quit,
        Move{ x: i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
}
impl Message{
        fn call(&self){

        }
}
fn main(){
        let m = Message::Write(String::from("Hello"));
        m.call();
} */
/* #[derive(Debug)]
enum UsState {
        Alabama,
        Alaska,
}
enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u32 {
        match coin{
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                        println!("State Quarter from {:?}", state);
                        25
                },
        }
}
fn main() {
        let  mut count = 0;
        if let Coin::Quarter(state) => {
                println!("State Quarter from {:?}", state);
        } else{
                count += 1;
        }
}*/
/* fn plus_one(x: Option<i32>) -> Option<i32> {
        match x{
                None => None,
                Some(i) => Some(i + 1),
        }
}
fn main() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("{:?}, {:?}", six, none);
} */
/* fn main() {
        let some_u8_value = Some(0u8);
        match some_u8_value {
                Some(3) => println!("three"),
                _ => (),
        }
        if let Some(3) = some_u8_value { println!("three!");}
} */
/* use std::collections::HashMap;
use rand::Rng;
fn main() {
        let mut map = HashMap::new();
        map.insert(1,2);
        let secret_number = rand::thread_rng().gen_range(1,101);
} */
/* fn main() {
        let _v = vec![1,2,3];
        let mut v = Vec::new();
        v.push(4);
        v.push(5);
        //v.push(7);
        //let third = &v[2];//索引越界时，[]方法会触发panic
        //println!("The third element of v is {:?}", third);
        match v.get(2){
                //get方法在索引越界时会简单返回None
                Some(third) => println!("The third element of v is {:?}", third),
                None => println!("There is no third element. "),
        }
        let mut v = vec![1,2,3,4,5];
        let first = &v[0];
        //v.push(6);
        println!("The first element of v is {:?}", first);
        for i in &v{
                println!("{:?}",i);
        }
        for i in &mut v{
                *i += 50;
        }
        enum SpreadsheetCall{
                Int(i32),
                Float(f64),
                Text(String),
        }
        let _row = vec![
                SpreadsheetCall::Int(3),
                SpreadsheetCall::Text(String::from("blue")),
                SpreadsheetCall::Float(10.12),
        ];
        let data = "first ";
        let mut s = data.to_string();
        s.push_str("second ");
        let s1 = "third ";
        s.push_str(s1);
        s.push('f');
        println!("{:?} add {:?}",s, s1);
        let s1 = String::from("Hello, ");
        let s2 = String::from("world! ");
        let s3 = s1 + &s2; //fn add(self, s: &str) -> String{ }，s1在此之后不再有效
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        //let s_all = s1 + "-" + &s2 + "-" +&s3;
        let s_all = format!("{}-{}-{}",s1,s2,s3);
} */
/*fn main(){
         for c in  "Здравствуйте".chars(){
                println!("{:?}", c);
        }
        /*use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Red"), 20);
        scores.insert(String::from("Yellow"), 30);
        let teams = vec![String::from("Blue"), String::from("Red"), String::from("Yellow")];
        let initial_scores = vec![10, 20, 30];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        //zip方法创建元组的数组，collect方法将动态数组转为hashmap
        let team_name = String::from("Red");
        let _team_score = scores.get(&team_name);
        for (key, value) in &scores {
                println!("{:?} : {:?}", key ,value);
        }
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);//field_name和field_value从这里开始失效*/
        use std::collections::HashMap;
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        println!("{:?}", scores);
        scores.insert(String::from("Blue"), 15);
        scores.entry(String::from("Yellow")).or_insert(20);
        scores.entry(String::from("Blue")).or_insert(25);
        println!("{:?}", scores);
        let text = "one world one dream";
        let mut map = HashMap::new();
        for word in text.split_whitespace(){
                let count = map.entry(word).or_insert(0);
                *count +=1;
        }
        println!("{:?}",map);
}*/
/*use std::io;
use std::io::Read;
use std::fs::File;
//use std::io::ErrorKind;
/* fn main() {
        //panic!("crash and burn");
        //let v = vec![1,2,3,4,5];
        //v[5];
        let f = File::open("Hello.txt");
        //Result<T,E>中，T为std::fs::File, E为std::io::Error
        let _f = match f{
                Ok(file) => file,
                Err(error) => match error.kind(){
                        ErrorKind::NotFound => match File::create("hello.txt"){
                                Ok(fc) => fc,
                                Err(e) => panic!("Tried to create file but failed: {:?}", e),
                        },
                        other_error => panic!("There was a problem opening file: {:?}", other_error),
                },
        };
} */
use std::error::Error;
fn main()  -> Result<(), Box<dyn Error>> {//Box<dyn Error>可理解为任何可能的错误类型
        let _f = File::open("hello.txt")?;
        Ok(())
        /* let f = File::open("hello.txt").map_err(|error| {
                if error.kind() == ErrorKind::NotFound{
                        File::create("hello.txt").unwrap_or_else(|error|  {
                                panic!("Tried to create file but failed: {:?}", error);
                        });
                } else {
                        panic!("There was a problem opening the file: {:?}", error);
                }
        }); */
        //let _f = File::open("hello.txt").unwrap();
        //let _f = File::open("hello.txt").expect("Couldn't open hello.txt");

}
//调用该函数将获得包裹在Ok中的String值，即从文件中读取的用户名
fn _read_username_from_file() -> Result<String, io::Error> {
        /* let f = File::open("hello.txt");
        let mut f = match f{
                Ok(file) => file,
                Err(e) => return Err(e),
        };
        let mut s = String::new();
        //File::open函数和read_to_string方法，都使用io::Error作为错误类型
        match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
        } */
        let mut f = File::open("hello.txt")?;
        //?所接收的错误值会隐式地被from函数转换为当前函数的返回错误类型
        //?会将存储在Ok内部的值返回给变量f，出现错误则?提前结束函数的执行，将Err值返回给函数调用者
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        //File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
}*/
/*pub struct Guess{
        value: i32,
}
impl Guess {
        pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                        panic!("Guess value must be between 1 and 100, got {:?}. ", value);
                }
                Guess {
                        value
                }
        }
        //value方法，又称为读取接口，value本身为私有，因此需要提供公共方法用于访问数据
        pub fn value(&self) -> i32 {
                self.value
        }
}*/
/* fn main(){
        let number_list = vec![34, 102, 50, 25, 100, 65, 89];
        //let result = largest_i32(&number_list);
        let result = largest(&number_list);
        println!("The largest number is {}", result);
        let char_list = vec!['y', 'n', 'a', 'f', 'o', 'j'];
        //let result = largest_char(&char_list);
        let result = largest(&char_list);
        println!("The largest char is {}", result);
}
/* fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &item in list.iter() {
                if item > largest {
                        largest = item;
                }
        }
        largest
}
fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];
        for &item in list.iter() {
                if item > largest {
                        largest = item;
                }
        }
        largest
} */
fn largest<T>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
                if item > largest {//虽然可实现泛型，但>符号只能被用于可排序的值类型
                        largest = item;
                }
        }
        largest
} */
/* struct Point<T> {
        x: T,
        y: T,
}
struct PointAnyType<U, T> {
        x: U,
        y: T,
}
enum _Result<T, E> {
        Ok(T),
        Err(E),
}
impl<T> Point<T> {
        fn x(&self) -> &T {
                &self.x
        }
}
impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
}
impl<T, U> PointAnyType<T, U> {
        fn mixup<V, W>(self, other: PointAnyType<V, W>) -> PointAnyType<T, W> {
                PointAnyType {
                        x: self.x,
                        y: other.y,
                }
        }
}
fn main() {
        let integer = Point { x: 5, y: 6 };
        println!("x = {:?}", integer.x());
        let float = Point { x: 1.4, y: 9.8};
        println!("The distance from origin is {:?}", float.distance_from_origin());
        let any_type = PointAnyType { x: 4, y: 7.3};
        let other_type = PointAnyType {x: "hello", y: 'c'};
        let mixed_type = any_type.mixup(other_type);
        println!("mixed_type.x = {:?}, mixed_type.y = {:?}", mixed_type.x, mixed_type.y);
} */

/* pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String{
                format!("(Read more from {}...)", self.summarize_author())
        }
}
pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
}
/* impl Summary for NewsArticle {
        fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
} */
pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
}
impl Summary for Tweet {
        fn summarize_author(&self) -> String {
                format!("@{}", self.username)
        }
} */
//pub fn notify1(item1: impl Summary, item2: impl Summary){}    //可以使用不同的类型
//pub fn notify2<T: Summary>(item1: T, item2: T){}              //强迫二者为相同的类型
//pub fn notify<T: Summary + Display>(item: T){}                //指定多个trait
/* fn some_function<T, U>(t: T, u: U) -> i32
        where T: Display + Clone,
                U: Clone + Debug        //用where来简化定义多个trait约束的函数签名
{

} */
/* fn main() {
        let article = NewsArticle {
                headline: String::from ("Penguins win the Stanley Cup Championship!"),
                location: String::from ("Pittsburgh, PA, USA"),
                author: String::from ("Iceburgh"),
                content: String::from ("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        };
        let tweet = Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people."),
                reply: false,
                retweet: false,
        };
        //println!("New article available! {}", article.summarize());
        println!("A new tweet: {}", tweet.summarize());
} */
/* fn reset(mut arr: [u32; 5]) {
        arr[0] = 5;
        arr[1] = 4;
        arr[2] = 3;
        arr[3] = 2;
        arr[4] = 1;
        println!("reset arr is {:?}", arr);
}
fn main() {
        let arr: [u32; 5] = [1, 2, 3, 4, 5];
        reset(arr);
        println!("origin arr is {:?}", arr); //实现了Copy trait，所以没有影响原来的数组
} */
/* fn reset(arr: &mut [u32]) {
        arr[0] = 5;
        arr[1] = 4;
        arr[2] = 3;
        arr[3] = 2;
        arr[4] = 1;
        println!("reset arr is {:?}", arr);
        println!("reset array length {:?}", arr.len()); //包含了长度信息
}
fn main() {
        let mut arr = [1, 2, 3, 4, 5];
        println!("reset before: origin array {:?}", arr);
        {
                let mut_arr: &mut[u32] = &mut arr;
                reset(mut_arr);
        }
        println!("reset after: origin array {:?}", arr); //将引用当作函数参数，被修改的是原数组
        println!("mem size of &[u32; 5] is {:?}", std::mem::size_of::<[u32;5]>());  //8
        println!("mem size of &mut [u32] is {:?}", std::mem::size_of::<&mut [u32]>()); //16
} */
/* use std::mem::size_of;
enum Void{}
struct Foo;
struct Baz {
        _foo: Foo,
        _qux: (),
        _baz: [u8; 0],
}
fn main() {
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", size_of::<()>(), size_of::<Foo>(), size_of::<Baz>(), size_of::<Void>(), size_of::<[(); 10]>());
} */
/* #![feature(never_type)]
fn foo() -> ! {
        //...
        loop{ println!("repeat!"); }
}
fn main() {
        let i = if false{
                foo();
        } else {
                100
        };
        assert_eq!(i, 100);
} */
/* enum Void{}
fn main() {
        let res: Result<u32, Void> = Ok(0);
        if let Ok(num) = res{
        }
} */
/* fn sum(a: u32, b: i32) -> u32 {
        a+ (b as u32)
}
fn main() {
        let a = 1;
        let b = 2;
        println!("{:?}", sum(a, b)); //3
        let elem = 5u8;
        let mut vec = Vec::new();
        vec.push(elem);
        println!("{:?}", vec); //[5]
} */
/* fn main() {
        let x = "1";
        let int_x: i32 = x.parse().unwrap();
        assert_eq!(int_x, 1);
        assert_eq!(x.parse::<i32>().unwrap(), 1); // ::<> 操作符形式称为turbofish操作符
} */
/* fn foo<T>(x:T) -> T{
        return x
}
fn main() {
        assert_eq!(foo(1), 1);
        assert_eq!(foo("hello"), "hello");
} */
/* #[derive(Debug)]
struct Point<T> {x:T, y:T}
impl<T> Point<T>{
        fn new(x:T, y:T) -> Self {
                Point{x:x, y:y}
        }
}
fn main() {
        let point1 = Point::new(1, 2);
        let point2 = Point::new("1", "2");
        println!("{:?}", point1);
        println!("{:?}", point2);
} */
/* #[derive(Debug, PartialEq)]
struct Foo(i32);
#[derive(Debug, PartialEq)]
struct Bar(i32, i32);
trait Inst {
        fn new(i32) -> Self;
}
impl Inst for Foo {
        fn new(i: i32) -> Foo {
                Foo(i)
        }
}
impl Inst for Bar {
        fn new(i: i32) -> Bar {
                Bar(i, i+10)
        }
}
fn foobar<T: Inst>(i:i32) -> T {
        T::new(i)
}
fn main() {
        let f: Foo = foobar(10);
        println!("{:?}", f); //Foo(10)
        let b: Bar = foobar(10);
        println!("{:?}", b); //Bar(10, 20)
} */
/* trait Add<RHS, Output> {
        fn my_add(self, rhs: RHS) -> Output;
}
impl Add<i32, i32> for i32 {
        fn my_add(self, rhs: i32) -> i32 {
                self + rhs
        }
}
impl Add<u32, i32> for u32 {
        fn my_add(self, rhs: u32) -> i32 {
                (self + rhs) as i32
        }
}
fn main() {
        let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
        let x: i32 = a.my_add(b);
        let y: i32 = c.my_add(d);
        println!("{:?}", x); //3i32
        println!("{:?}", y); //7i32
} */
/* pub trait Add<RHS = Self> { //Self为trait自带的隐式类型参数， 代表实现当前trait的具体类型
        type Output; //关联类型
        fn add(self, rhs: RHS) -> Self::Output;
}
impl Add for u32 {
        type Output = u32;
        fn add(self, other: u32) -> u32 {self + other}
}
impl Add<&str> for String { //标准库中为String类型实现Add trait //加号右侧为&str类型，Output为String类型
        type Output = String;
        fn add(mut self, other: &str) -> String{
                self.push_str(other);
                self
        }
}
fn main() {
        let a = "Hello,";
        let b = " world! ";
        let c = a.to_string() + b;
        println!("{:?}", c);
} */
/* //use std::ops::Add;
trait Add<RHS = Self> {
        type Output;
        fn add(self, rhs: RHS) -> Self::Output;
}
impl Add<u64> for u32 { //孤儿规则Orphan Rule，trait和实现该trait的类型至少有一个要在当前crate中定义
        type Output = u64;
        fn add(self, other: u64) -> Self:: Output {
                (self as u64) + other
        }
}
fn main() {
        let a = 1u32;
        let b = 2u64;
        //assert_eq!(a+ b, 3);
        assert(a.add(b), 3);
} */
/* use std::ops::Add;
#[derive(Debug)]
struct Point {
        x: i32, y:i32,
}
impl Add for Point {
        type Output = Point; //关联类型必须指定具体类型
        fn add(self, other: Point) -> Point {  //add的返回类型可以写Point，也可以写Self
                Point{
                        x: self.x + other.y,
                        y: self.y + other.x,
                }
        }
}
fn main() {
        println!("{:?}", Point{x: 5, y: 7} + Point{x: 2, y: 6}); //Point{x: 11, y: 9}
} */
/* fn longest<'a >(x: &'a str, y: &'a str) -> &'a str { //生命周期标记 &'a
        //指定生命周期参数时，并没有改变生命周期，只是向借用检查器指出了可用于检查非法调用的约束
        //泛型生命周期'a会被具体化为x与y两者中较短的那个生命周期
        //返回类型的生命周期参数必须与其中一个参数的生命周期参数相匹配
        //当返回的引用没有指向任何参数时，即指向函数内部创建的值，那么返回的内容也就成了悬垂引用
        if x.len() > y.len() {
                x
        } else {
                y
        }
}
//不能确定哪个分支会得到执行，所以无法通过分析作用域来确定返回的引用是否有效
fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {:?}", result);
} */
/* #[derive(Debug)]
struct ImportantExcerpt<'a> {
        part: &'a str,
}
fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
        println!("{:?}", i);
} */
//计算引用生命周期的三条规则：
//1. 每一个引用参数都会拥有自己的生命周期参数
//2. 当只存在一个输入生命周期参数时，这个生命周期会被赋予给所有输出参数
//3. 多个输入生命周期参数中其中一个是&self或&mut self，self的生命周期会被赋给所有输出参数（方法签名）
//特殊的生命周期&'static ，表示整个程序的执行期
/* use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y:&'a str, ann: T) -> &'a str where T: Display{ //生命周期也是泛型的一种
        println!("Announcement! {}", ann);
        if x.len() > y.len()
        {
                x
        } else {
                y
        }
} */
/* use std::thread;
use std::time::Duration;
/* fn simulated_expensive_calculation(intensity: u32) -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
} */
struct Cacher<T> where T: Fn(u32) -> u32 {
        calculation: T, //泛型T由trait约束为Fn trait
        value: Option<u32>, //value初始化为None，Cacher会运行闭包并将结果存储在value的Some变体中成为缓存，不必重复调用
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
        fn new(calculation: T) -> Cacher<T> {
                Cacher {
                        calculation,
                        value: None, //初始化为None
                }
        }
        fn value(&mut self, arg: u32) -> u32 { //先检查是否已经存在Some变体的值，有则直接返回该值，没有则先执行闭包
                match self.value {
                        Some(v) => v,
                        None => {
                                let v = (self.calculation)(arg);
                                self.value = Some(v);
                                v
                        },
                }
        }
}
fn main() {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;
        generate_workout(
                simulated_user_specified_value,
                simulated_random_number
        );
}
fn generate_workout(intensity: u32, random_number: u32) {
        //闭包的类型推导仅在第一次就会被绑定，使用其他类型再次调用就会触发错误
        /* let expensive_closure = |num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
        }; */
        let mut expensive_result = Cacher::new(|num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
        });
        //let expensive_result = simulated_expensive_calculation(intensity);
        if intensity < 25 {
                println!("Today, do {} pushups! ", expensive_result.value(intensity));
                println!(" next, do {} situps! ", expensive_result.value(intensity));
        } else {
                if random_number == 3 {
                        println!("Take a break today! Remember to stay hydrated! ");
                } else {
                        println!("Today, run for {} minutes! ", expensive_result.value(intensity));
                }
        }
} */
/* #[cfg(test)]
mod tests{
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2); //Cacher实例已将Some(1)存在self.value中，无法再改
        assert_eq!(v2, 2);
    }
} */
/* fn main() {
    let x = 4;
    let equal_to_x = |z| z==x; //闭包能够捕获同一作用域中的变量x
    //fn equal_to_x(z: i32) -> bool { z==x }  //函数则不能，该行代码会报错
    let y = 4;
    assert!(equal_to_x(y));
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z==x;
    //println!("Can't use x here: {:?}", x); move方法使得闭包拥有了x的所有权
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
} */
// 闭包捕获环境中的值有三种方式，与函数相类似，定义在Fn系列的trait中
//FnOnce 闭包可以从它的封闭作用域中消耗捕获的变量，即所有权发生转移
//FnMut 可变借用； Fn 不可变借用
/* pub trait Iterator { //Iterate trait只要求实现者手动定义一个方法： next方法，会在每次被调用时返回一个包裹在Some中的迭代器元素，并在迭代器结束时返回None
    type Item;
    fn next(&mut self) -> Option<Self::Item>; //关联类型(associated type)
}
fn main(){
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
} */
//迭代器适配器方法，可使已有的迭代器转换成其他不同类型的迭代器
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    //v1.iter().map(|x| x+1); //Warning: adaptors are lazy, and do nothing unless consumed.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //调用collect方法将其消耗掉
    assert_eq!(v2, vec![2, 3, 4]);
}
