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
/* fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    //v1.iter().map(|x| x+1); //Warning: adaptors are lazy, and do nothing unless consumed.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //调用collect方法将其消耗掉
    assert_eq!(v2, vec![2, 3, 4]);
} */
/* enum List {
    Cons(i32, Box<List>), //Cons一部分存储i32，另一部分存储装箱指针数据(Box)
                          //Box实现了Deref trait，可将Box<T>值当作引用来对待；Drop trait，在离开作用域时所指向的堆数据会被自动释放
    Nil,
}
use crate::List::{Cons, Nil};
fn main(){
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
} */
/* use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;  //定义了Deref trait的一个关联类型
    fn deref(&self) -> &T {
        &self.0  //返回一个指向值的引用，进而可通过*解引用来访问此值
    }            //Deref trait 的 deref 方法使编译器可以从任何实现了 Deref 的类型中获取值
                 //解引用转换：能够将实现了Deref trait的 T 的引用转换为T经过Deref操作后生成的引用
}
fn main(){
    let x = 5;
    let y = &x;  //y是x的引用
    assert_eq!(5, x);
    assert_eq!(5, *y);  //解引用操作符*

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);  //隐式展开为*(y.deref())
    let m = MyBox::new(String::from("Rust"));  //String提供的Deref trait实现会返回字符串切片
    hello(&m);
    //如果没有解引用转换功能，则为 hello(&(*m)[..])
}
fn hello(name: &str) {
    println!("Hello, {}! ", name);
} 
//解引用转换情形：
// Deref<Target = U> &T => &U 不可变到不可变
// DerefMut<Target = U> &mut T => &mut U 可变到可变
// Deref<Target = U> &mut T => &U 可变到不可变 */
/* struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data '{}' !", self.data);
    }
}
fn main(){
    let c = CustomSmartPointer{data: String::from("my stuff")};
    let d = CustomSmartPointer{data: String::from("other stuff")};  //变量丢弃顺序与创建顺序相反
    println!("CustomSmartPointers created.");
    //c.drop(); 已经在main函数结束自动调用drop，因此不能再次显式调用drop； drop函数——析构函数(destructor)，与constructor相对应
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
} */
/* enum List {
    Cons(i32, Rc<List>), //Rc<T>通过不可变引用允许共享只读数据
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main(){
    let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); //Rc::strong_count读取引用计数（强引用计数）
    let _b = Cons(3, Rc::clone(&a));  //Rc::clone不会执行数据的深度拷贝操作，只会增加引用计数
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    } //Rc<T>的Drop自动实现
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
} */
//RefCell<T>内部可变性模式，在数据结构中使用了unsafe代码来绕过Rust正常的可变性和借用规则
//对于RefCell<T>代码，Rust只会在运行时检查这些规则，并在出现违反借用规则情况下触发panic以提前终止

//Rc<T>允许一份数据有多个所有者，而Box<T>和RefCell<T>都只有一个所有者
//Box<T>允许在编译时检查的可变或不可变借用；Rc<T>仅允许编译时检查的不可变借用；RefCell<T>允许运行时检查的可变或不可变借用
//RefCell<T>本身不可变，但允许在运行时检查可变借用：内部可变性模式允许用户更改一个不可变值的内部数据

