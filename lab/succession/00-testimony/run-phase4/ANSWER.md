# Could somebody holding only `record/` establish this?

Rule applied throughout: a claim is YES only if everything it asserts about the world is
carried by the twelve files. Assertions about the crate's behaviour, the author's process,
a measurement taken, a counterfactual, or a cause are NO — even when the record-fact sitting
beside them is fully established. Purely rhetorical framing ("a real loss rather than a
technicality") is not treated as a separable assertion.

One fact underwrites many of the YESes: `custody.json` lists one address per journal entry
in journal order (the merged pair is the before pair plus finance's twentieth, in both
files), so labels in `journal.json` can be tied to the hex ids used everywhere else —
`fe0e…`=house, `0d3a…`=market, `3269…`=operations, `1080…`=finance, `5f52…`=spend,
`f08e…`=receive, `8098…`=account, `2f54…`/`4b8b…`/`652a…`=the three commitments.

| # | | reason |
|---|---|---|
| 1 | **YES** | `before/journal.json` and `theirs/journal.json` — both 20 entries, entries 1–19 byte-identical including `recorded_at`, the 20th differing. |
| 2 | **YES** | `before/journal.json` entry 20 (accountable `fe0e…`=house, statement `1cb6…`→action `5f52…`=spend, beneficiary `0d3a…`=market, magnitude 60, due 2026-01-20, recorded 2026-01-07) with `before/custody.json` position 20 giving `4b8b…`. |
| 3 | **YES** | `theirs/journal.json` entry 20 (accountable `0d3a…`=market, statement `dabb…`→action `f08e…`=receive, resource `8098…`=account, magnitude 40, due 2026-01-12, recorded 2026-01-08) with `theirs/custody.json` position 20 giving `652a…`. |
| 4 | **YES** | `before/lineage.json` + `before/worlds.json` (advance `known_at` 01-07, fork omitting `2f54…` and introducing `4b8b…`) against `theirs/*` (advance 01-08, fork omitting nothing, `dd20…` open on both); the four non-genesis worlds are disjoint. |
| 5 | NO | The `by`→party mapping is established (lineage + journal + custody), but that *the program derives it rather than assuming it* is a fact about the program, not the files. |
| 6 | NO | Whether `converge` refuses and what its comparison ranges over is crate behaviour; only "neither journal extends the other" is in the files. |
| 7 | NO | The wording and content of a refusal message, and that a run wrote nothing, are outside the record. |
| 8 | NO | The files show identical content carrying an identical address, but not the rule that an `EntryId` is derived from what admitting produced. |
| 9 | NO | The order is established (merged journal = operations' 20 then finance's 1); the reason offered — the Canon's watermark rule — is crate behaviour. |
| 10 | NO | A second run with roles swapped, and its failure, leave no trace in the record. |
| 11 | NO | Each decision does carry a prefix, but what `lineage::rebuild` demands is a property of code. |
| 12 | NO | The prefix mismatch is fully established (finance's two witnesses match no prefix of the merged journal), but "the program prints which entry each one trips on" is program output. |
| 13 | **YES** | `theirs/lineage.json` vs `merged/lineage.json` — the two finance decisions are identical field for field except `witness`, which in merged is exactly the 21-entry prefix of `merged/custody.json`. |
| 14 | **YES** | `merged/lineage.json` against `before/`+`theirs/lineage.json`: all five decisions occur in one of the two sources and none is new; `merged/worlds.json` shows `b939…` and `dd20…` as two surviving branches of a single-parent tree. |
| 15 | **YES** | `merged/*` — 21 journal entries, 21 custody addresses, 5 decisions, 5 worlds, and the four files are mutually consistent and self-describing to a reader given nothing else. |
| 16 | **YES** | `before/worlds.json` and `theirs/worlds.json` compared against `merged/worlds.json`: all six claimed identities, `558f…` and `dd20…` included, appear with identical `known_at`, `event_head`, `frozen` and `open`. |
| 17 | NO | That the program compares and refuses to write is behaviour; the files show the outcome, not the check. |
| 18 | NO | That the divergent entries are Commitments is in the journals, but the *because* — and the rule about what an advance absorbs — is not something a file can hold. |
| 19 | NO | A counterfactual about an Event that was never recorded. |
| 20 | **YES** | `merged/journal.json` is that sequence with its `recorded_at` order, and comparison with `before/journal.json` shows its first 20 entries unchanged. |
| 21 | **YES** | `merged/lineage.json` decision 4 — advance, `extends` `74a6…`, `known_at` 2026-01-08, `by` `1080…`=finance — with a 21-id witness equal to the whole of `merged/custody.json`. |
| 22 | **YES** | `merged/lineage.json` decision 5 — fork, `extends` `558f…`, `omitted` empty, `introduced` `652a…`, `by` finance — again witnessing the full 21. |
| 23 | NO | Mutual consistency is checkable, but "built together from one read" is provenance of the writing, which the files do not carry. |
| 24 | NO | The claim describes something the record explicitly does not store; its content cannot be got from the files. |
| 25 | NO | Both supporting dates are in the files (`652a…` recorded 01-08, `b939…` recognizes to 01-07), but the verdict *conflicted*, and that Synthesis was asked at all, are the operation's, not the record's. |
| 26 | **YES** | `before/`+`theirs/lineage.json` (one fork omits `2f54…`, the other keeps it) and `worlds.json`+`journal.json` (finance introduces `652a…` recorded 01-08; operations' tip `b939…` has `known_at` 01-07). |
| 27 | NO | What reconciling *would take*, and that the author left it open, are a counterfactual and an authorial act. |
| 28 | **YES** | `theirs/lineage.json` vs `merged/lineage.json` — finance's original 20-id witnesses are absent from merged, and an exhaustive comparison of the four files shows it is the only thing of finance's that merged does not carry; the closing valuation is rhetoric, not a further fact. |
| 29 | **YES** | `theirs/lineage.json` witnesses exclude `4b8b…`; because `4b8b…` sits at position 20 and `652a…` at 21 in `merged/custody.json`, no prefix holds one without the other, and merged's witnesses are exact prefixes rather than approximations. |
| 30 | **YES** | `merged/lineage.json` witnesses equal true prefixes of `merged/custody.json` and differ from the witnesses in `theirs/lineage.json`. |
| 31 | NO | An evaluation of the result and a statement of the author's intent; nothing to check. |
| 32 | **YES** | `merged/lineage.json` carries `by`=finance on two decisions whose witness is the merged prefix, with no field of any kind marking a re-application — the distinction simply is not recorded. |
| 33 | **YES** | `merged/lineage.json` has `introduced`/`omitted` and no field anywhere for a rationale; a file holding a *why* is exactly what these do not contain. |
| 34 | NO | The cost is measurable in the files, but the author's choosing is not in them. |
| 35 | NO | What operations the crate offers and what each requires is a fact about the software. |
| 36 | NO | Whose rules and whose composition — provenance of the authoring, not of the record. |
| 37 | NO | Second sentence makes it a claim about why the author's judgement was needed; the record can show the gap but not that. |
| 38 | NO | Ids do appear as hex, but the absence of a `FromStr` impl is a fact about the crate's API. |
| 39 | NO | A judgement about a workaround that leaves no trace here. |
| 40 | NO | A test that could not be built, and when a check refuses, are outside the files. |
| 41 | NO | An attempt on a tampered copy, and which layer caught it, are not recorded. |
| 42 | NO | What producing a different case would require — counterfactual. |
| 43 | NO | Unit tests, their names and their red runs are not in the record. |
| 44 | NO | A statement about the strength of the author's evidence. |
| 45 | NO | mtimes are not content of these files, and "unchanged" has no baseline here to compare against. |
| 46 | NO | What the author read is not something the record can witness. |

**Tally: 17 YES, 29 NO.**

## Genuinely difficult

- **5, 12** — both pair a fully established record-fact (the `by`→party mapping; that finance's
  two witnesses match no prefix of the merged journal) with a clause about what the *program*
  derives or prints. I answered NO on the rule that an unestablishable conjunct sinks the
  claim, the same way the briefing's miller sinks an otherwise readable overdraft; had I
  graded on the sentence's principal assertion, both would be YES.
- **9** — same shape, and the closest call of the three: the order of the merged journal is
  plainly in the files, but the sentence exists to say the order was *forced*, and that is the
  Canon's rule.
- **28** — I allowed a YES over "it is a real loss rather than a technicality" by treating a
  valuation as framing rather than a separable claim. Its two factual halves are checkable,
  the second only by an exhaustive comparison of all four of finance's files against merged.
- **32 vs 37** — near-identical gaps, scored differently. 32 asserts that *this* record does
  not mark a re-application, which `merged/lineage.json` shows; 37 generalizes it to what the
  software can record and then attaches the author's judgement call, which the files cannot
  reach.
- **15** — "reconstructs from disk for a reader told nothing" reads either as the program
  reloading (unestablishable) or as the self-sufficiency of the four files (establishable, and
  demonstrably so). I took the second.
- **8** — the conclusion is verifiable (`652a…` is the same address in `theirs/` and
  `merged/`), the premise about how an `EntryId` is derived is not; one instance does not
  establish the rule.
