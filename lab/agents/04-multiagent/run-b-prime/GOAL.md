# Your task

You act for **finance**.

The market has invoiced the house for storage: **30, payable by the 14th**. Finance has to take that
on.

## What you have

The house coordinates through the `ape` crate in `core/`, and keeps its records through `ape-cli`
in `cli/`.

Those records are in `repo/`. Read them — everything the house knows and everything it has decided
is in there, including who is who.

## What to do

1. **Record what finance intends**, through the crates above.
2. **Take the decision as finance**, and put it back so that `repo/` holds it.

Write Rust in `src/main.rs` and run `cargo run`. Print enough that a reader of the output can see
what you did.

## How to answer

Alongside the code, write `ANSWER.md` containing:

- what you recorded and what you decided;
- **for every object you construct, one plain sentence saying what it asserts**;
- anything you needed that you could not find. If something you wanted does not exist in the
  crates, say so plainly rather than working around it quietly — a note that you had to improvise
  is more useful than a workaround that looks clean.

Work only inside this directory.
