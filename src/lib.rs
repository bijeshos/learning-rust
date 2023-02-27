use std::env::var;
use std::error::Error;
use std::io;

mod variables;
mod data_types;
mod functions;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("input arg : query: {}", config.query);
    println!("input arg : file path: {}", config.file_path);
    //read_command_line_input();
    variables::basic_variables();
    data_types::scalar_data_types();
    data_types::compound_data_types();
    //functions:private_function(); // this won't compile
    functions::function_examples();

    Ok(())
}


/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust.
            safe, fast, productive.
            pick three.
            Duct tape.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}*/

/*pub fn search<'a>(query: &str, contents: &str) -> Vec<&'a str> {
    vec![];

    print!("query:{}", query);
    print!("contents:{}",contents);
}*/

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments; expecting query and file_path as arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}



fn read_command_line_input() {
    println!("input a value:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("you entered: {input}")
}