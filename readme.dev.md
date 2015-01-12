# Construct

The `vec!` macro is pretty great, but it does only create `Vec`s.  **Construct**
contains a macro that lets you construct containers from any type that implements
the `Construct` trait.  By default, all of the standard library containers are
included.

## Install

**Cargo.toml**

```
[dependencies]
construct = "*"
```

## Example

^code(./examples/demo.rs)
