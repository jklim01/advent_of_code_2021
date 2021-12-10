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


## Day 4:
1. mutate collections in place by chaining `iter_mut` and `for_each`
2. `map_err`
    - maps `Result<T, E>` to  `Result<T, F>` by:
        - leave `Ok` variant untouched
        - input closure: E -> F


## Day 5:
1. Operator Overloading
2. Apperantly, you can only create range iterators using `..` or `..=` if it is ascending. Otherwise, the iterator stops after the first element. :cry: This can be combated by testing if it is ascending and returning the correct version using `match`.
3. Bresenhams's Line Algorithm
    - general algorithm to create rasterized lines (the endpoints must have integer coordinates)
    - Assumptions:
        1. The magnitude of the slope is at most 1. Otherwise, flip the x and y axis to satisfy the condition, and flip the points that the algorithm returns to revert back to the original coordinates.
        2. The x coordinates is ascending from the start to the end. This can easily be satisfied by swapping the endpoints as neccesary.
    - Intuition:
        - Denote the exact value of the line's y component at the ith x coordinate by `yi`. This increases by `Dy/Dx` each time. The ith point chosen by the algorithm has the form `(xi, (yi)')`.
        - Since the slope magnitude is at most 1, at the ith x coordinate, we only need to determine whether `(yi)'` will increment w.r.t `(y(i-1))'`, or stay the same. This is determined by checking which point `yi` is closer to,  which boils down to the criteria `[yi - (y(i-1))'] >= 0.5`.
        - But we know that `[yi - (y(i-1))']` has the form `p/Dx`. We can also eaily derive how `p` changes at each step, giving us a way to track the desired ratio `p/Dx` only using integers. Finally, since `p/Dx >= 0.5` is equivalent to `2p-Dx >= 0`, it is possible to avoid floating-point arithmetic altogether!
    - Algorithm: let `D = 2p - Dx`
        1. Intially, `p = 0` and choose the starting point as the first point.
        2. Repeat for all remaining x coordinates:
            1. increment `p` by `Dy`, and set `(yi)' = (y(i-1))'`
            2. if `D >= 0`, decrement `p` by `Dx`, and `(yi)' += by Dy.signum()`
        - Even better, instead of keeping track of `p` to calculate `diff` at each step, just calculate the initial `D` and applying the relevant changes to `D` at each step
    - Example: (0, 0) -> (5, 2),
        - [y = 0, (0,0), D = -5], [y = 2/5, (1,0), D = -5+4 = -1], [y = 4/5, (2,1), D = (-1+4)-10 = -7], [y = 6/5, (3,1), D = -7+4 = -3], [y = 8/5, (4,2), D = (-3+4)-10 = -9], [y = 2, (5,2), D = -9+4 = -5]
4. `scan`
    - similar to `map`, but it is seeded with an initial state, where the input closure can also use and operate on the state on each iteration


## Day 5:
1. `append` appends another vector to the end of the current vector
2. `extend_from_slice` can do the same with a vector slice, while `extend` (nightly-only) can extend with elements of an iterator
3. If not needed to re`collect`, `into_iter` and `chain` is fast.
4. Alternatively, just `reserve` and `push`.