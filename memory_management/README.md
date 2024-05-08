## What is memory management?
Memory management is the process of controlling and coordinating computer memory, assigning portions called blocks to various running programs to optimize overall system performance. It is a critical computer science topic that is essential for understanding how operating systems work. Memory management is a fundamental part of computer architecture and operating systems, and it is crucial for the efficient and effective operation of computer systems.

## Garbage Collection
Garbage collection is a form of automatic memory management that automatically reclaims memory that is no longer in use by the program. It is a critical component of modern programming languages and runtime environments, as it helps prevent memory leaks and other memory-related issues. Garbage collection is used in many programming languages, including ```Java```, ```C#```, ```Python```, and ```JavaScript```, and it is an essential feature of many modern programming environments.

Example code in JavaScript:
```javascript
function main() {
    runLoop()
}

function runLoop() {
    let x = []
    for (let i = 0; i < 100000; i++) {
        x.push(i)
    }
    console.log(x)
}

main()
````

In this example, the ```runLoop``` function creates an array ```x``` and populates it with 100,000 elements. When the function completes, the array ```x``` is no longer needed, but it is not explicitly deallocated. In a garbage-collected environment, the memory used by the array ```x``` will be automatically reclaimed by the garbage collector when it determines that the array is no longer in use.


- Very smart people wrote it (Thanks!)
- Developers can't control it (Manual memory management is not possible)
- Usually no ```dangiling pointers``` (No memory leaks)

## Manual Memory Management
Manual memory management is the process of explicitly allocating and deallocating memory in a program. It requires the programmer to manage memory manually by allocating memory when it is needed and deallocating memory when it is no longer needed. Manual memory management can be error-prone and difficult to get right, as it requires the programmer to keep track of memory allocations and deallocations throughout the program.

Example code in C:
```c
#include <stdlib.h>

int main() {
    int *x = (int *)malloc(sizeof(int));
    *x = 42;
    free(x);
    return 0;
}
```

In this example, the program allocates memory for an integer ```x``` using the ```malloc``` function and then deallocates the memory using the ```free``` function. Manual memory management can be challenging, as it requires the programmer to ensure that memory is allocated and deallocated correctly to prevent memory leaks and other memory-related issues.

- Developers has full control over memory
- ```Dangling pointers``` can occur (Memory leaks)
- ```Buffer overflows``` can occur
- ```Memory corruption``` can occur
- Hard to get right
- ```Learning curve``` is high


## The Rust way

- Developers has full control over memory
- Memory management is extremely safe
- There're set of rules that developers must follow
- If the rules are followed, Rust guarantees that the program is memory safe
- If the rules are not followed, the compiler will catch the errors at compile time (No runtime errors)
- ```Rust literally forces developers to write safe code```
- It's hard to write unsafe code in Rust


Rust doesn't have a ```garbage collector``` and it is the key reason why Rust is so fast and efficient.

That's why Rust is called a ```systems programming language```.

These are the key concepts in Rust memory management:
1. ```Mutability```
2. ```Heap and Stack```
3. ```Borrowing and References```
4. ```Lifetimes```
5. ```Ownership model```

