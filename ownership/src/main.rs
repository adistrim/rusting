fn main() {
    let mut my_string = String::from("Hello, world!");
    my_string = takes_ownership(my_string);
    // println!("{}", my_string); // This will not work because my_string has been moved to takes_ownership
    println!("{}", my_string)
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string;
}
