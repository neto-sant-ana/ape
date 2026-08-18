# What operations should do next

**Take it — one fork of `1cd9af…` introducing `06b94…`, nothing omitted — but ask finance one
question first, and record two things the fork will not say by itself.**

The report is right, and it reproduces. What it does not cover is the part that decides whether
taking the transfer is a good idea, and one neighbouring world shows how much that part can
differ under the same verdict.

## What is actually on offer

Read through the record (`ape-cli`, and the probe in `src/main.rs`), the names resolve to:

| id | what it is |
| --- | --- |
| `bbee12…` | Base. Genesis, `known_at` 2026-01-06, claimed by nobody |
| `791528…` | Source. Fork of the Base, `known_at` 2026-01-06, claimed by **finance** |
| `ddef20…` → `1cd9af…` | Target line. Advance to 2026-01-07, then a fork, both claimed by **operations** |
| `7d86cc…` | `receive` 100 into `account`, settled 2026-01-02 — frozen in every world |
| `4de318…` | `spend` −20, due 2026-01-10, recorded 2026-01-05 |
| `06b94…` | `spend` −30, due 2026-01-14, recorded 2026-01-06, **executed by finance** |
| `0df2ea…` | `spend` −60, due 2026-01-20, recorded 2026-01-07 |

`cash` is bounded `[0, 1000]`, and `account` starts at zero, so the settled +100 is the whole of
what the spends draw on.

The transfer asks for **one introduction and no removal**: add `06b94…`, the −30 due 2026-01-14.

## What I verified

- **The report reproduces exactly.** Asked again from `repo/`, byte-for-byte the same status. The
  record also corroborates itself: `reading::corroborated` refuses a repository whose decisions
  no longer produce the worlds it recorded, and it did not refuse.
- **The Base was not a choice that could have gone otherwise.** Synthesis verifies a Base rather
  than searching for one, so a differently-measured difference was the thing to rule out. Both
  alternatives are refused — `791528…` and `1cd9af…` are each *not* a common ancestor of the pair.
  `bbee12…` is the only coherent Base here, so `introduced = {06b94…}` is not an artifact of framing.
- **Taking it lands where the report says.** Forking `1cd9af…` with the resolved transfer produces
  frozen and open sets identical to the named candidate, at identity
  `052eedc1db8edd7d7b7483776b15414a92d6f27afa6ab98eb8642f4fcd63d3eb`.
- **And that world is sound.** No feasibility conflicts under `FinalState`, `OnDueDateNet` or
  `OnDueDateInAnyOrder`. Final level 10, inside the bound.

## The one thing the report's verdict does not mean

`applicable` is a statement about membership invariants — freezing, historical availability,
dependency closure. It is not a statement that the resulting world holds together.

Measured, not argued: build the world one fork from operations' own that differs *only* by having
kept the −20 (`46192bc0…`, open `{−20, −60}`, itself feasible). Ask the identical question, and
Synthesis returns the **identical** `applicable` with the **identical** transfer. Interpret the
world that transfer produces and it comes back
`OutOfBounds { level: -10.0 }`.

So the verdict is portable and the soundness is not. Operations has to interpret the candidate
itself — which I did, and here it passes.

## The consequence that is nowhere in the report

Operations' world today has 40 of headroom (+100, −60). After taking finance's −30 it has **10**.
The −20 that operations dropped can no longer come back: that world is exactly the `−10` above.

Taking the transfer therefore forecloses reinstating `4de318…`. If operations wants that option, it
has to be decided now, not after the fork.

## The question that is not operations' to answer

Synthesis is explicit that it does not pair an omission with an introduction, and that treating two
commitments as alternatives would be judging intention rather than reporting evidence. So the record
cannot say whether operations' −60 *replaces* the −20 it dropped or merely joins it.

That matters here because **finance decided the −30 in a world that still held the −20** — finance's
world is +100, −20, −30, with 50 of headroom. The intention is being moved into a world finance never
saw, whose remaining headroom after the move is 10.

Two readings are both defensible, and the record settles neither:

- `06b94…` is canonical knowledge whatever anyone selects — house accountable, finance executing,
  due 2026-01-14. On that reading operations' plan currently *understates* the account by 30, and
  taking it is a correction.
- A selection is intention, not obligation. On that reading declining is a plan that does not
  reason about that commitment, not a denial that it exists.

Either way, **declining changes operations' plan, not the world**. Which is why the confirmation
below is a question to a party and not a computation.

## Also: the attribution is a claim, not a finding

The premise of the whole exercise — that `791528…` is *finance's* line — is witnessed by whoever
wrote the record and by nothing else. Demonstrated on a copy (`probe-attribution/`): change `by`
from finance to market and the record rebuilds, corroborates, and then answers that **market**
decided `791528…`. What the check does buy is only that the named party had already been admitted —
a `by` naming a role id is refused ("whom nothing had admitted when it was taken",
`probe-unknown-decider/`).

## So, in order

1. **Ask finance** — out of band, because the record cannot answer it — to confirm that `791528…`
   is theirs, and that the −30 due 2026-01-14 is meant to stand in a plan where the −20 due
   2026-01-10 is gone. This is the only genuinely open question.
2. **Then fork** `1cd9af…`, introducing `06b94…`, omitting nothing. Check that you land on
   `052eedc1…` and that the world reports no feasibility conflicts. **If the identity differs,
   stop** — something moved underneath the report.
3. **Record the provenance somewhere the fork is not.** A fork states omitted and introduced and
   never says that another line of thinking is why; `report.json` is not in the record either, and
   a question nobody acted on is not part of one. The only surviving trail is that `791528…` is
   still there claiming finance — and §"the attribution is a claim" is how much that is worth.
4. **Tell finance to advance its cut to 2026-01-07.** The reverse question is refused today: asking
   what operations' intention would be in finance's line comes back `conflicted` with
   `HistoricalUnavailability` on `0df2ea…` (recorded 2026-01-07, finance's world knows only up to
   2026-01-06). Until finance advances, it is planning against a world in which operations' −60
   does not exist, and the exchange only runs one way.
5. **Note the foreclosure from §"the consequence"** with the decision: after this fork the −20 is
   not reinstatable.

## What this was based on

`repo/` read through `ape-cli` — `reading::corroborated`, `reading::decided_by`,
`transfer::reconstruct`, `Thesis::fork`, `Interpretation::{conditions_at, feasibility_under}`,
`level::settled`, `movement_of`. Every number above is printed by `cargo run` (`src/main.rs`);
nothing here is computed by hand. `probe-attribution/` and `probe-unknown-decider/` are copies of
`repo/` with one field changed, kept so the two attribution claims can be re-run.
