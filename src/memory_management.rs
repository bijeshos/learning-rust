use crate::log;

pub fn run_memory_management_examples() {
    log::begin_topic("memory management");
    string_scope();
    stack_only_data();
    string_assignment();
    string_clone();
    int_reassignment();
    array_reassignment();
    ownership_example();
    function_return();
    immutable_reference();
    mutable_reference();
    multiple_references();
    log::end_topic("memory management");
}

fn string_scope() {
    {
        let s = String::from("Rust");
    }
}

fn string_clone() {
    //data is kept in heap
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1:{}", s1); //this works
    println!("s2:{}", s2); //this works
}

fn string_assignment() {
//data is kept in heap
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1:{}", s1); //this does not compile
    println!("s2:{}", s2);
}

fn stack_only_data() {
//integer values are pushed to stack
    let x = 5;
    let y = x;
    println!("x:{}, y:{}", x, y);

    //stack-only data
    let a1 = [1, 2, 3];
    let a2 = a1;
    println!("a1:{:?}", a1);
    println!("a2:{:?}", a2);
}

fn int_reassignment() {
    let n1 = 2015;  //trivia : 2015 is the year Rust 1.0 was released
    let n2 = n1;

    println!("{} is the first Rust edition", n1); //this works
}

fn array_reassignment() {
    let a1 = [2015, 2018, 2021]; //trivia : these are the Rust editions available at the moment
    let a2 = a1;

    println!("a1: {:?}", a1); //this works
    println!("a2: {:?}", a2); //this works
}

fn ownership_example() {
    let s = String::from("Rust");
    ownership_example_sub_fn_a(s);
    let n = 2015;
    ownership_example_sub_fn_b(n);
}

fn ownership_example_sub_fn_a(x: String) {
    println!("{}", x);
}

fn ownership_example_sub_fn_b(m: i32) {
    println!("{}", m);
}

fn function_return() {
    let s1 = function_return_sub_fn_a();
    let s2 = String::from(" is cool");
    let s3 = function_return_sub_fn_b(s2);
}

fn function_return_sub_fn_a() -> String {
    let x = String::from("Rust");
    x
}

fn function_return_sub_fn_b(y: String) -> String {
    y
}

fn immutable_reference() {
    let s1 = String::from("Rust");
    let len = immutable_reference_sub_fn_a(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn immutable_reference_sub_fn_a(x: &String) -> usize {
    //x.push_str(", is cool"); //this won't compile!
    x.len()
}

fn mutable_reference() {
    let mut s = String::from("Rust");
    mutable_reference_subn_fn_a(&mut s);
}

fn mutable_reference_subn_fn_a(x: &mut String) {
    x.push_str(", is cool");
}

fn multiple_references() {
    let mut s = String::from("Rust");
    let r1 = &s; // this works
    let r2 = &s; // this works
    println!("{} and {}", r1, r2);
    let r3 = &mut s; // this works
    println!("{}", r3);
}