# Answers

Method note: I opened every file under `record/` once, at the start, before answering anything
(`vocabulary.md`, `decision-1-74a6a53e.md`, `decision-2-1f093bfa.md`, `decision-3-b9392895.md`,
`decision-4-558f991d.md`, `decision-5-dd201a84.md`, `overflow.md`). Where a question is answered
from something already read, the `opened` line names the file it was answered from.

Throughout, I treat the *structured* part of each decision page (frontmatter plus the bulleted
facts: produces world / known at / frozen / open / taken by / taken after entry / witnessed
entries / extends / omitted / introduced) and the vocabulary entries as the record's evidence, and
the "What was said about it" bullets and `overflow.md` bullets as remarks — somebody's claims,
per the instructions.

## 1
verdict: not established
opened: all five decision pages, record/overflow.md, record/vocabulary.md
because: No file mentions two journals, their lengths, or a shared 19-entry prefix; the only entry
counts in the record are the per-decision `witnessed entries` (19, 20, 20, 21, 21) of the single
merged record, which do not give either original journal's size or their overlap.

## 2
verdict: not established
opened: record/decision-3-b9392895.md, record/vocabulary.md
because: Decision 3 does show operations introducing `4b8b9b88…` known at 2026-01-07, but nothing
anywhere gives that commitment's content — no quantity 60, no due date 2026-01-20, no house/market
parties for it. `vocabulary.md` carries only role/agent/resource/action/statement entries, and no
commitment bodies at all.

## 3
verdict: not established
opened: record/decision-5-dd201a84.md, record/vocabulary.md
because: Decision 5 shows finance introducing `652a011d…`, but the commitment's content (40, the
account, by 2026-01-12, market as the accountable party) appears in no file; commitments are named
by id only and `vocabulary.md` does not describe them.

## 4
verdict: established
opened: record/decision-2-1f093bfa.md, record/decision-3-b9392895.md, record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Decision 2 is an advance by operations known at 2026-01-07 with `open: [2f54506a…]`, and
decision 3 forks it with `omitted: [2f54506a…]`, `introduced: [4b8b9b88…]`, `open: [4b8b9b88…]`;
decision 4 is an advance by finance known at 2026-01-08 with `open: [2f54506a…]`, and decision 5
forks it with `introduced: [652a011d…]` and `open: [2f54506a…, 652a011d…]` and no `omitted` line.
The four produced world ids are pairwise distinct, two per party.

## 5
verdict: established
opened: record/vocabulary.md, record/decision-2-1f093bfa.md, record/decision-3-b9392895.md, record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Decisions 2 and 3 carry `by: 326993e9…` and 4 and 5 carry `by: 10807723…`, and
`vocabulary.md` — the record's own list of the journal entries that introduce names — holds agent
`326993e9…` labelled **operations** and agent `10807723…` labelled **finance**, both recorded
2026-01-03. The mapping is therefore read out of journal entries in the record, not assumed; what
the *program* did internally is not itself shown, only that the label lives in the journal.

## 6
verdict: not established
opened: record/overflow.md
because: This is verbatim an overflow remark (`methodlimit`) — the author's claim about running
`converge`. No file shows a run, a refusal, or `converge`'s comparison rule.

## 7
verdict: not established
opened: record/overflow.md, all five decision pages
because: Nothing in the record quotes a refusal message, names position 19, or says "divergent
rather than incompatible"; there is no output of any program in the files.

## 8
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: This is verbatim an `exposition` remark on decisions 4 and 5. Nothing in the record shows
how an `EntryId` is computed, so the identity-of-knowledge claim rests only on the remark.

## 9
verdict: not established
opened: all five decision pages, record/overflow.md
because: The record never states either journal's length, never distinguishes "operations' 20" from
"finance's 1", and contains no statement of a Canon watermark rule; the only relevant datum is
`witnessed entries: 21` on decisions 4 and 5.

## 10
verdict: not established
opened: record/decision-2-1f093bfa.md, record/decision-3-b9392895.md
because: Verbatim a `methodlimit` remark on decisions 2 and 3. The record holds no run log, no
rejection, and nothing about swapping roles.

## 11
verdict: not established
opened: record/overflow.md
because: Verbatim an `exposition` remark. No file shows the `Taken` type, `lineage::rebuild`, or
any code or schema — the record contains only rendered decisions.

