# Observation 2 — The carvings reach the same claims and repeat them differently

Three carvings, one generator, one record. Nothing has been asked of an agent yet; what follows is
what the generator produced and what the guards hold it to.

```text
carving           pages     bytes   placed overflowed
a-flat                1     17579       27          0
b-per-entity         16     17727        9         18
c-per-decision        7     18044        9         18
```

## B and C reach the same nine, and that is not the difference between them

The anchor rule reaches what it reaches, so both carvings place the same nine claims and overflow the
same eighteen. **Where they differ is distribution**, and the numbers are stark:

```text
B   3 pages carry claims     agent-finance 8, agent-operations 3, commitment-2f54506a 1
    12 placements

C   5 pages carry claims     8, 8, 3, 3, 1
    23 placements
```

C writes the same nine claims **twenty-three times**, because a party's claims go on every decision
that party took. B concentrates them on one page per party.

That is the trade the two units make, stated in the record's own arithmetic rather than argued: a
reader of C who lands on any decision by finance has finance's eight claims already; a reader of B has
to know that a claim about a decision may live on an agent. C pays for it with redundancy, and a
redundancy the generator cannot avoid — the same claim genuinely is about all four of the decisions
that party took.

**P4 is a prediction about cost, and none of this is a cost measurement.** Bytes on disk is not
*bytes opened per question answered*, and the three totals here sit within 3% of each other. What the
page counts give P4 is its premise — 16 against 7 — and nothing more.

## The four judgements the protocol left to the generator

Recorded where they were made, because a generator is where a preference hides as a design decision.

1. **What is not a page in B.** The protocol names four page kinds; the record holds nine. Roles,
   resources, instances, actions, statements and eligibilities — twelve of the twenty-one entries —
   are vocabulary: they introduce names and carry no reasoning. One `vocabulary.md`.
2. **C gets the same page.** It would be cheating C's cost to let it drop what B has to carry. So the
   asymmetry is 16 against 7, not 16 against 5.
3. **Where a claim goes.** B: the page of the thing named. C: every decision that *reaches* the thing
   named — by being it, by naming it in a selection, or by having been taken by it. That third rule
   is what makes C's reach comparable rather than smaller by construction.
4. **A has no overflow by construction**, and the guard says so as a property of the carving rather
   than of the placement. A reader comparing the three would otherwise read *A placed everything* as A
   having done something.

## Three defects the guards found, and one of them was in the record's shape

**`decides` is not an identity.** `lineage.json` tags the variant, so the value is `genesis`,
`advance` or `fork` — and a decision's world is nowhere in that file. It is derived, and the record
holds it in `worlds.json` at the same position. Before this was noticed, C's pages were named
`decision-2-fork` (three of which would collide in a record with three forks) and the branch matching
a claim against a decision's world was **comparing an identity to the string `fork`** — dead.

Fixing it changed no number here, because no unhoused claim in this testimony names a thesis. It is
recorded anyway: a branch that cannot fire is not the same as a branch that fires and finds nothing,
and the next record is not promised to be so convenient.

**B linked to pages B does not have.** Agent pages related to `decision-*`, which exists only in C.
Caught by the link guard on its first run — and it is the guard that matters most here, because a
carving whose links leave it is not a worse carving, it is a **different artefact**, and every number
measured against it would be measured against something this experiment did not build.

**And the relation format was ambiguous to the guard that reads it.** `[[[a]], [[b]]]` — a YAML flow
list of wikilinks — is unreadable to a person and parses wrong for anything scanning `[[`. Now a
comma-separated list, and `none` rather than `[]` for an empty relation.

An empty relation was itself a small defect: every agent page carried `held_by: []`, because an Agent
is in no world's selection. That tells a reader agents relate to nothing, which the record does not
claim. Agent pages now carry `decided`, which is a relation the record does have.

## The red pass

```text
a link that leaves the carving      B's agents pointed at `decision-*`; the guard names the page,
                                    the key and the target
a claim dropped from A              the verbatim guard names the carving and the first sixty
                                    characters of what is missing
```

The link guard also carries a floor — *the scan read N links in B* — because a scan that found none
would pass it without measuring anything.

## What is not measured

Everything the protocol asks agents for. Three agents, one question set, what each opened. The
carvings are on disk and committed, so what an agent is given is what this suite measured — which a
guard checks rather than assumes.
