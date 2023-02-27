//a public function that can be called outside of the current module
pub fn run_function_examples() {
    println!("-----------");
    simple_public_function();

    //this function can only be called from within this module
    private_function();

    function_with_args_and_no_returns(1, 2.0, 'a');

    let x = function_with_one_return_value();
    println!("x: {}", x);

    let y = function_with_args_and_one_return_value(1, 2.0, 'a');
    println!("y: {}", y);

    let (z1, z2) = function_with_args_and_two_return_value(1, 2.0, 'a');
    println!("z1: {}, z2 : {}", z1, z2);
}

// a private function that cannot be called outside of the current module
// unless specified with pub keyword, functions are private
fn private_function() {
    println!("inside a private function")
}

pub fn simple_public_function() {
    println!("inside simple_public_function");
}

fn function_with_args_and_no_returns(a: i32, b: f32, c: char) {
    println!("inside function_with_args_and_no_returns");
    println!("input args : a : {}, b: {}, c: {}", a, b, c);
}

fn function_with_one_return_value() -> i32 {
    println!("inside function_with_one_return_value");

    // a 'statement' does not return any value; it ends with a semi column
    // 'expression' returns a value; it does not end with a semi column

    //the last expression is considered as implicit return; this line does
    10
}

fn function_with_args_and_one_return_value(a: i32, b: f32, c: char) -> i32 {
    println!("inside function_with_return_value");
    println!("input args : a : {}, b: {}, c: {}", a, b, c);
    let a1 = 10;
    let a2 = 20;

    //below returns sum of a1 and a2
    a1 + a2
}

fn function_with_args_and_two_return_value(a: i32, b: f32, c: char) -> (i32, i32) {
    println!("inside function_with_args_and_two_return_value");

    println!("input args : a : {}, b: {}, c: {}", a, b, c);
    let a1 = 10;
    let a2 = 20;

    //below returns a tuple containing a and b
    (a1, a2)
}