## 12
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md, record/overflow.md
because: The record does not state what prefix finance's original records claimed (decisions 4 and
5 as rendered say `witnessed entries: 21`, i.e. the merged prefix), and nothing shows a program
printing which entry each trips on.

## 13
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: That finance's decisions were carried over "verbatim" and "re-witnessed" is not shown by
anything structural; the pages record `by`, `extends` and `witnessed entries: 21` but nothing
identifying a re-application or comparing to an original.

## 14
verdict: not established
opened: all five decision pages
because: Partly contradicted and partly unshown: decision 1 is `by: nobody — the decision claims no
party`, so not all five decisions are ones "the two parties took"; and nothing shows the decisions
are verbatim copies of anything. The tree shape (3 extends 2 extends 1; 5 extends 4 extends 1) is
the only part the pages do settle.

## 15
verdict: not established
opened: all five decision pages
because: "Decision N of 5" and five distinct produced worlds are shown, and `witnessed entries: 21`
appears on decisions 4 and 5, but nothing in the record mentions a "generation b", custody
addresses, or reconstruction from disk.

## 16
verdict: not established
opened: all five decision pages, record/overflow.md
because: The record contains five worlds, not six, and says nothing about what either original
record claimed or about reproducing identities.

## 17
verdict: not established
opened: record/overflow.md
because: Verbatim a `methodlimit` remark. No `worlds.json`, no comparison output, and no guard
behaviour appears anywhere in the files.

## 18
verdict: not established
opened: record/overflow.md, all five decision pages
because: Verbatim an `exposition` remark. The pages do show an unchanging `frozen:
["3167ccd3…"]` across all five decisions, which is consistent with it, but they never classify the
divergent entries as Commitments nor state any rule about Event heads or cuts.

## 19
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Verbatim a `qualification` remark, and a counterfactual besides — the record contains no
rule about how cuts resolve that would let a reader derive it.

## 20
verdict: not established
opened: all five decision pages, record/overflow.md
because: No file makes an assertion about a whole ordered sequence of entries, and the "twenty of
its entries are operations' own" figure appears nowhere; the only 20s are `witnessed entries` on
decisions 2 and 3.

## 21
verdict: established
opened: record/decision-4-558f991d.md
because: Decision 4 is `kind: decision (advance)`, `by: 10807723…` (finance), `known at:
2026-01-08`, `extends: 74a6a53e…`, `witnessed entries: 21` — every clause of the statement is one
of those fields.

## 22
verdict: established
opened: record/decision-5-dd201a84.md, record/decision-3-b9392895.md
because: Decision 5 is a fork by finance with `extends: 558f991d…`, `introduced: ["652a011d…"]`,
`witnessed entries: 21`, and no `omitted` line — where decision 3 shows that a fork that drops
something does carry `omitted:`, so its absence here is "nothing is dropped"; the same is visible
in the open set, which is decision 4's `[2f54506a…]` plus the new commitment.

## 23
verdict: not established
opened: record/overflow.md
because: Verbatim an `exposition` remark. No file shows a manifest, a read, or any consistency
check binding journal, lineage and worlds.

## 24
verdict: not established
opened: record/overflow.md, all five decision pages
because: Nothing in the record mentions a read-only projection of one tip into another, and no
page carries such an assertion.

## 25
verdict: not established
opened: record/decision-3-b9392895.md, record/decision-5-dd201a84.md, record/overflow.md
because: Decision 5's `introduced: ["652a011d…"]` and decision 3's `known at: 2026-01-07` are in
the record, but no Synthesis result, no verdict "conflicted", and no `recorded_at` for
`652a011d…` appear anywhere — `vocabulary.md` does not cover commitments.

## 26
verdict: not established
opened: record/decision-3-b9392895.md, record/decision-5-dd201a84.md, record/vocabulary.md
because: It rests on 25. The record gives no recording instant for `652a011d…` and no rule for
what a world at 2026-01-07 recognises, so "names knowledge the other's world has not yet
recognized" cannot be shown from the files.

