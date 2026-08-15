# How the house's account came to be projected 20 below its floor

Everything below was read by walking from the one identity handed to this audit — the world
the house is in now, `71250407…` — through the thesis archive and the canonical history.
The program that did the walking is `src/main.rs`; run it with `cargo run`. Section numbers
in brackets, e.g. **[3]**, point at the section of that output a claim was read from.

The replay's own bookkeeping (`Replay::intentions`, `Replay::worlds`) was never read; the
compiler confirms it, warning that both fields are dead.

---

## The short version

The account is **not** 20 below anything today. What actually settled is a single receipt of
+100, and the factual level is **+100** in every world of the lineage **[3, "factual level"]**.
The −20 is a *projection* of the world the house is in now: if both intentions it still holds
open are realized, the level closes at −20, and the floor is `cash >= 0` **[4]**.

Two decisions produced a world that was already infeasible at the moment it was taken. The
first was reversed within the same day. The second is the one the house is still standing in.

---

## The vocabulary, as read from the graph

| name | identity | what it says | recorded |
|------|----------|--------------|----------|
| C1 | `7998cbe1…` | market receives **+100** into `account`, due 2026-01-02 | 2026-01-01 |
| C2 | `834371ed…` | house spends **−120**, due 2026-01-08 | 2026-01-06 |
| C3 | `1f0d449a…` | house spends **−30**, due 2026-01-14 | 2026-01-06 |
| C4 | `5c6332ee…` | house spends **−90**, due 2026-01-20 | 2026-01-09 |

No commitment depends on any other **[1, "dependencies: none"]** — so nothing here is a
dependency-ordering problem.

Two events exist, in one chain **[2]**:

| identity | settles | observation | occurred / recorded | previous |
|----------|---------|-------------|---------------------|----------|
| `5664085b…` | C1 | `Settled` → **Fulfilled** | 2026-01-02 | none |
| `dd0c480b…` | C2 | `Cancelled` → **Cancelled** | 2026-01-06 | `5664085b…` |

The floor: the resource `cash` behind the instance `account` carries an opaque constraint,
so it was **probed** rather than read — `check(-0.0001)` refused, `check(0.0)` allowed
**[4]**. Its debug form agrees: `Constraint(GreaterThanOrEqual(0.0))`.

---

## 1. How the account got here

Five worlds, reached by resolving `parent` from `71250407…` until a genesis **[0, 3]**:

| | identity | cut (known_at, head) | frozen | open | projected level | verdict at its own cut |
|---|---|---|---|---|---|---|
| W0 | `a2c8c533…` | 2026-01-06, `5664085b` | C1 | C2 | **−20** | OutOfBounds at −20 |
| W1 | `d207a4e7…` | 2026-01-06, `dd0c480b` | C1, C2 | — | +100 | no conflict |
| W2 | `4688fa77…` | 2026-01-06, `dd0c480b` | C1, C2 | C3 | +70 | no conflict |
| W3 | `615cbca2…` | 2026-01-12, `dd0c480b` | C1, C2 | C3 | +70 | no conflict |
| W4 | `71250407…` | 2026-01-12, `dd0c480b` | C1, C2 | C3, C4 | **−20** | OutOfBounds at −20 |

The sequence, then, is: the account was opened by C1 (+100), fulfilled on 01-02. On 01-06 the
house committed to spending 120 and opened a world around it. That commitment was cancelled
the same day, and replaced by a smaller one of 30. On 01-09 a further 90 was committed, and on
01-12 the house recognized that later knowledge and folded the 90 into the world it holds.
100 − 30 − 90 = −20 **[3, the running arithmetic]**.

Two things the numbers alone would hide:

- **The level is a projection, not a balance.** Only C1 has settled. C2 was cancelled and
  therefore *contributes nothing* — its −120 never happened. C3 and C4 are still Unsettled
  **[3, conditions]**.
- **Nothing is late.** As of 2026-01-12 both open commitments report `within deadline`
  **[3]**. This is a capacity breach, not a timeliness one — and by the engine's own rule a
  deadline never enters a feasibility verdict at all.

*Inferred, not read:* the breach falls on **2026-01-20**, C4's due date. `Conflict::OutOfBounds`
carries only the instance and the level; I matched the reported −20 against the only step of
the punctual sequence (100 on 01-02 → 70 on 01-14 → −20 on 01-20) that reaches it.

---

## 2. The decisions, and what each was decided against

