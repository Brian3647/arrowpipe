#![forbid(missing_docs)]
#![forbid(clippy::all)]
#![forbid(unsafe_code)]

//! # ArrowPipe
//!
//! An [`Arrow`] is a function composition system that can be used to create
//! complex data processing pipelines.
//!
//! ## Example
//!
//! ```
//! use arrowpipe::Arrow;
//!
//! fn add_one(x: i32) -> i32 {
//!    x + 1
//! }
//!
//! fn double(x: i32) -> i32 {
//!   x * 2
//! }
//!
//! let mut arrow = Arrow::new(add_one);
//! arrow.symbiotize(Arrow::new(double));
//! arrow.symbiotize(Arrow::new(|x| x - 1));
//!
//! assert_eq!(arrow.shoot(1), 3);
//! ```
//! ## Multiple symbiotic [`Arrow`]s:
//!
//! ```
//! use arrowpipe::Arrow;
//!
//! fn add_one(x: i32) -> i32 {
//!    x + 1
//! }
//!
//! let mut first = Arrow::new(add_one); // Second: 2 -> 3
//! first.symbiotize(Arrow::new(|x| x * 2)); // Third: 3 -> 6
//!
//! let mut second = Arrow::new(add_one); // First: 1 -> 2
//! second.symbiotize(first); // 2 -> 6
//!
//! assert_eq!(second.shoot(1), 6);
//! ```

/// The essence of an [`Arrow`] is a function that takes a value of type `T` and
/// returns a value of type `U`. This is the core of the [`Arrow`].
pub type Essence<T, U> = fn(T) -> U;

/// The [`Arrow`] type. It is a function composition system that can be used to
/// create complex data processing pipelines. It is composed of an essence and
/// a list of symbiotic [`Arrow`]s.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Arrow<T, U = T> {
    /// The essence of the [`Arrow`]. It is a function that takes a value of
    /// type `T` and returns a value of type `U`.
    essence: Essence<T, U>,
    /// The list of symbiotic [`Arrow`]s. They are [`Arrow`]s that will be
    /// applied to the result of the essence.
    symbiosis: Vec<Arrow<U, U>>,
}

impl<T, U> Arrow<T, U> {
    /// Creates a new [`Arrow`] instance.
    pub fn new(essence: fn(T) -> U) -> Self {
        Arrow {
            essence,
            symbiosis: Vec::new(),
        }
    }

    /// Adds a symbiotic [`Arrow`]. It will be applied to the result of the
    /// essence function. Returns the ID of the symbiotic [`Arrow`], which can
    /// be used to remove it later.
    pub fn symbiotize(&mut self, other: Arrow<U, U>) -> usize {
        self.symbiosis.push(other);
        self.symbiosis.len() - 1
    }

    /// Shoots the [`Arrow`], applying the essence and all pipelines.
    pub fn shoot(&self, input: T) -> U {
        let mut result = (self.essence)(input);

        for symbiote in &self.symbiosis {
            result = symbiote.shoot(result);
        }

        result
    }

    /// Shoots the [`Arrow`] in reverse, applying the essence and all pipelines in reverse.
    pub fn shoot_reverse(&mut self, input: T) -> U {
        let mut result = (self.essence)(input);

        for symbiote in self.symbiosis.iter_mut().rev() {
            result = symbiote.shoot_reverse(result);
        }

        result
    }

    /// Clears the [`Arrow`], removing all symbiotic [`Arrow`]s.
    pub fn clear(&mut self) {
        self.symbiosis.clear();
    }

    /// Removes a symbiotic [`Arrow`] by its index in the array.
    pub fn remove(&mut self, idx: usize) {
        self.symbiosis.remove(idx);
    }

    /// Applies only the essence of the [`Arrow`].
    pub fn apply(&self, input: T) -> U {
        (self.essence)(input)
    }
}
