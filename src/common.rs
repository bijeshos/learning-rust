pub fn print_topic_start_message(topic_name: &str){
    println!("----------- topic [{}] : begin -----------\n", topic_name);
}
pub fn print_topic_end_message(topic_name: &str){
    println!("----------- topic [{}] : end -----------\n", topic_name);
}

pub fn print_example_start_message(example_name: &str){
    println!("** example [{}] : begin", example_name);
}
pub fn print_example_end_message(example_name: &str){
    println!("** example [{}] : end\n", example_name);
}