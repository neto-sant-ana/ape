# Answers

Method note, so the `opened:` lines are honest: I read `instructions.md`, `questions.md` and all
16 files in `record/` in one sweep before answering anything. So every `opened:` line below names
files **already read** at that point; I name the ones that actually bear on the statement rather
than repeating the whole list. The full list is in `## totals`.

A second note on how I judged: `overflow.md` and the "What was said about it" sections of the
agent/commitment pages carry many of these statements *verbatim*, tagged `(methodlimit)`,
`(exposition)`, `(want)`, `(qualification)`, `(loss)`, `(roadnottaken)`. Per the instructions those
are somebody's remarks, not the record's evidence, so I looked for the evidence elsewhere in the
files and, where there is none, answered `not established` — naming where the remark sits so the
reader can see I did not miss it.

## 1
verdict: not established
opened: all of `record/` (already read); nothing in it is a journal
because: The record holds no journal file and no entry counts at all — there is no page that lists
entries in order, per party or otherwise. Nothing here can show "20 entries", "first 19 identical",
or a twentieth unseen by the other.

## 2
verdict: established
opened: record/commitment-4b8b9b88.md, record/vocabulary.md, record/agent-house.md, record/agent-market.md, record/thesis-b9392895.md, record/agent-operations.md
because: `commitment-4b8b9b88` is accountable `fe0e80f6…` (= house, `agent-house.md`), beneficiary
`0d3a24e8…` (= market), statement `1cb6093b…` whose action is `5f5255e4…` = verb "spend"
(`vocabulary.md`), magnitude 60, due 2026-01-20, recorded 2026-01-07; it is `held_by`
`thesis-b9392895`, which `agent-operations.md` lists under `decided:`. Every cell of the row is
derived from those pages.

## 3
verdict: established
opened: record/commitment-652a011d.md, record/vocabulary.md, record/agent-market.md, record/thesis-dd201a84.md, record/agent-finance.md
because: `commitment-652a011d` is accountable `0d3a24e8…` (= market), statement `dabb76d3…` whose
action is `f08e6708…` = verb "receive", resource instance `8098643f…` = label "account"
(`vocabulary.md`), magnitude 40, due 2026-01-12, recorded 2026-01-08; it is `held_by`
`thesis-dd201a84`, which `agent-finance.md` lists under `decided:`.

## 4
verdict: established
opened: record/thesis-74a6a53e.md, record/thesis-1f093bfa.md, record/thesis-b9392895.md, record/thesis-558f991d.md, record/thesis-dd201a84.md, record/agent-operations.md, record/agent-finance.md
because: Operations decided `1f093bfa` (parent `74a6a53e`, known at 2026-01-07, open `2f54506a`) and
`b9392895` (parent `1f093bfa`, known 01-07, holding `3167ccd3` + `4b8b9b88`, `2f54506a` gone);
finance decided `558f991d` (parent `74a6a53e`, known at 2026-01-08) and `dd201a84` (parent
`558f991d`, open `2f54506a` **and** `652a011d`). That is the advance-then-fork shape described, four
distinct identities, two under each agent's `decided:` list, disjoint.

