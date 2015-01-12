# Construct

The `vec!` macro is pretty great, but it does only create `Vec`s.  **Construct**
contains a macro that lets you construct containers from any type that implements
the `Construct` trait.  By default, all of the standard library containers are
included.

### Example

```rust
#[macro_use]
extern crate construct;

use std::collections::HashMap;

fn main() {
    // Vector construction
    let v = construct!(Vec<_>, 1,2,3,4);
    assert_eq!(v, vec![1,2,3,4]);


    // Hashmap construction
    let m = construct!(HashMap<_,_>, (1, "hi"), (2, "bye"));

    let mut manual = HashMap::new();
    manual.insert(1, "hi");
    manual.insert(2, "bye");
    assert_eq!(m, manual);
}

```
