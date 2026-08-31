# Your task

An agent was asked to reconcile two divergent records of one house's dealings, and afterwards wrote
an account of what it had done. `claims.md` holds **46 statements taken verbatim from that account**,
in the order they appear in it.

`record/` holds the records themselves.

For each of the 46, answer one question:

> **Could somebody holding only `record/` — and no prose at all — establish this?**

Answer `YES` or `NO` for every one. Nothing else is being asked of you.

## What `record/` is

Three states of the same house's records, each in four files.

```text
record/before/   one party's records before the reconciliation
record/theirs/   the other party's records
record/merged/   the first party's records afterwards, which the account is mostly about
```

The four files, in every state:

```text
journal.json   every fact that entered the record, in the order it entered, each with the
               day it was recorded
lineage.json   every decision taken: what was decided, the journal entry it was taken
               after, which entries stood at the time, and which party claims it
worlds.json    the worlds those decisions produced — for each, its identity, its parent,
               the day up to which it recognizes history, and which obligations it treats
               as settled and which as still open
custody.json   the addresses the journal comes to
```

## What counts as YES

**Established, not literally printed.** A number that anybody can compute by adding up what is in
these files is a YES; so is a fact that follows from comparing two of the files, or two of the
states. You do not have to run anything — decide whether the files contain what it would take.

Two examples of the distinction, and they are the whole of it. **Neither is from `claims.md`** — they
are about a different house entirely, so that nothing here answers anything there:

- *"The baker's account was overdrawn on the third of March"* — **YES**, if the files hold the
  movements and the day. Adding them up is still reading them.
- *"The baker overdrew it because the miller called the loan in early"* — **NO**. A file can hold
  that the loan was called in and that the account went under. Nothing in one can hold *because*.

## What counts as NO

Anything the files cannot settle. Some of these will be true statements — about how the author
worked, about what the software does in general, about what could not be found. **True is not the
question. Established by these files is the question.**

Do not soften a NO because the statement is obviously correct, and do not stretch a YES because the
answer is nearly there. Where you are genuinely torn, answer and say so in one clause.

## How to answer

Write `ANSWER.md` in this directory:

- a line per claim: its number, `YES` or `NO`, and **at most one sentence** of reason;
- for a YES, name which file or files would establish it;
- a short list at the end of any you found genuinely difficult, with what made them difficult.

Work only inside this directory.
