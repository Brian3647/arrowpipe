# ArrowPipe: Build complex pipelines easily

An [`Arrow`] is a function composition system that can be used to create
complex data processing pipelines.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
arrowpipe = { git = "https://github.com/Brian3647/arrowpipe" }
```

## Example

```rust
use arrowpipe::Arrow;

fn add_one(x: i32) -> i32 {
   x + 1
}

fn double(x: i32) -> i32 {
  x * 2
}

let mut arrow = Arrow::new(add_one);
arrow.symbiotize(Arrow::new(double));
arrow.symbiotize(Arrow::new(|x| x - 1));

assert_eq!(arrow.shoot(1), 3);
```

## Order of execution example:

```rust
use arrowpipe::Arrow;

fn add_one(x: i32) -> i32 {
   x + 1
}

let mut first = Arrow::new(add_one); // Second: 2 -> 3
first.symbiotize(Arrow::new(|x| x * 2)); // Third: 3 -> 6

let mut second = Arrow::new(add_one); // First: 1 -> 2
second.symbiotize(first);

assert_eq!(second.shoot(1), 6);
```

## Why?

This is specially useful for long pipelines like a build system. With the `Arrow` struct, you can simply add another `Arrow` to an existing one to have a new step.

## License

This project is licensed under the MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).
