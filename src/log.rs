pub fn begin_topic(topic_name: &str) {
    println!("----------- topic [{}] : begin -----------\n", topic_name);
}

pub fn end_topic(topic_name: &str) {
    println!("----------- topic [{}] : end -----------\n", topic_name);
}

pub fn topic_not_implemented(topic_name: &str) {
    println!("----------- topic [{}] : not implemented -----------\n", topic_name);
}

pub fn start_example(example_name: &str) {
    println!("** example [{}] : begin", example_name);
}

pub fn end_example(example_name: &str) {
    println!("** example [{}] : end\n", example_name);
}