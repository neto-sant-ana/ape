# Answers

Note on `opened`: I read every file in `record/` once, at the start, in a single pass. For each
question below I name the file(s) that actually settle it; "(already read)" means the content was
in that first pass and I did not re-open it.

Standing rule I applied: a bullet under "What was said about it" or in `overflow.md` is somebody's
remark, so it never counts as its own evidence. Where the arm/thesis/commitment/vocabulary data
independently show the same thing, I mark it established and say what I derived it from. Where a
statement asserts something about a program, a crate, a test, a file the record never mentions, or
the author's own process, I mark it not established even when the rest of the sentence is
derivable — and I say which half failed.

## 1
verdict: established
opened: record/arm-finance.md, record/arm-operations.md, record/commitment-4b8b9b88.md, record/commitment-652a011d.md
because: Both arms declare `entries: 20`, and their journal lists agree line for line — hash and recorded date — through position 19; position 20 is `4b8b9b88…` (2026-01-07) in operations and `652a011d…` (2026-01-08) in finance. Neither twentieth appears in the other's journal or custody list, and each commitment page names only its own arm (plus merged) in `arms:`.

## 2
verdict: established
opened: record/commitment-4b8b9b88.md, record/agent-house.md, record/agent-market.md, record/vocabulary.md, record/arm-operations.md
because: `commitment-4b8b9b88` gives accountable `fe0e80f6…` (= agent-house), beneficiary `0d3a24e8…` (= agent-market), magnitude 60, due 2026-01-20, recorded 2026-01-07, and its statement `1cb6093b…` points at action `5f5255e4…`, which the vocabulary gives as verb "spend"; the commitment sits only in operations' (and merged's) journal.

## 3
verdict: established
opened: record/commitment-652a011d.md, record/agent-market.md, record/agent-house.md, record/vocabulary.md, record/arm-finance.md
because: `commitment-652a011d` gives accountable `0d3a24e8…` (= agent-market), magnitude 40, due 2026-01-12, recorded 2026-01-08, resource instance `8098643f…` (vocabulary: label "account"), and its statement `dabb76d3…` points at action `f08e6708…` (verb "receive"); it sits only in finance's (and merged's) journal.

## 4
verdict: established
opened: record/thesis-1f093bfa.md, record/thesis-b9392895.md, record/thesis-558f991d.md, record/thesis-dd201a84.md, record/arm-operations.md, record/arm-finance.md
because: Operations' advance `1f093bfa…` and fork `b9392895…` are both known at 2026-01-07, and the fork records `omitted: ["2f54506a…"]`, `introduced: ["4b8b9b88…"]`; finance's advance `558f991d…` and fork `dd201a84…` are both known at 2026-01-08, the fork has no `omitted` and `introduced: ["652a011d…"]` with `2f54506a…` still in `open`. Four distinct world ids, two per party, and no party's two appear in the other's arm.

## 5
verdict: not established
opened: record/thesis-1f093bfa.md, record/thesis-b9392895.md, record/thesis-558f991d.md, record/thesis-dd201a84.md, record/agent-operations.md, record/agent-finance.md
because: The identity half is shown — the theses carry `taken by: 326993e9… (operations)` / `108077234… (finance)`, matching the two agent pages' identities. But the parenthetical claims how a program obtained that attribution, and nothing in the files records any program behaviour.

## 6
verdict: not established
opened: record/overflow.md, record/arm-finance.md, record/arm-operations.md
because: Only a remark in `overflow.md`. The journals do show neither sequence extends the other (they differ at position 20), but that `converge` refuses, and that the author measured it, is a claim about code the record does not contain.

## 7
verdict: not established
opened: record/overflow.md, record/arm-finance.md, record/arm-operations.md
because: No error text, refusal message or write log appears anywhere in `record/`; the quoted phrase "so they are divergent rather than incompatible" occurs in none of the files.

## 8
verdict: not established
opened: record/agent-finance.md, record/arm-finance.md, record/arm-merged.md
because: A remark on `agent-finance`. The arms do show `652a011d…` carrying the same hex id in finance and in merged, but that an `EntryId` is *derived from what admitting produced* is a statement about the identity scheme that the files never state.

