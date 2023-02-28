use crate::common;

pub fn run_control_flow_examples(){
    common::print_topic_start_message("control flow");
    if_example();
    if_else_example();
    if_with_let();
    loop_with_counter();
    loop_with_label();
    common::print_topic_end_message("control flow");

}

fn if_example() {
    common::print_example_start_message("if");
    let n = 3;
    if n < 5 {
        println!("n is less than 5");
    } else {
        println!("n is greater than 5")
    }
    common::print_example_end_message("if");
}

fn if_else_example(){
    common::print_example_start_message("if-else");
    let n = 6;

    if n % 4 ==0 {
        println!("n is divisible by 4");
    } else if n % 3 ==0 {
        //this will get executed
        println!("n is divisible by 3")
    }else if n % 2 ==0 {
        //this won't get executed
        println!("n is divisible by 2")
    }else{
        //this won't get executed
        println!("n is not divisible by 2, 3 or 4")
    }
    common::print_example_end_message("if-else");
}

fn if_with_let(){
    common::print_example_start_message("if-with-let");
    let condition = true;
    let number = if condition{
        5
    }else{
        6
    };
    println!("number: {}",number);
    common::print_example_end_message("if-with-let");
}

fn loop_with_counter(){
    common::print_example_start_message("loop-with-counter");
    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter ==10{
            break counter*2;
        }
    };
    println!("the result is: {}",result);
    common::print_example_end_message("loop-with-counter");
}

fn loop_with_label(){
    common::print_example_start_message("loop-with-label");
    let mut count = 0;
    'counting_up: loop{
        println!("count");
        let mut remaining = 10;
        loop{
            println!("remaining: {}",remaining);
            if remaining ==9 {
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
    common::print_example_end_message("loop-with-label");
}