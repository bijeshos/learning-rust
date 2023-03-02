use crate::log;

pub fn run_vector_examples() {
    log::begin_topic("vector");
    declaration_get_push_example();
    iteration_example();
    using_enum_example();
    log::end_topic("vector");
}


fn declaration_get_push_example() {
    log::start_example("vector: declaration|get|push");
    //declaring a new empty vector of type i32
    let mut v1: Vec<i32> = Vec::new();

    //adding value to vector
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);

    //declaring a new vector with 3 values; i32 would be assumed as the type
    let v2 = vec![11, 12, 13, 14, 15];

    //read elements from vector (implicit type assignment)
    let third1 = v2[2];     //this gives the value ?
    let third2 = &v2[2];   //this gives reference to the value

    //read elements from vector (explicit type assignment)
    let third3: i32 = v2[2];   //this gives the value ?
    let third4: &i32 = &v2[2];  //this gives reference to the value ; if the index is out of range, this panics

    //read elements from vector (explicit type assignment)
    let third5: Option<&i32> = v2.get(2); //this gives an Option<T>; if the index is out of range, this returns None

    println!("v2: {:?}", v2);
    println!("third1: {}, third2: {}", third1, third2);
    println!("third3: {}, third4: {}", third3, third4);

    println!("third5: {:?}", third5);
    log::end_example("vector: declaration|get|push");
}

fn iteration_example() {
    log::start_example("vector iteration");
    //iterating over a vector : using immutable reference to each element
    let v3 = vec![20, 21, 22];
    for element3 in &v3 {
        println!("element in vector : {}", element3);
    }

    //iterating over a vector : using mutable reference to each element
    let mut v5 = vec![40, 41, 42];
    for element5 in &mut v5 {
        println!("element in vector (before change) : {}", element5);
        *element5 += 50; //need to use * dereference option
        println!("element in vector (after change): {}", element5);
    }

    //iterating over a vector
    let v4 = vec![30, 31, 32];
    for element4 in v4 {
        println!("element in vector : {}", element4);
    }
    log::end_example("vector iteration");
}

fn using_enum_example() {
    log::start_example("vector using_enum");
    #[derive(Debug)]
    enum Data{
        Int(i32),
        Float(f32),
        Text(String),
    }

    let v = vec![
        Data::Int(4),
        Data::Float(5.0),
        Data::Text(String::from("6xx")),
    ];
    println!("vector: {:?}",v);
    //dbg!(v);
    log::end_example("vector using_enum");
}