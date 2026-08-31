# Answers

Method note: `record/record.md` is the only file in `record/`. It has a data part (frontmatter,
Journal of 21 entries, Lineage of 5 decisions, Worlds of 5 worlds) and a closing section headed
**"What was said about it"**, which is a list of tagged remarks. Per the instructions, a remark on
that list is somebody's claim, not the record's evidence; several statements below are verbatim
copies of those remarks and are judged only on whether the data part independently shows them.

## 1
verdict: not established
opened: record/record.md
because: The record holds one journal of 21 entries, not two journals of 20; nothing in the files names a second journal, its length, or which entries the two would share. Decision 1's "witnessed entries: 19" shows a 19-entry prefix exists, but that is not two records agreeing on it.

## 2
verdict: established
opened: record/record.md
because: Commitment `4b8b9b88` is accountable `fe0e80f6` (Agent labelled **house**), statement `1cb6093b` whose action `5f5255e4` is verb "spend", magnitude 60, beneficiary `0d3a24e8` (**market**), due 2026-01-20, recorded 2026-01-07; and the only decision introducing it is Decision 3, taken by `326993e9` (operations).

## 3
verdict: established
opened: record/record.md
because: Commitment `652a011d` is accountable `0d3a24e8` (**market**), statement `dabb76d3` whose action `f08e6708` is verb "receive", magnitude 40, resource instance `8098643f` labelled "account", due 2026-01-12, recorded 2026-01-08; and the only decision introducing it is Decision 5, taken by `10807723` (finance).

## 4
verdict: established
opened: record/record.md
because: Decision 2 (advance, operations, known at 2026-01-07) then Decision 3 (fork, operations, omitted `2f54506a`, introduced `4b8b9b88`); Decision 4 (advance, finance, known at 2026-01-08) then Decision 5 (fork, finance, introduced `652a011d`, world `dd201a84` open = [`2f54506a`, `652a011d`]). The four non-genesis worlds `1f093bfa`/`b9392895` and `558f991d`/`dd201a84` are distinct, two per party.

## 5
verdict: not established
opened: record/record.md
because: The first half is shown — Decisions 2 and 3 are taken by `326993e9`, Decisions 4 and 5 by `10807723`, and the journal labels those agents operations and finance. The parenthetical about what *the program* derives versus assumes is a claim about code that is not in the files.

## 6
verdict: not established
opened: record/record.md
because: This is verbatim a *(methodlimit)* remark in "What was said about it". Nothing in the journal, lineage or worlds mentions `converge`, its comparison, or any measurement, so the remark stands alone.

## 7
verdict: not established
opened: record/record.md
because: The claimed refusal message, "position 19", the shared-19 count and "nothing was written" appear nowhere in the files — not even as a remark.

## 8
verdict: not established
opened: record/record.md
because: Verbatim an *(exposition)* remark. The record shows entry addresses but says nothing about how an `EntryId` is derived, and there is no second record to compare finance's entry against.

## 9
verdict: not established
opened: record/record.md
because: The journal does not attribute entries to parties, so "operations' 20 followed by finance's 1" cannot be checked; and no watermark rule of the Canon appears in the files.

## 10
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark about running the program with swapped roles. No run, output or rejection is recorded in the data part.

## 11
verdict: not established
opened: record/record.md
because: Verbatim an *(exposition)* remark. The files contain no definition of `Taken` and no description of what `lineage::rebuild` demands.

## 12
verdict: not established
opened: record/record.md
because: Requires finance's own records, which are absent. Inside this record finance's decisions (4 and 5) state "witnessed entries: 21" and stand after `652a011d`, so no prefix conflict is visible here, and no program output is included.

## 13
verdict: not established
opened: record/record.md
because: Showing that a `Decision` "crossed over" verbatim and was re-witnessed needs the pre-merge decision to compare with; the files hold only the merged lineage.

## 14
verdict: not established
opened: record/record.md
because: "Every one of the five decisions is verbatim one the two parties took" cannot be checked without the parties' records — and Decision 1 is marked "taken by: nobody", which the statement does not fit. Only the two-branch shape (Decision 3 extends `1f093bfa`, Decision 5 extends `558f991d`, both descending from `74a6a53e`) is visible.

## 15
verdict: not established
opened: record/record.md
because: The counts match (frontmatter entries: 21, decisions: 5, worlds: 5, and 21 journal entries are listed), but "Generation `b`", "custody addresses" and the reconstruction-from-disk claim have no counterpart anywhere in the files.

## 16
verdict: not established
opened: record/record.md
because: The record contains five worlds, not six, and holds no other record's claims to compare against, so "come back identically" cannot be shown.

## 17
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark. No `worlds.json` files, comparison output or refusal appear in the data part.

## 18
verdict: not established
opened: record/record.md
because: The two divergent-side entries `4b8b9b88` and `652a011d` are indeed Commitments in the journal, but the rules invoked ("move no Event head", "an advance absorbs only what its cut froze") are stated nowhere in the files, and the antecedent of "It survives" is not in them either. The sentence itself is only an *(exposition)* remark.

## 19
verdict: not established
opened: record/record.md
because: A counterfactual about an Event that does not exist, appearing only as a *(qualification)* remark; nothing in the data part shows how a cut would have resolved.

