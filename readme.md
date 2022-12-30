# BasedScript

A WIP Toy Language written in Rust.

### Install Instructions

To use, clone the repo and install the [Rust Compiler](https://www.rust-lang.org/tools/install)\
Next, compile the interpreter with `rustc main.rs` (make sure you have navigated to the correct directory)\
After that, run `./main ./examples/main.based` and watch the magic happen! Feel free to change things around and see what works.

#### Currently Working

- variable declarations (strings and numbers only)
- printing values (strings, numbers, and variables only)
- freeing variables from the stack with the free(var) function
- addition (set values to expressions (2 addends only) and print expressions (unlimited addends))
- valueless var declarations (defaults to 'undefined')
- simple syntax errors through Rust's panic! macro

#### Planned Features

- more data types
- more functions
- basic arithmetic (began implementing)

**If you like what you see, consider starring the repo and following me <3**
