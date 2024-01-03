fn main() {
    println!("Hello, Rust!");
    format!("Hello, Rust!");
    println!("Hello, {}!", format!("Rust"));
    eprintln!("Hello, {}!", format!("Rust"));
}

// what's format!()?
// format!() is similar to println!() but instead of printing the output to the screen,
// it returns a String with the output. This String is then passed to println!() which
// prints it to the screen. This extra step is necessary because format!() is a macro
// and not a function. We'll learn more about macros in a later chapter.

// what's a macro?
// A macro is a way of writing code that writes other code for you, which is known as
// metaprogramming. Macros are expanded before the compiler interprets your code, so a
// macro can expand into any valid Rust code, even code that may cause a compiler error
// if you tried to write it manually.

// How can I make content inside format!() print to the screen?
// You can't. format!() returns a String, which is then passed to println!() to print
// to the screen. If you want to print the output of format!() to the screen, you need
// to use println!().

// Tell me how can I do that!
// You can use the {} placeholder to print the output of format!() to the screen. For
// example, println!("Hello, {}!", format!("Rust")); will print Hello, Rust! to the
// screen.

// Fix the error in the code above.
// The error is in the format!() call. The placeholder {} is missing. The correct code
// is println!("Hello, {}!", format!("Rust"));

// But why does the error message say "expected a literal"?
// The error message is misleading. The problem is that the placeholder {} is missing
// from the format!() call. The compiler thinks you are trying to pass a literal to
// format!() instead of a placeholder.

// What's eprintln!()?
// eprintln!() is similar to println!() but prints to the standard error stream instead
// of the standard output stream. The standard error stream is normally used to print
// error messages to the screen. The standard output stream is normally used to print
// normal messages to the screen.

// I still didn't get it!
// The standard output stream is normally used to print normal messages to the screen.
// The standard error stream is normally used to print error messages to the screen.

// Why would anyone want to print error messages to the screen?
// Because it's easier to read error messages on the screen than in a log file.

