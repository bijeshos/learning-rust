use std::env;
use std::process;

use learning_rust::Config;

fn main() {
    println!("starting program");
    let args: Vec<String> = env::args().collect();
    /* if args.len()<3{
         println!("not enough arguments. stopping program");
         process::exit(1)
     }*/

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("error parsing arguments: {}", err);
        process::exit(1)
    });
    //dbg!(args);

    if let Err(e) = learning_rust::run(config) {
        println!("application error: {e}");
        process::exit(1)
    }
}


