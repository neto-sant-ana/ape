# Answers

Convention used throughout: a line under a page's "What was said about it", or a bullet in
`overflow.md`, is somebody's remark. It is treated as evidence only where the structured parts of
the record (arm listings, decision fields, `vocabulary.md`) independently show the same thing.

## 1
verdict: established
opened: record/arm-operations.md, record/arm-finance.md, record/vocabulary.md
because: Both arms declare 20 entries and their entries 1–19 are byte-identical in hash and in
"recorded" date (diffed). Entry 20 differs — operations `4b8b9b88…` (2026-01-07), finance
`652a011d…` (2026-01-08) — and `vocabulary.md` scopes `4b8b9b88` to "operations, merged" and
`652a011d` to "finance, merged", so neither party's twentieth is in the other's journal.

## 2
verdict: established
opened: record/vocabulary.md, record/arm-operations.md
because: `vocabulary.md` gives `4b8b9b88` as in operations/merged, accountable `fe0e80f6` (agent
"house"), statement `1cb6093b` whose action is `5f5255e4` "spend", beneficiary `0d3a24e8` ("market"),
magnitude 60, due 2026-01-20, recorded 2026-01-07 — exactly the row.

## 3
verdict: established
opened: record/vocabulary.md, record/arm-finance.md
because: `652a011d` is in finance/merged, accountable `0d3a24e8` ("market"), statement `dabb76d3`
whose action is `f08e6708` "receive", resource instance `8098643f` labelled "account", magnitude 40,
due 2026-01-12, recorded 2026-01-08.

## 4
verdict: established
opened: record/decision-1f093bfa.md, record/decision-b9392895.md, record/decision-558f991d.md, record/decision-dd201a84.md, record/vocabulary.md
because: `1f093bfa` (advance, known 2026-01-07) and `b9392895` (fork, same day, `omitted:
["2f54506a…"]`, `introduced: ["4b8b9b88…"]`, `open: ["4b8b9b88…"]`) are both `by` `326993e9`
(operations); `558f991d` (advance, 2026-01-08) and `dd201a84` (fork, `introduced: ["652a011d…"]`,
`open: ["2f54506a…","652a011d…"]`) are both `by` `10807723` (finance). Four distinct world ids, two
per party, none shared.

## 5
verdict: established
opened: record/decision-1f093bfa.md, record/decision-b9392895.md, record/decision-558f991d.md, record/decision-dd201a84.md, record/vocabulary.md, record/arm-operations.md
because: The four non-genesis decisions carry `by` hashes `326993e9…` and `10807723…`, and those two
hashes are themselves journal entries (positions 15 and 16 in every arm), admitted as Agents
labelled **operations** and **finance** — so the party name is read off the journal, not supplied
from outside it.

## 6
verdict: not established
opened: record/overflow.md
because: Only an `overflow.md` methodlimit remark. Nothing in the files shows a `converge` operation,
its comparison rule, or that a refusal was observed.

## 7
verdict: not established
opened: record/overflow.md, record/arm-operations.md, record/arm-finance.md
because: No refusal message appears anywhere in the record, not even as a remark. The files do let
you derive that the two journals share 19 entries, but the wording, the "position 19" naming and
"nothing was written" are not in them.

## 8
verdict: not established
opened: record/decision-558f991d.md, record/decision-dd201a84.md, record/vocabulary.md
because: An exposition remark on two decision pages. The record does show `652a011d` carrying the
same id in finance and in merged, but how an `EntryId` is derived — and therefore that sameness of id
means sameness of knowledge — is a fact about the crate that the files do not contain.

