//! `02-hindsight/run-01` — the longest testimony, and an audit rather than a decision.
//!
//! In-memory. The agent was handed one world identity and asked how the account came to be projected
//! below its floor, whether any decision was unsound when taken, and what alternatives existed.
//!
//! # Why its shape differs from every other testimony
//!
//! It is the only one organised around **what it could not determine** — seven numbered items, named
//! rather than filled in. That section alone carries more unhoused claims than most testimonies carry
//! in total, and it is the reason this run matters more to H4 than its word count suggests.
//!
//! It is also the run that corrected `01-the-corpus.md`: it says outright that *the audit reads only
//! — it admits nothing to the canon and stores nothing in the archive*, which makes it a second
//! reader in a corpus that was published as having one. See `03-the-corpus-had-two-readers.md`.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Standing, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::Hindsight,
        text,
        verdict: Verdict::Housed(carrier),
        standing: None,
    }
}

/// A road not taken, a qualification or a method limit — the three kinds that ask for nothing.
const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::Hindsight,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
        standing: None,
    }
}

const fn want(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::Hindsight,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Want)),
        standing: Some(standing),
    }
}

const fn loss(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::Hindsight,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Loss)),
        standing: Some(standing),
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::Hindsight,
        text,
        verdict: Verdict::Exposition,
        standing: None,
    }
}

/// Experiment 01's result, which recorded the two frictions this run met again and reached no queue.
const IN_01: Standing = Standing::Recorded("lab/agents/01-single-agent/99-result.md");

/// This run's own result.
const IN_02: Standing = Standing::Recorded("lab/agents/02-hindsight/99-result.md");

/// The ontology, which states that what is not necessary to coordination belongs to the application.
const ONTOLOGY: Standing = Standing::ByDesign("core/src/docs/01-ontology.md");

/// The journal, which enumerates — the capability the auditor's *lower bound* caveat asked for.
///
/// Queued as re-run debt S2: *a journal enumerates. Whether an auditor lifts the caveat is
/// unmeasured.* So the capability arrived and the measurement did not, which is the shape of a met
/// want this row can say something about.
const ENUMERATION: Standing = Standing::Met("the repository's journal — re-run debt S2");

const COMMITMENT: Carrier = Carrier::Entity("Commitment");
const EVENT: Carrier = Carrier::Entity("Event");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const JOURNAL: Carrier = Carrier::File(File::Journal);
const FEASIBILITY: Carrier = Carrier::Derived(Derived::Feasibility);
const CONDITIONS: Carrier = Carrier::Derived(Derived::Conditions);
const CUT: Carrier = Carrier::Derived(Derived::Cut);
const FOLD: Carrier = Carrier::Derived(Derived::Interpretation);

