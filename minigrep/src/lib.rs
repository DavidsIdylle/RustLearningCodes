use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> { //env::args函数的返回类型为std::env::Args的迭代器
        args.next(); //env::args返回值的第一个值是程序本身的名字，因此需要先调用一次并忽略
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        /* if args.len() < 3 {
            return Err("not enough arguments ");
        }
        let query = args[1].clone();
        let filename = args[2].clone(); */
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive, })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    //println!("With text:\n{}", contents);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /* let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results */
    contents.lines().filter(|line| line.contains(query)).collect() //使用了filter适配器来进行过滤，
                                                                   //collect方法将其收集为一个动态数组
//迭代器是一种零开销抽象，因此不会引入额外的运行时开销
//直接“展开”循环，这是一种优化策略，可以消除循环控制语句带来的性能开销
                                                                }
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}