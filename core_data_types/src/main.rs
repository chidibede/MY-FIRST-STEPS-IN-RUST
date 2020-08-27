use std::mem;

fn main() {
    // integer types
    let a: u8 = 28;

    println!(
        "a = {} and the size is {} bytes which is {}-bits",
        a,
        mem::size_of_val(&a),
        1 * 8
    );
    // mutablity
    let mut b = 12; // The default integer type inferred by the rust compiler is 32 bits 
    println!("b default = {}", b);
    b = 10;
    println!("b updated = {}", b);

    // check size of an inferred variable
    println!(
        "b = {} and the size is {} bytes which is {}-bits",
        b,
        mem::size_of_val(&b),
        4 * 8
    );

    // check size of a variable you explicitly set

    // isize and usize

    // character data type and size

    // float data types and sizes

    // boolean and size
}
