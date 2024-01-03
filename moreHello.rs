fn main() {
    println!("Base 10: {}", 1000); // decimal
    println!("Base 2: {:b}", 1000); // binary
    println!("Base 8: {:o}", 1000); // octal
    println!("Base 16: {:x}", 1000); // hexadecimal
    println!("Base 16: {:X}", 1000); // hexadecimal
    println!("My name is {0}, {1} {0}", "James", "Bond");
}

// What's the difference between :x and :X?
// :x prints the hexadecimal number in lowercase and :X prints the hexadecimal number
// in uppercase.
