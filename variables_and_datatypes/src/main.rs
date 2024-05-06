fn main() {
    let str = String::from("Hello, World!");
    println!("{}", str);

    let char1 = str.chars().nth(4);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character found"),
    }
}
