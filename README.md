# `or-else-map`

Fix `Option::map_or_else` and `Result::map_or_else`, to make sense.

## Why?

- `Option::map_or_else` and `Result::map_or_else` were presumably written by
  yoda; the `map` and `or_else` are reversed from what they should be.
- "if you aint first you're last" (Bobby, et al., 2006)

## Example

```rust
use or_else_map::prelude::*;

fn option_eg() {
    let x = Some(2);
    let y = x.map_or_else(|| 0, |v| v * 2); // yoda
    let z = x.or_else_map(|| 0, |v| v * 2); // not-yoda
    assert_eq!(y, 4);
    assert_eq!(z, 4);

    let x: Option<i32> = None;
    let z = x.or_else_map(|| 0, |v| v * 2);
    assert_eq!(z, 0);
}

fn result_eg() {
    let x: Result<i32, &str> = Ok(2);
    let y = x.map_or_else(|e| 0, |v| v * 2); // yoda
    let z = x.or_else_map(|e| 0, |v| v * 2); // not-yoda
    assert_eq!(y, 4);
    assert_eq!(z, 4);

    let x: Result<i32, &str> = Err("oof");
    let z = x.or_else_map(|e| 0, |v| v * 2);
    assert_eq!(z, 0);
}

fn main() {
    option_eg();
    result_eg();
}
```

---

## Acknowledgements

- my fans and family
- the academy and the hollywood foreign press
- the rust developers
