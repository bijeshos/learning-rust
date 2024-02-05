pub fn run_structs_examples() {

   // definition of the struct
    #[derive(Debug)] // added so that the struct can be used in println
    struct User {
        user_id: u64,
        username: String,
        email: String,
        is_active: bool,

    }

    // creating an immutable instance
    let user1 = User {
        user_id: 1,
        username: String::from("abc"),
        email: String::from("abc@example.com"),
        is_active: true
    };

    // user1.email = String::from("xyz-new@example.com");  //this will fail because the instance is immutable

    println!("{:?}", user1);

    // creating a mutable instance
    let mut user2 = User {
        user_id: 2,
        username: String::from("xyz"),
        email: String::from("xyz@example.com"),
        is_active: true
    };

    user2.email = String::from("xyz-new@example.com");
    println!("{:?}", user2);

    println!("-----------");
    println!("structs_examples: not implemented yet")
}