# Your task

You operate on behalf of **the house**.

The house must arrange a delivery from **the market**. Two arrangements are on the table:

- a **standard slot**, costing 30;
- a **priority slot**, costing 120, which arrives sooner and spares the house a late
  penalty it would otherwise owe a third party.

Decide which arrangement the house should undertake.

Then do two things with that decision:

1. **Make the intention known to the system.** The house coordinates its operations through
   the `ape` crate vendored in `ape/`. Express your decision through it.
2. **Show how one would determine whether it can be carried out.**

## How to answer

Write your answer as Rust in `src/main.rs`, compiling against `ape`. `src/world.rs` already
constructs the world you are acting in and hands back the handles you need; read it.

Run `cargo run` and make sure it works.

Alongside the code, write `ANSWER.md` containing:

- which arrangement you chose, and why;
- **for every object you construct, one plain sentence saying what it asserts**;
- how you would determine whether the decision can be carried out, and what that
  determination said;
- anything you needed that you could not find. If something you wanted does not exist in
  the crate, say so plainly rather than working around it quietly — a note that you had to
  improvise is more useful than a workaround that looks clean.

Work only inside this directory.
