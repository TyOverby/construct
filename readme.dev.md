# Construct

The `vec!` macro is pretty great, but it does only create `Vec`s.  The
`construct!` macro works for any type that implements `iter::Extend`, which
is basically every collection!

## Install

**Cargo.toml**

```
[dependencies]
construct = "*"
```

## Example

^code(./examples/demo.rs)
