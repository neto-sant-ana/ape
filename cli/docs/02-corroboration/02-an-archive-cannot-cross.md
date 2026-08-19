# Observation 2 — An archive cannot cross a process; what crosses is a witness

Phase 4 was written to store decided worlds through `ThesisArchive` and read them back on the
other side of process death.

It cannot. A `Thesis` derives `Serialize` and not `Deserialize`, so an archive can be written
out and never read in.

```text
Canon      →  reconstruction, Observation 1:  knowledge is replayed, not hydrated
Thesis     →  the same boundary, one layer up: a world is derived, not loaded
```

The reconstruction experiment met this at the Canon and concluded that the inputs must be
stored and the path is replay. Nothing about the Thesis layer changes that conclusion; what it
changes is what an archive can be *for*.

---

## Two things that were one thing in the protocol

**Inside a process, `ThesisArchive` is a real port.** It resolves worlds by identity and it
keeps ancestry walkable, which is what Synthesis reads when it decides whether a Base is a
common ancestor. This laboratory now has an adapter for it, and the adapter conforms.

**Across a process, there is no archive.** What is written is a record of what a world *was*,
in the application's own vocabulary — identity, ancestry, the instant and chain it recognizes,
and the two halves of its selection. It is compared against the world the decisions produce
and it can never replace it.

The protocol asked whether a stored world corroborates or supersedes. The boundary answers:
only corroborates, and not as a discipline anyone has to keep.

> *Criterion 6 — "reconstruction still derives every world" — is not a rule this experiment
> obeys. It is a fact it cannot avoid.*

That also settles the third claim of the hypothesis before Phase 5 runs it. Decisions cannot
be made redundant by stored worlds, because a stored world is not a world.

---

## What it caught

The witness over the sequence turned one silent row. This turns the other.

```text
a coordinate repointed at an entry that exists   SILENT → REFUSED   (Phase 3)
the genesis's intention narrowed                 SILENT → REFUSED   (Phase 4)
two roles reordered                              HARMLESS
```

The two refusals are not one finding. An altered intention leaves the journal untouched and
every address resolving, so no witness over the sequence can see it — the world it produces is
the only place it shows. Corroborating the coordinate and corroborating the world are different
checks over different corruptions, and this subject needed both.

Nothing passes in silence any more.

---

## Naming what disagreed, and the order that decides it

A world's identity is derived from every other coordinate, so it differs whenever anything
does. Weighed first, it would mask what actually moved and report "a different world" — which
is the half-refusal this experiment set out to avoid.

So identity is weighed **last**, and reaching it means something else:

```text
a named coordinate differs  →  the repository disagrees with itself
only identity differs       →  every coordinate agrees and the hash does not
                               — a changed derivation, not a corrupted record
```

The second has not happened here. It is worth being able to say.

---

## Consequences to carry

* Whatever Synthesis needs from an archive, it needs *within* a process. A lineage that
  travels between processes travels as decisions, and the worlds are rebuilt at the far end.
* Two witnesses, two scopes: one over the knowledge a decision stood on, one over the world it
  produced. Neither subsumes the other, and this experiment can say so because the table
  distinguishes the corruptions they each catch.
* The witnesses are the application's vocabulary, not the engine's serialized form — the same
  answer the reconstruction experiment gave about the journal, for the same reason plus one:
  a witness computed over an encoding makes the encoding load-bearing.
* Phase 5 has less to subtract than the protocol expected. What remains open is narrower and
  sharper: with the world recorded, is the *coordinate* still needed, or does the world
  witness already refuse everything it refuses?
