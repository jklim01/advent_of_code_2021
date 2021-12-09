# advent_of_code_2021
My attempt of Advent of Code 2021. I'm also using this oppurtunity to pick up Rust.

Hope to get 50 stars by the 25th :P.

# Highlights
## Day 1:
1. Self-implemented Iterators

## Day 3:
1. `ok_or_else`
    - maps `Option<T>` to `Result<T, E>` by:
        - Some(T) |-> Ok(T)
        - input closure: None |-> Err(E)
    - can easily return Err by chaining `ok_or_else` and `?` to an Option
2. `collect` into a result or option
    - when applying higher-order functions on iterators, if the `Err` or `None` variant is returned by the closure, it can be "collected" out
    -  convenient when you want to have an early return from a function, but cannot return because the `Err` or `None` is found inside the closure

# Day 4:
1. mutate collections in place by chaining `iter_mut` and `for_each`
2. `map_err`
    - maps `Result<T, E>` to  `Result<T, F>` by:
        - leave `Ok` variant untouched
        - input closure: E -> F


## Day 5:
1. Operator Overloading
2. Bresenhams's Line Algorithm
    - the algorithm's intuition is commented within the code
    - general algorithm to create rasterized lines