# `or-else-map`

Fix `Option::map_or_else` and `Result::map_or_else`, to make sense.

## Why?

- `Option::map_or_else` and `Result::map_or_else` were presumably written by
  yoda; the `map` and `or_else` are reversed from what they should be.
- "if you aint first you're last" (Bobby, et al., 2006)

## Example

```rust
use or_else_map::prelude::*;

fn main() {
    let x = Some(2);
    let y = x.map_or_else(|| 0, |v| v * 2);
    let y = x.or_else_map(|| 0, |v| v * 2);
    assert_eq!(y, 4);

    let x: Option<i32> = None;
    let y = x.or_else_map(|| 0, |v| v * 2);
    assert_eq!(y, 0);
}
```
