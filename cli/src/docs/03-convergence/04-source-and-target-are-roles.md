# Observation 4 — Source and Target are roles, not sides

Asked in both directions between the same two lines, over the same Base, Synthesis gives two
different answers.

```text
the equipment line's intention, in the inventory line
    introduced   { equipment, contingency }
    conflicted   HistoricalUnavailability { contingency, recorded 01-12, known 01-10 }

the inventory line's intention, in the equipment line
    introduced   { inventory, maintenance }
    applicable   introduce { inventory, maintenance }
                 candidate { funding, equipment, contingency, inventory, maintenance }
```

The first draft of this document recorded that as a tension. It is not one, and the reason is
worth stating clearly, because an experiment that reads a defined behaviour as friction will go
looking for a repair nobody needs.

## They are not the same question

A difference is measured **between the Base and the Source**. Nothing about the Target enters
it; the Target arrives afterwards, when the difference is resolved against what it already
holds. So Source and Target are not two ends of one relation that could be read either way —
they are two roles in one operation, and each earns a different thing.

The Base's ancestry to the Source is what makes an absence readable as a decision. Its ancestry
to the Target is what makes that decision applicable there. Swap the two and you have not
reversed a question, you have asked another.

Measured rather than asserted: the two directions here introduce **disjoint** sets. Neither is
the other's inverse, and there is no operation between them that would turn one into the other.

Symmetry would therefore be a loss. If both directions agreed, the operation would have stopped
distinguishing *whose* intention is being moved, which is the whole of what it is for.

## What the two directions do share, and what fixes them

Both are measured over the same three worlds and neither line withdrew anything, so what
separates the answers is only which line's decisions are being carried. That much is definition.

What is not definition is which direction can *succeed*, and it is fixed by something neither
line decided. The rule is the engine's: a Target cannot be given what its cut could not have
known. What the arrangement adds is that the rule's direction is settled by arithmetic, in
advance:

```text
Source.known_at  ≥  recorded_at  >  Target.known_at
```

An introduced commitment is open in the Source, so it was recorded no later than the Source's
cut; the conflict says it was recorded later than the Target's. So this refusal is reachable
only when the Source is strictly later than the Target. And because a fork copies its parent's
cut and an advance may not regress the instant, a coherent Base is never later than either — so
such a commitment was not knowledge at the Base either, and can only have entered the Source
through an advance the Target has not matched.

The usable form of all this is not a complaint but a shortcut:

> Which direction between two lines can carry an intention wholesale is readable from their two
> cuts, before any set arithmetic is done. An application never has to discover it by trying.

## The difference is flat, and portability is not

An `IntentionalDifference` is computed from selections alone and never consults a cut, so it does
not mark which of the commitments a Source introduced were already available at the Base and
which arrived only after the Source advanced. The two are one set.

That is right for what a difference *is* — it answers what was decided, not what is portable —
but it means a caller holding one cannot tell which half of an intention can travel. That appears
downstream, as a conflict, and only against a particular Target.

Which is why this phase asked a third time. One report says the transfer is refused; a second,
against the same Target with the Source one step earlier, says the refusal is about one
commitment and not about the intention:

```text
the equipment line, at its first decision, in the inventory line
    introduced   { equipment }
    applicable   candidate { funding, equipment, inventory, maintenance }
```

Same source line, same Target, same Base. The only thing that changed is whether the intention
was built over knowledge the Target has recognized.

## What the record had to carry

The report is recorded in the application's vocabulary, like the journal and the reading before
it, and for the same reason: an `ApplicabilityReport` derives no serialization, is not an entity,
and is obtained again by asking again.

It carries three identities and not two — a record naming only the worlds it moves between would
be an answer whose question is missing, and a different admissible Base is a different legitimate
question. The status is kept as three cases rather than a verdict plus two optional collections,
which mirrors the engine and buys the same thing: a conflicted result without conflicts and an
applicable one carrying them are both unwritable.
