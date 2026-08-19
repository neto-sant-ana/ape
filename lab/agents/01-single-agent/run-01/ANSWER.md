# The delivery slot

## The arrangement chosen

**The house wanted the priority slot and must undertake the standard one.**

On the merits, the priority slot is the arrangement to want. The two differ in one thing
the house cares about — the priority slot spares a penalty owed to a third party — and
nothing else in the world tells against it. So that is the decision the program forms
first, and makes known to the system first.

It cannot be carried out. The account holds 100, its resource is bounded by `cash >= 0`,
and spending 120 puts it at −20. That is not a preference to weigh against the penalty;
it is a bound the world declares, and every hypothesis the engine offers agrees on it.
The house therefore falls back to the standard slot, which leaves the account at 70 and
against which nothing was found.

Two things about that reasoning are weaker than they look, and both are the world's doing
rather than the engine's:

- **The penalty has no number.** "Spares a late penalty" is the only reason to prefer
  paying 120 over 30, and the world carries no third party, no penalty commitment and no
  amount. I could not compare the arrangements; I could only rank them, treating "spares a
  penalty" as strictly better a priori. See *What I could not find*.
- **The comparison would have flipped on that number.** The account holds 100. Any
  penalty above 70 makes the standard path *also* end below zero, and then neither
  arrangement is realizable as the world stands and the real answer is that the house must
  raise cash first. Which arrangement the house should undertake genuinely depends on a
  quantity the world does not carry.

## What each constructed object asserts

In the order the program builds them.

| Object | What it asserts |
| --- | --- |
| `Assignment` | The house is accountable for this spending and performs it; the market is the one it benefits. |
| `Term` | This spending was committed today and is due on the slot's date. |
| `ActionValue` | The magnitude moved is 120 (resp. 30) — the direction is the Action's, not this value's. |
| `Commitment` (priority) | The house intends to spend 120 from the account to the market by 2026-01-08. |
| `KnowledgeCut::at(today)` | What could be known today: every commitment recorded by today, against the event chain current at it. |
| `Thesis` (genesis, *intended*) | One complete world: the settled past that cut makes unavoidable, plus the priority slot as its only open intention. |
| `Commitment` (standard) | The house intends to spend 30 from the account to the market by 2026-01-12. |
| `Event` (cancellation) | It was observed today that the priority slot will not happen. |
| `KnowledgeCut::at(today)`, again | The same instant against a longer chain: the head is now the cancellation. |
| `Advancement` → `Thesis` | The same intention under later knowledge — the world now recognizes the cancellation, and nothing new was chosen in doing so. |
| `Thesis` (fork, *undertaken*) | The world the house undertakes: that frozen past, plus the standard slot as its only open intention. |
| `Interpretation` | Nothing. It is the fold of the knowledge one world recognizes — the thing questions are put to, not a claim. |
| `FeasibilityReport` | Under one named hypothesis, at one named head, these are the conflicts that world carries. |
| `ProjectedConditions` | As of today, each selected commitment stands in this condition. |

The two candidate commitments coexist in canonical history, and only one of them is
selected by the world the house ends up in. That is the engine's division and not a
workaround: a Commitment is knowledge, a Thesis is which knowledge you mean.

## How one determines whether it can be carried out

Not from the Commitment. Whether an intention is realizable is a property of the whole
graph a world selects, so the determination is: name the cut you are asking under, build
the `Thesis` that selects the intention at that cut, hand it to `Interpretation::of`, and
ask `feasibility_under` for each hypothesis.

Three hypotheses exist and they differ in what they assume about *when* unsettled
commitments land, never in what is known: `FinalState` checks the level once every
movement has landed; `OnDueDateNet` checks it date by date, each date netted;
`OnDueDateInAnyOrder` checks every level any arrangement within a date could pass through.
Asking all three is cheap — one fold answers them all — and it is what separates "this is
impossible" from "this is impossible only in some order".

What it said (`cargo run`):

