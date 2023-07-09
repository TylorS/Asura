# Asura

Asura is a high-level programming language designed for composability, with the help of Algebraic Effects, 
extensibility through compile-time macros and operator overloading, type-safety with support for higher-rank polymorphism, and with support for compiling to multiple targets, such as WebAssembly or TypeScript.

## High-level Design

Code would be lexed + parsed into an AST, the AST would be converted into a ByteCode. A VM would be able to execute the ByteCode to determine type assignability and produce additional type-directed information to the compiler for output in whatever format desired (WASM, Javascript, etc) or utilized directly in a REPL-style execution.