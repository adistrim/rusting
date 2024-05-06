## Numbers:
```rust
fn main() {
    let x: i32 = 42;
    println!("x = {}", x);
}
```

What is ```i32```?
- i32 is a ```signed``` 32-bit integer type.
- The range of i32 is -2^31 to 2^31 - 1.
- The default integer type is i32.

```i8, i16, i32, i64, i128```
- i8: signed 8-bit integer type
- i16: signed 16-bit integer type
- i32: signed 32-bit integer type
- i64: signed 64-bit integer type
- i128: signed 128-bit integer type

Rang of ```signed``` integer types ```i8, i16, i32, i64, i128```:
- range of i8: -2^7 to 2^7 - 1
- range of i16: -2^15 to 2^15 - 1
- range of i32: -2^31 to 2^31 - 1
- range of i64: -2^63 to 2^63 - 1
- range of i128: -2^127 to 2^127 - 1


```What is signed integer type?```
- A signed integer type can represent both positive and negative numbers.
- The leftmost bit is used to represent the sign of the number.
- 0 means positive and 1 means negative.

```What is unsigned integer type?```
- An unsigned integer type can represent only positive numbers.
- The leftmost bit is used to represent the value of the number.
- 0 means positive.

```rust
fn main() {
    let x: u32 = 42;
    println!("x = {}", x);
}
```

What is ```u32```?
- u32 is an ```unsigned``` 32-bit integer type.
- The range of u32 is 0 to 2^32 - 1.

```u8, u16, u32, u64, u128```
- u8: unsigned 8-bit integer type
- u16: unsigned 16-bit integer type
- u32: unsigned 32-bit integer type
- u64: unsigned 64-bit integer type
- u128: unsigned 128-bit integer type

Rang of ```unsigned``` integer types ```u8, u16, u32, u64, u128```:
- range of u8: 0 to 2^8 - 1
- range of u16: 0 to 2^16 - 1
- range of u32: 0 to 2^32 - 1
- range of u64: 0 to 2^64 - 1
- range of u128: 0 to 2^128 - 1

```What is floating-point type?```
- A floating-point type can represent real numbers.
- Floating-point types are used to represent numbers with a fractional part.
- Rust has two floating-point types: f32 and f64.

```rust
fn main() {
    let x: f32 = 3.14;
    println!("x = {}", x);
}
```

What is ```f32```?
- f32 is a 32-bit floating-point type.
- The range of f32 is approximately ±3.40282347E+38.
- f32 is a single-precision floating-point type.

```f32, f64```
- f32: 32-bit floating-point type
- f64: 64-bit floating-point type

What is ```f64```?
- f64 is a 64-bit floating-point type.
- The range of f64 is approximately ±1.7976931348623157E+308.
- f64 is a double-precision floating-point type.

```What is the difference between f32 and f64?```
- f32 is a single-precision floating-point type with 32 bits.
- f64 is a double-precision floating-point type with 64 bits.
- f64 has a higher precision than f32.
- f64 is the default floating-point type in Rust.

```What is the default integer type in Rust?```
- The default integer type in Rust is i32.
- The default floating-point type in Rust is f64.

```What if we specify a integer type that is fine for number it is being assigned? But a for loop will overflow it?```

```rust
fn main() {
    let mut x: i8 = 120;

    for _i in 0..10 {
        x = x + 1;
    }

    println!("x = {}", x);
}
```
That's fine. The compiler will not give any error. But the output will be wrong.

```Output:```
```bash
adistrim@Mac variables_and_datatypes % cargo run  
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/variables_and_datatypes`
thread 'main' panicked at src/main.rs:5:13:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

What's ```mut```?
- The ```mut``` keyword is used to make a variable mutable.
- By default, variables are immutable in Rust.
- You can change the value of a mutable variable.

What is the difference between mutable and immutable variables?
- Immutable variables cannot be changed after they are initialized.
- Mutable variables can be changed after they are initialized.

What is ```_i```?
- The ```_i``` is a variable name that is used to ignore the value.
- It is a common convention to use ```_``` as a variable name when you want to ignore the value.

## Booleans:
```rust
fn main() {
    let male: bool = true;
    let mut _is_18_plus: bool = true;

    let age: u8 = 30;

    if age < 18 {
        _is_18_plus = false;
    } else {
        _is_18_plus = true;
    }

    if male {
        println!("You're a male");
    } else {
        println!("You're not a male");
    }

    if _is_18_plus {
        println!("You're 18+");
    } else {
        println!("You're not 18+");
    }
}
```
It's pretty simple stuff. We have two boolean variables ```male``` and ```_is_18_plus```. We have an unsigned integer variable ```age```. We are checking if the age is less than 18, then we are setting ```_is_18_plus``` to false. Otherwise, we are setting it to true. We are checking the value of ```male``` and ```_is_18_plus``` and printing the message accordingly.

```Output:```
```bash
adistrim@Mac variables_and_datatypes % cargo run                 
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/variables_and_datatypes`
You're a male
You're 18+
```

## Strings:
```rust
fn main() {
    let str = String::from("Hello, World!");
    println!("{}", str);

    let char1 = str.chars().nth(4);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character found"),
    }
}
```

What is ```String```?
- The ```String``` type is used to store a sequence of characters.
- The ```String``` type is a growable, heap-allocated string.
- The ```String``` type is provided by the Rust standard library.

What is ```String::from```?
- The ```String::from``` function is used to create a new ```String``` from a string literal.
- The ```String::from``` function is a associated function of the ```String``` type.

What is ```chars```?
- The ```chars``` method is used to get an iterator over the characters of a string.
- The ```chars``` method returns an iterator of type ```Chars```.

What is ```nth```?
- The ```nth``` method is used to get the nth element of an iterator.
- The ```nth``` method returns an ```Option``` that contains the nth element.

What is ```match```?
- The ```match``` keyword is used to match the value of an expression to a pattern.
- The ```match``` keyword is similar to the switch statement in other programming languages.
- The ```match``` keyword is used to handle multiple cases.

What is ```Some```?
- The ```Some``` is an enum variant that represents a value.
- The ```Some``` variant is used to wrap a value.
- The ```Some``` variant is used with the ```Option``` type.

What is ```None```?
- The ```None``` is an enum variant that represents no value.
- The ```None``` variant is used to represent the absence of a value.
- The ```None``` variant is used with the ```Option``` type.