## 9
verdict: not established
opened: record/arm-merged.md, record/arm-operations.md, record/arm-finance.md
because: The first half is derivable (merged entries 1–20 are operations' 20 in order, entry 21 is
finance's `652a011d`). The reason given — that the Canon refuses an admission dated before its
watermark, so the order was forced rather than chosen — is a rule about the software that appears
nowhere in the files.

## 10
verdict: not established
opened: record/decision-1f093bfa.md, record/decision-b9392895.md
because: A methodlimit remark repeated on two decision pages. No run, no rejection and no
role-swapped attempt is recorded anywhere in the files.

## 11
verdict: not established
opened: record/overflow.md
because: An exposition remark about the crate's `Taken` type and `lineage::rebuild`. Neither the type
nor the function is otherwise present in the record.

## 12
verdict: not established
opened: record/arm-finance.md, record/arm-merged.md, record/overflow.md
because: The first half is derivable — finance's two decisions stand after `652a011d` on 20 entries,
and in the merged journal `652a011d` sits at position 21 behind `4b8b9b88`, so no merged prefix
matches. But "the program prints which entry each one trips on" is a claim about program output that
the files contain no trace of.

## 13
verdict: established
opened: record/arm-finance.md, record/arm-merged.md, record/decision-558f991d.md, record/decision-dd201a84.md
because: `558f991d` and `dd201a84` appear in both arms with the same world id, kind, `by`, and
"after" entry — the decision pages are single pages held by `arm-finance` and `arm-merged` alike —
while the witness count is 20 in finance and 21 in merged. Same decision, standing on the merged
prefix.

## 14
verdict: established
opened: record/arm-operations.md, record/arm-finance.md, record/arm-merged.md, record/decision-74a6a53e.md, record/decision-b9392895.md, record/decision-dd201a84.md
because: The merged arm's five decisions are exactly the union of operations' three and finance's
three (genesis `74a6a53e` shared), with no id, kind or "after" entry that does not already appear in
one of the party arms. The `extends` fields make a tree: both advances extend `74a6a53e`, `b9392895`
extends `1f093bfa`, `dd201a84` extends `558f991d` — two branches kept apart, nothing arbitrated
between them.

## 15
verdict: not established
opened: record/arm-merged.md
because: The counts are right for the arm named "merged" (frontmatter: entries 21, custody 21,
decisions 5, worlds 5), but nothing in the files names a "generation `b`", and no reconstruction
from disk for an uninformed reader is recorded.

## 16
verdict: established
opened: record/arm-operations.md, record/arm-finance.md, record/arm-merged.md
because: Operations claims worlds `74a6a53e`, `1f093bfa`, `b9392895`; finance claims `74a6a53e`,
`558f991d`, `dd201a84` — six claimed identities, five distinct. All five appear in the merged arm's
decision list with identical hashes, including both of finance's.

## 17
verdict: not established
opened: record/overflow.md
because: A methodlimit remark. No `worlds.json`, no comparison and no refusal-to-write appears
anywhere else in the files.

## 18
verdict: not established
opened: record/overflow.md, record/vocabulary.md, record/decision-1f093bfa.md
because: `vocabulary.md` does show both divergent entries are Commitments and every decision carries
the same event head `6d336a99`, but "Commitments move no Event head" and "an advance absorbs only
what its cut froze" are rules of the engine that the files do not state, and the referent of "it
survives" is not given.

## 19
verdict: not established
opened: record/decision-558f991d.md, record/decision-dd201a84.md
because: A counterfactual qualification remark. The record contains no Event on either divergent
side and nothing that would let a reader work out how a cut would have resolved instead.

## 20
verdict: established
opened: record/arm-merged.md, record/arm-operations.md
because: The merged arm presents its list under "Journal, in the order it was admitted" and declares
21 entries with a custody section covering "every address this journal comes to"; its entries 1–20
are operations' 20, in the same order with the same dates (diffed).

## 21
verdict: established
opened: record/decision-558f991d.md, record/arm-merged.md, record/vocabulary.md
because: `558f991d` is an advance, `by` `10807723` (agent labelled finance), known at 2026-01-08,
`extends: 74a6a53e…`; in the merged arm that decision is listed as witnessing 21 entries.

## 22
verdict: established
opened: record/decision-dd201a84.md, record/decision-b9392895.md, record/arm-merged.md
because: `dd201a84` is a fork by finance extending `558f991d`, with `introduced:
["652a011d…"]` and no `omitted` list at all — `b9392895` shows what an omission looks like when there
is one — and the merged arm lists it witnessing 21 entries.

## 23
verdict: not established
opened: record/overflow.md
because: An exposition remark with no named subject. Nothing in the files records how the parts were
built, or that they came from one read.

## 24
verdict: not established
opened: record/overflow.md, record/arm-merged.md
because: No synthesis artifact exists in the record — there is no page, field or file holding what
finance's tip would come to inside operations', nor anything showing it is read rather than stored.

## 25
verdict: not established
opened: record/decision-dd201a84.md, record/decision-b9392895.md, record/vocabulary.md
because: The ingredients are there (`dd201a84` introduces `652a011d`; `652a011d` recorded 2026-01-08;
operations' tip `b9392895` known at 2026-01-07), but no synthesis question, answer or "conflicted"
verdict is recorded anywhere in the files.

## 26
verdict: established
opened: record/decision-b9392895.md, record/decision-dd201a84.md, record/vocabulary.md
because: `b9392895` omits `2f54506a` while `dd201a84` still carries it open — the disagreement — and
`dd201a84` introduces `652a011d`, whose recorded date (2026-01-08, `vocabulary.md`) is later than the
"known at: 2026-01-07" of both operations worlds, so finance's intention names an entry operations'
worlds do not reach.

## 27
verdict: not established
opened: record/decision-74a6a53e.md, record/decision-1f093bfa.md, record/decision-b9392895.md
because: A roadnottaken remark on several pages. That operations omitted `2f54506a` and finance kept
it is derivable, but what reconciling *would* take, and that no answer is derivable, is the author's
reasoning about decisions the record does not contain.

## 28
verdict: established
opened: record/arm-finance.md, record/arm-merged.md
because: Finance's arm lists decisions 2 and 3 witnessing 20 entries; the merged arm lists the same
two worlds witnessing 21. Comparing the two arms field by field, that witness count is the only
finance claim the merged record does not carry over — entries, custody addresses, world ids, kinds
and "after" entries all reappear unchanged. ("A real loss rather than a technicality" is the author's
judgement, not something the files settle.)

## 29
verdict: established
opened: record/arm-finance.md, record/arm-merged.md
because: Finance's decisions stand after `652a011d` on a 20-entry prefix that has no `4b8b9b88` in
it; in the merged journal `4b8b9b88` is position 20 and `652a011d` position 21, so no merged prefix
ends at `652a011d` without operations' commitment. The merged arm records those decisions at 21
witnessed entries — the true merged prefix — rather than a restated 20. (The artifact name
`lineage.json` does not appear in the files; its content does, as the arms' decision sections.)

## 30
verdict: established
opened: record/arm-merged.md, record/arm-finance.md
because: In the merged journal `652a011d` is admitted at position 21, so "witnessing 21 entries" is a
true statement about the merged history; finance's own arm claims 20 for the same two worlds, so the
witnesses now carried are not the ones finance asserted.

## 31
verdict: not established
opened: record/overflow.md, record/decision-1f093bfa.md
because: A qualification remark and pure evaluation — "the weakest thing in the result" is not a fact
about the record that any file could settle.

## 32
verdict: not established
opened: record/decision-558f991d.md, record/decision-dd201a84.md, record/overflow.md
because: A "want" remark. The five decision pages happen to show no re-application field, but showing
the record has *no way* to express one needs the format or crate schema, which the files do not
contain.

## 33
verdict: not established
opened: record/overflow.md, record/decision-b9392895.md, record/decision-dd201a84.md
because: The first half is visible (`introduced` lists on `b9392895` and `dd201a84`); the "never"
half is a claim about what the record format can express, which these rendered pages cannot settle —
and the pages do in fact carry reasoning, as remarks.

## 34
verdict: established
opened: record/decision-558f991d.md, record/decision-dd201a84.md, record/arm-merged.md, record/arm-finance.md
because: Both consequences are visible: the merged record keeps `by: 10807723…` (finance) on the two
retaken decisions, preserving whose intention it was, and it records them as witnessing 21 entries
where finance itself claimed 20 — the overstatement. Only the motive ("I chose") is the author's.

## 35
verdict: not established
opened: record/overflow.md
because: A "want" remark about the crate's API surface — `converge`, its extension requirement, and
what Synthesis takes as subject appear nowhere else in the files.

## 36
verdict: not established
opened: record/overflow.md
because: A qualification remark. The files contain no crate rules, no policy inventory and nothing
showing what the crate guards.

## 37
verdict: not established
opened: record/overflow.md
because: A "want" remark, and its "described above" points at another remark. Nothing in the files
demonstrates the absence of such a facility.

## 38
verdict: not established
opened: record/overflow.md
because: A "want" remark about Rust trait implementations on id types. The files contain hex ids but
nothing about `FromStr`, `Display` or serde.

## 39
verdict: not established
opened: record/overflow.md
because: A methodlimit remark; the workaround and the missing constructor it refers to appear nowhere
in the files.

## 40
verdict: not established
opened: record/overflow.md
because: A methodlimit remark about a test that could not be built. No tests, guard or write-decision
check appears in the record.

## 41
verdict: not established
opened: record/decision-558f991d.md, record/decision-dd201a84.md
because: A methodlimit remark on two decision pages. No tampering attempt and no
`reading::corroborated` behaviour is recorded elsewhere.

## 42
verdict: not established
opened: record/overflow.md
because: A roadnottaken remark. What producing such a case would require is reasoning about records
that were never made, and the files hold no Event on a divergent side.

## 43
verdict: not established
opened: record/overflow.md
because: A methodlimit remark. No function `unreproduced`, no unit tests and no red run appear in the
files.

## 44
verdict: not established
opened: record/overflow.md
because: A qualification remark about test wiring and a suite the record does not contain.

## 45
verdict: not established
opened: record/overflow.md
because: A methodlimit remark about how the source records were handled. The files carry no mtimes
and no access log.

## 46
verdict: not established
opened: record/overflow.md
because: A methodlimit remark about the author's reading scope, which nothing in the files could
witness.

## totals
opened at least once: record/arm-operations.md, record/arm-finance.md, record/arm-merged.md, record/vocabulary.md, record/overflow.md, record/decision-74a6a53e.md, record/decision-1f093bfa.md, record/decision-b9392895.md, record/decision-558f991d.md, record/decision-dd201a84.md (all ten files in `record/`; also instructions.md and questions.md in the task directory)
answered established: 16