/* #[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>), // List保持了表面的不可变状态，且能够在必要时借RefCell<T>来修改内部存储的数据
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
fn main(){
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;  // borrow_mut会返回一个RefMut<T>智能指针
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
} */
/* use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // 可以灵活修改Cons变体指向的下一个List值
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail()); //调用tail方法来得到a的RefCell<Rc<List>>值的引用并暂存至link变量中

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); //将Rc<List>中存储的值由Nil修改为b中存储的Rc<List>
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count agter changing a = {}", Rc::strong_count(&a)); //由于a仍然持有一个指向b中Rc<List>的引用，所以引用计数仍为1，Rc<List>在堆上的内存不会得到释放
    //println!("a next item - {:?}", a.tail()); //循环引用，会造成栈的溢出
} */
/* use std::rc::{Rc, Weak};
use std::cell::RefCell;
#[derive(Debug)]
struct Node {
    value:i32,
    parent: RefCell<Weak<Node>>,  //子节点可以指向父节点却不持有它的所有权
    children: RefCell<Vec<Rc<Node>>>,  //持有自身所有的子节点并通过变量来共享它们的所有权
}
fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); //leaf强引用计数为1，弱引用计数为0
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());  //使用upgrade方法获得指向leaf父节点的引用
    {
        let branch = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),  // leaf的Node分别拥有了leaf和branch两个所有者。可以使用branch.leaf来从branch访问leaf
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //borrow_mut方法取出leaf中parent字段的可变借用，
                                                        //Rc::downgrade函数获取branch中Rc<Node>的Weak<Node>引用并存入leaf的parent字段中
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch)); //branch中Rc<Node>的强引用计数为1，若引用计数为1
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); //leaf的强引用计数为2， 弱引用仍为0
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); //包含了branch实际内容的Some变体，在作用域结束后访问会得到None值
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); //只有leaf变量指向了存储在自身中的Rc<Node>，所以其强引用计数为1
} */
/* 不同线程在执行过程中的具体顺序无法确定，所以：
*当多个线程以不一致的顺序访问数据或资源时产生的竞争状态
*两个线程同时尝试获取对方持有的资源时产生的死锁
*特定情况下出现且难以稳定重现和修复的bug
*/
//use std::thread;
/* use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| { //thread::spawn返回类型是一个自持有所有权的JoinHandle
        for i in 1..10{
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap(); //主线程会等待新线程提前执行完毕后才开始执行自己的for循环
    for i in 1..5{
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); //在线程句柄上调用join函数会阻塞当前线程，直到句柄对应的新线程运行结束
} */
//use std::sync::mpsc; //multiple producer, single consumer(多个生产者，单个消费者)
/* fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {      //闭包前添加move关键字，可以强制闭包获得它所需值的所有权
        println!("Here's a vector: {:?}", v); //由于Rust不知道新线程会运行多久，因此无法确定v的引用一直有效
    });
    //drop(v);  //move将v移动到了闭包环境中，所以无法在主线程中继续使用它来调用drop函数
    handle.join().unwrap();
} */
//use std::time::Duration;
/* fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {  //使用move关键字将tx移动到新线程中
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();  //send方法会返回Result<T,E>类型，使用unwrap()来触发panic
            //println!("val is {}", val); //所有权已被转移给接收者
            thread::sleep(Duration::from_secs(1)); //新线程中将数据发送至主线程，并调用Duration值来稍作暂停
        }      
    });
    for received in rx{
        println!("Got: {}", received);  //主线程中rx被视为迭代器
    }
    //let received = rx.recv().unwrap();  //recv会将传入值包裹在Result<T,E>中返回
    //recv会阻塞主线程的执行直到有值被传入通道；try_recv方法不会阻塞线程，会立即返回Result<T,E>
    //println!("Got: {}", received);
} */
/* fn main() {
    let (tx,rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);  //clone生成可以传入新线程的发送端句柄
    thread::spawn(move || {   //clone得到的发送端句柄传入第一个新线程
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {   //原始通道发送端传入第二个新线程
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
} */
// 互斥体 mutex（mutual exclusion）一个互斥体在任意时刻只允许一个线程访问数据
// 互斥体包含包含锁，以记录当前谁拥有数据的唯一访问权
// *必须在使用数据前尝试获取锁
// *必须在使用完互斥体守护的数据后释放锁
//use std::sync::Mutex;
/* fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();  //阻塞方式获取锁，unwrap以触发panic
        *num = 6;  //其返回值将视为指向内部数据的可变引用
                   //Mutex<T>的lock方法调用会返回一个名为MutexGuard的智能指针
                   //这个智能指针通过Deref指向存储在其内部的数据，Drop实现自动解锁
    }
    println!("m = {:?}", m);
} */
/* use std::thread;
use std::rc::Rc;  //Rc<T>在跨线程使用时并不安全。
                  //虽然可以实现引用计数，但没有使用人格并发原语来保证修改计数的过程不会被另一个线程所打断
fn main() {
    let counter = Rc::new(Mutex::new(0));   //Rc<T>包裹Mutex<T>，并在每次需要移动所有权至线程时克隆Rc<T>
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);  //Rc<T>无法安全地在线程间传递，即该类型不满足Send trait约束
        let handle = thread::spawn(move || {       //通过迭代创建出10个线程
            let mut num = counter.lock().unwrap(); //闭包会把计数器移入线程中，并调用Mutex<T>的lock方法来进行加锁并为其值加1
            *num += 1;                             //counter的所有权被移动到多个线程中，这是不允许的，应用多重所有权来解决
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
} */
/* use std::sync::{Mutex, Arc};  //Arc<T> 原子引用计数，和原生类型的用法十分相似，且可以在多个线程间安全共享
                              //接口与Rc<T>一致，但其代价是一定的性能开销
use std::thread;
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {:?}", *counter.lock().unwrap());
} */
// RefCell<T> / Rc<T> 和 Mutex<T> / Arc<T> 之间的相似性
// Rc<T>循环指向时会造成内存泄漏；Mutex<T>也会有产生死锁的风险

