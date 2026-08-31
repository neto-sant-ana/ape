# Answers

Note on method: `record/` contains exactly one file, `record/record.md`. It holds three data
arms (`operations`, `finance`, `merged`) — journal, custody, decisions, entries and decisions in
full — followed by a closing section, "What was said about it", which is a list of somebody's
remarks. Per the instructions I treat that closing list as claims, not as the record's evidence:
several statements below are quoted verbatim from it, and I judged each against the three data
arms only. Nothing in the data arms mentions a program, a crate, `converge`, a Canon, Synthesis,
tests, file names (`journal.json`, `lineage.json`, `worlds.json`) or a "generation" — I checked by
grep over lines 1–1361 (everything before the remarks).

## 1
verdict: established
opened: record/record.md
because: Both journals list 20 entries and their first 19 are identical in id and in "recorded"
date, entry for entry; entry 20 is `4b8b9b88…` (2026-01-07) for operations and `652a011d…`
(2026-01-08) for finance, and each of those ids is absent from the other arm's journal, custody
and entries-in-full.

## 2
verdict: established
opened: record/record.md
because: `4b8b9b88…` appears only in operations' journal (position 20), recorded 2026-01-07;
its commitment body has accountable `fe0e80f6…` (the Agent labelled **house**), beneficiary
`0d3a24e8…` (**market**), magnitude 60, due 2026-01-20, and statement `1cb6093b…` whose action
`5f5255e4…` has verb "spend".

## 3
verdict: established
opened: record/record.md
because: `652a011d…` appears only in finance's journal (position 20), recorded 2026-01-08;
accountable `0d3a24e8…` (**market**), beneficiary house, magnitude 40, due 2026-01-12, resource
instance `8098643f…` labelled "account", statement `dabb76d3…` whose action `f08e6708…` has verb
"receive".

## 4
verdict: established
opened: record/record.md
because: Operations' advance `1f093bfa…` is known at 2026-01-07 and its fork `b9392895…` lists
`omitted: [2f54506a…]`, `introduced: [4b8b9b88…]`, `open: [4b8b9b88…]`; finance's advance
`558f991d…` is known at 2026-01-08 and its fork `dd201a84…` has `open: [2f54506a…, 652a011d…]`,
`introduced: [652a011d…]`, no `omitted`. The four world ids produced by those four decisions are
distinct, two per party (the shared genesis world `74a6a53e…` is not among them).

## 5
verdict: not established
opened: record/record.md
because: The `taken by` fields do carry `326993e9…` (operations) and `10807723…` (finance), and
journal entries 15–16 label those agents — but each arm's genesis decision reads "taken by:
nobody — the decision claims no party", so it is not true that each party's decisions name that
party; and nothing in the record describes a program deriving anything.

## 6
verdict: not established
opened: record/record.md
because: This is one of the closing remarks. The data arms contain no run, no error, no
`converge`, and no statement of what its comparison is over; the record shows only the two
journals, from which neither extending the other can be derived, but not that any operation was
invoked or measured.

## 7
verdict: not established
opened: record/record.md
because: The record contains no refusal message at all. That the two journals share 19 entries is
derivable, but the quoted wording, the named position, and "nothing was written" are nowhere in
the files.

## 8
verdict: not established
opened: record/record.md
because: A closing remark about how an `EntryId` is derived. The data show only that identically
bodied entries carry identical ids across arms; nothing states that ids are derived from what
admitting produced.

