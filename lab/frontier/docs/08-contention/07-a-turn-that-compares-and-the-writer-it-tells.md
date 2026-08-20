# Observation 7 — A turn that compares, and the writer it tells

Part B. A [`Prepared`] keeps what it wrote, and turning reads those three files back before renaming
the pointer — refusing, by name, to publish a generation this write did not write.

```rust
for (name, encoded) in &self.written {
    if fs::read_to_string(into.join(name))? != *encoded {
        return Err(RepositoryError::Contended { generation });
    }
}
```

That is the whole of it. No lock, no lease, no file nothing refers to, no clock, and nothing outside
the record — which matters, because the protocol named *a repair that needs something outside the
record* as a finding rather than a remedy.

## Against the criterion

**It removes a state a reader can be misled by.** Three of them, and they had one cause:

```text
another writer's whole state, published by the writer it displaced   Observation 2
a mixture of two writers' files, published as a finished commit      Observation 4
a state two commits old, put back by a stale handle                  Observation 5
```

All three came from a turn being a claim about a **name** whose contents were nobody's business. One
comparison answers all three, and Phase 7 measures it the way the atomicity experiment measured its
own: by producing every one of the six mixtures in a generation two writers shared and finding that
none of them reaches a reader, with the repository answering by value what it answered before through
all twelve schedules.

**And what the repair replaces survives.** Measured again rather than inherited, because this changes
the operation that keeps that promise: after the collision, the writer whose bytes are still there
commits, and the generation it replaced is byte-identical to what was there before.

## The second criterion is satisfied by construction, which is worth saying out loud

This experiment added a condition of its own, because it was the first whose remedy could be a lock:

> A repair that serializes writers must say what it costs a writer that waits, and what happens to one
> that does not come back.

Nothing here waits. No writer is held, nothing is claimed, and there is no state that a writer failing
to come back would leave behind — a `Prepared` that is dropped abandons three files in a generation
nothing reads, and the next write overwrites them. So the condition is met vacuously, and that is a
property of the shape rather than an achievement: a comparison is not a lock, and the reason to prefer
it here is exactly that the second criterion would otherwise have had a bill to pay.

A refused writer's recovery is the one the coordination experiment established and this one measured
six ways: read again, decide again, converge. Phase 7 runs it and both lines are there.

## What changed is not who wins

The rule Phase 3 found is untouched: what a reader reads is still the state of whoever prepared
**last**, and which writer that is depends on an ordering nobody chose. The repair does not arbitrate
and does not try to.

```text
before   the last preparer commits, and both writers are told they did
after    the last preparer commits, and the other one is told it did not
```

So the fix is not to the outcome; it is to the **silence**. Which is the shape of every repair in this
sequence: the coordination experiment did not stop a party from being late, it stopped a late party
from being unaware.

## What it costs, and this is the sequence's first observable price

```text
before   turn = one rename over a pointer written beside itself
after    turn = three file reads, then that rename
         Prepared = an encoded copy of the repository, held until it turns
```

Nine experiments have deferred cost as a variable. This is the first repair that has a price which can
be stated without measuring anything: every write now pays for a refusal that only two writers can
provoke. Naming it is the point — nothing here measured whether it matters, and the *Cost* variable
stays open with one entry in it.

## What it does not see, and cannot

A writer holding an old **reading** overwrites nobody. It prepares a legitimate whole state of its own
in a generation nobody else is in, its bytes are still its bytes when it turns, and it commits — losing
the other line in silence. Phase 2 measured that before Part B and measures the same thing after, which
is why it is kept as a green test rather than a sentence.

```text
what protects a WRITE     the generation it prepared, compared at the turn
what protects KNOWLEDGE   the journal on disk, compared at the merge
```

Two comparisons, neither substituting for the other, and a writer that skips `converge` has only the
first. That is a statement about the application's shape rather than a defect in either: the record
cannot make a writer merge.

## And the honest limit

Between the last comparison and the `rename` there is a window. Closing it takes an atomic filesystem
primitive rather than two calls, and this experiment excluded threads — so what is refused here is an
interleaving of **calls**, which is what was measured and all that is claimed.

The same shape as the atomicity result, one layer along: the states are **out of reach through the
write, and not impossible**. The single-file writes still reach every mixture, because five experiments
need them to; what a turn refuses is publishing one.
