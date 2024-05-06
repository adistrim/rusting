## Conditionals, Loops, and Functions
```rust
fn main() {
    let sentence = String::from("Rust is a lot");
    let is_string: bool = true;

    if is_string {
        println!("{}", get_first_word(sentence))
    } else {
        println!("Not a string");
    }
}

fn get_first_word(sentence: String) -> String {
    let mut word = String::from("");
    for c in sentence.chars() {
        if c == ' ' {
            break;
        }
        word.push(c);
    }
    return word;
}
```
The above code snippet demonstrates the use of conditionals, loops, and functions in Rust. The `main` function initializes a string `sentence` and a boolean `is_string`. It then checks if `is_string` is true, and if so, calls the `get_first_word` function to get the first word of the sentence. The `get_first_word` function takes a string as input and iterates over its characters to find the first word by breaking at the first space character encountered.

This is a ```very simple example of for loop``` which counts from 0 to 9999999 and prints the number of iterations it took to reach the end.
```rust
fn main() {
    for i in 0..10000000 {
        println!("{}", i);
    }
}
```

Same implementation can be done using ```while loop``` as well.
```rust
fn main() {
    let mut i = 0;
    while i < 10000000 {
        println!("{}", i);
        i += 1;
    }
}
```

This is a simple example of a ```function``` that takes two integers as input and returns their sum.
```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

This is a simple example of a ```function``` that takes a string as input and returns the length of the string.
```rust
fn get_string_length(s: String) -> usize {
    return s.len();
}
```

This is a very simple example of ```if-else``` conditional statement in Rust.
```rust
fn main() {
    let x = 5;
    if x < 10 {
        println!("x is less than 10");
    } else {
        println!("x is greater than or equal to 10");
    }
}
```