# Observation 4 — The write succeeds and the read is refused

E4's second clause, measured. A repeat costs a journal record and no knowledge — and, depending on
what falls between the two occurrences, it costs the repository's readability. Nothing at the moment
of writing says so.

## The three interleavings

One journal, three arrangements of it. `Taken::after` is an address; `replay_through` resolves an
address to its **first** occurrence; corroboration weighs sets.

```text
                                           records  distinct   read back
readmitted adjacently, decision after           16        15   reconstructs
something learned between, decision after       17        16   REFUSED
the same, decision taken between the two        17        16   REFUSED
```

In all three the **write succeeds**. The repository stops being readable at the moment it is written,
and the write returns `Ok`.

The first line is not a success. It reconstructs because a set cannot tell fifteen distinct entries
out of sixteen records from fifteen out of fifteen — the same blindness that refuses the second line.
Two of the three outcomes are the record correctly refusing a journal whose addresses are ambiguous,
and the third is that refusal being unable to fire.

## Neither refusal is a defect, and only one of them could explain itself

The replay is handed an address that occurs twice and refuses rather than guessing which occurrence
was meant, which is what this layer does everywhere else. What is new is that **exploration produces
that journal by accident** — by the route E4 itself predicts an application would take, revisiting
ground rather than enumerating fresh ground.

The two refusals were asymmetric, and the asymmetry was structural rather than editorial:

```text
EntryAlreadyPassed          names the repeated entry        true
WitnessedKnowledgeAbsent    names the entry learned in      false — the journal
                            between, and says the journal   offers it, at position
                            does not offer it               15 of 17
```

A replay can see the repetition; corroboration weighs two sets, the cause is a **multiplicity**, and a
set cannot hold one. So the guard that could explain itself did, and the guard that could not blamed
an innocent entry — and did it by asserting something false about the file, which sends a reader
looking for a truncated journal that is not there.

## The repair, and where it had to go

Not a rewording. The message was missing a fact, so the diagnosis moved to the place that has it:
`rebuild`, which holds the whole journal. On the failure path only, it replays into a canon of its own
and asks whether the address the decision names occurs more than once.

```text
the decision names entry <A>, which the journal admits more than once;
resolving it to the first occurrence leaves witnessed entry <B> unadmitted
```

`WitnessedKnowledgeAbsent` stays, because it is still the truth about a different fault — a witness
naming an entry no journal ever held. Two causes, two names, and the reader is sent to the right one.

Unwiring the diagnosis puts the old message back and turns the phase red naming
`WitnessedKnowledgeAbsent` where an ambiguous readmission is expected, so the repair is measured and
not assumed.

## Two guards stopped being hypotheses

`EntryAlreadyPassed` and `WitnessedKnowledgeAbsent` were exercised by no suite in this laboratory. By
this sequence's own standard they were hypotheses rather than guards, and these are the first
exercises of either. Both turn out to be reachable by the same journal, differing only in where the
decision falls.

## What an application needs, named and not built

One line, and it belongs to the application rather than to the record: **ask whether an address is
already held before readmitting it.** The repository cannot make that choice — it cannot tell an
ambiguous journal from a faithful log of what was supplied, because that is what an ambiguous journal
*is*.

Which restates the variable the protocol left open rather than resolving it. The trade is no longer
*faithful log versus smaller file*; it is faithful log versus addresses a decision can name.