## 20
verdict: not established
opened: record/record.md
because: The subject of "asserts" is not present, and the record never attributes entries to a party, so "twenty of its entries are operations' own" cannot be checked.

## 21
verdict: established
opened: record/record.md
because: Decision 4 is an **advance** taken by `10807723` (finance), known at 2026-01-08, extending world `74a6a53e`, with "witnessed entries: 21". Reading "under the world X" as the world the decision extends is the reading that also makes statement 22 match its decision.

## 22
verdict: established
opened: record/record.md
because: Decision 5 is a fork taken by finance, extending `558f991d`, with "introduced: [`652a011d`]", no `omitted` line (Decision 3 shows one when something is dropped, and world `dd201a84` keeps `2f54506a` open), and "witnessed entries: 21".

## 23
verdict: not established
opened: record/record.md
because: Verbatim an *(exposition)* remark with no named subject; the files contain no manifest and no evidence that the three parts were built from one read.

## 24
verdict: not established
opened: record/record.md
because: No such artefact or reading appears in the files — there is no synthesis output, and no statement of what is stored versus read.

## 25
verdict: not established
opened: record/record.md
because: The ingredients are in the record (`652a011d` introduced by finance's fork and recorded 2026-01-08; operations' tip world `b9392895` known at 2026-01-07), but "Synthesis was asked and answered" and the verdict **conflicted** are the result of an operation whose invocation and output are not in the files.

## 26
verdict: established
opened: record/record.md
because: Decision 3 omits `2f54506a` while Decision 5's world `dd201a84` keeps it open — the disagreement about the dropped commitment; and finance's introduced `652a011d` is journal entry 21, recorded 2026-01-08, while operations' tip `b9392895` is known at 2026-01-07 and its decision witnessed only 20 entries.

## 27
verdict: not established
opened: record/record.md
because: Verbatim a *(roadnottaken)* remark. That operations omitted `2f54506a` and finance kept it is in the lineage, but "would take two decisions neither party made" and "I left it open" are the author's reasoning and actions, not record content.

## 28
verdict: not established
opened: record/record.md
because: Verbatim a *(loss)* remark. Finance's original witnesses are not in the files, so the loss cannot be shown by comparison.

## 29
verdict: not established
opened: record/record.md
because: Finance's `lineage.json` is not among the files, so what it asserted cannot be checked. (Within the merged journal a prefix containing `652a011d` but not `4b8b9b88` is indeed impossible, since they are entries 21 and 20 — but that is only half the claim.)

## 30
verdict: not established
opened: record/record.md
because: Verbatim a *(qualification)* remark; establishing it needs finance's own claims, which the files do not contain.

## 31
verdict: not established
opened: record/record.md
because: Verbatim a *(qualification)* remark, and a judgement about which part of the result is weakest — not something the data part can show.

## 32
verdict: not established
opened: record/record.md
because: Verbatim a *(want)* remark. The files show the fields these five decisions happen to carry, not the record format's full vocabulary, so "has no way to distinguish" cannot be shown from them.

## 33
verdict: not established
opened: record/record.md
because: Verbatim an *(exposition)* remark. The `introduced` lines are visible on Decisions 3 and 5, but the "never says why" half is a claim about what the format can express, which the files do not define.

## 34
verdict: not established
opened: record/record.md
because: Verbatim a *(roadnottaken)* remark about the author's choice and its cost; no alternative attribution or finance-side witness data is in the files to weigh it against.

## 35
verdict: not established
opened: record/record.md
because: Verbatim a *(want)* remark describing the crate's API surface. The files contain no API, no `converge`, and no Synthesis definition.

## 36
verdict: not established
opened: record/record.md
because: Verbatim a *(qualification)* remark about the crate's rules and the author's composition; the files contain neither the crate nor the composing program.

## 37
verdict: not established
opened: record/record.md
because: Verbatim a *(want)* remark, and the same absence-of-capability claim as 32 — the record format is not documented in the files.

## 38
verdict: not established
opened: record/record.md
because: Verbatim a *(want)* remark about Rust trait implementations on id types; no source, trait or type definition is in the files.

## 39
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark. The workaround and the missing constructor it refers to appear nowhere in the files.

## 40
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark about a test that could not be built; no tests, guard or check are present.

## 41
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark. There is no `reading::corroborated`, no tampered copy and no run output in the files.

## 42
verdict: not established
opened: record/record.md
because: Verbatim a *(roadnottaken)* remark reasoning about a case that was never produced; nothing in the data part bears on it.

## 43
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark. No function named `unreproduced`, no unit tests and no failure messages appear in the files.

## 44
verdict: not established
opened: record/record.md
because: Verbatim a *(qualification)* remark about the state of the wiring and the suite; there is no suite or wiring in the files.

## 45
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark about how the author's run touched files. The record's own content cannot show what was read or written, or that mtimes were unchanged.

## 46
verdict: not established
opened: record/record.md
because: Verbatim a *(methodlimit)* remark about the author's access during the run; nothing in the record documents what was or was not read.

## totals
opened at least once: instructions.md, questions.md, record/record.md
answered established: 6
