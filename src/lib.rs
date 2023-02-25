use std::error::Error;
use std::io;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("arg: query: {}", config.query);
    println!("arg: file path: {}", config.file_path);
    read_command_line_input();
    Ok(())
}


fn read_command_line_input() {
    println!("input a value:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("you entered: {input}")
}