//std::marker模块内的Sync trait和Send trait
//只有实现了Send trait才能安全地在线程间转移所有权
//只有实现了Sync trait才能安全地被多个线程引用：RefCell<T> Cell<T> Rc<T>都不满足Sync约束
/* pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    pub fn average(&self) -> f64 {  //average方法读取average字段值，但不能修改
        self.average
    }
    // 公共方法add、remove和average
    fn update_average(&mut self) {  // add 和 remove 会调用update_average来更新average字段
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }  
} */
/* fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday{
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { //if let 可以对变量进行覆盖
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    while let Some(top) = stack.pop(){
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c', 'd'];
    for (index, value) in v.iter().enumerate() {  
        //enumerate作为迭代器的适配器，在每次迭代过程中生成一个包含值本身及值索引的元组
        println!("{} is at index {}", value, index);
    }
    //let PATTERN = EXPRESSION; 
    let _x = 5; //将此处匹配到的所有内容绑定至变量x，x就是整个模式本身
    let (_x, _y, _z) = (1, 2, 3); //该元组模式可理解为嵌套至3个独立变量模式
} */
/* fn print_coordinates(&(x, y): &(i32, i32)) {  //函数参数同样也是模式
    println!("Current location: ({}, {})", x, y);
}
fn main() {
    let point = (3, 5);  //&(x, y)能够和&(3, 5)匹配
    print_coordinates(&point);

    //let x = 5; //不可失败模式irrefutable：能够匹配表达式右侧所有可能的值
    //if let Some(x) = a_value; //可失败模式refutable：可能因为某些特定的值而匹配失败

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 | 4 => println!("three or four"),
        5..=9 => println!("five through nine"),  //范围模式只被允许使用数值或char值
        _ => println!("others"),
    }
    let x = 'c';
    match x {
        'a'..= 'j' => println!("early ASCII letter"),
        'k'..= 'z' => println!("late ASCII letter"),
        _ => println!("something else"), 
    }
    let x= Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),  //这里的y可以匹配Some变体中的任意值
                                                          //在match创建的新作用域中，y是一个新的变量
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
} */
/* struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p = Point {x: 0, y: 7};
    let Point {x: a, y: b} = p;  //a, b两个变量分别匹配p结构体中字段x和y的值
    assert_eq!(0, a);
    assert_eq!(7, b);
    let Point {x , y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y),
    }
} */
/* enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}
fn foo(_: i32, y: i32) {  //在函数签名中忽略某一参数
    println!("This code only uses the y parameter: {}", y);
}
fn main() {
    foo(3,4);
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));  //匹配语法还支持嵌套结构
    match msg {
        Message::Quit => {  //无法从其内部解构出其他值
            println!("The Quit variant ha no data to destructure.")
        },
        Message::Move {x, y} => {  //类似于结构体的匹配模式，内部字段可分解并用于匹配分支的代码块中
            println!("Move in the x direction {} and in the y direction {}", x, y)
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
        },
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {  //通过该分支确定是否都是Some变体
            println!("Can't overwrite an existing customized value");
        }
        _ => {  //剩余所有情形
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {  //_完全不会进行变量绑定操作
        println!("found a string");
    }
    println!("{:?}", s);  //这里仍符合所有权规则

    if let Some(_s) = s {  //以下划线开头的未使用变量仍然将值绑定到了变量上
        println!("found a string");
    }
    println!("{:?}",s);  //s中的值被移动到了_s中，因此违背了所有权规则

} */
/* struct Point {
    x: i32, y: i32, z: i32,
}
enum Message {
    Hello { id: i32 },
}
fn main() {
    let origin = Point {x: 0, y: 0, z: 0};
    match origin {
        Point {x, ..} => println!("x is {}", x),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last)
        },
    }
    let num = Some(4);
    match num {
        Some(x) if x < 5 => { //匹配守卫：出现在match分支中的if条件语句
            println!("less than five: {}", x)
        },
        Some(x) => println!("{}", x),
        None => (),
    }
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("got 50"),
        Some(n) if n==y => println!("Matched, n = {:?}", n), //避免覆盖y变量，此处y来自外部值
        _ => println!("Default case, x = {:?}", x), 
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), // 优先级关系  (4 | 5 | 6) if y 
        _ => println!("no"),
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => { //@运算符：测试是否匹配的同时创建存储该值的变量
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10 ..= 12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {  //所有值都可以
            println!("Found some other id: {}", id)
        },
    }
} */