## 27
verdict: not established
opened: record/decision-1-74a6a53e.md, record/decision-2-1f093bfa.md, record/decision-3-b9392895.md, record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Verbatim a `roadnottaken` remark repeated on all five pages. Its factual sub-clause
(operations omitted `2f54506a…`, finance kept it) is shown by decisions 3 and 5, but the claim
about what reconciling would require, and about non-derivability, is the author's judgement with
no supporting material in the files.

## 28
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Verbatim a `loss` remark. Nothing in the record shows finance's original witnesses, so
neither their non-preservation nor the "one claim" uniqueness can be checked.

## 29
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md, record/overflow.md
because: No `lineage.json` content is in the record, and the pages give no per-position prefixes
against which to check that no such prefix exists.

## 30
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Verbatim a `qualification` remark. The pages state `witnessed entries: 21` but nothing
establishes whose claim that number originally was.

## 31
verdict: not established
opened: record/overflow.md
because: Verbatim a `qualification` remark, and an evaluative one ("the weakest thing"); the record
offers no basis for ranking parts of the result.

## 32
verdict: not established
opened: record/overflow.md, record/decision-2-1f093bfa.md, record/decision-3-b9392895.md, record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Verbatim a `want` remark on four pages. The files are a rendering of decisions, not the
record's schema, so the absence of a "re-applied by" field here cannot show the record has no way
to express it — and nothing in the structural part shows a retake happened at all.

## 33
verdict: not established
opened: record/overflow.md, record/decision-3-b9392895.md, record/decision-5-dd201a84.md
because: The first half is visible (`introduced:` on decisions 3 and 5), but the second half is a
universal negative about the record's expressive range; these rendered pages cannot show that no
rationale can ever be recorded, only that none is printed here.

## 34
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Verbatim a `roadnottaken` remark about the author's own choice and its cost; the record
holds no alternative attribution and nothing on what finance actually witnessed.

## 35
verdict: not established
opened: record/overflow.md
because: Verbatim a `want` remark about the crate's operations. No API, code or documentation of
`converge` or Synthesis is present in the record.

## 36
verdict: not established
opened: record/overflow.md
because: Verbatim a `qualification` remark about the author's composition and the crate's
guarantees; nothing in the files shows either.

## 37
verdict: not established
opened: record/overflow.md
because: Verbatim a `want` remark, and the same gap as 32 — the files do not contain the record's
schema, only rendered decisions.

## 38
verdict: not established
opened: record/overflow.md, record/vocabulary.md
because: Verbatim a `want` remark. The record shows ids rendered as hex strings but contains no
type definitions, trait implementations or code from which a missing `FromStr` could be shown.

## 39
verdict: not established
opened: record/overflow.md
because: Verbatim a `methodlimit` remark, and it refers to a workaround that appears nowhere in the
files.

## 40
verdict: not established
opened: record/overflow.md
because: Verbatim a `methodlimit` remark. No test, guard or write-decision logic is present in the
record.

## 41
verdict: not established
opened: record/decision-4-558f991d.md, record/decision-5-dd201a84.md
because: Verbatim a `methodlimit` remark on decisions 4 and 5, describing an attempted run;
`reading::corroborated` and the tampering experiment appear nowhere else in the files.

## 42
verdict: not established
opened: record/overflow.md
because: Verbatim a `roadnottaken` remark, a claim about what a hypothetical test case would
require; the record states no rule connecting Events to world survival.

## 43
verdict: not established
opened: record/overflow.md
because: Verbatim a `methodlimit` remark. No test files, function `unreproduced`, or red-run output
are in the record.

## 44
verdict: not established
opened: record/overflow.md
because: Verbatim a `qualification` remark about the suite; no suite, run, or wiring appears in the
files.

## 45
verdict: not established
opened: record/overflow.md
because: Verbatim a `methodlimit` remark. The record contains no mtimes, file listing, or access
log against which "read only, never written" could be checked.

## 46
verdict: not established
opened: record/overflow.md
because: Verbatim a `methodlimit` remark about the author's own conduct; nothing in the record
records what was read.

## totals
opened at least once: instructions.md, questions.md, record/vocabulary.md, record/decision-1-74a6a53e.md, record/decision-2-1f093bfa.md, record/decision-3-b9392895.md, record/decision-4-558f991d.md, record/decision-5-dd201a84.md, record/overflow.md
answered established: 4
