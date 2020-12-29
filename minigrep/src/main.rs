use std::env;
//use std::fs;
use std::process;
//use std::error::Error;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        //执行自定义的且不会产生panic!的错误处理策略
        //unwrap会返回Ok中的值；Err则会调用闭包(closure)中编写的代码，闭包参数即|err|
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    //let config = parse_config(&args);
    /*let query = &args[1];
    let filename = &args[2];*/
    /* println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents); */
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
/* fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}
struct Config {
    query: String,
    filename: String,
} //Config包含了拥有自身所有权的String值
//args变量是程序参数值的所有者，而parse_config只是借用了这个值
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Note enough arguments! ");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config {query, filename})
    }
} */
/* fn parse_config(args: &[String]) -> Config {
    let query = arg[1].clone();
    let filename = arg[2].clone();
    Config{query, filename}
} */