```
the intended world — 2 selected (1 frozen, 1 open):
  FinalState             out of bounds: 9920b32a would reach -20
  OnDueDateNet           out of bounds: 9920b32a would reach -20
  OnDueDateInAnyOrder    out of bounds: 9920b32a would reach -20

the undertaken world — 3 selected (2 frozen, 1 open):
  FinalState             nothing found
  OnDueDateNet           nothing found
  OnDueDateInAnyOrder    nothing found
```

Read precisely, because the engine is precise about it:

- The priority world **cannot** be carried out. All three hypotheses report the account
  leaving its bounds at −20, so no ordering and no deadline rescues it.
- The undertaken world is **not reported feasible**. There is no `Feasible` verdict in this
  crate on purpose: an empty conflict list means nothing was found under the hypothesis
  asked, not that the graph is realizable. The strongest honest claim is that three
  hypotheses were asked of the standard slot and none of them found anything.

I checked that these verdicts are live rather than decorative, by moving the numbers and
watching the answers move with them: at a priority cost of 100 the intended world reports
`nothing found` under all three; at a standard cost of 130 the undertaken world reports
`would reach -30` — −30 and not −150, which is also the proof that the cancelled priority
commitment stopped contributing its movement. Both mutations were reverted.

Two observations from applying it that are not obvious from the outside:

- **The money exists only because the cut recognizes the event that made it.** The +100 is
  not a stored balance; it is the movement of a fulfilled commitment, and it enters the
  fold because the genesis cut recognizes the settling Event. A cut taken before that event
  would make every spend look out of bounds, and the report would look identical.
- **Cancelling is what removes an intention's movement; omitting is what removes the
  intention.** Once the cancellation Event is recognized, the priority commitment is frozen
  into the world's past and can no longer be forked away — but as a cancelled commitment it
  moves no level. I cancelled rather than omitted because the house did form the intention,
  and the fact that it will not happen is worth recording; the omit-in-a-fork route was
  available up to the moment the event was admitted.

## What I could not find

**The vendored crate does not compile as delivered.** `ape/Cargo.toml` inherits `version`,
`edition`, `license` and `repository` from a workspace root that was not vendored with it,
so cargo fails at manifest parsing before any code is read. I added a `[workspace]` /
`[workspace.package]` block to the top-level `Cargo.toml`, with values chosen only to
satisfy the inheritance. Nothing about the answer depends on them, but the answer does not
build without them.

**There is no way to ask "would this be feasible?" without asserting it first.** A Thesis
selects commitment *ids*, and both `ensure_selectable` and the fold resolve them out of
canonical history — so the priority commitment had to be admitted to the Canon before its
own realizability could be examined, and history now permanently contains an intention that
never could be realized. That is coherent with the engine's stance that a commitment is
never invalid, and the goal's ordering asked for exactly this, but it is worth naming: there
is no dry run. An application that wanted one would have to implement `Knowledge` +
`CanonicalKnowledge` itself as an overlay of the real history plus the candidate, and
interpret against that — possible, since both are traits, but nothing in the crate offers it.

**The penalty is not expressible from the world as given.** The third party is not an
admitted Agent, holds no role, and the penalty has no amount. Everything needed to model it
exists — admit an agent, make it eligible as `counterparty`, commit a spend under the
`outbound` statement — but the amount would be invented by me, and the whole comparison
turns on it, so I left it out rather than fabricate the number that decides the answer.

**`Conflict::OutOfBounds` names the level but not the bound it left.** The report carries
the resource instance and the offending level; explaining "−20 against `cash >= 0`" means
going back to the resource behind the instance, and even then `Constraint` exposes only
`check(value)` — no accessor for its bound and no `Display`. The bound in my printed prose
is therefore copied by hand out of `world.rs`, which is exactly the kind of second copy that
goes stale.

**There is no "what is the level now".** `Accumulation` answers conditions and feasibility;
a projected balance is not among them. `movement_of` is public and per-commitment, so an
application sums them itself under whichever criterion it means. It did not block me —
feasibility answered the question I had — but I expected to be able to print the account's
level and could not, without writing the sum myself.

**Minor:** `world.rs` exposes `today()` but keeps its `day()` helper private, so
`src/main.rs` re-derives January 2026 dates of its own. Two copies of one calendar.
