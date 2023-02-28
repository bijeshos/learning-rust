use crate::common;

pub fn run_variable_examples() {
    common::print_topic_start_message("variable");
    immutable_variable();
    mutable_variable();
    constant();
    shadowing();
    common::print_topic_end_message("variable");
}

fn mutable_variable() {
    common::print_example_start_message("mutable variable");
    //mutable variable
    let mut y = 10;
    println!("value of y (after initialization) is: {}", y);
    y = 11;
    println!("value of y (after re-assignment) is: {}", y);
    common::print_example_end_message("mutable variable");

}

fn immutable_variable() {
    common::print_example_start_message("immutable variable");
    //immutable variable (variable are immutable by default)
    let x = 5;
    println!("value of x is: {}", x);

    //below line won't compile since x is immutable
    //x = 6;
    common::print_example_end_message("immutable variable");
}

fn shadowing() {
    common::print_example_start_message("shadowing");
    //shadowing
    //first declaration
    let x = 6;
    println!("value of x(after declaration) : {:?}", x);
    {
        //second declaration of same variable; this can be of different type as well
        let x = 7.0;
        println!("value of x(after shadowing) : {:?}", x);
    }
    common::print_example_end_message("shadowing");
}

fn constant() {
    common::print_example_start_message("constant");
    //constants : always immutable
    const PI: f32 = 3.141;
    println!("const PI : {}", PI);
    common::print_example_end_message("constant");
}