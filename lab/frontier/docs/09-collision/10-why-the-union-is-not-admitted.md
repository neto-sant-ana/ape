# Observation 10 — Why the union is not admitted, and which layer says so

The refusal Observation 5 measures is raised **before anything is admitted**, so the obvious question is
what would happen if the engine were simply asked. In the twinned case the union of two repositories'
knowledge needs no interleaving at all — one of them already holds it:

```text
right   base, plan                     14 entries
left    base, inflow, plan             15 entries  ← this IS the union
```

That journal admits. The left repository is it. So *admit the inflow and then the plan* is not something
the engine cannot do; it is something nothing asks it to do. Doing it by hand puts the refusal one layer
down rather than removing it, and on the way there is a third refusal that explains the shape of the
whole thing.

## Three refusals, at three layers, and none of them is the engine's

```text
1  ConvergeError::Diverged { position: 13 }              cli/src/converge.rs — appended()
   "the journal diverges at entry 13: this party holds <plan>, and <inflow> is there"

   The application, comparing two journals positionally before admitting anything. This is
   what Phase 4 measures.

2  JournalError::EntryAlreadyPassed(<the fund>)           cli/src/journal.rs — replay_through()
   "entry … was admitted before the point the journal has reached"

   What the union produces if the decisions are handed over unordered. A rebuild admits the
   journal IN STEP with the lineage, so a decision cannot be applied after the replay has
   passed the entry it was taken at. The rebuild is a walk, not a set operation — and this
   is the refusal that says so.

3  LineageError::UnwitnessedKnowledge { entry: <the inflow> }   cli/src/lineage.rs — corroborate()
   "entry … was admitted, and the decision was not taken against it"

   The union, with the decisions ordered the way the merge orders them. This is the real
   answer.
```

The third is measured, not argued: `the_natural_merge_is_refused_one_layer_down` builds the union, sorts
the four decisions by where each was taken, and hands them to `rebuild`. It names the inflow.

## Why, and it is one line of the application

```rust
if let Some(unexpected) = offered.difference(&witnessed).next() {
    return Err(LineageError::UnwitnessedKnowledge { entry: … });
}

if let Some(missing) = witnessed.difference(&offered).next() {
    return Err(LineageError::WitnessedKnowledgeAbsent { entry: … });
}
```

Two directions. Which makes the witness a test of **set equality**, not of containment.

So the right side's planning decision, which was taken over `base + plan`, cannot be applied at a point
where the replay has admitted `base + inflow + plan` — even though the second contains the first. The
decision is a fact about *exactly* that prefix, and re-hosting it over a longer one is refused.

## And equality rather than containment is the whole reason

> **A world is a function of the knowledge that stood when the decision was applied.** So knowledge
> inserted *before* a decision may change the world that decision produces — and the record has no way
> to tell an insertion that matters from one that does not without re-deciding.

`lineage::decide` resolves a cut against the knowledge in hand and a selection absorbs whatever that cut
froze. An Event inserted earlier moves a head; a commitment frozen by a cut changes what a fork is
proposing against. Containment would let all of that through silently, which is the shape the divergence
experiment measured as *a coordinate that is wrong but well-formed is not detectable from the record*.

In this particular case the insertion genuinely does not matter. The inflow is a Commitment nobody
selects, a fork inherits its parent's cut, and the world the right side's decision produces over the
longer prefix would be **the same world**. The record refuses anyway, because the rule that would let it
through cannot be stated without deciding on the other side's behalf.

Which is the honest answer to *why not just admit both*:

> The engine can admit the union. What it cannot do is claim that a decision taken against one prefix was
> taken against a longer one — and a merge that admitted the union while keeping both lineages would be
> claiming exactly that.

## What it does not say

That equality is the only defensible rule. A record that could say *this insertion cannot have changed
what I decided* would be able to admit the union safely, and nothing here measures whether such a
statement is expressible — it is the sharp form of Request 1, and the reason that request is harder than
it sounds. What is settled is that the current rule is a rule about **prefixes**, that it is stricter
than the world identities require, and that the strictness is where the safety comes from.