## 9
verdict: not established
opened: record/arm-merged.md, record/arm-operations.md, record/arm-finance.md
because: The first sentence is established — merged's entries 1–20 are operations' twenty verbatim and entry 21 is finance's `652a011d…`. The justification is not: no file mentions a Canon, a watermark, or any rule about refusing an admission dated before it.

## 10
verdict: not established
opened: record/agent-operations.md
because: A `methodlimit` remark on `agent-operations` about running the program with roles swapped; nothing else in the files records that measurement or the Canon's behaviour.

## 11
verdict: not established
opened: record/overflow.md
because: An `exposition` remark. No file shows a `Taken` type, a `lineage::rebuild`, or what either demands.

## 12
verdict: not established
opened: record/arm-finance.md, record/arm-merged.md, record/thesis-558f991d.md, record/thesis-dd201a84.md
because: The prefix claim is derivable — finance's two decisions stand after `652a011d…` witnessing 20 entries, and in merged `652a011d…` is position 21 while operations' `4b8b9b88…` is position 20, so no merged prefix reaching `652a011d…` excludes it. But "the program prints which entry each one trips on" describes output the record does not contain.

## 13
verdict: established
opened: record/arm-finance.md, record/arm-merged.md, record/thesis-558f991d.md, record/thesis-dd201a84.md
because: Merged decisions 4 and 5 have the same kind (advance, fork), the same produced worlds `558f991d…`/`dd201a84…` and the same `after` entry `652a011d…` as finance's decisions 2 and 3, differing only in witnessing 21 entries instead of 20 — the decision carried over unchanged, witnessed against the longer merged prefix.

## 14
verdict: established
opened: record/arm-merged.md, record/arm-operations.md, record/arm-finance.md, record/thesis-74a6a53e.md, record/thesis-1f093bfa.md, record/thesis-b9392895.md, record/thesis-558f991d.md, record/thesis-dd201a84.md
because: Merged lists exactly five decisions, and each matches one already in a party arm by kind, produced world and `after` entry (genesis `74a6a53e…` from both; `1f093bfa…`/`b9392895…` from operations; `558f991d…`/`dd201a84…` from finance) — none is new. The `parent` fields form a tree rooted at `74a6a53e…` in which both forks survive as separate branches.

## 15
verdict: not established
opened: record/arm-merged.md (whole record/ directory checked for the term "generation")
because: The four counts match `arm-merged`'s frontmatter (21/21/5/5), but nothing in the files calls anything "generation `b`", and "it reconstructs from disk for a reader told nothing" asserts a reconstruction run the record does not contain.

## 16
verdict: established
opened: record/arm-operations.md, record/arm-finance.md, record/arm-merged.md
because: Operations claims three worlds (`74a6a53e…`, `1f093bfa…`, `b9392895…`) and finance three (`74a6a53e…`, `558f991d…`, `dd201a84…`) — six claims over five distinct ids — and merged's five decisions produce exactly those five ids, character for character, including both of finance's.

## 17
verdict: not established
opened: record/overflow.md
because: A remark. No `worlds.json` file, comparison run, or refusal-to-write appears in the record.

## 18
verdict: not established
opened: record/overflow.md, record/arm-finance.md, record/arm-operations.md, record/thesis-1f093bfa.md, record/thesis-558f991d.md
because: The premise is checkable — both divergent entries are commitments and every thesis carries the same `event head: 6d336a99…` — but "an advance absorbs only what its cut froze", and the causal "it survives here because", state a rule of the machinery that the files never state.

## 19
verdict: not established
opened: record/agent-finance.md
because: A counterfactual remark on `agent-finance`. Nothing in the files says how a cut resolves against an Event, so the consequence cannot be shown.

## 20
verdict: established
opened: record/arm-merged.md, record/arm-operations.md
because: `arm-merged` heads its list "Journal, in the order it was admitted", declares `entries: 21`, and lists 21; its first twenty lines are byte-identical to operations' twenty, same hashes and same recorded dates.