## 5
verdict: not established
opened: record/agent-operations.md, record/agent-finance.md, record/thesis-*.md (all five)
because: The identity halves are derivable — `326993e9…` is labelled operations and `10807723…`
finance — but no page in the record shows a `by` field on a decision (attribution here runs the other
way, via the agents' `decided:` lists), and there is no program, journal or output in the record to
show the mapping was *derived from the journal rather than assumed*.

## 6
verdict: not established
opened: record/overflow.md
because: This sits in `overflow.md` verbatim as a `(methodlimit)` remark — the author's claim about
running `converge`. The record contains no code, no invocation and no output, so nothing here shows
either the refusal or that it was measured.

## 7
verdict: not established
opened: record/overflow.md, all of `record/` (already read)
because: Nothing in the record mentions position 19, a count of shared entries, or the wording of any
refusal — it does not even appear as a remark. There is no journal to have 19 positions.

## 8
verdict: not established
opened: record/agent-finance.md
because: Appears only as an `(exposition)` remark on `agent-finance.md`. Nothing in the record defines
`EntryId`, shows how an id is derived, or exhibits the two records being compared.

## 9
verdict: not established
opened: all of `record/` (already read)
because: No journal, no entry ordering, no watermark rule and no Canon behaviour appears anywhere in
the record — not even as a remark. Ordering of admissions cannot be reconstructed from the
per-entity pages, which carry only `recorded at` dates.

## 10
verdict: not established
opened: record/agent-operations.md
because: Present only as a `(methodlimit)` remark under "What was said about it" on
`agent-operations.md`. There is no run, log or output in the record to show the swapped-roles attempt
failed, or where.

## 11
verdict: not established
opened: record/overflow.md
because: An `(exposition)` remark in `overflow.md`. The record contains no `Taken`, no prefix data and
no `lineage::rebuild` — the thesis pages record parents and holdings, never the prefix a decision
stood on.

## 12
verdict: not established
opened: all of `record/` (already read)
because: The record has no prefixes and no positions of entries, so it cannot show what finance's two
decisions claimed to stand on, nor that no position offers it. It does not appear even as a remark.

## 13
verdict: not established
opened: all of `record/` (already read)
because: Nothing in the record speaks of a `Decision` being carried over or re-witnessed; there are no
witnesses recorded on any page. The statement appears nowhere in the files, as remark or otherwise.

## 14
verdict: not established
opened: record/thesis-*.md (all five), record/agent-operations.md, record/agent-finance.md
because: The tree shape is derivable (`74a6a53e` → `1f093bfa` → `b9392895` and `74a6a53e` →
`558f991d` → `dd201a84`), but the claim fails on its own terms: of the five theses only four appear
in an agent's `decided:` list — `74a6a53e` is attributed to nobody — so "every one of the five
decisions … is verbatim one the two parties took" cannot be shown. Nothing shows that nothing was
arbitrated.

## 15
verdict: not established
opened: record/vocabulary.md, record/agent-*.md (all four), record/commitment-*.md (all four), record/event-6d336a99.md, record/thesis-*.md (all five)
because: The counts do check out — 12 vocabulary entries + 4 agents + 4 commitments + 1 event = 21
entities, each with one hash identity, and 5 theses. But nothing in the record names a "generation
`b`", and nothing shows the record reconstructs from disk for an uninformed reader; that is a claim
about a program run, and no run is in the files.

## 16
verdict: not established
opened: record/thesis-*.md (all five), record/agent-operations.md, record/agent-finance.md
because: The record holds five distinct thesis identities (six if `74a6a53e` is counted once per
party), but "come back identically" is a claim about re-deriving them in a rerun. There are no two
source records here to compare against and no output of any comparison.

## 17
verdict: not established
opened: record/overflow.md
because: A `(methodlimit)` remark in `overflow.md`. No `worlds.json`, no comparison and no refusal
behaviour exists anywhere in the record.

## 18
verdict: not established
opened: record/overflow.md, record/commitment-4b8b9b88.md, record/commitment-652a011d.md, record/thesis-*.md (all five)
because: Two ingredients are derivable — the divergent entries `4b8b9b88` and `652a011d` are indeed
Commitments, and all five theses carry the same event head `6d336a99…`. But "it survives" has no
referent in the record (nothing here is at risk or survives anything), and the rule that an advance
absorbs only what its cut froze is asserted in the `(exposition)` remark, not shown.

## 19
verdict: not established
opened: record/agent-finance.md
because: A `(qualification)` remark on `agent-finance.md`, and a counterfactual besides — the record
contains only what happened, so nothing in it can settle how a cut would have resolved had the
divergent entry been an Event.

## 20
verdict: not established
opened: all of `record/` (already read)
because: There is no journal page for this to be said of, no ordering of what entered, and no way to
attribute twenty entries to operations — the per-entity pages carry no owning party for vocabulary,
agents, commitments or the event.

## 21
verdict: not established
opened: record/agent-finance.md, record/thesis-558f991d.md, record/thesis-74a6a53e.md
because: The first clause is derivable — finance's `decided:` list holds `558f991d`, whose parent is
`74a6a53e` and whose `known at` is 2026-01-08. The second is not: the record nowhere states the
prefix a decision stood on, so "these 21 entries stood" when it was applied cannot be shown (the
record does not order its 21 entities against the five decisions at all).

## 22
verdict: not established
opened: record/agent-finance.md, record/thesis-dd201a84.md, record/thesis-558f991d.md, record/commitment-652a011d.md
because: Same split as 21. "Nothing dropped, `652a011d…` added under `558f991d…`" is derivable —
`dd201a84`'s parent is `558f991d` and it holds its parent's `3167ccd3` + `2f54506a` plus `652a011d`.
But the record carries no prefix per decision, so "these 21 entries stood" is unsupported.

## 23
verdict: not established
opened: record/overflow.md
because: An `(exposition)` remark in `overflow.md`, and it describes a file (a journal asserting joint
consistency) that is not in the record. Nothing here shows the parts were built together from one
read.

## 24
verdict: not established
opened: all of `record/` (already read)
because: The record contains no synthesis, projection or read-only derivation of one tip inside
another — the statement appears nowhere in the files, and there is no artefact for "it is read, not
stored" to be true of.

## 25
verdict: not established
opened: record/thesis-dd201a84.md, record/thesis-558f991d.md, record/thesis-74a6a53e.md, record/thesis-b9392895.md, record/commitment-652a011d.md
because: The supporting facts are derivable (finance's tip introduces `652a011d…` over the shared base
`74a6a53e`; that commitment is recorded 2026-01-08; operations' tip `b9392895` is known at
2026-01-07). What is not in the record is that Synthesis was run at all or that it returned
**conflicted** — no operation, verdict or output appears in any file.

## 26
verdict: established
opened: record/thesis-b9392895.md, record/thesis-dd201a84.md, record/thesis-1f093bfa.md, record/commitment-652a011d.md, record/commitment-2f54506a.md
because: Operations' tip `b9392895` drops `2f54506a…` (its parent `1f093bfa` holds it, it does not)
while finance's tip `dd201a84` keeps it open — that is the disagreement about the dropped commitment.
And `dd201a84` holds `652a011d…`, whose `recorded at` is 2026-01-08, while `b9392895` is `known at`
2026-01-07 — so finance's intention names an entry dated after the horizon operations' world
recognises. Both halves come off the pages themselves, not off a remark.

## 27
verdict: not established
opened: record/agent-operations.md, record/agent-finance.md, record/commitment-2f54506a.md
because: A `(roadnottaken)` remark repeated on three pages. One fact inside it is derivable
(`2f54506a…` is held by finance's `dd201a84` and not by operations' `b9392895`), but that two
decisions would be needed, that neither party made them, and that the answer is not derivable, are
the author's claims with no supporting artefact in the record.

## 28
verdict: not established
opened: record/agent-finance.md
because: A `(loss)` remark on `agent-finance.md`. The record contains no witnesses on any page —
finance's original or otherwise — so there is nothing here against which a preserved-or-lost claim
could be checked.

## 29
verdict: not established
opened: all of `record/` (already read)
because: There is no `lineage.json` in the record, and no prefixes at any position, so neither what
finance's lineage asserted nor the absence of a matching prefix can be shown.

## 30
verdict: not established
opened: record/agent-finance.md
because: A `(qualification)` remark on `agent-finance.md`. No witnesses appear on any record page, so
nothing here shows what the witnesses on those two records now say, or whose claim they are.

## 31
verdict: not established
opened: record/overflow.md
because: A `(qualification)` remark in `overflow.md`. No `by` field appears anywhere in the record and
no decision here is marked as retaken; "the weakest thing in the result" is in any case a judgement,
not a fact the files could settle.

## 32
verdict: not established
opened: record/agent-operations.md, record/agent-finance.md, record/thesis-*.md (all five)
because: A `(want)` remark on both agent pages. The thesis pages do carry no authorship field and no
re-application marker, but that shows what these 16 files happen to contain, not that *the record*
(the format) has no way to express the distinction — and neither a retake nor finance's own prefix
appears here to be distinguished in the first place.

## 33
verdict: not established
opened: record/overflow.md, record/thesis-*.md (all five)
because: An `(exposition)` remark in `overflow.md`, and a fragment whose referent ("another line of
thinking") is nowhere in the record. The thesis pages do show which commitments each decision
introduced, but nothing in the files establishes the negative half about reasons.

## 34
verdict: not established
opened: record/agent-finance.md
because: A `(roadnottaken)` remark on `agent-finance.md` describing a choice the author made. The
record shows no attribution of anything to a retake, and no witness data, so neither the choice nor
its cost can be checked here.

## 35
verdict: not established
opened: record/overflow.md
because: A `(want)` remark in `overflow.md`. The record contains no crate, no API surface and no
operations — nothing here can show what `converge` requires or what Synthesis takes as its subject.

## 36
verdict: not established
opened: record/overflow.md
because: A `(qualification)` remark in `overflow.md`. There is no crate, no policy and no composition
in the record to check the claim against.

## 37
verdict: not established
opened: record/overflow.md, record/agent-operations.md, record/agent-finance.md
because: A `(want)` remark in `overflow.md` (the same gap the agent pages state). It is a claim about
what the system cannot record; the files contain no schema or API to demonstrate the absence, and no
re-applied decision.

## 38
verdict: not established
opened: record/overflow.md
because: A `(want)` remark in `overflow.md`. The record contains no source code or type definitions —
ids appear only as hex strings on the pages — so nothing here shows what `ThesisId` does or does not
implement.

## 39
verdict: not established
opened: record/overflow.md
because: A `(methodlimit)` remark in `overflow.md`, and it depends on the workaround and missing
constructor of 38, neither of which appears in the record.

## 40
verdict: not established
opened: record/overflow.md
because: A `(methodlimit)` remark in `overflow.md`. No test, guard or check exists anywhere in the
record, so nothing here can show one could not be made to refuse.

## 41
verdict: not established
opened: record/agent-finance.md
because: A `(methodlimit)` remark on `agent-finance.md`. There is no `reading::corroborated`, no
tampered copy and no run output in the record — only the author's account of the attempt.

## 42
verdict: not established
opened: record/overflow.md
because: A `(roadnottaken)` remark in `overflow.md`. It is a claim about what constructing a different
case would require; the record holds one event and no such case, and cannot settle what a merge would
do to a world.

## 43
verdict: not established
opened: record/overflow.md
because: A `(methodlimit)` remark in `overflow.md`. No function named `unreproduced`, no unit tests and
no red-run output are in the record.

## 44
verdict: not established
opened: record/overflow.md
because: A `(qualification)` remark in `overflow.md` about a test suite that is not in the record —
there is no wiring, suite or result here to be proven or unproven.

## 45
verdict: not established
opened: record/overflow.md
because: A `(methodlimit)` remark in `overflow.md`. The record carries no filesystem metadata and no
before/after mtimes of anything, so it cannot show that the source records were only read.

## 46
verdict: not established
opened: record/overflow.md
because: A `(methodlimit)` remark in `overflow.md` about the author's own conduct. The record contains
no log of what was read, so nothing in it could establish the negative.

## totals
opened at least once: instructions.md, questions.md, record/vocabulary.md, record/overflow.md,
record/agent-house.md, record/agent-market.md, record/agent-operations.md, record/agent-finance.md,
record/commitment-2f54506a.md, record/commitment-3167ccd3.md, record/commitment-4b8b9b88.md,
record/commitment-652a011d.md, record/event-6d336a99.md, record/thesis-74a6a53e.md,
record/thesis-1f093bfa.md, record/thesis-b9392895.md, record/thesis-558f991d.md,
record/thesis-dd201a84.md
(That is every file present: all 16 in `record/`, plus the two at the top level. Two of them —
`record/agent-house.md` and `record/agent-market.md` — were opened and turned out to matter only for
resolving the labels behind `fe0e80f6…` and `0d3a24e8…`; `record/commitment-3167ccd3.md` and
`record/event-6d336a99.md` bore on nothing that was asked.)
answered established: 4