## 9
verdict: not established
opened: record/record.md
because: The first half is derivable (merged entries 1–20 are operations' 20 in the same order,
entry 21 is finance's `652a011d…`), but the record says nothing about a Canon, a watermark, or a
rule refusing admissions dated before it — so the stated reason for the order is unsupported.

## 10
verdict: not established
opened: record/record.md
because: A closing remark about running the program with roles swapped. No run, output or
rejection appears anywhere in the data arms.

## 11
verdict: not established
opened: record/record.md
because: A closing remark about the crate's types and `lineage::rebuild`. The record contains no
API, no `Taken`, and no statement of what rebuilding demands.

## 12
verdict: not established
opened: record/record.md
because: The first half is derivable — finance's two decisions witness 20 entries against a
journal lacking `4b8b9b88…`, and every merged prefix that reaches `652a011d…` is 21 long and
contains `4b8b9b88…`, so no merged position offers finance's prefix. But "the program prints which
entry each one trips on" has no support: the record contains no program output.

## 13
verdict: established
opened: record/record.md
because: Comparing finance's decisions-in-full with merged's, `558f991d…` and `dd201a84…` are
line-for-line identical (world, known at, event head, frozen, open, taken by, taken after entry,
extends, introduced) except that `witnessed entries` goes 20 → 21, which is exactly the merged
prefix ending at `652a011d…`.

## 14
verdict: established
opened: record/record.md
because: Merged holds exactly five decisions and each one matches, field for field, a decision in
operations' or finance's arm (the only differences are the arm label on the "held by" line and
finance's two witness counts, 20 → 21); no decision appears in merged that neither party took. The
lineage branches: `b9392895…` extends `1f093bfa…`, `dd201a84…` extends `558f991d…`, both rooted at
genesis `74a6a53e…` — two forks, two branches.

## 15
verdict: not established
opened: record/record.md
because: The merged arm's header does read "21 journal entries, 21 custody addresses, 5 decisions,
5 worlds", but the record never uses the label "generation `b`", and it says nothing about
reconstructing from disk or about what a reader told nothing would obtain.

## 16
verdict: established
opened: record/record.md
because: Operations claims worlds `74a6a53e…`, `1f093bfa…`, `b9392895…`; finance claims
`74a6a53e…`, `558f991d…`, `dd201a84…` — six claims, five distinct ids. All five appear in the
merged arm with identical known-at, event head, frozen, open and extends fields, so every claimed
identity is present unchanged. (What the record shows is identity, not that a recomputation was
run.)

## 17
verdict: not established
opened: record/record.md
because: A closing remark. No `worlds.json`, no comparison step and no refusal-to-write condition
appears anywhere in the data arms.

## 18
verdict: not established
opened: record/record.md
because: Half is checkable — both divergent entries are Commitments, and every decision in every
arm carries the same event head `6d336a99…`. But "an advance absorbs only what its cut froze" is a
rule about the machinery, and the causal claim that survival follows from it is nowhere in the
record.

## 19
verdict: not established
opened: record/record.md
because: A counterfactual about a record that does not exist. Nothing in the files says how a cut
resolves against an Event, so the consequence cannot be shown.

## 20
verdict: established
opened: record/record.md
because: The merged arm presents its journal under "Journal, in the order it was admitted" and
declares 21 entries; entries 1–20 are operations' 20 in the same order, and the full bodies of all
20 are byte-identical between the operations arm and the merged arm.

## 21
verdict: established
opened: record/record.md
because: Merged decision `558f991d…` is an advance, taken by `10807723…` (finance), known at
2026-01-08, extends world `74a6a53e…`, witnessed entries: 21.

## 22
verdict: established
opened: record/record.md
because: Merged decision `dd201a84…` is a fork, taken by finance, extends world `558f991d…`,
`introduced: [652a011d…]`, no `omitted` list (nothing dropped), witnessed entries: 21.

## 23
verdict: not established
opened: record/record.md
because: Nothing in the record asserts mutual consistency between journal, lineage and worlds, or
that they were built together from one read. The three arms present data; no attestation of that
kind exists in the files.

## 24
verdict: not established
opened: record/record.md
because: No artifact in the record states what finance's tip would come to inside operations' tip,
and nothing says any such reading is taken, stored or decided.

## 25
verdict: not established
opened: record/record.md
because: The ingredients are present — `dd201a84…` introduces `652a011d…`, that commitment is
recorded 2026-01-08, and operations' tip `b9392895…` is known at 2026-01-07 — but the record
contains no Synthesis, no question asked of it and no verdict "conflicted".

## 26
verdict: established
opened: record/record.md
because: Operations' fork omits `2f54506a…` while finance's fork keeps it open, so the two forks
disagree about it; and finance's fork introduces `652a011d…`, an entry that does not appear at all
in operations' journal or custody, whose latest decision is known at 2026-01-07 while
`652a011d…` is recorded 2026-01-08.

## 27
verdict: not established
opened: record/record.md
because: That operations omitted `2f54506a…` and finance kept it is derivable; the rest — that
reconciling would take exactly those two decisions, that neither party made them, and that the
answer is not derivable — is reasoning about operations the record does not describe.

## 28
verdict: established
opened: record/record.md
because: Comparing finance's arm with the merged arm field by field, every claim of finance's is
carried over identically (journal, custody, entry bodies, and all decision fields) with exactly
one exception: `witnessed entries` on `558f991d…` and `dd201a84…` reads 21 in merged where finance
recorded 20. So the original witnesses are the one thing not carried. (The evaluative half — "a
real loss rather than a technicality" — is an opinion the record cannot settle.)

## 29
verdict: established
opened: record/record.md
because: Finance's two decisions are taken after `652a011d…` witnessing 20 entries, and finance's
20-entry journal contains no `4b8b9b88…`. In the merged journal `652a011d…` is at position 21 and
every prefix reaching it contains `4b8b9b88…`, while the 20-entry prefix ends at `4b8b9b88…` — so
no merged position offers finance's prefix, and merged restates the witness as 21 rather than
approximating 20. (The record shows this as finance's decisions section; it names no file called
`lineage.json`.)

## 30
verdict: established
opened: record/record.md
because: Merged's journal has 21 entries and both retaken decisions are taken after `652a011d…`,
its entry 21, so "witnessed entries: 21" is true of the merged history; finance's own arm records
20 for the same two decisions, so 21 is not finance's claim.

## 31
verdict: not established
opened: record/record.md
because: A ranking of which part of the result is weakest, and a wish that it be named, are
judgements of the author. Nothing in the record grades its own fields.

## 32
verdict: established
opened: record/record.md
because: The merged `558f991d…` and `dd201a84…` carry `taken by: 10807723… (finance)`, exactly as
finance's own arm does, and the field set used by all 11 decisions in the record (produces world,
known at, event head, frozen, open, taken by, taken after entry, witnessed entries, extends,
omitted, introduced, selection) contains nothing that names a re-applier. Read on its own terms
the merged record cannot separate the two cases.

## 33
verdict: established
opened: record/record.md
because: The fork decisions carry `introduced` (and `omitted`) lists naming commitments, and no
decision anywhere in the record carries any field giving a reason, motive or rival line of
thinking — the enumerated field set above has no such slot.

## 34
verdict: established
opened: record/record.md
because: Both halves are visible in the result: the merged record attributes the two retaken
decisions to finance, keeping the provenance of the intention, and it states finance witnessed 21
entries where finance's own record says 20 — an overstatement of what finance witnessed. (The
deliberation behind "I chose" is not itself in the record; the attribution and its cost are.)

## 35
verdict: not established
opened: record/record.md
because: A closing remark about the crate's operations. The record contains no `converge`, no
Synthesis, no repository model and no inventory of what operations exist, so no claim about what
none of them takes as its subject can be shown.

## 36
verdict: not established
opened: record/record.md
because: Nothing in the record identifies which rules came from a crate, whether any policy was
supplied, or what the crate guards. The data arms carry no provenance for the rules at all.

## 37
verdict: not established
opened: record/record.md
because: The record does not distinguish author from re-applier (see 32), but this statement goes
further — that there is no way to record it, and that this gap turned a lookup into a judgement
call about the author's own process. Neither the capability claim about the format nor the
author's deliberation is in the files.

## 38
verdict: not established
opened: record/record.md
because: The record contains no code, no type definitions, no `ThesisId`, and no mention of
`FromStr`, `Display` or serde.

## 39
verdict: not established
opened: record/record.md
because: No workaround and no missing constructor appear in the record; there is nothing to
inspect.

## 40
verdict: not established
opened: record/record.md
because: The record contains no tests, no guard and no check that decides whether anything is
written.

## 41
verdict: not established
opened: record/record.md
because: No tampered copy, no `reading::corroborated`, and no attempt of any kind is recorded. The
files are the three arms and a remarks list.

## 42
verdict: not established
opened: record/record.md
because: This is a claim about what a hypothetical case would require. The record contains only
Commitments on the divergent side and says nothing about what an Event would do or what
hand-authoring would be needed.

## 43
verdict: not established
opened: record/record.md
because: No function named `unreproduced`, no unit tests and no red-run output exist in the
record.

## 44
verdict: not established
opened: record/record.md
because: There is no wiring, no suite and no test result in the record to be proven or unproven.

## 45
verdict: not established
opened: record/record.md
because: The record carries no filesystem metadata — no mtimes, no access log — so nothing in it
can show that any file was read only and never written.

## 46
verdict: not established
opened: record/record.md
because: The record says nothing about what was read, where, or about a scratchpad. A record
cannot attest to the reading behaviour of whoever produced it unless it says so, and this one does
not.

## totals
opened at least once: instructions.md, questions.md, record/record.md
answered established: 17