A Thesis records no operation — only a parent, a cut and a selection. Which of the three
derivations produced each edge was therefore **derived by comparison**: an edge whose cut is
unchanged can only be a fork, and one whose cut moved can only be an advancement **[3,
"derivation:"]**.

| edge | kind | what changed | level before → after |
|---|---|---|---|
| — → W0 | **genesis** | selects C2 against a cut knowing only C1 fulfilled | — → **−20** |
| W0 → W1 | advance | head moves to the cancellation; C2 moves open → frozen | −20 → +100 |
| W1 → W2 | **fork (a decision)** | introduces C3 | +100 → +70 |
| W2 → W3 | advance | instant moves 01-06 → 01-12; head unchanged | +70 → +70 |
| W3 → W4 | **fork (a decision)** | introduces C4 | +70 → **−20** |

So there are **three decisions** (the genesis and two forks) and **two advancements**. The
advancements decided nothing: `advance` cannot add an intention, and neither of these imposed
a commitment the parent had not already selected — "imposed by history: none" in both cases
**[3]**. W0 → W1 changed the level from −20 to +100 without anyone choosing anything: the
cancellation is what did that.

**What each decision was decided against** is the cut it recognizes, and this is the part an
audit cannot fudge. `Interpretation::of` takes the event chain from the Thesis itself, never
from the caller, so every verdict in the table above is computed from the knowledge of that
world and nothing later. Concretely:

- the genesis knew C1 fulfilled, +100 on the table, and nothing about the cancellation;
- W2's fork knew C1 fulfilled, C2 cancelled — 100 available, and it spent 30 of it;
- W4's fork knew C1 fulfilled, C2 cancelled, and **C3 already selected in its own parent**.
  It was not deciding about 90 against 100. It was deciding about 90 against the 70 that C3
  left **[5]**.

---

## 3. Was any decision unsound at the time it was taken?

**Yes — two of the three, and the same way both times: the world was already infeasible when
it was created, under every hypothesis the engine offers.**

**The genesis (W0, `a2c8c533…`).** Selecting C2 (−120) against a cut whose only settled fact
is +100 produces OutOfBounds at −20 under `FinalState`, `OnDueDateNet` and
`OnDueDateInAnyOrder` alike **[3]**. No later knowledge is needed to see it; the arithmetic
is +100 − 120.

**The last fork (W4, `71250407…`).** Its parent W3 was clean at +70. Introducing C4 (−90)
produces OutOfBounds at −20 under all three hypotheses **[3]**, and its parent's own verdict
one line above is "no conflict" — so this edge is exactly where a choice first put the account
below the floor **[5, "← this edge is where the floor was first breached by a choice"]**.

**The fork that introduced C3 (W2) was sound.** 100 − 30 = 70, clean under all three
hypotheses. Read together with the cancellation that precedes it, W1 → W2 looks like a
correction: the house withdrew the 120 and replaced it with something it could afford.

Three qualifications, because the difference between them matters:

1. **Admitting C2 and C4 into the canon was not itself unsound.** The Canon admits knowledge;
   feasibility is derived, never enforced at admission. What the verdict attaches to is
   *selecting* a commitment into a world, not asserting it.
2. **An infeasible thesis is not by itself a mistake.** Theses exist to compare alternatives,
   and evaluating a world that turns out infeasible is what they are for. The graph carries no
   marker distinguishing a plan from an exploration — there are no named references in it, and
   "main is a convention" is the engine's own position. The only thing privileging W4 is that
   `hindsight::build()` handed it over as *the world the house is in now*, which is application
   state outside the graph. On that footing, W0 was infeasible and abandoned within the day;
   W4 is infeasible and is where the house is standing.
3. **"No conflict found" is not a clean bill of health.** The engine reports findings, never a
   verdict of feasible; an empty list means nothing was found under the hypothesis asked. The
   soundness claims above are one-directional: the breaches are proven, the clean readings are
   not proofs of realizability.

**Should anyone have seen it coming?** The information required was present at each cut, and
required no hindsight to compute — that is exactly what "interpreted only under its own cut"
buys. What the graph does **not** show is whether anyone looked: projections are derived and
never stored, so there is no record of a feasibility check having been run, or skipped.

---

## 4. What alternatives the house had

Enumerated by constructing the alternative worlds and interpreting each **[7]**. Every line
below is a real Thesis built at the real cut, not an estimate.

**At the genesis cut (2026-01-06, head `5664085b`):**

