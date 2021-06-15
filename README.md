# chance_the_wrapper

For when a regular `Wrapper<T>(T)` gets too boring.

## Example

```rust
use chance_the_wrapper::ChanceTheWrapper;

// Creates a new wrapper with a 80% chance of succeeding
let w = ChanceTheWrapper::with_chance(0, 0.8);

match w.get() {
    Some(value) => println!("{}", value),
    None => println!(":("),
}
```
