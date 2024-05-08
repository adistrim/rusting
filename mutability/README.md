# Mutability
In Rust, variables are ```immutable``` by default. This means that once a variable is bound to a value, it cannot be changed. This is in contrast to languages like ```JavaScript```, where variables are ```mutable``` by default.

Example code in ```JavaScript```:
```javascript
const x = 42 // x is immutable
let y = 43 // y is mutable
x = 43 // Error: Assignment to constant variable
y = 44 // OK
```

In the above code, ```x``` is immutable and cannot be changed, while ```y``` is mutable and can be changed.

Example code in ```Rust```:
```rust
fn main() {
    let x = 42; // x is immutable
    let mut y = 43; // y is mutable
    x = 43; // Error: Cannot assign twice to immutable variable
    y = 44; // OK
}
```

In Rust, variables are immutable by default, but we can use the ```mut``` keyword to make them mutable. This allows us to control whether a variable can be changed or not, which can help prevent bugs and make out code more predictable.

```Seems like a small thing, but it has a big impact on the safety and predictability of our code.```
___

### **```Note:```**  
```const``` in ```JavaScript``` is not the same as immutable variables in ```Rust```. In ```JavaScript```, we can still update the contents of an object or array that is declared with ```const```, while in ```Rust```, immutable variables ```cannot be changed at all```.

Example code in ```JavaScript```:
```javascript
const arr = [1, 2, 3];
arr.push(4); // OK
console.log(arr); // [1, 2, 3, 4]
```

In the above code, we can still update the contents of the array ```arr``` even though it is declared with ```const```.

To get immutable behavior in ```JavaScript```, we can use a library like [```immutable```](https://www.npmjs.com/package/immutable).
___

### Race Conditions
In ```concurrent programming```, a ```race condition``` occurs when two or more threads access shared data and try to change it at the same time. This can lead to unpredictable behavior and bugs in our code.

Immutable variables can help prevent this by ensuring that the data cannot be changed once it is shared between threads. There is no ```synchronization``` required, as the data is guaranteed to be read-only.