| selection | level | verdict |
|---|---|---|
| — (C1 alone, frozen) | +100 | no conflict |
| C3 | +70 | no conflict |
| **C2** ← taken | **−20** | OutOfBounds |
| C2, C3 | −50 | OutOfBounds |

**At the last decision (forking W3, 2026-01-12, head `dd0c480b`):**

| open selection | level | verdict |
|---|---|---|
| — (omit C3) | +100 | no conflict |
| C4 only (omit C3) | **+10** | no conflict |
| C3 only | — | refused: identical to the parent — holding still is not a fork |
| **C3, C4** ← taken | **−20** | OutOfBounds |

The finding worth stating plainly: **at the last decision the house had a feasible way to take
on C4** — omitting C3 leaves +10 within bounds. It did not have a feasible way to take on
both. Whether dropping the 30 was an acceptable trade is a business question the graph cannot
answer; it has no notion of what the house needed.

Two limits on this enumeration, both structural:

- **The pool is a lower bound.** It contains exactly the commitments some world in the
  lineage selected. `CanonicalKnowledge` offers lookup by identity and `head_as_of`, and no
  enumeration at all — so a commitment the house admitted and never selected anywhere is
  unreachable from the entry point I was given and cannot appear in the table.
- **Alternatives that were never asserted cannot be enumerated.** "Commit to 70 instead of 90"
  would have been feasible arithmetically, but no such commitment exists in the canon, and
  inventing one to test it would be writing knowledge, not auditing it.

---

## What I could not determine

Named plainly rather than filled in.

1. **Why C2 was cancelled, and who observed it.** The event carries an observation name
   (`Cancelled`), an `occurred_at` and a `recorded_at`. There is no field for a reason and none
   for an author **[8]**. Whether the cancellation was a correction the house made on seeing
   the −20, or a counterparty walking away, is not in the graph — the reading in §3 that calls
   W1 → W2 "a correction" is an interpretation of the shape of the sequence, not a fact read
   from it.
2. **Who took each decision.** A Thesis carries parent, cut and selection — the fields its
   identity is derived from. No author **[8]**.
3. **When each decision was physically taken.** The cut is *declared* knowledge, not attested
   provenance; the Thesis layer states this as a non-responsibility. One piece of ordering
   evidence was recovered, and only one: the genesis recognizes head `5664085b`, while the
   instant 2026-01-06 addresses `dd0c480b` today, and rebuilding the genesis's exact cut is now
   refused outright — `head 5664085b… precedes the cut … which its instant addresses` **[6]**.
   That proves the genesis was taken **before** the cancellation was recorded. Nothing
   comparable exists for the other decisions.
4. **The intraday order of C2 and C3.** Both were recorded on 2026-01-06 **[8]**. A commitment
   never enters the event chain, so recording date is its only knowledge-time coordinate and
   there is nothing finer to read. Consequence for §4: I cannot tell whether "select C3 instead
   of C2" was genuinely available at the genesis, or whether C3 was only asserted afterwards.
   The row is a valid world at that cut; whether it was a real option at that moment is
   undeterminable.
5. **Whether the house held worlds outside this lineage.** `ThesisLookup` resolves a thesis by
   identity and offers no enumeration, so an abandoned sibling — a fork taken and discarded —
   would be invisible from the entry point given. Everything above describes the ancestry of
   the current world, not necessarily everything the house considered.
6. **Whether the −20 will happen.** Feasibility says no completion of *this* selection stays
   within bounds under any hypothesis offered. It says nothing about what the house may still
   do: C3 or C4 may be cancelled, a fork may omit either, and the level would move. The graph
   also shows only movements on `account` that some world selected — it carries no claim that
   no other cash exists.
7. **Whether a feasibility check was ever run before a decision.** Nothing records it. This is
   the one gap that bears directly on "should anyone have seen it coming": the answer to
   *could* they is a demonstrable yes, and the answer to *did* they is not in the graph.

---

## Reproducing this

```
cargo run
```

`src/main.rs` takes only `Graph { canon, archive, current }` from `hindsight::build()`, walks
the lineage from `current`, and for each world prints its cut, its selection, its conditions,
the level arithmetic line by line, and the engine's feasibility verdict under all three
hypotheses. The audit's own arithmetic is cross-checked against the engine's reported breach
level for every world; all five agree **[3, "cross-check"]**. The audit reads only — it admits
nothing to the canon and stores nothing in the archive.
