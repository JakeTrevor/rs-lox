# Lox in Rust

This is my implementation of the Lox language from Robert Nystrom's [Crafting Interpreters book](https://craftinginterpreters.com/contents.html).

## Extensions
Here is a list of extensions I have implemented:

###  Lexing:
- Multiline/inline comments
- Escape characters in strings
- Some pretty nice error reporting (with [ariadne](https://docs.rs/ariadne/latest/ariadne/index.html))