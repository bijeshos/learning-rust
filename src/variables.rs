pub fn variable_examples() {
    println!("-----------");
    immutable_variable();
    mutable_variable();
    constant();
    shadowing();
}

fn shadowing() {
    //shadowing
    //first declaration
    let x = 6;
    println!("value of x(after declaration) : {:?}", x);
    {
        //second declaration of same variable; this can be of different type as well
        let x = 7.0;
        println!("value of x(after shadowing) : {:?}", x);
    }
    println!("-----------");
}

fn mutable_variable() {
    //mutable variable
    let mut y = 10;
    println!("value of y (after initialization) is: {}", y);
    y = 11;
    println!("value of y (after re-assignment) is: {}", y);
    println!("-----------");
}

fn immutable_variable() {
    //immutable variable (variable are immutable by default)
    let x = 5;
    println!("value of x is: {}", x);

    //below line won't compile since x is immutable
    //x = 6;
    println!("-----------");
}

fn constant(){
    //constants : always immutable
    const PI: f32 = 3.141;
    println!("const PI : {}", PI);
    println!("-----------");
}