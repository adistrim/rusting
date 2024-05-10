# Borrowing and References in Rust

Example code in Rust:

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1; // s2 borrows the value of s1

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
```

In the above code, `s1` is a `String` and `s2` is a reference to `s1`. The reference `s2` borrows the value of `s1`. The reference `s2` is immutable by default. If we try to modify the value of `s1` after borrowing, the code will not compile.

`It like s2 has the address of s1 and s1 has the address of the value in the heap. So, s2 is a pointer to the address of s1.`

Let's modify the code from the ownership section:

```rust
fn main() {
    let my_string = String::from("Hello, world!");
    borrow_variable(&my_string); // Pass a reference using '&'
    println!("{}", my_string);
}

fn borrow_variable(some_string: &String) { // Accept a reference using '&'
    println!("{}", some_string);
}
```

Output:

```bash
adistrim@Mac borrowing_and_references % cargo run
   Compiling borrowing_and_references v0.1.0 (/Users/adistrim/rusting/borrowing_and_references)
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/borrowing_and_references`
Hello, world!
Hello, world!
```

In the above code, we are passing a reference to the `borrow_variable` function. The function accepts a reference to a `String` using `&String`. The reference is immutable by default. If we try to modify the value of `my_string` inside the `borrow_variable` function, the code will not compile.

### Mutable References

What if we want to modify the value of the borrowed variable? We can use mutable references. Let's modify the code to use mutable references:

```rust
fn main() {
    let mut my_string = String::from("Hello");
    update_variable(&mut my_string);
    println!("{}", my_string);
}

fn update_variable(some_string: &mut String) {
    some_string.push_str(", World!");
}
```

Output:

```bash
adistrim@Mac borrowing_and_references % cargo run
   Compiling borrowing_and_references v0.1.0 (/Users/adistrim/rusting/borrowing_and_references)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/borrowing_and_references`
Hello, World!
```

In the above code, we are passing a mutable reference to the `update_variable` function. The function accepts a mutable reference to a `String` using `&mut String`. The mutable reference allows us to modify the value of the borrowed variable.

`Note`: We cannot pass &mut reference if our variable is immutable. We need to make the variable mutable using `mut` keyword.

Now how can we do it in variables only previous example was about functions.

```rust
fn main() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;

    s2.push_str(", World!");

    // println!("{}", s1); // Can't print s1 and s2 both at the same time
    println!("{}", s2);
}
```

Output:

```bash
adistrim@Mac borrowing_and_references % cargo run
   Compiling borrowing_and_references v0.1.0 (/Users/adistrim/rusting/borrowing_and_references)
    Finished dev [unoptimized + debuginfo] target(s) in 1.44s
     Running `target/debug/borrowing_and_references`
Hello, World!
```

Notice I added a comment that say can't print s1 and s2 at the same time. If we try to print both at the same time, the code will not compile. This is because we are trying to borrow `s1` and `s2` at the same time. We can only have one mutable reference to a variable at a time.

And this is what `chatGPT` told me about the code:

```
This code isn't working because you're attempting to create a mutable reference to s1 (s2), but then you're trying to use s1 after that mutable reference is created. In Rust, you can't have both a mutable and immutable reference to the same data at the same time, to prevent data races.

Here's a breakdown of what's happening:

You create a mutable string s1.
You create a mutable reference s2 to s1.
You modify s2 by pushing "World!" to it.
You try to print s1, which you've already borrowed mutably with s2. This is not allowed in Rust, as it would violate the borrowing rules.

To fix this, you need to either use s2 or s1, not both.
```

There are some rules to follow when using references in Rust:

- There can be `multiple immutable references` to a variable.
- There can only be `one mutable reference` to a variable.
- If there is a `mutable reference`, there can't be any `immutable references`.

`Damn rust is strict.` But this is what makes Rust `memory safe`!

Rust is like a strict Indian parent who doesn't let you go out after 7 PM. 😂
