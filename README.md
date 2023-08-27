# Level 1 Crook Calculator

A package that comes with a library and a binary crate. Library crate provides one function to compute mathematical equations from string slices. Binary crate provides command line interface for this function.

## Usage

### Binary crate

Build and run the binary, then enter you precious numbers like this:

```
3 * ( ( 2 + 1 - 1 ) ^ 2 ) / 2 - 3.5
```

**Remember about spaces.** Any ammount of spaces or any different whitespace characters to separate numbers and operators.

### Library crate

```rust
use crook_calculator;

fn main() {
    println!("{}", crook_calculator::compute("2 + 2").unwrap());
}
```