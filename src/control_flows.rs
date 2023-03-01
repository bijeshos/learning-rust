use crate::log;

pub fn run_control_flow_examples() {
    log::begin_topic("control flow");
    if_example();
    if_else_example();
    if_with_let();
    loop_with_counter();
    loop_with_label();
    while_example();
    for_example();
    log::end_topic("control flow");
}

fn if_example() {
    log::start_example("if");
    let n = 3;
    if n < 5 {
        println!("n is less than 5");
    } else {
        println!("n is greater than 5")
    }
    log::end_example("if");
}

fn if_else_example() {
    log::start_example("if-else");
    let n = 6;

    if n % 4 == 0 {
        println!("n is divisible by 4");
    } else if n % 3 == 0 {
        //this will get executed
        println!("n is divisible by 3")
    } else if n % 2 == 0 {
        //this won't get executed
        println!("n is divisible by 2")
    } else {
        //this won't get executed
        println!("n is not divisible by 2, 3 or 4")
    }
    log::end_example("if-else");
}

fn if_with_let() {
    log::start_example("if-with-let");
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("number: {}", number);
    log::end_example("if-with-let");
}

fn loop_with_counter() {
    log::start_example("loop-with-counter");
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is: {}", result);
    log::end_example("loop-with-counter");
}

fn loop_with_label() {
    log::start_example("loop-with-label");
    let mut count = 0;
    'counting_up: loop {
        println!("count");
        let mut remaining = 10;
        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break; //this will break the inner loop
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("end count :{}", count);
    log::end_example("loop-with-label");
}

fn while_example() {
    log::start_example("while");

    let mut number = 5;
    while number > 0 {
        println!("number : {}", number);
        number -= 1;
    }

    log::end_example("while");
}

fn for_example() {
    log::start_example("for");

    let a = [1,2,3,4,5];
    for element in a{
        println!("value in a is : {}",element);
    }

    /*for number in (1..4).rev() {
        println!("{number}!");
    }*/

    log::end_example("for");
}