pub const CLAIMS: &[Claim] = &[
    // ---- the audit's own discipline ----------------------------------------------------------
    unhoused(
        "The replay's own bookkeeping (`Replay::intentions`, `Replay::worlds`) was never read; the \
         compiler confirms it, warning that both fields are dead.",
        Kind::MethodLimit,
    ),
    // ---- The short version -------------------------------------------------------------------
    housed(
        "The account is **not** 20 below anything today. What actually settled is a single receipt of \
         +100, and the factual level is **+100** in every world of the lineage",
        FOLD,
    ),
    housed(
        "The −20 is a *projection* of the world the house is in now: if both intentions it still \
         holds open are realized, the level closes at −20, and the floor is `cash >= 0`",
        FEASIBILITY,
    ),
    housed(
        "Two decisions produced a world that was already infeasible at the moment it was taken. The \
         first was reversed within the same day. The second is the one the house is still standing \
         in.",
        FEASIBILITY,
    ),
    // ---- The vocabulary, as read from the graph -----------------------------------------------
    housed(
        "| C1 | `7998cbe1…` | market receives **+100** into `account`, due 2026-01-02 | 2026-01-01 |",
        COMMITMENT,
    ),
    housed(
        "| C2 | `834371ed…` | house spends **−120**, due 2026-01-08 | 2026-01-06 |",
        COMMITMENT,
    ),
    housed(
        "| C3 | `1f0d449a…` | house spends **−30**, due 2026-01-14 | 2026-01-06 |",
        COMMITMENT,
    ),
    housed(
        "| C4 | `5c6332ee…` | house spends **−90**, due 2026-01-20 | 2026-01-09 |",
        COMMITMENT,
    ),
    housed(
        "No commitment depends on any other **[1, \"dependencies: none\"]** — so nothing here is a \
         dependency-ordering problem.",
        COMMITMENT,
    ),
    housed(
        "| `5664085b…` | C1 | `Settled` → **Fulfilled** | 2026-01-02 | none |",
        EVENT,
    ),
    housed(
        "| `dd0c480b…` | C2 | `Cancelled` → **Cancelled** | 2026-01-06 | `5664085b…` |",
        EVENT,
    ),
    // The same want experiment 01 met, from an agent that never saw 01. There it was copied out of
    // the fixture by hand; here it is probed through `check`. Two agents, two workarounds, one
    // absent accessor.
    want(
        "the resource `cash` behind the instance `account` carries an opaque constraint, so it was \
         **probed** rather than read — `check(-0.0001)` refused, `check(0.0)` allowed",
        IN_01,
    ),
    // ---- 1. How the account got here ---------------------------------------------------------
    housed(
        "| W0 | `a2c8c533…` | 2026-01-06, `5664085b` | C1 | C2 | **−20** | OutOfBounds at −20 |",
        WORLDS,
    ),
    housed(
        "| W1 | `d207a4e7…` | 2026-01-06, `dd0c480b` | C1, C2 | — | +100 | no conflict |",
        WORLDS,
    ),
    housed(
        "| W2 | `4688fa77…` | 2026-01-06, `dd0c480b` | C1, C2 | C3 | +70 | no conflict |",
        WORLDS,
    ),
    housed(
        "| W3 | `615cbca2…` | 2026-01-12, `dd0c480b` | C1, C2 | C3 | +70 | no conflict |",
        WORLDS,
    ),
    housed(
        "| W4 | `71250407…` | 2026-01-12, `dd0c480b` | C1, C2 | C3, C4 | **−20** | OutOfBounds at \
         −20 |",
        WORLDS,
    ),
    housed(
        "the account was opened by C1 (+100), fulfilled on 01-02. On 01-06 the house committed to \
         spending 120 and opened a world around it. That commitment was cancelled the same day, and \
         replaced by a smaller one of 30. On 01-09 a further 90 was committed, and on 01-12 the house \
         recognized that later knowledge and folded the 90 into the world it holds.",
        JOURNAL,
    ),
    housed(
        "**The level is a projection, not a balance.** Only C1 has settled. C2 was cancelled and \
         therefore *contributes nothing* — its −120 never happened. C3 and C4 are still Unsettled",
        CONDITIONS,
    ),
    housed(
        "**Nothing is late.** As of 2026-01-12 both open commitments report `within deadline` \
         **[3]**. This is a capacity breach, not a timeliness one",
        CONDITIONS,
    ),
    exposition("by the engine's own rule a deadline never enters a feasibility verdict at all"),
    unhoused(
        "*Inferred, not read:* the breach falls on **2026-01-20**, C4's due date.",
        Kind::Qualification,
    ),
    want(
        "`Conflict::OutOfBounds` carries only the instance and the level",
        IN_01,
    ),
    // ---- 2. The decisions, and what each was decided against ---------------------------------
    exposition("A Thesis records no operation — only a parent, a cut and a selection."),
    housed(
        "an edge whose cut is unchanged can only be a fork, and one whose cut moved can only be an \
         advancement",
        WORLDS,
    ),
    housed(
        "| — → W0 | **genesis** | selects C2 against a cut knowing only C1 fulfilled | — → **−20** |",
        WORLDS,
    ),
    housed(
        "| W0 → W1 | advance | head moves to the cancellation; C2 moves open → frozen | −20 → +100 |",
        WORLDS,
    ),
    housed(
        "| W1 → W2 | **fork (a decision)** | introduces C3 | +100 → +70 |",
        WORLDS,
    ),
    housed(
        "| W2 → W3 | advance | instant moves 01-06 → 01-12; head unchanged | +70 → +70 |",
        WORLDS,
    ),
    housed(
        "| W3 → W4 | **fork (a decision)** | introduces C4 | +70 → **−20** |",
        WORLDS,
    ),
    housed(
        "So there are **three decisions** (the genesis and two forks) and **two advancements**.",
        WORLDS,
    ),
    housed(
        "The advancements decided nothing: `advance` cannot add an intention, and neither of these \
         imposed a commitment the parent had not already selected — \"imposed by history: none\" in \
         both cases",
        Carrier::Derived(Derived::Selection),
    ),
    housed(
        "W0 → W1 changed the level from −20 to +100 without anyone choosing anything: the \
         cancellation is what did that.",
        FOLD,
    ),
    exposition(
        "`Interpretation::of` takes the event chain from the Thesis itself, never from the caller, so \
         every verdict in the table above is computed from the knowledge of that world and nothing \
         later.",
    ),
    housed(
        "the genesis knew C1 fulfilled, +100 on the table, and nothing about the cancellation",
        CUT,
    ),
    housed(
        "W2's fork knew C1 fulfilled, C2 cancelled — 100 available, and it spent 30 of it",
        CUT,
    ),
    housed(
        "It was not deciding about 90 against 100. It was deciding about 90 against the 70 that C3 \
         left",
        CUT,
    ),
    // ---- 3. Was any decision unsound at the time it was taken? -------------------------------
    housed(
        "**Yes — two of the three, and the same way both times: the world was already infeasible \
         when it was created, under every hypothesis the engine offers.**",
        FEASIBILITY,
    ),
    housed(
        "Selecting C2 (−120) against a cut whose only settled fact is +100 produces OutOfBounds at \
         −20 under `FinalState`, `OnDueDateNet` and `OnDueDateInAnyOrder` alike",
        FEASIBILITY,
    ),
    housed(
        "Its parent W3 was clean at +70. Introducing C4 (−90) produces OutOfBounds at −20 under all \
         three hypotheses **[3]**, and its parent's own verdict one line above is \"no conflict\" — \
         so this edge is exactly where a choice first put the account below the floor",
        FEASIBILITY,
    ),
    housed(
        "**The fork that introduced C3 (W2) was sound.** 100 − 30 = 70, clean under all three \
         hypotheses.",
        FEASIBILITY,
    ),
    unhoused(
        "Read together with the cancellation that precedes it, W1 → W2 looks like a correction: the \
         house withdrew the 120 and replaced it with something it could afford.",
        Kind::Qualification,
    ),
    exposition(
        "The Canon admits knowledge; feasibility is derived, never enforced at admission. What the \
         verdict attaches to is *selecting* a commitment into a world, not asserting it.",
    ),
    exposition(
        "**An infeasible thesis is not by itself a mistake.** Theses exist to compare alternatives, \
         and evaluating a world that turns out infeasible is what they are for.",
    ),
    // The queue holds this one, as "a decision that says it weighed rather than meant" — the request
    // nearest the ontology, and load-bearing for the training candidate. Named there by experiment
    // 06 of the other row, and reached here independently by an agent auditing one lineage.
    loss(
        "The graph carries no marker distinguishing a plan from an exploration — there are no named \
         references in it, and \"main is a convention\" is the engine's own position.",
        Standing::Tracked("lab/QUEUE.md"),
    ),
    // And this half is nowhere: that the world a house is *in* is application state, so a reader of
    // the record cannot tell which of many worlds is the live one. Measured — neither the queue nor
    // any candidate mentions a named reference.
    loss(
        "The only thing privileging W4 is that `hindsight::build()` handed it over as *the world the \
         house is in now*, which is application state outside the graph.",
        Standing::Untracked,
    ),
    unhoused(
        "On that footing, W0 was infeasible and abandoned within the day; W4 is infeasible and is \
         where the house is standing.",
        Kind::Qualification,
    ),
    exposition(
        "The engine reports findings, never a verdict of feasible; an empty list means nothing was \
         found under the hypothesis asked.",
    ),
    unhoused(
        "The soundness claims above are one-directional: the breaches are proven, the clean readings \
         are not proofs of realizability.",
        Kind::Qualification,
    ),
    housed(
        "The information required was present at each cut, and required no hindsight to compute — \
         that is exactly what \"interpreted only under its own cut\" buys.",
        CUT,
    ),
    // This run's own result ruled on it: "absent by design rather than by omission, and filling it is
    // an application's business if any application wants it." Which is H4's question in the
    // laboratory's own words, two years before H4 was written.
    loss(
        "What the graph does **not** show is whether anyone looked: projections are derived and never \
         stored, so there is no record of a feasibility check having been run, or skipped.",
        Standing::ByDesign("lab/agents/02-hindsight/99-result.md"),
    ),
    // ---- 4. What alternatives the house had --------------------------------------------------
    unhoused(
        "Enumerated by constructing the alternative worlds and interpreting each **[7]**. Every line \
         below is a real Thesis built at the real cut, not an estimate.",
        Kind::MethodLimit,
    ),
    unhoused(
        "| — (C1 alone, frozen) | +100 | no conflict |",
        Kind::RoadNotTaken,
    ),
    unhoused("| C3 | +70 | no conflict |", Kind::RoadNotTaken),
    housed("| **C2** ← taken | **−20** | OutOfBounds |", WORLDS),
    unhoused("| C2, C3 | −50 | OutOfBounds |", Kind::RoadNotTaken),
    unhoused("| — (omit C3) | +100 | no conflict |", Kind::RoadNotTaken),
    unhoused(
        "| C4 only (omit C3) | **+10** | no conflict |",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "| C3 only | — | refused: identical to the parent — holding still is not a fork |",
        Kind::RoadNotTaken,
    ),
    housed("| **C3, C4** ← taken | **−20** | OutOfBounds |", WORLDS),
    unhoused(
        "**at the last decision the house had a feasible way to take on C4** — omitting C3 leaves \
         +10 within bounds. It did not have a feasible way to take on both.",
        Kind::RoadNotTaken,
    ),
    loss(
        "Whether dropping the 30 was an acceptable trade is a business question the graph cannot \
         answer; it has no notion of what the house needed.",
        ONTOLOGY,
    ),
    want(
        "**The pool is a lower bound.** It contains exactly the commitments some world in the \
         lineage selected. `CanonicalKnowledge` offers lookup by identity and `head_as_of`, and no \
         enumeration at all — so a commitment the house admitted and never selected anywhere is \
         unreachable from the entry point I was given",
        ENUMERATION,
    ),
    unhoused(
        "**Alternatives that were never asserted cannot be enumerated.** \"Commit to 70 instead of \
         90\" would have been feasible arithmetically, but no such commitment exists in the canon, \
         and inventing one to test it would be writing knowledge, not auditing it.",
        Kind::RoadNotTaken,
    ),
    // ---- What I could not determine ----------------------------------------------------------
    // "No field for a reason" is H4's own subject, asked here before H4 existed — which is the
    // strongest single sentence in the corpus for the hypothesis, and it is tracked in the charter
    // rather than in the queue because the charter is where the hypothesis lives.
    want(
        "**Why C2 was cancelled, and who observed it.** The event carries an observation name \
         (`Cancelled`), an `occurred_at` and a `recorded_at`. There is no field for a reason and \
         none for an author",
        Standing::Tracked("lab/CHARTER.md"),
    ),
    unhoused(
        "the reading in §3 that calls W1 → W2 \"a correction\" is an interpretation of the shape of \
         the sequence, not a fact read from it.",
        Kind::Qualification,
    ),
    // MET, and the meeting is the interesting part: the application grew `Taken.by` when the
    // repository landed, and then experiment 17 measured that `by` names who TOOK a decision and not
    // who relayed it — so the want was served and the serving exposed a different gap, which is now
    // the queue's "a name for a record".
    want(
        "**Who took each decision.** A Thesis carries parent, cut and selection — the fields its \
         identity is derived from. No author",
        Standing::Met("cli/src/lineage.rs — `Taken.by`"),
    ),
    loss(
        "**When each decision was physically taken.** The cut is *declared* knowledge, not attested \
         provenance; the Thesis layer states this as a non-responsibility.",
        Standing::ByDesign("core/src/docs/06-thesis.md"),
    ),
    housed(
        "the genesis recognizes head `5664085b`, while the instant 2026-01-06 addresses `dd0c480b` \
         today, and rebuilding the genesis's exact cut is now refused outright […] That proves the \
         genesis was taken **before** the cancellation was recorded.",
        CUT,
    ),
    loss("Nothing comparable exists for the other decisions.", IN_02),
    loss(
        "**The intraday order of C2 and C3.** Both were recorded on 2026-01-06 **[8]**. A commitment \
         never enters the event chain, so recording date is its only knowledge-time coordinate and \
         there is nothing finer to read.",
        ONTOLOGY,
    ),
    unhoused(
        "I cannot tell whether \"select C3 instead of C2\" was genuinely available at the genesis, or \
         whether C3 was only asserted afterwards. The row is a valid world at that cut; whether it \
         was a real option at that moment is undeterminable.",
        Kind::Qualification,
    ),
    want(
        "**Whether the house held worlds outside this lineage.** `ThesisLookup` resolves a thesis by \
         identity and offers no enumeration, so an abandoned sibling — a fork taken and discarded — \
         would be invisible from the entry point given.",
        ENUMERATION,
    ),
    unhoused(
        "Everything above describes the ancestry of the current world, not necessarily everything the \
         house considered.",
        Kind::Qualification,
    ),
    unhoused(
        "**Whether the −20 will happen.** Feasibility says no completion of *this* selection stays \
         within bounds under any hypothesis offered. It says nothing about what the house may still \
         do",
        Kind::Qualification,
    ),
    loss(
        "The graph also shows only movements on `account` that some world selected — it carries no \
         claim that no other cash exists.",
        ONTOLOGY,
    ),
    loss(
        "**Whether a feasibility check was ever run before a decision.** Nothing records it. […] the \
         answer to *could* they is a demonstrable yes, and the answer to *did* they is not in the \
         graph.",
        Standing::ByDesign("lab/agents/02-hindsight/99-result.md"),
    ),
    // ---- Reproducing this --------------------------------------------------------------------
    unhoused(
        "The audit's own arithmetic is cross-checked against the engine's reported breach level for \
         every world; all five agree",
        Kind::MethodLimit,
    ),
    unhoused(
        "The audit reads only — it admits nothing to the canon and stores nothing in the archive.",
        Kind::MethodLimit,
    ),
];
