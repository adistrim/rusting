# Learning Rust (Basically)

Documenting my learning (Idk, maybe it can help someone else too who knows)

`Rust says: I might be hard to type and compile but I will make sure you don't have to worry about memory leaks.`

### Topics in this repo goes like:

1. variables_and_datatypes
2. conditionals_loops_and_functions
3. memory_management
4. mutability
5. stack_and_heap
6. ownership
7. borrowing_and_references
8. structs
9. enums
10. error_handling

## Why not JavaScript (node.js) or python (fastAPI, Django)?

### **1. Type Safety**:

Rust is statically typed, which means that the compiler can catch type errors at compile time. This is a huge advantage over dynamically typed languages like JavaScript and Python, where type errors can only be caught at runtime.

Sample code (JavaScript):

```javascript
let x = 5;
x = "hello";
console.log(x);
```

This code will compile and run without any errors in JavaScript, but it will throw a runtime error because we are trying to assign a string to a variable that was previously assigned a number.

Sample code (Python):

```python
x = 5
x = "hello"
print(x)
```

This code will also compile and run without any errors in Python.

Sample code (Rust):

```rust
fn main() {
    let x = 5;
    x = "hello";
    println!("{}", x);
}
```

This code will not compile in Rust because the compiler will catch the type error at compile time.

`Type safety is important and that is why language like typescript was created to add type safety to JavaScript. But Rust is statically typed from the beginning, which means that type safety is built into the language from the ground up.`

### **2. Memory Safety**:

Rust is memory safe, which means that the compiler can catch memory errors at compile time. This is a huge advantage over languages like C and C++, where memory errors can lead to security vulnerabilities and crashes.

Sample code (C):

```c
#include <stdio.h>

int main() {
    int *x = malloc(sizeof(int));
    *x = 5;
    free(x);
    *x = 10;
    printf("%d\n", *x);
    return 0;
}
```

This code will compile and run without any errors in C, but it will throw a runtime error because we are trying to access memory that has already been freed.

Sample code (Rust):

```rust
fn main() {
    let x = Box::new(5);
    drop(x);
    *x = 10;
    println!("{}", x);
}
```

This code will not compile in Rust because the compiler will catch the memory error at compile time.

### **3. Concurrency / Multithreading**:

Rust has built-in support for multithreading, which makes it easy to write concurrent programs that take advantage of modern hardware.

`JavaScript is single-threaded, which means that it can only run one task at a time. This can be a bottleneck for performance-critical applications that need to take advantage of multiple CPU cores. Although there're ways to run multiple threads in JavaScript, it's not as easy as in Rust.`

Sample code (JavaScript):

```javascript
const start = Date.now();

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function main() {
  await sleep(1000);
  console.log("Task 1 done");
  await sleep(1000);
  console.log("Task 2 done");
  await sleep(1000);
  console.log("Task 3 done");
  console.log("Total time:", Date.now() - start);
}

main();
```

This code will run three tasks sequentially, each taking one second to complete. The total time taken will be around three seconds.

Sample code (Rust):

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let start = std::time::Instant::now();

    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Task 1 done");
    });

    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Task 2 done");
    });

    let handle3 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Task 3 done");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();

    println!("Total time: {:?}", start.elapsed());
}
```

This code will run three tasks concurrently, each taking one second to complete. The total time taken will be around one second.

### **4. Performance**:

Rust is a systems programming language, which means that it is designed to be fast and efficient. This makes it a great choice for performance-critical applications like game engines, operating systems, and web servers.

`JavaScript and Python are high-level languages that are designed for ease of use and developer productivity, but they sacrifice performance in the process. Rust strikes a good balance between performance and productivity, making it a great choice for performance-critical applications.`

### **5. Ecosystem**:

Rust has a growing ecosystem of libraries and tools that make it easy to build a wide range of applications. This includes web development, systems programming, game development, and more.

`JavaScript and Python have large ecosystems with a wide range of libraries and tools, but they are not as well-suited for performance-critical applications. Rust's ecosystem is growing rapidly, and it is becoming a popular choice for a wide range of applications.`
