pub fn run_scalar_data_type_examples() {
    println!("----------- scalar data type examples : begin -----------\n");
    unsigned_integer();
    signed_integer();
    float();
    boolean();
    character();
    println!("----------- scalar data type examples : end -----------\n");
}

pub fn run_compound_data_type_examples() {
    println!("----------- compound data type examples : begin -----------\n");
    tuple_example();
    array_example();
    println!("----------- compound data type examples : end -----------\n");
}

fn character() {
    println!("**  character example : begin");
    //character types : single quotes, four bytes in size, represent unicode scalar value
    let ch: char = 'a';

    println!("char : {}", ch);
    println!("**  character example : end\n");
}

fn boolean() {
    println!("**  boolean example : begin");
    // boolean types
    let bool_false: bool = false;
    let bool_true: bool = true;

    println!("bool_false : {}", bool_false);
    println!("bool_true : {}", bool_true);
    println!("**  boolean example : end\n");
}

fn float() {
    println!("**  float example : begin");
    //single precision
    let float_f32: f32 = 3.0;

    //double precision
    let float_f64: f64 = 11.20;

    println!("float_f32 : {}", float_f32);
    println!("float_f64 : {}", float_f64);
    println!("**  float example : end\n");

}

fn signed_integer() {
    println!("**  signed integer example : begin");
    // integers : signed
    let signed_int_8bit: u8 = 31;
    let signed_int_16bit: u16 = 31;
    let signed_int_32bit: u32 = 31;
    let signed_int_64bit: u64 = 31;
    let signed_int_128bit: u128 = 31;
    //below depends on the architecture of the machine the program runs on
    let signed_int_arch: usize = 31;

    println!("signed_int_8bit : {}", signed_int_8bit);
    println!("signed_int_16bit : {}", signed_int_16bit);
    println!("signed_int_32bit : {}", signed_int_32bit);
    println!("signed_int_64bit : {}", signed_int_64bit);
    println!("signed_int_128bit : {}", signed_int_128bit);
    println!("signed_int_arch : {}", signed_int_arch);
    println!("**  signed integer example : end\n");
}

fn unsigned_integer() {
    println!("**  unsigned integer example : begin");
    // integers : unsigned
    let unsigned_int_8bit: i8 = 31;
    let unsigned_int_16bit: i16 = 31;
    let unsigned_int_32bit: i32 = 31;
    let unsigned_int_64bit: i64 = 31;
    let unsigned_int_128bit: i128 = 31;
    //below depends on the architecture of the machine the program runs on
    let unsigned_int_arch: isize = 31;

    println!("unsigned_int_8bit : {}", unsigned_int_8bit);
    println!("unsigned_int_16bit : {}", unsigned_int_16bit);
    println!("unsigned_int_32bit : {}", unsigned_int_32bit);
    println!("unsigned_int_64bit : {}", unsigned_int_64bit);
    println!("unsigned_int_128bit : {}", unsigned_int_128bit);
    println!("unsigned_int_arch : {}", unsigned_int_arch);
    println!("** unsigned integer example : end\n");
}



fn tuple_example() {
    println!("**  tuple example : begin");
    //tuples : group of values with different data types; has fixed length
    let tup1: (i32, f64, u8) = (500, 6.8, 8);
    let tup2 = (501, 7.8, 9);

    let a = tup1.0;
    let b = tup1.1;
    let c = tup1.2;

    let (x, y, z) = tup2;

    //empty value; it's called unit
    let k = ();

    println!("tuple 1 : {:?}", tup1);
    println!("tuple 2 : {:?}", tup2);

    println!("tup1.0 : {}, tup1.1: {}, tup1.2: {}", tup1.0, tup1.1, tup1.2);
    println!("tup2.0 : {}, tup2.1: {}, tup2.2: {}", tup2.0, tup2.1, tup2.2);

    println!("a : {}, b: {}, c: {}", a, b, c);
    println!("x : {}, y: {}, z: {}", x, y, z);

    println!("k (unit) : {:?}", k);
    println!("**  tuple example : end\n");
}

fn array_example() {
    println!("**  array example : begin");
    //array : group of values with same data types; has fixed length; data is allocated to stack; not heap
    let arr1 = [0, 2, 3, 4, 5];
    let arr2: [i32; 5] = [6, 7, 8, 9, 10];

    //contains 5 elements, all having the value of 0
    let arr3 = [0; 5];
    let q1_months = ["Jan", "Feb", "Mar"];

    println!("arr1 : {:?}", arr1);
    println!("arr2 : {:?}", arr2);
    println!("arr3 : {:?}", arr3);
    println!("q1_months : {:?}", q1_months);
    println!("second month : {}", q1_months[1]);
    println!("**  array example : end\n");
}
