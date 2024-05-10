fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;

    s2.push_str(", World!");

    // println!("{}", s1); // Can't print s1 and s2 both at the same time
    println!("{}", s2);
}
