fn main() {
    stack_fn(); // Call the function that uses stack
    heap_fn(); // Call the function that uses heap
    update_string(); // Call the function that changes size of the variable at runtime
}

fn stack_fn() {
    let a = 10; // a is stored on the stack
    let b = 20; // b is stored on the stack
    let c = a + b; // c is stored on the stack
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

// This function will create new strings on the heap
fn heap_fn() {
    let s1 = String::from("Hello"); // s1 is stored on the heap
    let s2 = String::from("World"); // s2 is stored on the heap
    let s3 = format!("{} {}", s1, s2); // s3 is stored on the heap
    println!("Heap function: Combined string is '{}'", s3);
}

// This function will update the size of the string at runtime it will not create new string on the heap
fn update_string() {
    let mut s = String::from("Hello"); // s is stored on the heap
    println!("Original string: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    for _ in 0..10 {
        s.push_str(", World, Everyone, Rust!"); // s is updated on the heap
        println!(
            "Capacity: {}, Length: {}, Pointer: {:p}",
            s.capacity(),
            s.len(),
            s.as_ptr()
        );
    }
}
