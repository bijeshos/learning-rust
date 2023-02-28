pub fn run_variable_examples() {
    println!("----------- variable examples : begin -----------\n");
    immutable_variable();
    mutable_variable();
    constant();
    shadowing();
    println!("----------- variable examples : end -----------\n");
}

fn mutable_variable() {
    println!("**  mutable variable example : begin");
    //mutable variable
    let mut y = 10;
    println!("value of y (after initialization) is: {}", y);
    y = 11;
    println!("value of y (after re-assignment) is: {}", y);
    println!("**  mutable variable example : end \n");
}

fn immutable_variable() {
    println!("** immutable variable example : begin");
    //immutable variable (variable are immutable by default)
    let x = 5;
    println!("value of x is: {}", x);

    //below line won't compile since x is immutable
    //x = 6;
    println!("** immutable variable example : end\n");
}

fn shadowing() {
    println!("** shadowing example : begin");
    //shadowing
    //first declaration
    let x = 6;
    println!("value of x(after declaration) : {:?}", x);
    {
        //second declaration of same variable; this can be of different type as well
        let x = 7.0;
        println!("value of x(after shadowing) : {:?}", x);
    }
    println!("** shadowing example : end \n");
}

fn constant() {
    println!("** constant example : begin");
    //constants : always immutable
    const PI: f32 = 3.141;
    println!("const PI : {}", PI);
    println!("** constant example : end\n");
}