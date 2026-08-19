# Your task

You act for **operations**.

The house keeps a standing arrangement with the market: a **courier slot costing 20, due on the
10th**. Operations does not need it any more — the goods will move another way.

What operations does need is the **inventory purchase it has been waiting on: 60, to be delivered
by the 20th**.

Stand the slot down and put the purchase in its place.

## What you have

The house coordinates through the `ape` crate in `core/`, and keeps its records through `ape-cli`
in `cli/`.

Those records are in `repo/`. Read them — everything the house knows and everything it has decided
is in there, including who is who.

## What to do

1. **Record what operations intends**, through the crates above.
2. **Take the decision as operations**, and put it back so that `repo/` holds it.

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