## 21
verdict: established
opened: record/thesis-558f991d.md, record/arm-merged.md, record/thesis-74a6a53e.md
because: `thesis-558f991d` is finance's advance, `known at: 2026-01-08`, with `parent`/`extends` `74a6a53e…`; merged's decision 4 records that same advance as witnessing 21 entries.

## 22
verdict: established
opened: record/thesis-dd201a84.md, record/arm-merged.md, record/thesis-b9392895.md
because: `thesis-dd201a84` is finance's fork extending `558f991d…`, with `introduced: ["652a011d…"]` and no `omitted` field at all (unlike `b9392895…`, which has one), so nothing is dropped; merged's decision 5 records it witnessing 21 entries.

## 23
verdict: not established
opened: record/overflow.md
because: An `exposition` remark with no named subject. Nothing in the files shows the journal, lineage and worlds were built together from one read, or records any consistency check between them.

## 24
verdict: not established
opened: record/overflow.md, record/arm-merged.md (whole record/ directory searched for "synthes"/"tip")
because: No file in `record/` contains such an assertion, any synthesis output, or any notion of one tip measured inside another.

## 25
verdict: not established
opened: record/thesis-dd201a84.md, record/thesis-b9392895.md, record/commitment-652a011d.md, record/overflow.md
because: The three ingredients are in the files — `dd201a84…` introduces `652a011d…`, that commitment is recorded 2026-01-08, and operations' tip `b9392895…` is known at 2026-01-07 — but no synthesis was run or recorded anywhere, and no verdict "conflicted" appears in the record.

## 26
verdict: established
opened: record/thesis-b9392895.md, record/thesis-dd201a84.md, record/commitment-652a011d.md, record/commitment-2f54506a.md
because: `b9392895…` omits `2f54506a…` while `dd201a84…` still lists it under `open` — the disagreement about the dropped commitment — and separately `dd201a84…` introduces `652a011d…`, recorded 2026-01-08, whereas the other world's `known at` is 2026-01-07, so it names an entry that world's instant does not reach.

## 27
verdict: not established
opened: record/agent-finance.md, record/agent-operations.md, record/commitment-2f54506a.md, record/thesis-b9392895.md, record/thesis-dd201a84.md
because: The same remark is repeated on three pages. Its factual core is checkable (operations omitted `2f54506a…`, finance kept it), but "would take two decisions neither party made", "not derivable from either record" and "so I left it open" are the author's judgement and process, which the files do not evidence.

## 28
verdict: established
opened: record/arm-finance.md, record/arm-merged.md, record/agent-finance.md
because: Comparing the two arms directly: finance's decisions 2 and 3 witness 20 entries, while the same two decisions in merged witness 21 — finance's original witness counts are not carried. Everything else finance claimed is carried: all 20 journal entries and 20 custody addresses appear in merged's 21, and all three of its world ids appear among merged's five. So the witnesses are the one thing lost, without relying on the remark that says so.

## 29
verdict: not established
opened: record/arm-finance.md, record/arm-merged.md (whole record/ directory searched for "lineage.json")
because: The substance is derivable (finance decided witnessing 20 entries against a journal lacking `4b8b9b88…`, and no merged prefix reaching `652a011d…` can exclude it), but the record contains no file called `lineage.json` and no evidence about whether the author approximated anything.

## 30
verdict: established
opened: record/arm-merged.md, record/arm-finance.md
because: Merged's decisions 4 and 5 witness 21 entries, which is true of merged's own 21-entry journal, while finance's own arm records the identical two decisions as witnessing 20 — so the witnesses on those records state the merged history, not finance's claim about its own.

## 31
verdict: not established
opened: record/overflow.md, record/thesis-558f991d.md, record/thesis-dd201a84.md
because: A `qualification` remark, and purely evaluative ("the weakest thing in the result", "I want it named"). Nothing in the files can rank the parts of the result against each other; the theses render the attribution as "taken by", and no field named `by` appears.

