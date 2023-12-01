# Advent of Code in rust
## Structure
Each day has a `solve()` function that returns a pair of `Solution`. The type `Solution` is an enum that can contain any integer or a string.

You can create a `Solution` by specifying its type, for example `Solution::U32(value)`, or by using the From trait which is implemented for all supported types, for example, `Solution::from(value)`.

To run: `cargo run --release [days...]`

# Credit

Template made by [@agubelu on github](https://github.com/agubelu/AoC-rust-template/tree/master), with small edits made by me.

All solutions to the problems are my own.
