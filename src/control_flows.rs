pub fn run_control_flow_examples(){
    println!("-----------");
    if_example();

}

fn if_example() {
    let n = 3;
    if n < 5 {
        println!("n is less than 5");
    } else {
        println!("n is greater than 5")
    }
}