## 32
verdict: established
opened: record/thesis-558f991d.md, record/thesis-dd201a84.md, record/arm-merged.md
because: In merged, decisions 4 and 5 carry only `taken by: 108077234… (finance)` and a witness count; the thesis pages have no field for who applied a decision as against who authored it. Reading the merged record on its own, the two situations render identically.

## 33
verdict: established
opened: record/thesis-b9392895.md, record/thesis-dd201a84.md, record/thesis-1f093bfa.md, record/thesis-558f991d.md, record/thesis-74a6a53e.md
because: The forks do carry `introduced:` (and `omitted:` where applicable), and across all five thesis pages the fields are exhausted by world/instant/head/frozen/open/taken-by/taken-after/witnessed/extends/omitted/introduced/selection — there is no field anywhere that records a reason.

## 34
verdict: established
opened: record/arm-merged.md, record/arm-finance.md, record/thesis-558f991d.md, record/thesis-dd201a84.md
because: Both halves are visible in the data: merged attributes the two retaken decisions to finance (`taken by: 108077234…`), keeping the intention's provenance, while recording them as witnessing 21 entries when finance's own arm shows it witnessed 20 — an overstatement of what finance saw.

## 35
verdict: not established
opened: record/overflow.md
because: A `want` remark describing the crate's API surface (`converge`, Synthesis, archives). The record contains no API, no operation list and no code.

## 36
verdict: not established
opened: record/overflow.md
because: A `qualification` remark about whose rules and whose composition produced the result, and about what the crate guards. None of that is observable in the files.

## 37
verdict: not established
opened: record/overflow.md, record/thesis-558f991d.md, record/thesis-dd201a84.md
because: The absence of a re-applier field is visible on the thesis pages (see 32), but the rest of the statement — that this gap made a `by` choice a judgement call rather than a lookup — is about the author's process, which the record does not evidence.

## 38
verdict: not established
opened: record/overflow.md
because: A `want` remark about the id types' trait implementations. Nothing in `record/` shows any type, trait or serialization behaviour.

## 39
verdict: not established
opened: record/overflow.md
because: A `methodlimit` remark about a workaround and a missing constructor; no workaround, constructor or code appears in the files.

## 40
verdict: not established
opened: record/overflow.md
because: A `methodlimit` remark about a test that could not be built and a check that never refuses. The record contains no tests and no such check.

## 41
verdict: not established
opened: record/agent-finance.md
because: A `methodlimit` remark on `agent-finance` about editing a copy of a record and `reading::corroborated` refusing it; nothing else in the files records the attempt or that function.

## 42
verdict: not established
opened: record/overflow.md
because: A `roadnottaken` remark, and a counterfactual about what producing a different case would require; nothing in the files bears on it.

## 43
verdict: not established
opened: record/overflow.md
because: A `methodlimit` remark. No function `unreproduced`, no unit tests and no red-test run appear in the record.

## 44
verdict: not established
opened: record/overflow.md
because: A `qualification` remark about unproven wiring and a green suite; the record contains no suite and no wiring.

## 45
verdict: not established
opened: record/overflow.md
because: A `methodlimit` remark. The files carry no mtimes, no access log and nothing else that could show reads happened without writes.

## 46
verdict: not established
opened: record/overflow.md
because: A `methodlimit` remark, and it references a scratchpad that appears nowhere else in the record. Nothing in the files can bound what was read.

## totals
opened at least once: instructions.md, questions.md, record/agent-finance.md, record/agent-house.md, record/agent-market.md, record/agent-operations.md, record/arm-finance.md, record/arm-merged.md, record/arm-operations.md, record/commitment-2f54506a.md, record/commitment-3167ccd3.md, record/commitment-4b8b9b88.md, record/commitment-652a011d.md, record/event-6d336a99.md, record/overflow.md, record/thesis-1f093bfa.md, record/thesis-558f991d.md, record/thesis-74a6a53e.md, record/thesis-b9392895.md, record/thesis-dd201a84.md, record/vocabulary.md
answered established: 16
