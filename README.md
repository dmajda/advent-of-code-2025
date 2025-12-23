# Advent of Code 2025

My solutions to the Advent of Code 2025 puzzles written in Rust.

The goal was to practice the basics of Rust, keep my knowledge of the language
fresh, and learn something new along the way. Oh, and also have fun!

In general, I tried to **come up with efficient solutions and implement them
cleanly**. This means I avoided obvious algorithmic inefficiencies, but I mostly
didn’t pursue various small optimizations that conflicted with code readability.

See below for my personal [rules](#rules), a few [highlights](#highlights), and
brief notes on my [approach](#approach).

## Code

The code is organized as a [Cargo workspace][cargo-workspaces] with a separate
package for each day (`day-1`, `day-2`, etc.). Every package contains a binary
that reads puzzle input from the standard input and writes answers to both parts
of the puzzle to the standard output. Puzzle inputs are [not included with the
code][aoc-faq-copying].

To run the binaries, use `cargo run`:

```console
$ cd day-3
$ cargo run < input.txt
...
16927
167384358365132
```

## Rules

I played by the following rules (mostly):

1. **Use only the Rust standard library.** I relaxed this rule a bit after a few
   days, though — see [below](#approach).

2. **Programs should not panic on malformed input.** The only exception was
   integer overflows, which I didn’t protect against. My goal here was to get a
   feel for Rust’s error handling capabilities.

3. **Come up with the solution myself.** I only looked at solutions produced by
   others after I finished my solution and pushed it to GitHub.

4. **No code written by AI.** I used AI to discuss specific topics or techniques,
   and sometimes for research, but never to write solution code.

5. **No going back.** When a solution for the day was done, I didn’t touch it
   anymore (except for trivial fixes).

   This rule was a bit painful to adhere to, as I learned how best to do various
   things in Rust and as I saw solutions to the problems that were better than
   mine. But not adhering to it would have led to madness.

## Highlights

My favorite solutions:

  * **[Day 3](day-3/src/main.rs):** I managed to come up with an efficient
    algorithm, and the solution code is short & sweet.

  * **[Day 7](day-7/src/main.rs):** This was an easy problem, and I think the
    code expresses its solution particularly cleanly.

My least favorite solutions:

* **[Day 2](day-2/src/main.rs):** I resorted to a brute-force algorithm after a
  better algorithm based on generating invalid IDs seemed too complex to
  implement. I completely missed an elegant way to generate the IDs using
  multiples of numbers like 101, 10101, etc.

* **[Day 10](day-10/src/main.rs):** After devising a nice solution for part 1, I
  failed to see a reasonable way to solve part 2 without a linear programming
  solver. I ended up using [`microlp`][crates-microlp], which felt like
  cheating. It turned out that a [beautiful solution][reddit-1pk87hl] exists,
  but to be fair, it was *really* hard to see.

* **[Day 11](day-11/src/main.rs):** Let’s just say I’ll remember now that the
  number of paths from A to B through C is equal to the number of paths from A
  to C multiplied by the number of paths from C to B.

## Approach

Initially, I tried to write my solutions “properly”: with a nice data model,
careful error handling, and at least basic unit tests. However, after **days 1
and 2**, it became obvious this was an overkill. I was spending more time on
peripheral issues than on actual problems.

As a result, from **day 3**, I started using [`anyhow`][crates-anyhow] to
simplify error handling and allowed some low-level errors to just bubble
through.

From **day 6**, I stopped writing unit tests, which didn’t add much value over
testing with real inputs.

I also relaxed input validation: I now mostly just checked conditions necessary
for the algorithms to work, but otherwise allowed invalid inputs to produce
invalid results.

From **day 10**, I started using [`regex`][crates-regex] to simplify input
parsing and validation.

During all the days, I experimented a bit with various approaches to structuring
the code, reading input, etc. This means the solutions are not particularly
uniform in this regard.

Because of the tight schedule, I didn’t adhere to my usual standards regarding
code structure, comments, commit messages, etc. But I guess that wasn’t the
point.

## License

This project is licensed under the [Apache License, Version 2.0](LICENSE-APACHE)
and the [MIT License](LICENSE-MIT). You may choose either license at your
option.

[aoc-faq-copying]: https://adventofcode.com/2025/about#faq_copying
[cargo-workspaces]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[crates-anyhow]: https://crates.io/crates/anyhow
[crates-microlp]: https://crates.io/crates/microlp
[crates-regex]: https://crates.io/crates/regex
[reddit-1pk87hl]: https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