//unsafe superpower 不安全超能力：unsafe关键字允许的四种操作
//解引用裸指针
    //两种裸指针：*const T和*mut T（*是类型名的一部分而非解引用操作）
    //允许忽略借用规则，可以同时拥有指向同一个内存地址的可变和不可变指针，或者拥有指向同一个地址的多个可变指针
    //不能保证自己总是指向了有效的内存地址
    //允许为空
    //没有实现任何自动清理机制
/* fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; //可以在安全代码内合法创建裸指针，但不能解引用
    let r2 = &mut num as *mut i32;
    unsafe{  //在unsafe块中解引用裸指针
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
    let address = 0x012345usize;
    let _r = address as *const i32; //指向内存中任意地址的裸指针
} */
//调用不安全的函数或方法
/* unsafe fn dangerous() {} //不安全函数的函数体也是unsafe代码块
use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();  //得到切片的长度
    let ptr = slice.as_mut_ptr();  //返回类型为*mut i32的裸指针
    assert!(mid <= len); //通过断言，确保unsafe中裸指针都会指向有效的切片数据，且不会产生任何的数据竞争
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),  //接收一个裸指针和长度来创建一个切片
        slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid) //使用mid参数调用offset方法得到从mid开始的裸指针
        )
    }
    //(&mut slice[..mid], &mut slice[mid..])  //元组不能同时被可变借用两次
}
fn main() {
    unsafe {
        dangerous(); //需要单独在unsafe代码块中调用dangerous函数
    }
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3); //在实现时以安全方式使用了unsafe代码
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
extern "C" { //C指明了外部函数使用的应用二进制接口ABI
    fn abs(input: i32) -> i32;
}
fn main() {
    unsafe {
        println!("Absolute value of -3 according to C:{}", abs(-3));
    }
} */
/* #[no_mangle]  //可以在编译并链接后被C语言代码访问的函数
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
} */
//访问或修改可变的静态变量
/* static HELLO_WORLD: &str = "Hello, world! ";  //静态变量只能存储拥有'static生命周期的引用
//静态变量的值在内存中拥有固定的地址；常量允许在任何被使用到的时候复制其数据
static mut COUNTER: u32 = 0;  //访问和修改可变静态变量是不安全的
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn main() {
    println!("name is :{}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);  //单线程，若有多个线程同时访问COUNTER时，则可能会出现数据竞争
    }
} */
//实现不安全trait
    //trait中存在至少一个方法拥有编译器无法校验的不安全因素
/* unsafe trait Foo{
    //某些方法
}
unsafe impl Foo for i32 {
    //对应的方法实现
} */
//在trait的定义中使用关联类型指定占位类型
/* pub trait Iterator {  //关联类型，不需要再使用该trait时标注类型，
                      //只能实现一次impl Iterator for _ 操作，即 _ 只能拥有一个特定的Item类型
    type Item;  //占位类型，trait的实现者需要为Item指定具体的类型，并在next方法中返回包含该类型的Option<T>
    fn next(&mut self) -> Option<Self::Item>;
} */
/* pub trait Iterator<T> {  //泛型需要在每次实现该trait时标注类型，且next方法也必须提供类型标注
                            //impl Iterator for _ , 则 _ 可以拥有一个类型的不同trait实现
    fn next(&mut self) -> Option<T>;
}
 */
