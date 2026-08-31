# Observation 3 — The run, and three defects in the instrument that produced it

Three agents, one per carving, isolated by `chmod 000` over both `lab/` and the session scratchpad —
verified by attempting to read the protocol and being denied, not promised. Each was handed one
carving, the same 46 questions, and nothing else. None had read the protocol.

```text
                                established
baseline, four JSON files          19 / 17     this laboratory, and an independent classifier
A  flat, 1 page                     6
B  per entity, 16 pages             4
C  per decision, 7 pages            4
```

**Every carving landed far below the baseline**, and the protocol only anticipated the opposite
hazard — *a number inside that range is no result at all*. Most of the gap is my instrument, and this
observation is mostly about that.

## What is clean, and it is the experiment's actual answer

**Zero false positives, across three independent readers.** Every claim any agent established is one
the classification had called housed. Nobody confabulated, and three readers who never conferred
agreed with a classification none of them had seen.

```text
A   {2, 3, 4, 21, 22, 26}      all housed
B   {2, 3, 4, 26}              all housed
C   {4, 5, 21, 22}             all housed
```

**And zero of the twenty-seven non-housed claims were established by any carving.** Not one. B and C
each *placed* nine of them on pages — the anchor rule's whole yield — and the placement bought
**nothing**.

That is P1, P2 and P3 answered together, and answered no:

```text
P1  every carving beats flat                    REFUTED — flat scored highest, 6 against 4 and 4
P2  B wins on motivation, C on accountability    REFUTED — neither established any unhoused claim
P3  the eight specific ones are established
    by all three                                 REFUTED — by none
```

All three agents gave the same reason, independently and in their own words: the claim was on the
page, verbatim, and a remark is somebody's claim rather than the record's evidence for it. They went
looking for the evidence elsewhere and it was not there.

**This result is robust to the defects below**, and that is worth stating because the defects are
severe. The defects cost *housed* claims — record content the carvings failed to carry. The placed
claims were present, in full, on pages the agents demonstrably opened and quoted back.

## Three defects, all mine, found by the agents reading the artefact

**1. The generator carved half the record.** The protocol says the source is
`agents/05-reconciliation/run-a`. `SOURCE` points at `run-a/mine`. `run-a` holds **`mine` and
`theirs`** — the two parties' records — and four housed claims are about both of them:

```text
 1   "Both journals hold 20 entries and their first 19 are identical"
 7   "the two share 19 entries, so they are divergent rather than incompatible"
 9   "The merged journal is operations' 20 followed by finance's 1"
20   "Twenty of its entries are operations' own, unchanged"
```

Unreachable in every carving, because the other half of the record was never carved. Both A and C
said so unprompted — *this arm of the record holds only the merged result*. The narrowing is in
`record.rs`'s docstring, written down and never weighed: I recorded the choice and not its cost.

**2. `custody.json` is emitted nowhere.** Measured: the string does not occur in the generator. The
carvings carry three of the record's four files. Claim 15 — *21 journal entries, 21 custody
addresses, 5 decisions and 5 worlds* — is housed by custody and is unreachable in all three.

**3. C is missing five of the twenty-one entries.** Measured: A and B render 21, C renders 16, and no
page of C holds a Commitment or an Event. C's vocabulary admits an entry when it has no page of its
own *or* is an Agent; Commitment and Event have pages in B, so they fall out of both arms of that
condition and get none in C.

```text
a-flat            21 of 21 entries
b-per-entity      21 of 21
c-per-decision    16 of 21      4 commitments and 1 event, absent
```

C's agent named it exactly: *no amounts, due dates or parties for `2f54506a…`, `4b8b9b88…`,
`652a011d…`*. **This is not a carving choice, it is content loss**, and it breaks the protocol's
premise that only the carving differs. It shows in the results — C is the only carving that failed
claims 2 and 3, which are commitment bodies.

### And the guard that existed for this did not catch it

`the_same_twenty_seven_claims_appear_in_all_three_carvings` checks the **claims**. Nothing checked
that the **record** reaches all three. I guarded the half I wrote and not the half the experiment is
about — which is the same shape as an assertion that reads the wrong source.

## One limit that is not a defect, and is a finding

Claims 24 and 25 are housed by a **derived** carrier — Applicability, which is Synthesis being asked
and answering. No static rendering of any carving can produce a computation, so those two are
unreachable by construction rather than by omission.

That is a real property of carving-as-hypertext and it belongs in the result: **a record whose
answers are derived cannot be fully carved into pages.** The pages can hold the inputs; the answer
exists only when something runs.

## The one place a carving difference is visible through the noise

C established claim 5 — *each party's decisions name that party in `by`* — and A, holding the whole
record, did not. C's reason is structural: a decision page carries `by` in its frontmatter and
`vocabulary.md` carries the agent labels, so the cross-reference is two files. In A the same two
facts are four hundred lines apart in one document.

One claim is not a result. It is recorded because it is the only thing in this run that looks like
what the experiment was built to see, and because P4 — cost — is the prediction it belongs to.

## What this run cannot say

**Nothing about P4.** Cost was to be *bytes opened and pages traversed per question answered*, and
with 4 to 6 answers out of 46 there is no denominator worth dividing by. The three agents reported
what they opened and all three opened everything.

**Nothing about P5**, which needs the human reading, and which was always n = 1.

**And nothing about the housed-claim comparison**, which is what the baseline was for. Twelve of the
nineteen were unreachable for reasons that are about the generator and not about the carving.
