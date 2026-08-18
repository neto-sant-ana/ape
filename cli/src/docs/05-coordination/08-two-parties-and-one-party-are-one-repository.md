# Observation 8 — Two parties and one party are one repository

Phase 4 asked the repository who decided each world, and the answer is not merely absent. It is
absent in a stronger way than provenance's was, and the phase's job was to establish which.

## The population exists, which is what makes the question worth asking

The repository knows agents. Two of them, by name, and it knows what each is a party to:

```text
customer   accountable, executor of the inflows
merchant   accountable, executor of the outflows
```

So this is not a record with nothing in it about people. It is a record that models people
thoroughly on one axis and not at all on the other. Every field of every decision, asserted as a
closed set:

```text
genesis   decides  known_at  selection  after  witness
fork      decides  extends   omitted    introduced  after  witness
world     thesis   thesis_parent  known_at  event_head  frozen  open
```

No decision names an agent — checked by taking the agents *out of the journal* rather than from a
list written beside the assertion, so that an agent added to the subject is searched for too.

## And one nuance that keeps the finding honest

The agents **are** in the lineage file. Every one of them appears inside a decision's `witness`,
because admitting an agent produces an `AgentId` and the witness is a set of entry addresses.

```text
witness ∋ <customer>     the agent was known when the decision was taken
```

Which says the agent was known, and never that it took the decision. A guard reading the witness
instead of the intention would have found agents everywhere in the lineage and reported the opposite
of the truth — and the reason it cannot is structural rather than careful: the helper is handed a
`Decision`, so it has no access to the witness at all.

## The measurement that makes this a demonstration

An inventory of fields shows that nothing is written down. It does not show that nothing can be
derived, and that is the whole distinction between this and provenance. So:

```text
two parties, reading before either wrote, each deciding, each converging
one party, reading once, deciding both, converging once
                                                        →  the same repository
```

Byte for byte, all three files.

> There is no function from a repository to how many minds wrote it, so there is none to which of
> them wrote what.

Provenance's unanswerable question had an answer that was a **world**, and worlds are derived — so a
search could be attempted, and it found too many candidates rather than none. Here a search has
nothing to enumerate. Not an ambiguous answer: no candidate.

## Part A removed the last accidental trace

Before the repair, the surviving decision was the last writer's — Phase 1 measured exactly that. So
position in the file weakly encoded arrival, and arrival weakly encoded party. A determined reader
could have guessed.

The canonical order is derived from content, so the first party's world holds the same position
whichever party converged first. Blinding the tie-break puts it at position 1 in one order and 2 in
the other, which is what the accidental signal looked like.

```text
before Part A   position ≈ arrival ≈ who wrote it
after Part A    position = what the decision says
```

Part A's success removed Part B's only accidental evidence. Worth stating plainly, because it is the
kind of thing that reads as a cost of the repair and is not: a signal that was an artifact of a
defect was never evidence. What it does mean is that Part B starts from nothing, by construction
rather than by oversight.

## Which the previous experiment says is a different question

Provenance failed because a claim can only be refused where it predicts something derivable, and its
claim predicted the transfer — which every rival account agreed about. This one has the opposite
problem, and it is the one the abstract promised: nothing here is derivable, so nothing can be
predicted, so nothing can be contradicted.

That is not yet a refutation. A record can be worth holding for what it lets a reader *do* rather
than for what it lets a reader check, and Phase 5 is where that has to be answered on its own terms
instead of borrowed from either side.