//使用泛型参数时为其指定默认的具体类型，常用于运算符重载中
    //允许扩展一个类型而不破坏现有代码
    //允许在特定场合进行自定义
/* use std::ops::Add;
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters { //指定Meter为Add trait的RHS类型参数的值
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
/* trait Add<RHS = Self> {  //RHS=Self即默认类型参数
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
} */
impl Add for Point {
    type Output = Point;  //add trait拥有一个Output的关联类型，用以确定add方法的返回类型
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    assert_eq!(Point {x: 1, y: 0} + Point {x: 2, y: 3},
    Point {x: 3, y: 3});
} */
//完全限定语法：调用相同名称的方法
    //Rust不会阻止两个trait拥有相同名称的方法，也不会阻止你为同一个类型实现这样的两个trait
/* trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person); //在方法前指定trait名称可以清晰表明调用的具体fly实现
    person.fly();
} */
/* trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    //println!("A baby dog is called a {}", Animal::baby_name()); 
    //没有self参数的关联函数，所以无法推断它的具体实现
    //完全限定语法形式：
    //<Type as Trait>::function(receiver_if_method, next_arg, ...);
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
} */
//超trait：在trait中附带另一个trait功能
/* use std::fmt;
struct Point {
    x: i32,
    y: i32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
//Outline_print必须注明自己只能用于提供了Display功能的类型
trait OutlinePrint: fmt::Display { //依赖于Display trait，因此能够使用to_string函数
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len+4));
        println!("*{}*", " ".repeat(len+2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len+2));
        println!("{}", "*".repeat(len+4));
    }
} */
//newtype模式在外部类型上实现外部trait
/* use std::fmt;
struct Wrapper(Vec<String>);  //创建一个持有Vec<T>实例的Wrapper结构体，为其实现Display 
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))  //self.0来访问Vec<T>
    }
}
fn main() {
    let w = Wrapper(vec![String::from("hello"),String::from("world")]);
    println!("w = {}", w);
} */
//newtype模式实现类型安全与抽象
    //静态地保证各种值之间不会被混淆及表明值使用的单位
    //为类型的某些细节提供抽象能力
    //隐藏内部的实现细节
//使用类型别名创建同义类型
/* fn main(){
    type Kilometers = i32;  //同义，但并不认为是新类型，
                            //因此无法享有newtype模式附带的类型检查便利
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x+y);
    type Thunk = Box<dyn Fn() + Send + 'static>;  //减少字符重复性
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk){}
    fn returns_long_type() -> Thunk {
        Box::new(|| ())
    }
}
use std::io::*; //type Result<T> = Result<T, std::io::Error>; 
use std::fmt;
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) ->Result<()>;
} */
//永不返回的Never类型
    //不会返回值的函数也被称作发散函数
/* fn main() {
    let guess = match guess.trim().parse() {
        Ok(_) => 5,  //将u32作为guess的类型，或者!可以被强制转换成其他任意类型
        Err(_) => continue, //continue的返回值是！，即Never类型
    };
}
//panic!宏定义也使用了Never类型
//panic!只会中断当前程序而不会产生值
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {  //整个match表达式返回类型为T
            Some(val) => val,
            None => panic!("Called 'Option::unwrap()' on a 'None' value"),
        }
    }
} */
//loop返回类型也为!
//动态大小类型和sized trait
    //&T被视为存储了T所在内存地址的单个值，但&str实际上由str的地址及其长度组成
    //trait也是可以通过其名称来进行引用的动态大小类型
    //Sized trait可确定一个类型的大小在编译时是否可知
//fn generic<T: Sized>(t: T) {} //为每个泛型函数隐式添加Sized约束
//fn generic<T: ?Sized>(t: &T) {} //表达了与Sized相反的含义
                                  //类型可能不是Sized的，因此需要将t放置在某种指针之后
fn main() {}