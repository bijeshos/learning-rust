use crate::log;

pub fn run_variable_examples() {
    log::begin_topic("variable");
    immutable_variable();
    mutable_variable();
    constant();
    shadowing();
    log::end_topic("variable");
}

fn mutable_variable() {
    log::start_example("mutable variable");
    //mutable variable
    let mut y = 10;
    println!("value of y (after initialization) is: {}", y);
    y = 11;
    println!("value of y (after re-assignment) is: {}", y);
    log::end_example("mutable variable");

}

fn immutable_variable() {
    log::start_example("immutable variable");
    //immutable variable (variable are immutable by default)
    let x = 5;
    println!("value of x is: {}", x);

    //below line won't compile since x is immutable
    //x = 6;
    log::end_example("immutable variable");
}

fn shadowing() {
    log::start_example("shadowing");
    //shadowing
    //first declaration
    let x = 6;
    println!("value of x(after declaration) : {:?}", x);
    {
        //second declaration of same variable; this can be of different type as well
        let x = 7.0;
        println!("value of x(after shadowing) : {:?}", x);
    }
    log::end_example("shadowing");
}

fn constant() {
    log::start_example("constant");
    //constants : always immutable
    const PI: f32 = 3.141;
    println!("const PI : {}", PI);
    log::end_example("constant");
}