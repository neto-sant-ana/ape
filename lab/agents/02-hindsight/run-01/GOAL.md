# Your task

You have been asked to review how **the house** has been running its account.

The account may not go below zero. As things stand, it is projected to end up **20 below**
that floor. Somebody wants to know how that happened and whether anyone should have seen it
coming.

## What you have

The house coordinates its operations through the `ape` crate, vendored in `ape/`.

`src/world.rs` sets up the vocabulary the house operates in. `src/hindsight.rs` holds the
sequence of things that happened and folds it back into what the house knows —
`hindsight::build()` returns knowledge, the archive of the worlds the house has held, and the
identity of the world it is in now.

That identity is your way in. Everything else you reach by walking from it.

## What to determine

1. **How the account got here.** What was committed to, and when.
2. **Which decisions were taken along the way**, and what each of them was decided against —
   that is, what the house knew at the moment it took each one.
3. **Whether any of those decisions was unsound at the time it was taken.** Not whether it
   looks unfortunate now. Whether, given what the house could know then, it should have been
   taken.
4. **What alternatives the house had** at each decision, if you can tell.

## How to answer

Write Rust in `src/main.rs` to interrogate the graph, and run `cargo run`. Show your working
in the output rather than asserting conclusions.

Then write `ANSWER.md` with:

- your answers to the four questions above;
- **for every conclusion, the thing in the graph you read it from** — an identity, a value, a
  comparison you ran. If you inferred something rather than reading it, say which;
- **what you could not determine, and why.** This matters as much as what you could. If the
  graph does not carry something you needed, name it plainly rather than filling the gap with
  a plausible guess.

Work only inside this directory.
