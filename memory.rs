fn main() {
    let mut v = Vec::new(); // allocate dynamic vector
    v.push(10);
    v.push(20);

    // Borrow reference (no ownership transfer)
    print_vector(&v);

    // Ownership automatically cleaned up at end of scope
    println!("Rust: memory freed automatically when vector goes out of scope");
}

fn print_vector(v: &Vec<i32>) {
    for val in v {
        println!("{}", val);
    }
}
