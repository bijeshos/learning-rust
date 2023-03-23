use crate::log;

pub fn run_string_examples() {
    log::begin_topic("string");
    declaration_example();
    updating_string();
    slicing_strings();
    string_iteration();
    log::end_topic("string");
}


fn declaration_example() {
    log::start_example("string: declaration");
    // str is string slice; usually used as &str; part of core language
    //String is part of standard library; not part of core language; UTF-8 encoded
    //String is implemented as a vector of bytes, with a wrappers provided to handle useful operations

    //creates an empty non-mutable string
    let s1 = String::new();

    //string literal
    let s2 = "this is test sentence 1";
    //This coverts string literal to String
    let s3 = s2.to_string();
    let s4 = "this is test sentence 2".to_string();

    //creates a String from string literal
    let s5 = String::from("this is test sentence 3");

    println!("s1:{}", s1);
    println!("s2:{}", s2);
    println!("s3:{}", s3);
    println!("s4:{}", s4);
    println!("s5:{}", s5);

    log::end_example("string: declaration");
}

fn updating_string() {
    log::start_example("string: update");
    //using push_str : append a string slice
    let mut s1 = String::from("hi");
    println!("s1 (before push_str) : {}", s1);
    s1.push_str(" there");
    println!("s1 (after push_str) : {}", s1);

    //using push - append one char
    let mut s2 = String::from("hello");
    println!("s2 (before push) : {}", s2);
    s2.push('x');
    println!("s2 (before push) : {}", s2);

    //using + operator
    let s3 = String::from("hello");
    let s4 = String::from(" there");
    let s5 = s3 + &s4;
    println!("s5 (after +) : {}", s5);
    //note: s3 would no longer valid now, since it's ownership has been changed
    //reference to s4 was passed; not s4 itself
    // above statement takes ownership of s3, appends copy of contents to it and returns the ownership to the result
    //following won't compile; would raise compilation error : "borrow of moved value: `s3`"
    //println!("s3 (after +) : {}",s3);


    //using format! macro
    let s6 = String::from("one");
    let s7 = String::from("two");
    let s8 = String::from("three");
    let s9 = format!("{s6}-{s7}-{s8}"); //this does not take ownership of args
    println!("s9 (after format) : {}", s9);

    log::end_example("string: update");
}

fn slicing_strings() {
    log::begin_topic("string: slice");
    let s1 = String::from("hello there");
    let s2 = &s1[0..4];  //gets first 4 bytes (not chars) of s1
    println!("s2: {}", s2);
    log::end_example("string: slice");
}

fn string_iteration() {
    log::begin_topic("string: iteration");
    let s1 = String::from("hello there");

    for c in s1.chars() {
        println!("c: {}", c);
    }

    for b in s1.bytes() {
        println!("b: {}", b);
    }

    log::end_example("string: iteration");
}