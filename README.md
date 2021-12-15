# advent_of_code_2021
My attempt of Advent of Code 2021. I'm also using this oppurtunity to pick up Rust.

Hope to get 50 stars by the 25th :P.



# Highlights
Basically a record of any cool or important things I learnt about Rust, and any algorithms or derivations that I found or discovered myself along the way.


## Table of Contents
  - [Day 1](#day-1)
  - [Day 3](#day-3)
  - [Day 4](#day-4)
  - [Day 5](#day-5)
  - [Day 6](#day-6)
  - [Day 7](#day-7)
  - [Day 8](#day-8)



## Day 1
1. Self-implemented Iterators



## Day 3
1. `ok_or_else`
    - maps `Option<T>` to `Result<T, E>` by:
        - Some(T) |-> Ok(T)
        - input closure: None |-> Err(E)
    - can easily return Err by chaining `ok_or_else` and `?` to an Option

2. `collect` into a result or option
    - when applying higher-order functions on iterators, if the `Err` or `None` variant is returned by the closure, it can be "collected" out
    -  convenient when you want to stop and immediately return from a function, but cannot return because the `Err` or `None` variant is found inside the closure (the process terminates once `Err` is found)



## Day 4
1. mutate collections in place by chaining `iter_mut` and `for_each`

2. `map_err`
    - maps `Result<T, E>` to  `Result<T, F>` by:
        - leave `Ok` variant untouched
        - input closure: E -> F



## Day 5
1. Operator Overloading

2. Apperantly, you can only create range iterators using `..` or `..=` if it is ascending. Otherwise, the iterator stops after the first element. :cry: This can be combated by testing if it is ascending and returning the correct version using `match`.

3. Bresenhams's Line Algorithm
    - general algorithm to create rasterized lines (the endpoints must have integer coordinates)

    - Assumptions:
        1. The magnitude of the slope is at most 1. Otherwise, flip the x and y axis to satisfy the condition, and flip the points that the algorithm returns to revert back to the original coordinates.
        2. The x coordinates is ascending from the start to the end. This can easily be satisfied by swapping the endpoints as neccesary.

    - Intuition:<br/>
        Denote the exact value of the line's y component at the ith x coordinate by `yi`. This increases by `Dy/Dx` each time. The ith point chosen by the algorithm has the form `(xi, (yi)')`.

        Since the slope magnitude is at most 1, at the ith x coordinate, we only need to determine whether `(yi)'` will increment w.r.t `(y(i-1))'`, or stay the same. This is determined by checking which point `yi` is closer to,  which boils down to the criteria `[yi - (y(i-1))'] ≥ 0.5`.

        But we know that `[yi - (y(i-1))']` has the form `p/Dx`. We can also eaily derive how `p` changes at each step, giving us a way to track the desired ratio `p/Dx` only using integers. Finally, since `p/Dx ≥ 0.5` is equivalent to `2p-Dx ≥ 0`, it is possible to avoid floating-point arithmetic altogether!

    - Algorithm: let `D = 2p - Dx`
        1. Intially, let `p = 0` and choose the starting point as the first point.
        2. Repeat for all remaining x coordinates:
            1. increment `p` by `Dy.signum()`, and set `(yi)' = (y(i-1))'`
            2. if `D ≥ 0`, decrement `p` by `Dx`, and `(yi)' += by Dy.signum()`

        Even better, instead of keeping track of `p` to calculate `D` at each step, just calculate the initial `D` and apply the relevant changes to `D` at each step!

    - Example: (0, 0) -> (5, 2),
        - `[y=0, (0,0), D=-5] -> [y=2/5, (1,0), D=-5+4=-1] -> [y=4/5, (2,1), D=(-1+4)-10=-7] -> [y=6/5, (3,1), D=-7+4=-3] -> [y=8/5, (4,2), D=(-3+4)-10=-9] -> [y=2, (5,2), D=-9+4=-5]`

4. `scan`
    - similar to `map`, but it allows the use of an internal state
    - the return value of the closure must be an Option, which is yielded by the resulting iterator
    - it's nice to be able to keep the state internal, but the use-case just seems too niche



## Day 6
1. `append` appends another vector to the end of the current vector

2. `extend_from_slice` can do the same with a vector slice, while `extend` (currently nightly-only) can extend with elements of an iterator

3. If not needed to re`collect`, `into_iter` and `chain` is fast.

4. Alternatively, just `reserve` and `push`.



## Day 7
1. Part 1: Minimize `f(x) = Σ |x - x`<sub>i</sub>`|`
    - By the linearity of the derivative and observing the graph of the absolute value function, it can be thought that each crab contributes a value of `-1` and `1` to the slope of f to its left and right. Thus, `f` is continuous (sum of continuous functions) and linear between the kinks at points where there are crabs.

    - This tells us that `f` always increases as we move away from the "minimum zone", which occurs where the slope changes sign or is zero. Where the latter case exists, the minimum zone is a closed interval (which looks like `\_/`. Otherwise it is just a kink where the slope directly changes from positive to negative ( which looks like `\/`). It can be seen that this zone always contains the median.

    - tldr: Use a BTreeMap to store the the number of crabs at each position. Iterating from the smallest to largest position, given that `P` crabs have been seen (including the current position), the slope of f to the right of the current position is `2P-n`. The minimum zone starts from the first non-negative value found and ends at the first positive value found.

2. Part 2: Minimize `f(x) = Σ(x - x`<sub>i</sub>`)`<sup>2</sup>` + Σ|x - x`<sub>i</sub>`|`
    - Using  `x`<sup>2</sup>` ≫ x` approximation, the solution will be the mean. However, I don't like to live that dangerously :zany_face:, so let's have a closer look.

    - Lucky for us, the first term is smooth (quadratic), while the second term is the same as in part 1. Again, since we are restricted to integer points and `f` is continuous, we will start by searching for points where the slope changes sign (for convenience, let's say sign is `+ve`, `0` or `-ve`).

    - Using calculus and knowledge from part 1, we can show the following:
        > Denote `S = Σ x`<sub>i</sub>. Given a point `x` with `K(x)` crabs, and `L(x)` crabs to its left,  <br/>
        > `f'(x)`<sup>-</sup>` = 2nx - 2S + 2L(x)-n` <br/>
        > `f'(x)`<sup>+</sup>` = 2nx - 2S + 2[L(x)+K(x)]-n` <br/>

    - Thus, the interval of interest is `I = [ supremum(X`<sub>1</sub>`),  infimum(X`<sub>2</sub>`) ]` (and yes I had to search for these 2 words), where (`M` is the mean position):
        1. `X`<sub>1</sub>` = Region of -ve slope = { x | x < M + 0.5 - L(x)/n }`
        2. `X`<sub>2</sub>` = Region of +ve slope = { x | x > M + 0.5 - [L(x)+K(x)]/n }`

        By the continuity of `f` and the fact that its slope is an increasing function outside `I` (ignoring points where it is undefined), it is guaranteed that `f` increases as we move away from the edges of `I`.

        Furthermore, We know that for any point `x`,  `0 ≤ L(x) ≤ L(x)+K(x) ≤ n`, which tells us that the `I` is contained within `[ M - 0.5 , M + 0.5 ]`. Thus, the only 3 possible cases and their handling methods are:
        1. `I` contains 0 integer points: only check the two integer points closest to each end of the interval
        2. `I` contains 1 integer point: this is the only integer argmin
        3. `I` contains 2 integer points: both are the only integer argmin

        Finally, it is not hard to convince yourself that checking `⌈ M ⌉`, `⌊ M ⌋` and the next closest integer to `M` is sufficient to cover all cases, and that I have wasted a whole day thinking about this :rofl:.

    - tldr: check the positions `M.round() - 1`, `M.round()`, `M.round() + 1`

3. Use `entry(key).or_insert(default_val)` to either get ref to the existing value at the key, or insert the key paired with the specified default value and get its value ref.
   - convenient when when using hashmap to count key occurences, example:
    > `map.entry(some_key).or_insert(0) += 1`



## Day 8
1. Multiple char delimiters for `split` by providing the an array slice containing the char delimiters as the argument.
2. `?` can actually be used on Options????!!!!!! What have I been missing :cry:. Clippy is a life-saver for beginners trying to learn to write more idiomatic Rust!
3. When iterating over Results, other than `collect`ing (mentioned in Day 3), 2 other ways are:
   1. ignore failed items with `filter_map`
   2. collect all valid values and failures separately using `partition`
4. Results can also be converted to Options using `or`.



## Day 9:
1. Idea: use a stack to find all points in a basin.
2. First time I managed to accept and return iterators from a function!
3. `product`, `sort`, `unstable_sort`
4.  - `then`: `false` |-> `None`, `true` |-> `Some(f())`
        - `and_then`: `None` |-> `None`, `Some(T)` |-> `f(T)` (`f` returns `Option`)
        - `and_then`: `Err(e)` |-> `Err(e)`, `Ok(T)` |-> `f(T)` (`f` returns `Result`)
        - similar to `map`, but avoids double layer if closure also returns `Option` or `Result`
5. Tried benchmarking using Criterion.rs and `time {executable}` terminal command.



## Day 10:
1. Wasted **WAY** too much time playing around with traits.
    - soting requires `Ord`, which requires `PartialOrd` and  `Eq`
    - `Eq` requires `PartialEq`, but doesn't need any new methods, so we can just add `#[derive(Eq)]` once `PartialEq` is implemented to inform that it is a (non-partial) equivalence relation
    - arithmetic operators can be overloaded to operate on other types
        - example: `impl std::ops::Mul<T> for Vector3<T>` defines scalar-vector multiplication (`c * [x, y, z]`, where `x, y, z, c: T`)
2. `collect` collects the iterator elements using the `from_iter` method in the `FromIterator` trait. Thus, we can collect into our custom collections in our desired way by implementing the trait.