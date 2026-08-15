# Observation 3 — A conflict names the level it reached, not the bound it left

`Conflict::OutOfBounds` carries the resource instance and the offending level. It does not
carry the bound that was breached.

Recovering the bound means walking instance → resource → kind → constraint, and the walk does
not end anywhere useful: `Constraint` exposes `check(value)` and nothing else. No accessor for
its bounds, no `Display`.

So the agent, wanting to explain its refusal in a sentence, copied `cash >= 0` out of
`world.rs` by hand — and named the problem with that itself:

> *"exactly the kind of second copy that goes stale."*

---

## Why this is not the level question again

It resembles the boundary the engine already drew around levels, and it is not the same one.

A level is withheld because *which movements count* is an operational question with several
defensible answers, and an engine answering it would be choosing in silence. The refusal
protects the caller.

A constraint's bound has no such ambiguity. There is one bound, the engine holds it, and it is
already deciding against it — the conflict exists because `check` returned false. Nothing is
being chosen on the caller's behalf by keeping it.

What keeps it is opacity, and opacity was chosen for a reason the module states: a `Constraint`
is built only through validating constructors, so every one has finite bounds and ordered
ranges by construction. An opaque predicate can gain kinds without breaking callers who read
its structure, because there are none.

The tension is therefore real on both sides, and this observation does not resolve it. It
records that presentation is where it bites.

---

## The consequence of applying it

An application that must explain a refusal cannot derive the explanation. It must keep its own
copy of the bounds, in the layer that renders text, beside the copy the engine holds and
enforces.

Two representations of one fact, in layers that cannot import from each other, and the
application's copy is the one nothing compares against the engine's. Nobody notices when the
bound changes and the sentence does not.

What the agent wanted was not the structure. It wanted a rendering — a way to say what the
constraint says. Whether the engine should offer one is not this experiment's to decide, and
proposing an interface would be exactly the ontology growth the methodological constraint
refuses.

---

## Counting honestly

This is one occurrence in one run.

It is worth noting that the CLI's own experiment met the same shape from the other side, where
presenting a level was the thing the public boundary did not hand over. The common root is
that presentation needs facts the engine holds and does not surface, and the two cases are
about different facts.

Two contexts is not a pattern. It is a reason to watch for a third.

---

## Smallest reproducing case

`run-01/ANSWER.md`, where the printed prose reads `−20 against cash >= 0`, and the `cash >= 0`
half of that sentence has no source in the report it describes.
