# Stack and Heap

### Stack

- The stack is a region of memory that operates in a last-in, first-out `(LIFO)` manner.
- The stack is used to store local variables and function call information.
- The stack is a fixed size and is not resizable.
- The stack is fast to allocate and deallocate memory.
- The stack is not suitable for large objects.
- The stack is not suitable for objects that need to be accessed outside of the function that created them.
- It is very organized and efficient storage.

### Heap

- The heap is a region of memory that operates in a first-in, first-out `(FIFO)` manner.
- The heap is used to store objects that need to be accessed outside of the function that created them.
- The heap is resizable.
- The heap is slower to allocate and deallocate memory than the stack.
- The heap is suitable for large objects.
- The heap is suitable for objects that need to be accessed outside of the function that created them.
- It is less organized and less efficient storage.

### Role of Stack & Heap in `Rust`

Rust has clear rules for the stack and heap and data arrangement.

- Stack: Fast allocation and deallocation. `Rust uses the stack for most primitive data types and for data where the size is known at compile time (eg: numbers)`.

- Heap: Used for data that can grow at runtime, such as vectors or strings.

### What's stored on the stack?

- Numbers - i32, f64, etc....
- Booleans - true, false
- Fixed sized arrays

### What's stored on the heap?

- Strings
- Vectors
- Hashmaps

### So how does Rust know when to use the stack or heap?

When Rust compiles your code, it makes a decision on where to store data based on the following rules:

1. If the size of a type is known at compile time, it will be stored on the stack.
2. If the size of a type is not known at compile time, it will be stored on the heap.

- So what's stored on the stack, if the size is unkown at compile time? The answer is `pointers`. Pointers are stored on the stack, but the data they point to is stored on the heap.
- Because the heap is unoganised the data that grows at runtime is stored on the heap and it's address is stored on the stack.

Example code in Rust:

```rust
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
    s.push_str(", World!"); // s is updated on the heap
    println!("Updated string: {}", s);
}
```

### Output

```bash
adistrim@Mac stack_and_heap % cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/stack_and_heap`
Stack function: The sum of 10 and 20 is 30
Heap function: Combined string is 'Hello World'
Original string: Hello
Updated string: Hello, World!
```

Let's see the capacity, length and pointer of the string `s` in the `update_string` function before and after the string is updated.

```rust
fn update_string() {
    let mut s = String::from("Hello"); // s is stored on the heap
    println!("Original string: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    s.push_str(", World!"); // s is updated on the heap
    println!("Updated string: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
}
```

### Output

```bash
adistrim@Mac stack_and_heap % cargo run
   Compiling stack_and_heap v0.1.0 (/Users/adistrim/rusting/stack_and_heap)
    Finished dev [unoptimized + debuginfo] target(s) in 0.93s
     Running `target/debug/stack_and_heap`
Stack function: The sum of 10 and 20 is 30
Heap function: Combined string is 'Hello World'
Original string: Hello
Capacity: 5, Length: 5, Pointer: 0x600002b0c050
Updated string: Hello, World!
Capacity: 13, Length: 13, Pointer: 0x600002b0c050
```

Notice that the pointer of the string `s` is the same before and after the string is updated. This is because the string is updated in place on the heap.

#### But the thing is the pointer will change it is not like that specific space in the heap has the infinite storage if it exceeds the capacity it will move to another space in the heap and the pointer will change.

Let's see this in action I'll add a for loop upto 10 and print the capacity, length and pointer of the string `s` in the `update_string` function.

```rust
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
```

Output

```bash
adistrim@Mac stack_and_heap % cargo run
   Compiling stack_and_heap v0.1.0 (/Users/adistrim/rusting/stack_and_heap)
    Finished dev [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/stack_and_heap`
Stack function: The sum of 10 and 20 is 30
Heap function: Combined string is 'Hello World'
Original string: Hello
Capacity: 5, Length: 5, Pointer: 0x600002618050
Capacity: 29, Length: 29, Pointer: 0x60000241d1c0
Capacity: 58, Length: 53, Pointer: 0x60000311c2c0
Capacity: 116, Length: 77, Pointer: 0x600000a1c100
Capacity: 116, Length: 101, Pointer: 0x600000a1c100
Capacity: 232, Length: 125, Pointer: 0x600001b1c000
Capacity: 232, Length: 149, Pointer: 0x600001b1c000
Capacity: 232, Length: 173, Pointer: 0x600001b1c000
Capacity: 232, Length: 197, Pointer: 0x600001b1c000
Capacity: 232, Length: 221, Pointer: 0x600001b1c000
Capacity: 464, Length: 245, Pointer: 0x7ff1e2f045c0
```

Notice that the pointer of the string `s` is changing after the capacity exceeds the initial capacity of the string otherwise it will remain the same.

`Now that's how it works.`
