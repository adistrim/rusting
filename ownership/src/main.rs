fn main() {
    let my_string = String::from("Hello, world!");
    takes_ownership(my_string);
    // println!("{}", my_string); // This will not work because my_string has been moved to takes_ownership
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
