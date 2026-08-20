# Observation 2 — The pointer's turn is not the commit

C1 predicted that two writers preparing before either turns choose the same generation. Phase 2
measured it, and nothing refuses it: two `prepare` calls, one directory, and the second writer's three
files land on top of the first's.

C2 predicted that the **last turn** wins. It does not. Phase 3 enumerated all six orderings the two
operations admit — four calls, with each writer preparing before it turns — and in every one of them
what a reader reads is the state of whoever **prepared last**. The turn does not appear in the rule.

```text
ordering                                     live         beside it
──────────────────────────────────────────────────────────────────────
prepare A  turn A  prepare B  turn B         B            A
prepare A  prepare B  turn A  turn B         B            the base
prepare A  prepare B  turn B  turn A         B            the base
prepare B  prepare A  turn A  turn B         A            the base
prepare B  prepare A  turn B  turn A         A            the base
prepare B  turn B  prepare A  turn A         A            B
```

Nothing refused any of the twenty-four calls, and no outcome is a mixture: an application's write is
one call, so two of them leave one writer's three files entire. C2's *outcome* holds and its mechanism
does not.

## Which means a writer can publish bytes it did not write

In the four orderings where the prepares collided, the first writer's turn names a generation holding
the second writer's repository. It returns `Ok`. Both writers return `Ok`. One of them committed the
other's work, and neither can find out.

Stated so that it is about the operation rather than about this repository:

> **A turn is a claim about a name, and the name's contents are not part of the claim.** Preparing
> decides *what* a reader will read; turning decides only *when*. Two writers sharing a name make the
> second decision meaningless and the first one silent.

That is what makes a whole write a compare followed by a swap: the comparison the coordination
experiment built is in `converge`, one layer above, and there is none in the write. `prepare` reads the
pointer in order to *avoid* the live generation, which is a question about where a reader is looking —
not about who else is writing.

## The two failures are not the same failure

Both of them lose a line and say nothing, and they are reached differently, which matters for what
could answer them.

```text
a stale whole write     the writer prepared after the other turned, so it had a
                        generation to itself. Nothing was overwritten. What was
                        lost was lost because the writer's KNOWLEDGE was old

a collided prepare      the writers shared a generation. What was lost was
                        overwritten, and the writer that lost had read everything
                        there was to read
```

The first cannot be answered by anything the write knows: the files are a legitimate whole state, put
somewhere nobody else is, and only comparing journals finds it — which is `converge`'s job and
Observation 1's subject. The second is visible from inside the write, because the bytes on disk stop
being the bytes the writer put there.

## What it does not say

That either ordering is likely. It says what each leaves. How often two writers prepare before either
turns is a question about an application's shape, and this experiment measured six orderings rather
than their frequency.
