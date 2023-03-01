use crate::log;

pub fn run_string_examples() {
    log::begin_topic("string");
    declaration_example();

    log::end_topic("string");
}


fn declaration_example() {
    log::start_example("string: declaration");
    // str is string slice; usually used as &str; part of core language
    //String is part of standard library; not part of core language; UTF-8 encoded
    //String is implemented as a vector of bytes, with a wrappers provided to handle useful operations

    //creates an empty, mutable string
    let mut s1 = String::new();

    //string literal
    let s2 = "this is test sentence 1";
    //This coverts string literal to String
    let s3 = s2.to_string();
    let s4 = "this is test sentence 2".to_string();

    //creates a String from string literal
    let s5 = String::from("this is test sentence 3");

    log::end_example("string: declaration");
}