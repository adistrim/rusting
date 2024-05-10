# Ownership in Rust

Ownership is a set of rules that govern how a Rust program uses memory.

### Stack variables

Stack variables are stored in a stack and are popped off the stack when they go out of scope.

```rust
fn main() {
    let x = 5;
    let y = 10;
    println!("sum: {}", sum(x, y));
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}
```

Just saying this still has nothing to do with Ownership till now\*.

### Heap variables

Heap variables are stored in a heap and are not popped off the stack when they go out of scope. They are stored in the heap and are only removed when the program is done with them.

Let's go a little deeper into the heap variables.

```rust
let s1 = String::from("hello");
```

Now we know that `String` get's stored in the heap. And `s1` is pointing to the memory location of the string "hello". We can say `s1` is the owner of the string "hello". `Sounds obvious right? And yes it is.`

But what happens when we do this?

```rust
let s1 = String::from("hello");
let s2 = s1;
```

This is where Ownership comes into play. When we assign `s1` to `s2`, the ownership of the string "hello" is transferred from `s1` to `s2`. Now `s2` is the owner of the string "hello" and `s1` is no longer the owner of the string "hello". This is because Rust does not allow two variables to own the same memory location.

`Same variable pointing to the same memory location is not allowed in Rust.`

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // This will throw an error
println!("{}", s2); // This will print "hello"
```

This is because the ownership of the string "hello" has been transferred from `s1` to `s2`. So, `s1` is no longer the owner of the string "hello".

```rust
fn main() {
    let my_string = String::from("Hello, world!");
    takes_ownership(my_string);
    // println!("{}", my_string); // This will not work because my_string has been moved to takes_ownership
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
```

In the above code, `my_string` is passed to `takes_ownership` function. The ownership of `my_string` is transferred to `some_string` in the `takes_ownership` function. So, `my_string` is no longer the owner of the string "Hello, world!".

### Cloning

If we want to create a copy of the string "hello" and assign it to `s2`, we can use the `clone` method.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{}", s1); // This will print "hello"
println!("{}", s2); // This will print "hello"
```

In the above code, `s1` and `s2` are both the owners of the string "hello".

Because in the heap memory, the string "hello" is stored in a different memory location and `s1` and `s2` are pointing to different memory locations.

`Sounds infficient right? And yes it is.`

So use `clone` only when you really need it.

What if we want to get back the ownership of the string?

```rust
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
```

Output

```bash
adistrim@Mac ownership % cargo run
   Compiling ownership v0.1.0 (/Users/adistrim/rusting/ownership)
    Finished dev [unoptimized + debuginfo] target(s) in 0.98s
     Running `target/debug/ownership`
Hello, world!
Hello, world!
```

So we have to return the ownership of the string from the function.
