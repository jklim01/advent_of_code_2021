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
  - [Day 9](#day-9)
  - [Day 10](#day-10)
  - [Day 11](#day-11)
  - [Day 12](#day-12)
  - [Day 13](#day-13)
  - [Day 14](#day-14)
  - [Day 15](#day-15)
  - [Day 16](#day-16)
  - [Day 17](#day-17)
  - [Day 18](#day-18)

---

## Day 1
1. Self-implemented Iterators

---

## Day 3
1. `ok_or_else`
    - maps `Option<T>` to `Result<T, E>` by:
        - Some(T) |-> Ok(T)
        - input closure: None |-> Err(E)
    - can easily return Err by chaining `ok_or_else` and `?` to an Option

2. `collect` into a result or option
    - when applying higher-order functions on iterators, if the `Err` or `None` variant is returned by the closure, it can be "collected" out
    -  convenient when you want to stop and immediately return from a function, but cannot return because the `Err` or `None` variant is found inside the closure (the process terminates once `Err` is found)

---

## Day 4
1. mutate collections in place by chaining `iter_mut` and `for_each`

2. `map_err`
    - maps `Result<T, E>` to  `Result<T, F>` by:
        - leave `Ok` variant untouched
        - input closure: `E -> F`

---

## Day 5
1. Operator Overloading

2. Apperantly, you can only create range iterators using `..` or `..=` if it is ascending. Otherwise, the iterator stops after the first element. :cry: This can be combated by testing if it is ascending and returning the correct version using `match`.

3. Bresenhams's Line Algorithm
    - general algorithm to create rasterized lines (the endpoints must have integer coordinates)

    - Assumptions:
        1. The magnitude of the slope is at most 1. Otherwise, flip the x and y axis to satisfy the condition, and flip the points that the algorithm returns to revert back to the original coordinates.
        2. The x coordinates is ascending from the start to the end. This can easily be satisfied by swapping the endpoints as neccesary.

    - Intuition:<br/>
        Denote the exact value of the line's y component at the ith x coordinate by `y`<sub>`i`</sub>. This increases by `Dy/Dx` each time. Denonte the ith point chosen by the algorithm `(x`<sub>`i`</sub>`, y`<sub>`i`</sub><sup>`'`</sup>`)`.

        Since the slope magnitude is at most 1, at the ith x coordinate, we only need to determine whether `y`<sub>`i`</sub><sup>`'`</sup> will increment w.r.t `y`<sub>`i-1`</sub><sup>`'`</sup>, or stay the same. This is determined by checking which point `yi` is closer to, which boils down to the criteria `(y`<sub>`i`</sub>` - y`<sub>`i-1`</sub><sup>`'`</sup>`) ≥ 0.5`.

        But we know that `(y`<sub>`i`</sub>` - y`<sub>`i-1`</sub><sup>`'`</sup>`)` has the form `p/Dx`, where the changes to `p` at each step can be easily derived, giving us a way to track the desired ratio `p/Dx` only using integers. Finally, since `p/Dx ≥ 0.5` is equivalent to `2p-Dx ≥ 0`, it is possible to avoid floating-point arithmetic altogether!

    - Algorithm: let `D = 2p - Dx`
        1. Intially, let `p = 0` and choose the starting point as the first point.
        2. Repeat for all remaining x coordinates:
            1. increment `p` by `Dy`, and set `y`<sub>`i`</sub><sup>`'`</sup>` = y`<sub>`i-1`</sub><sup>`'`</sup>
            2. if `D ≥ 0`, decrement `p` by `Dx`, and `y`<sub>`i`</sub><sup>`'`</sup>` += Dy.signum()`

        Even better, instead of keeping track of `p` to calculate `D` at each step, just calculate the initial `D` and apply the relevant changes to `D` at each step!

    - Example: (0, 0) -> (5, 2),
        - `[y=0, (0,0), D=-5] -> [y=2/5, (1,0), D=-5+4=-1] -> [y=4/5, (2,1), D=(-1+4)-10=-7] -> [y=6/5, (3,1), D=-7+4=-3] -> [y=8/5, (4,2), D=(-3+4)-10=-9] -> [y=2, (5,2), D=-9+4=-5]`

4. `scan`
    - similar to `map`, but it allows the use of an internal state
    - the return value of the closure must be an Option, which is yielded by the resulting iterator
    - it's nice to be able to keep the state internal, but it doesn't seem to be flexible enough to fit many use-case

---

## Day 6
1. `append` appends another vector to the end of the current vector

2. `extend_from_slice` can do the same with a vector slice, while `extend` (currently nightly-only) can extend with elements of an iterator

3. If not needed to re`collect`, `into_iter` and `chain` is fast.

4. Alternatively, just `reserve` and `push`.

---

## Day 7
1. Part 1: Minimize `f(x) = Σ |x - x`<sub>i</sub>`|, x ∈ ℤ`
    - To simplify analysis we will extend the function domain to the reals. By the linearity of the derivative and observing the graph of the absolute value function, it can be thought that each crab contributes a value of `-1` and `1` to the slope of f to its left and right. Thus, `f` is continuous (sum of continuous functions) and linear between the kinks at points where there are crabs.

    - This tells us that `f` always increases as we move away from the "minimum zone", which occurs where the slope changes sign or is zero. Where the latter case exists, the minimum zone is a closed interval (which looks like `\_/`). Otherwise it is just a kink where the slope directly changes from positive to negative ( which looks like `\/`). It can be seen that this zone always contains the median (think of the median position as the point where the number of crabs to the left and right is most balanced and use the intuiton above of how each crab contributes to the slope of f to its left and right).

    - tldr: Use a BTreeMap to store the the number of crabs at each position. Iterating from the smallest to largest position, given that `P` crabs have been seen (including the current position), the slope of f to the right of the current position is `2P-n`. The minimum zone starts from the first non-negative value found and ends at the first positive value found.

2. Part 2: Minimize `f(x) = Σ(x - x`<sub>i</sub>`)`<sup>2</sup>` + Σ|x - x`<sub>i</sub>`|, x ∈ ℤ`
    - Using  `x`<sup>2</sup>` ≫ x` approximation, the solution will be the mean. However, I don't like to live that dangerously :zany_face:, so let's have a closer look.

    - Lucky for us, the first term is smooth (quadratic), while the second term is the same as in part 1. Again, since we are restricted to integer points and `f` (with its domain extended to the reals) is continuous, we will start by searching for points where the slope changes sign (for convenience, let's say sign is `+ve`, `0` or `-ve`).

    - Using calculus and knowledge from part 1, we can show the following:
        > Denote `S = Σ x`<sub>i</sub>. Given a point `x` with `K(x)` crabs, and `L(x)` crabs to its left,  <br/>
        > `f'(x)`<sup>-</sup>` = 2nx - 2S + 2L(x)-n` <br/>
        > `f'(x)`<sup>+</sup>` = 2nx - 2S + 2[L(x)+K(x)]-n` <br/>

    - Thus, the interval of interest (ie the interval with minimum `f` value) is `I = [ supremum(X`<sub>1</sub>`),  infimum(X`<sub>2</sub>`) ]` (and yes I had to search for these 2 words), where (`M` is the mean position):
        1. `X`<sub>1</sub>` = Region of -ve slope = { x | x < M + 0.5 - L(x)/n }`
        2. `X`<sub>2</sub>` = Region of +ve slope = { x | x > M + 0.5 - [L(x)+K(x)]/n }`

        By the continuity of `f` and the fact that its slope is an increasing function outside `I` (ignoring points where it is undefined), it is guaranteed that `f` increases as we move away from the edges of `I`.

        Furthermore, We know that for any point `x`,  `0 ≤ L(x) ≤ L(x)+K(x) ≤ n`, which tells us that the `I` is contained within `[ M - 0.5 , M + 0.5 ]`. Thus, the only 3 possible cases and their handling methods are:
        1. `I` contains 0 integer points: only check the two integer points closest to each end of the interval
        2. `I` contains 1 integer point: this is the only integer argmin
        3. `I` contains 2 integer points: both are the only integer argmin

        Finally, it is not hard to convince yourself that checking `⌈ M ⌉`, `⌊ M ⌋` and the next closest integer to `M` is sufficient to cover all cases, and that I have wasted a whole day thinking about this :rofl:.

    - tldr: check the positions `M.round() - 1`, `M.round()`, `M.round() + 1`

    - After thinking about this problem more, I think it is possible to use the fact that the former quadratic term quickly overwhelms the the latter term to derive that `M.round()` is straightup the answer. By playing around with the expressions I suspect that this is highly possible, but I can't seem to pin it down in a formal way, so I'll just leave it like this for now :rofl:.

3. Use `entry(key).or_insert(default_val)` to either get ref to the existing value at the key, or insert the key paired with the specified default value and get its value ref.
   - convenient when when using hashmap to count key occurences, example:
    > `map.entry(some_key).or_insert(0) += 1`

---

## Day 8
1. Multiple char delimiters for `split` by providing the an array slice containing the char delimiters as the argument.

2. `?` can actually be used on Options????!!!!!! What have I been missing :cry:. Clippy is a life-saver for beginners trying to learn to write more idiomatic Rust!

3. When iterating over Results, other than `collect`ing (mentioned in Day 3), 2 other ways are:
   1. ignore failed items with `filter_map`
   2. collect all valid values and failures separately using `partition`

4. Results can also be converted to Options using `or`.

5. Looking back now, using a hashset is probably more appropriate.
    1. duplicate segments make no sense
    2. `contains_digit` can be replaced with `is_superset`
    3. `is_same_digit` can be replaced with `==`

---

## Day 9
1. Idea: use a stack to find all points in a basin.

2. First time I managed to accept and return iterators from a function!

3. `product`, `sort`, `unstable_sort`

4.  - `then`: `false` |-> `None`, `true` |-> `Some(f())`
        - `and_then`: `None` |-> `None`, `Some(T)` |-> `f(T)` (`f` returns `Option`)
        - `and_then`: `Err(e)` |-> `Err(e)`, `Ok(T)` |-> `f(T)` (`f` returns `Result`)
        - similar to `map`, but avoids double layer if closure also returns `Option` or `Result`

5. Tried benchmarking using Criterion.rs and `time {executable}` terminal command.

---

## Day 10
1. Wasted **WAY** too much time playing around with traits.
    - soting requires `Ord`, which requires `PartialOrd` and  `Eq`
    - `Eq` requires `PartialEq`, but doesn't need any new methods, so we can just add `#[derive(Eq)]` once `PartialEq` is implemented to inform that it is a (non-partial) equivalence relation
    - arithmetic operators can be overloaded to operate on other types
        - example: `impl std::ops::Mul<T> for Vector3<T>` defines scalar-vector multiplication (`c * [x, y, z]`, where `x, y, z, c: T`)

2. `collect` collects the iterator elements using the `from_iter` method in the `FromIterator` trait. Thus, we can collect into our custom collections in our desired way by implementing the trait.

---

## Day 11
1. `std::num::Wrapping<T>` can be used as a wrapper to perform intentionally-wrapped arithmetic on `T`.
    - Used it to get the coordinates of the 8 neighbours of a `Point` in `get_neighbours`. This way, we can simply calculate the coordinates and avoid checking for many cases in the function. Although it doesn't feel very nice to have such a loose thread, the calleer should check whether the coordinates is within the bounds anyway when indexing, so I supoose it's ok...

2. `core::ops::{Index, IndexMut}` can be used to define a custom index into a collection.
    - example: define `type Point = (usize, usize)` to index into `[[T]; SIZE]`

3. Implemented `std::iter::FromIter` to `collect` into an array (`[[u8; SIZE]; SIZE]`).

4. Implemented `std::str::FromStr` to `parse` into `OctopusGrid`.

---

## Day 12
1. I'm pretty proud of the error handling of cave-parsing functionality. This is the first time I have defined a somewhat decent and complete Error myself. XD

    - `ParseCaveError` returns different errors wrapping different data depending on the problem found.
        - example: If the problem is with a particular node, `ParseCaveError` wraps the `ParseNodeError` together with the line number. I think it's pretty good first attempt of combining errors and gathering context.

    - The errors I chose to go with also feel pretty natural, resulting in the handling of the errors at the callee end also being quite elegant in my beginner's opionion (check `day12_main` and `CaveMap`'s `from_str`).

    - I also tested out some cool stuff like:
        - `impl From<(usize, ParseNodeError)> for ParseCaveError` (because `ParseCaveError` can wrap `ParseNodeError`)
        - `impl Error for ParseCaveError` with `source` function (in the case of `ParseCaveError::NodeError`, show the cause of the `ParseNodeError`)

2. When doing graph traversal, we can push copies of the current traversed paths each appended with a different child node onto the data structure. This allows us to know the traversed path up to the visited node. (seems to have a huge memory requirement though :thinking:)

3. Not sure how this compares with the method above in terms of performance, but I thought of a way that allows us to have the "context" of the current traversed path without having to store a copy of the context together with each node. This can be done by exploiting the pattern of how the context changes between successive pops. A "divider" is introduced to remind us when to update the context.

    In the following example, we will take the traversed path to be the "context" and use a stack.

    We only need 1 vector to store the traversed path. The stack will store 2 different items: one that tells us to visit a node (`NextNode`), and another (what we'll call a divider) that tells us to remove a node from the path (`StackDiv`). Upon popping
    - `NextNode`: If end is found, handle it and `continue`. Otherwise, we will add the current node to the path vector, and push on a `StackDiv` before pushing its child nodes.
    - `StackDiv` : This signifies that all child nodes of the last node currently in the path vector have been traversed, and we will be exploring its sibling nodes next. Thus, we need to update the context by removing the last node from the path vector.

    Note that in recursion, the context would be implicitly updated when the function returns, where the stack frame is popped and we change to using the context in the following stack frame. The `StackDiv` method explicitly reminds us to update the context which is shared (uses the same variable) throughout the entire process. To be able to make the appropraite changes upon encountering this "reminder", there needs to be some pattern between the two contexts that `StackDiv` separates so that we know what to do. In the day 12 code, I used `StackDiv(Node)`, that when encountered, tells me that I can remove the record of visiting `Node` from `visited_small_caves` (the context).

    To be honest though, I'm not sure how nicely this works with a queue. I think that in most situations, the "context" will be highly dependent on the path, which could change a lot between successive de-queues. Whereas when traversing using stacks, each `StackDiv` signifies only a one-node-change in the path. In such cases, the only method I've come up with is to have the dividers separate non-sibling nodes and to store the context to switch in the divider or a separate queue. However, this seems to be equivalent to the original situation, but with the graph depth reduced by 1.

---

## Day 13
1. Hashset provides many set-theoretic operations!
    - union, intersection, difference, symmetric differnce (in one but not both)
    - `is_disjoint`, `is_subset`, `is_superset`
    - `==` overloaded to test for set equality (exact same set of elements)

2. In string literals, we can escape the newline in strings with `\`. Placing `r` before any string literal can make it a raw string literal too.

3. I think this is another nice example of defining, combining and handling errors.

---

## Day 14
1. If `std::str::FromStr` cannot be implemented due to the need of lifetime annotations, `std::convert::TryFrom` or `std::convert::From` can be used instead.

---

## Day 15
1. Notation. Graph: `G(V, E)`, start and goal vertices: `s` and `t`, cost estimate (minimum cost found so far) of a vertex from `s` and `t`: `df` and `db`, true minimum cost between vertices: `δ`, previous vertex on the best found path to a vertex: `p`, edge weight: `w`

    The notation is quite messy as I tried to include the ideas from many sources. Since I was mainly focused on creating a complete set of notes for convenient reference, most of the following will be almost an exact copy from the sources. I don't take credit for their work.

    The following list of sources is **not** complete:
    1. http://ai.stanford.edu/~nilsson/OnlinePubs-Nils/PublishedPapers/astar.pdf (good read, most complete explanation on A* I've read so far, it even proves it for multiple goal vertices, the pdf is included in the root)
    2. Bi-Directional and Heuristic Search in Path Problems (Ira Pohl)
    3. Bidirectional Search Reconsidered, Kaindl and Kanz
    4. https://www.homepages.ucl.ac.uk/~ucahmto/math/2020/05/30/bidirectional-dijkstra.html

2. Dijkstra's Algorithm: finds the minimum cost path by finding the minimum cost of each vertex from `s` in ascending order, stopping when `t` is found
    1. Create a priority queue `Q` where higher priority is given to vertices with shorter approximated distance `d` from `s`, and a hashmap `S` where the key is a vertex and the value is its `p` value.

    2. Start by pushing all vertices into `Q` s.t. `d(x) = ∞, ∀ x ∈ Q`. Set `d(s) = 0` and repeat while `Q ≠ Ø`:
        1. Pop `Q` to get some `v` with `(d(v), p(v))`. If `v = t`, trace backwards using `S` and return the path.
        2. Add `v` with value `p(v)` to `S`.
        3. For all `u ∈ v.neighbours`:
            - If `u ∈ S`, skip to the next vertex.
            - Calculate the cost `C` to get to `u` through `v`. If `C < d(u)`, update `d(u)` to `C` and its `p(u)` to `v`. This is called relaxing the edge `(v, u)`.

    3. If we don't return by the time `Q = Ø`, then no path exists between `s` and `t`.

    - Every vertex `x` can be in one the following states: unreached (`d(x) = ∞`), labeled (`x ∈ Q`), scanned (`x ∈ S`)

    - If the priority queue uses a data structure that doesn't allow querying or updating specific elements, we can externally store the current `d` value of each vertex, and push the same vertex but with an updated priority when we want to update its priority in `Q`. When a vertex that has been popped before is popped again, we will just ignore it.

    - A more practical implementation is to only have vertices with non-infinite priority in `Q`.

    - At any stage, for any `v ∈ Q`, `δ(s, v) > δ(s, u)` is guaranteed `∀ u ∈ S`. Proof:
        1. Let `(r, v)` be the first relaxed edge s.t. `d(v) < d(u)` for some `u ∈ S`, where `d(r) = λ` at the time.
        2. Then `d(r) < d(v) < d(u)`, and `d(r)` must have been set to `λ` before `u` entered `S`, otherwise the firts relaxation defined above is contradicted. So the order is
            - `d(r)` set to `λ`
            - `u` entered `S`
            - `(r, v)` relaxed as `r` enters `S`
        3. But the second step is impossible, since `d(r) < d(u)` and `r` must enter `S` before `u`

    - Corollary: When a vertex `v` is scanned, the current `d(v)` is guaranteed to be the minimum cost from `s` to `v`, where the previous vertex on the path is the `src` vertex that is stored together.

3. Bidirectional Dijkstra: attempts to reduce the search "area" by splitting the "radius" between 2 "circles" (since Dijkstra's searches outwards from `s` uniformly by cost, like a circle where all points are of uniform distance from its center)
    - Perform Dijkstra starting from the start and goal vertices in parallel. Throughout the process, we keep track of the current found shortest distance between the endpoints, `μ`, and the edge `α = (u, v), u ∈ Sf, v ∈ Sb` that was used to find `μ`. `α`, `Sf` and `Sb` are used to construct the shortest path.

    - Stopping Condition: In a certain iteration, `u` and `v` are popped from `Qf` and `Qb`, where `df(u) + db(v) > μ`
        - Proof: Let's assume that there is a shorter path `P`, where the furthest the backwards search has scanned is up to `Y`, and its adjacent vertex in `P` towards the `s` direction is `X`. `X` cannot have been forward scanned as this would mean `μ` is already the cost of `P`. This means that `df(X) > df(u)` and `db(X) > db(v)` and `P.cost = df(X) + db(X) > μ`, giving a contradiction. A similar argument can be used to show that `μ` only needs to be updated when an edge `(u, v)` satisfying `u ∈ Sf, v ∈ Sb` is found, because even if we update `μ` when one of the vertices has yet to be scanned, both vertices are guaranteed to be scanned before the stopping condition if they are in the shortest path.

    - Instead of popping `Qf` and `Qb` symmetrically (pop 1 element off each queue at each iteration), [2] shows that given no a priori information about the structure of `G`, choosing to pop the queue with smaller cardinality at each iteration is statistically better.

4. A* search algorithm: Dijkstra traverses uniformly by cost across all "directions", A* incorporates heuristics to add bias to certain "directions"
    - assume `G` has finite branching factor and `∃ ε > 0: ε ≤ w(e), ∀ e ∈ E`

    - The heuristic function `h` estimates the cost from a vertex to `t`
        - Admissible: `h(v) ≤ δ(v, t), ∀ v ∈ V`
        - Consistent: `h(t) = 0` and `h(u) - h(v) ≤ w(u, v), ∀ (u, v) ∈ E`
            - imagine adding edges `e1 = (u, t)`, `e2 = (v, t)` with weights `h(u)`, `h(v)`, then passing through`(u, v)` & `e2` is a detour compared to `e1`, and cannot be of lower cost (can see as `e1`, `e2`, `(u, v)` obeying 2 of the 3 triangle inequalities)
            - Consistent => Admissible
                - Proof: `h(v1) ≤ w(v1, v2) + h(v2)`, and repeatedly expand `h` on RHS by moving backwards along the shortest path to `t`. Use `h(t) = 0` for special case of `v1 = t`.
                - Intuiton: admissibility is a "global" notion of not overestimating, while consistency is "local"
        - Monotonic: given any path `P = (v1, ..., vn)`, define `Pk = (v1, ..., vk), 1 ≤ k ≤ n`, then `(Pi.cost + h(vi)) ≤ (Pj.cost + h(vj)),  ∀ 1 ≤ i ≤ j ≤ n`
            - the cost estimate of any path is non-decreasing as we move down the path
            - Monotonic <=> Consistent (relatively easy to show by converting between their definitions)

    - `Q` prioritizes vertices with lower `d + h`, which is an estimate of the cost of the optimal path constrained to go through the vertex. The algorithm terminates when `t` is scanned, where `μ` is the true minimum cost.

    - Summarizing the big ideas from the brilliant paper [1] (slightly modified):
        - Lemma 1: For any unclosed vertex `v` and optimal path `P` from `s` to `v`, `∃ v' ∈ Q, P: d(v') = δ(v', t)`
            - If `s` is open, the lemma is trivially true since `s` is available
            - Let `u ∈ S` be the furthest vertex in `P` s.t. `d(u) =  δ(s, u)` (this always exists if `s` is closed since `s` satisfies it). Then its successor on `P` will be in `Q` and also have minimum `d` value.

        - Corollary 1: If `h` is admissible and A* hasn't terminated, `∃ v ∈ Q, P: (d+h)(v) ≤ δ(s, t)`, where `P` is the optimal path
            - By Lemma 1, there exists a vertex in `Q` and `P` with minimum `d` value. Using the fact that `h` is admissible, it can be seen that this vertex satisfies the condition.

        - Theorem 1: If `h` is admissible, A* is admissible (terminates with the optimal solution if it exists).
            - Case 1: no termination
                - all vertices further than `(δ(s, t)/ε)` steps away from `s` will never be scanned by Corollary 1 (since such vertices have `d+h` value greater than `δ(s, t)`)
                - the finite number of vertices within `(δ(s, t)/ε)` steps can only be reopened a finite number of times since there are a finite number of paths from `s` to it that passes through only vertices within `(δ(s, t)/ε)` steps (note that previous paths with loops would not even be considered)
            - Case 2: terminating before finding the optimal path
                - by Corollary 1, A* will not terminate by popping `t` with a sub-optimal path (cost higher than `δ(s, t)`) because there will always be a vertex on the optimal path that will be popped first

        - Lemma 2: If `h` is consistent, `d(v) = δ(s, v), ∀ v ∈ S`.
            - Suppose a vertex `v` is about to be closed without the optimal path `P` to it from `s` being found. By Lemma 1, there exists a vertex `u ∈ Q, P` with mnimum `d` value. `d(v) > δ(s, v) =  d(u) +  δ(u, v)`, `(d+h)(v) > d(u) + δ(u, v) + h(v)`, and by the consistency of `h`, `δ(u, v) + h(v) > h(u)`. Thus, `(d+h)(v) > (d+h)(u)`, contradicting the fact that `v` would be closed before the optimal path to it is found.

        - Lemma 3: If `h` is consistent, the priority of `Q` is nondecreasing.
            - Let `n` be the next vertex to be closed after `m`. If the optimal path to `n` does not pass through `m`, then the lemma is trivially true because `n` would be in `Q` when `m` was popped, requiring `(d+h)(m) ≤ (d+h)(n)`. Otherwise, by lemma 2, `m` and `n` have achieved minimum `d`, and `(d+h)(n) = δ(s, m) + w(m, n) + h(n) ≥ δ(s, m) + h(m) = (d+h)(m)`.

        - Corollary 2: If `h` is consistent and A* has yet to terminate, `(d+h)(v) ≤ δ(s, t),  ∀ v ∈ S`.

        - Summary of optimality proof (since I'm not completely sure of my understanding of the paper):
            - Given a graph where no two distinct vertices have the same `d+h` and consistent `h`, it can be shown that any admissible algorithm that is no more informed than A* must scan all vertices scanned by A*. We use the fact that A* scans all non-goal vertices satisfying `(d+v)(v) < δ(s, t)` (by Lemma 3 and no ties allowed), and that we can construct a graph for which an algorithm will be inadmissible if it doesn't scan all of such vertices. (see def of "no more informed" in the paper, I don't really understand it well enough to explain)
            - Let `L` be the set of vertices scanned by a no more informed, admissible algorithm A using the same consistent `h` as A*. If it exists, the first vertex `v ∉ L` that A* scans must satisfy `(d+h)(v) = δ(s, t)` (otherwise A is inadmissible by the previous argument). Since the vertex `v'` that Corollary 1 states exists was not chosen, it means that `(d+h)(v') = δ(s, t)`. Thus, we can modify the tie-breaking rule to ensure `v` is not scanned. Repeating the procedure gives us an A* algorithm that doesn't scan any more vertices than A, and that there always exists such an A* algorithm which doesn't do worse than A when using the same consistent `h`.
            - Under the above premises, any A* algorithm satisfies `N(A*) ≤ N(A) + R(A*)`, where `N` is the number of scanned vertices and `R` is the number of critical ties (ie a pair of vertices `v, v': (d+h)(v) = (d+h)(v') = δ(s, t)`). This is because all non-critical tie vertices scanned by A* must be scanned by A for A to be admisible, and the only chance for A to have lower `N` is for A* to be "unlucky" in handling the critical ties. In most situations, `R` is not likely to be large as the critical ties likely only occur close to `t`, where `h` should be a pretty good estimator (by admissibility).

    - It can shown that given admissible `h1` and `h2`, if `h1` dominates `h2` (i.e. `h2(v) ≤ h1(v), ∀ v ∈ V`), then A* using `h1` expands no more vertices than when using `h2`.

    - It is not hard to see that A* with a consistent heuristic `h` is practically equivalent to Dijkstra using modified edge weights `w'(u, v) = w(u, v) - h(u) + h(v)`. Consistency guarantees non-negative edge weights, and the priority of all vertices in `Q` relative to each other is the same because all are just added by a constant `-h(s)`. Consequently, the shortest-cost path is also unchanged because the cost of all paths from `s` to `t` are just added by `h(t) - h(s) = -h(s)`. Thus, it can be seen that A* using a consistent `h` is optimal and complete, and the nice properties of consistency proven above are easily seen to be inherited from Dijkstra's algorithm.

5. Bidirectional Heuristic Search:
    - Assuming `hf` and `hb` are consistent, the stopping condition is: in a certain iteration, `max(priority(Qf) , priority(Qb)) ≥ μ`.
        - Proof: WLOG, suppose that `priority(Qf) ≥ μ` caused the termination. By consistency, the `d+h` value of any popped vertex is at least the cost of the shortest path which is constrained to pass through that vertex. Combining this with the guarantee of consistency that the priority of `Qf` is nondecreasing, any later popped vertex in `Qf` cannot be part of a shorter path. Lastly, `Qb` cannot uncover any shorter path because any later found path would have also been uncovered by `Qf` after the stopping criteria, and it has been shown above that such a path cannot be shorter.

    -  In the case where the heuristics are also balanced (consistency is still required), implementing bidirectional A* by viewing it as bidirectional Dijkstra with modified edge weights will also work. Balanced means that the modified edge weights between two fixed vertices using either heuristic is the same: `w(u, v) + hf(u) - hf(v) = w(u, v) + hb(v) - hb(u)`. Thus, the required condition is `(hf+hb)(v) = constant c`.
        - Just implement A* on both sides, and pay extra attention to ensure that `df`, `db`, `hf`, `hb` have the correct interpretation. Then the termination condition is just: `priority(Qf) + priority(Qb) ≥ μ + c` (`c` is from above).

    - ADD-BAA [3] attempts to make the stopping condition be satisfied ealier by dynamically improving the heuristic. Let `hf` be consistent, `DIFF_f(v) = δ(v, t) - hf(v)`, and `MinDIFF_f = min (DIFF_f(v), v ∈ Sb)`. It can be shown given the optimal path P from `v` to `t`, `DIFF_f` is non-increasing as we move along the path; and that `Hf(v) = hf(v) + MinDiff_f, v ∉ Sb` is admissible (and consistent due to being no smaller than `hf`). Since this is a constant offset, the order between elements in the priority queues are unchanged, and we only need to apply this offset when evaluating the stopping condition.

    - These are just the simpler implementations and there are much more complex ones.
        - example: exploiting heuristic inaccuracies, meet in the middle (or some fraction of the optimal path), more complex dynamic heuristics, contraction hierarchies, improved algorithm and termination condition (for example "A new bidirectional search algorithm with shortened postprocessing"), ...

    - Furthermore, it seems like there is still no clear answer on which bidirectional heuristic search algorithm is better in general.

6. Tried `std::collections::BinaryHeap`, which is a max-heap but we can use a custom `Ord` implementation to change the ordering of elements.
    - when objects are wrapped with `std::cmp::Reverse`, their order during comparisons are reversed

7. `wrapping_add`, `wrapping_sub` exists (also with signed versions)

8. When using vectors instead of arrays to store a grid, we can flatten the grid to a 1D vector to ensure contiguous memory.

9. Use `then` on a boolean to map it to an Option. This was used in the `get_neighbours` function when doing bounds checking. (much better than just allowing overflow and underflow like in previous days)

---

# Day 16
1. `matches!` works using `match`, so the same basic patterns used in `match` can also be used.

2.  `RangeInclusive<char>` exists!
    - useful for stuff like `'A'..='Z'` or hexadecimal check `'0'..='9' | 'A'..='F'`

3. Round-up integer division for `a / b`: `1 + (a-1)/b`
    - careful of `a = 0` for unsigned integers
    - `(a+b-1)/b` works but has the possibility for overflow

4. `std::panic::catch_unwind` catches panics inside a closure, returning a `std::thread::Result<T>`, ie `Result<T, Box<dyn Any + Send + 'static>>`.
    - Note: `assert!` invokes `panic!` and thus can be caught

5. Use `as_ptr` on `&str` to directly get the `u8` pointer. This can be used for pointer comparisons of string slices instead of string comparisons.

---

# Day 17
1. `min`, `max`, `clamp` in `std::cmp` (default implementations for the functions provided in `trait Ord`)

---

# Day 18
1. `unreachable!` can be used to mark unreachable code points. If it is ever reached, the program will panic and print the provided formatted message.

2. This is the first time I parsed the puzzle input with the help of a tokenizer. `SnailfishTokenStream` lazily tokenizes the internal string slice. After trying the usual method and refactoring to this version, I can really see how it makes the code simpler and more flexible to change. The same goes for the returned parse errors. Seeing that even my toy version which only handles numbers and one type of brace with simple rules is this complex makes me really amazed at what IDEs and compilers can do.

3. Probably the most "fun" one so far. All the required skills are just barely within reach, and it was sufficiently challenging without leaving me feeling completely helpless. It was particularly satisfying when I thought to refactor the code to parse using a tokenizer, and when I figured out how to explode `SnailfishNumber`s using recursion.
