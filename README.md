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

    - Intuition:<br/>
        Denote the exact value of the line's y component at the ith x coordinate by `yi`. This increases by `Dy/Dx` each time. The ith point chosen by the algorithm has the form `(xi, (yi)')`.

        Since the slope magnitude is at most 1, at the ith x coordinate, we only need to determine whether `(yi)'` will increment w.r.t `(y(i-1))'`, or stay the same. This is determined by checking which point `yi` is closer to,  which boils down to the criteria `[yi - (y(i-1))'] ≥ 0.5`.

        But we know that `[yi - (y(i-1))']` has the form `p/Dx`. We can also eaily derive how `p` changes at each step, giving us a way to track the desired ratio `p/Dx` only using integers. Finally, since `p/Dx ≥ 0.5` is equivalent to `2p-Dx ≥ 0`, it is possible to avoid floating-point arithmetic altogether!

    - Algorithm: let `D = 2p - Dx`
        1. Intially, let `p = 0` and choose the starting point as the first point.
        2. Repeat for all remaining x coordinates:
            1. increment `p` by `Dy.signum()`, and set `(yi)' = (y(i-1))'`
            2. if `D ≥ 0`, decrement `p` by `Dx`, and `(yi)' += by Dy.signum()`

        Even better, instead of keeping track of `p` to calculate `D` at each step, just calculate the initial `D` and applying the relevant changes to `D` at each step

    - Example: (0, 0) -> (5, 2),
        - `[y=0, (0,0), D=-5] -> [y=2/5, (1,0), D=-5+4=-1] -> [y=4/5, (2,1), D=(-1+4)-10=-7] -> [y=6/5, (3,1), D=-7+4=-3] -> [y=8/5, (4,2), D=(-3+4)-10=-9] -> [y=2, (5,2), D=-9+4=-5]`

4. `scan`
    - similar to `map`, but it is seeded with an initial state, where the input closure can also use and operate on the state on each iteration



## Day 5:
1. `append` appends another vector to the end of the current vector

2. `extend_from_slice` can do the same with a vector slice, while `extend` (currently nightly-only) can extend with elements of an iterator

3. If not needed to re`collect`, `into_iter` and `chain` is fast.

4. Alternatively, just `reserve` and `push`.



## Day 7:
1. Part 1: Minimize `f(x) = Σ |x - x`<sub>i</sub>`|`
    - By the linearity of the derivative and observing the graph of the absolute value function, it can be thought that each crab contributes a value of `-1` and `1` to the slope of f to its left and right. `f` is continuous (by the sum of continuous functions) and linear between the kinks at points where there are crabs.

    - This trend of the slope tells us that `f` always increases as we move away from the "minimum zone", which occurs where the slope changes sign or is zero (same number of crabs to the left and right). This zone always contains the median, and where the latter case exists, can even be a closed interval (which looks like `\_/`, otherwise it is just a kink where the slope directly changes from positive to negative, which looks like `\/`).

    - tldr: Use a hashmap to store the the number of crabs at each position. Iterating from the smallest to largest position, the minimum zone starts and ends where the sum of the values at the keys that have been visited changes sign (for convenience, let's say the sign could be one of `{+ve, 0, -ve}`).

2. Part 2: Minimize `f(x) = Σ(x - x`<sub>i</sub>`)`<sup>2</sup>` + Σ|x - x`<sub>i</sub>`|`
    - Using  `y`<sup>2</sup>` ≫ y` approximation, the solution will be the mean. However, I don't like to live that dangerously :zany_face:, so let's have a closer look.

    - Lucky for us, the first term is smooth (quadratic), while the second term is the same as in part 1. The expression might look too complex to derive an efficient algorithm, but as we will soon see, the quadratic term actually makes things even nicer! Again, since we are restricted to integer points, we will search within an interval bounded by points where the slope changes sign.

    - Using calculus and knowledge from part 1, we can show the following:
        > Denote `S = Σ x`<sub>i</sub>, and `M` as the mean position. Given a point `x` with `K` crabs, and `L` crabs to its left,  <br/>
        > `f'(x)`<sup>-</sup>` = 2nx - 2S + 2L-n` <br/>
        > `f'(x)`<sup>+</sup>` = 2nx - 2S + 2(L+K)-n` <br/>

    - Thus, the interval of interest is `[ max(x`<sub>1</sub>`),  min(x`<sub>2</sub>`) ]`, where:
        1. `x`<sub>1</sub>` ∈ Region of -ve slope = { x | x < M + 0.5 - L`<sub>1</sub>`/n }`
        2. `x`<sub>2</sub>` ∈ Region of +ve slope = { x | x > M + 0.5 - (L`<sub>2</sub>`+K`<sub>2</sub>`)/n }`

        But we know that for any point,  `0 ≤ L ≤ (L+K) ≤ n`. Furthermore, by the continuity of `f` and the fact that its slope is a monotonically increasing function outside the interval, we are guaranteed that all points outside the interval will be larger than any point inside it. This leaves us with all integer points within `[ ⌈ M - 0.5 ⌉,  ⌊ M + 0.5 ⌋ ]`. Finally, it is not hard to convince yourself that actually, `⌈ M ⌉` and `⌊ M ⌋` are the only integers that could be within this interval, and that I have wasted a whole day thinking about this :rofl:.

        One very interesting thing to note, is that contrary to how complex `f` looks, the solution is very simple. This is due to the presence of the `x` term in the slope formula contributed by the quadratic, which allows us to apply the bounds and remove `L` and `K`, which are point dependent (meaning that we will have to iterate through the points to find the ones that satisfies our criteria), from the equation (HAHA).

    - tldr: the minimum ALWAYS occurs at `round(mean({x`<sub>i</sub>`}))`
