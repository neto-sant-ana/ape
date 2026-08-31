//! `04-multiagent/run-reader` — the second of the corpus's two readers, and the run that was asked
//! for advice.
//!
//! It decided nothing and wrote nothing. It was handed an applicability report and asked what
//! operations should do next, and it answers with a numbered list of instructions.
//!
//! # It corrects `05-the-kinds-depend-on-the-audience.md` rather than confirming it
//!
//! This is a **technical** report — `HistoricalUnavailability`, `OnDueDateInAnyOrder`, identities
//! quoted in full — and its unclassified rate is 8 of 22, next door to
//! `03-narrative-mismatch`'s 7 of 17 and an order of magnitude above every other technical run.
//!
//! So the line is not technician-versus-principal, and it is not *report against advise* either —
//! `03` was asked to explain and volunteered its advice. It is **whether the reader of the testimony
//! has a decision to make**:
//!
//! ```text
//! written for somebody who will CHECK    wants, losses, qualifications, method limits
//! written for somebody who will CHOOSE   and recommendations, which fit none of the five
//! ```
//!
//! `03` was addressed to the principal, who acts. This one was addressed to operations, who acts. One
//! volunteered the advice and one was asked for it; the reader's position is the same either way.
//! See `07-the-line-is-whether-the-reader-must-choose.md`, which is the **third** formulation of this
//! split and says so.
//!
//! # And it contains the corpus's plainest statement of H4
//!
//! > *A fork states omitted and introduced and never says that another line of thinking is why;
//! > `report.json` is not in the record either, and a question nobody acted on is not part of one.*
//!
//! Followed immediately by the instruction **record the provenance somewhere the fork is not** —
//! which is H4's question in an agent's own words, and it is asking for a place *beside* the entity.

use crate::classification::{Carrier, Claim, Derived, File, Kind, Standing, Verdict};
use crate::corpus::Run;

const fn housed(text: &'static str, carrier: Carrier) -> Claim {
    Claim {
        run: Run::MultiagentReader,
        text,
        verdict: Verdict::Housed(carrier),
        standing: None,
    }
}

const fn unhoused(text: &'static str, kind: Kind) -> Claim {
    Claim {
        run: Run::MultiagentReader,
        text,
        verdict: Verdict::Unhoused(Some(kind)),
        standing: None,
    }
}

const fn loss(text: &'static str, standing: Standing) -> Claim {
    Claim {
        run: Run::MultiagentReader,
        text,
        verdict: Verdict::Unhoused(Some(Kind::Loss)),
        standing: Some(standing),
    }
}

const fn unclassified(text: &'static str) -> Claim {
    Claim {
        run: Run::MultiagentReader,
        text,
        verdict: Verdict::Unhoused(None),
        standing: None,
    }
}

const fn exposition(text: &'static str) -> Claim {
    Claim {
        run: Run::MultiagentReader,
        text,
        verdict: Verdict::Exposition,
        standing: None,
    }
}

const COMMITMENT: Carrier = Carrier::Entity("Commitment");
const WORLDS: Carrier = Carrier::File(File::Worlds);
const LINEAGE: Carrier = Carrier::File(File::Lineage);
const FEASIBILITY: Carrier = Carrier::Derived(Derived::Feasibility);
const APPLICABILITY: Carrier = Carrier::Derived(Derived::Applicability);
const FOLD: Carrier = Carrier::Derived(Derived::Interpretation);

const AUTHENTICITY: Standing = Standing::Tracked("lab/candidates/00-authenticity.md");

pub const CLAIMS: &[Claim] = &[
    unclassified(
        "**Take it — one fork of `1cd9af…` introducing `06b94…`, nothing omitted — but ask finance \
         one question first, and record two things the fork will not say by itself.**",
    ),
    unhoused("The report is right, and it reproduces.", Kind::MethodLimit),
    unhoused(
        "What it does not cover is the part that decides whether taking the transfer is a good idea, \
         and one neighbouring world shows how much that part can differ under the same verdict.",
        Kind::Qualification,
    ),
    // ---- What is actually on offer -----------------------------------------------------------
    housed(
        "| `bbee12…` | Base. Genesis, `known_at` 2026-01-06, claimed by nobody |",
        LINEAGE,
    ),
    housed(
        "| `791528…` | Source. Fork of the Base, `known_at` 2026-01-06, claimed by **finance** |",
        LINEAGE,
    ),
    housed(
        "| `ddef20…` → `1cd9af…` | Target line. Advance to 2026-01-07, then a fork, both claimed by \
         **operations** |",
        LINEAGE,
    ),
    housed(
        "| `7d86cc…` | `receive` 100 into `account`, settled 2026-01-02 — frozen in every world |",
        COMMITMENT,
    ),
    housed(
        "| `4de318…` | `spend` −20, due 2026-01-10, recorded 2026-01-05 |",
        COMMITMENT,
    ),
    housed(
        "| `06b94…` | `spend` −30, due 2026-01-14, recorded 2026-01-06, **executed by finance** |",
        COMMITMENT,
    ),
    housed(
        "| `0df2ea…` | `spend` −60, due 2026-01-20, recorded 2026-01-07 |",
        COMMITMENT,
    ),
    housed(
        "`cash` is bounded `[0, 1000]`, and `account` starts at zero, so the settled +100 is the whole \
         of what the spends draw on.",
        Carrier::Entity("Resource"),
    ),
    housed(
        "The transfer asks for **one introduction and no removal**: add `06b94…`, the −30 due \
         2026-01-14.",
        APPLICABILITY,
    ),
    // ---- What I verified ---------------------------------------------------------------------
    unhoused(
        "**The report reproduces exactly.** Asked again from `repo/`, byte-for-byte the same status. \
         The record also corroborates itself: `reading::corroborated` refuses a repository whose \
         decisions no longer produce the worlds it recorded, and it did not refuse.",
        Kind::MethodLimit,
    ),
    unhoused(
        "**The Base was not a choice that could have gone otherwise.** Synthesis verifies a Base \
         rather than searching for one, so a differently-measured difference was the thing to rule \
         out. Both alternatives are refused […] `bbee12…` is the only coherent Base here, so \
         `introduced = {06b94…}` is not an artifact of framing.",
        Kind::MethodLimit,
    ),
    housed(
        "Forking `1cd9af…` with the resolved transfer produces frozen and open sets identical to the \
         named candidate, at identity \
         `052eedc1db8edd7d7b7483776b15414a92d6f27afa6ab98eb8642f4fcd63d3eb`.",
        APPLICABILITY,
    ),
    housed(
        "**And that world is sound.** No feasibility conflicts under `FinalState`, `OnDueDateNet` or \
         `OnDueDateInAnyOrder`. Final level 10, inside the bound.",
        FEASIBILITY,
    ),
    // ---- The one thing the report's verdict does not mean ------------------------------------
    exposition(
        "`applicable` is a statement about membership invariants — freezing, historical availability, \
         dependency closure. It is not a statement that the resulting world holds together.",
    ),
    unhoused(
        "Measured, not argued: build the world one fork from operations' own that differs *only* by \
         having kept the −20 (`46192bc0…`, open `{−20, −60}`, itself feasible). Ask the identical \
         question, and Synthesis returns the **identical** `applicable` with the **identical** \
         transfer. Interpret the world that transfer produces and it comes back",
        Kind::MethodLimit,
    ),
    exposition(
        "So the verdict is portable and the soundness is not. Operations has to interpret the \
         candidate itself",
    ),
    // ---- The consequence that is nowhere in the report ---------------------------------------
    housed(
        "Operations' world today has 40 of headroom (+100, −60). After taking finance's −30 it has \
         **10**.",
        FOLD,
    ),
    housed(
        "The −20 that operations dropped can no longer come back: that world is exactly the `−10` \
         above.",
        FEASIBILITY,
    ),
    housed(
        "Taking the transfer therefore forecloses reinstating `4de318…`.",
        FEASIBILITY,
    ),
    unclassified("If operations wants that option, it has to be decided now, not after the fork."),
    // ---- The question that is not operations' to answer --------------------------------------
    exposition(
        "Synthesis is explicit that it does not pair an omission with an introduction, and that \
         treating two commitments as alternatives would be judging intention rather than reporting \
         evidence.",
    ),
    loss(
        "So the record cannot say whether operations' −60 *replaces* the −20 it dropped or merely \
         joins it.",
        Standing::Untracked,
    ),
    housed(
        "**finance decided the −30 in a world that still held the −20** — finance's world is +100, \
         −20, −30, with 50 of headroom. The intention is being moved into a world finance never saw, \
         whose remaining headroom after the move is 10.",
        FOLD,
    ),
    unhoused(
        "`06b94…` is canonical knowledge whatever anyone selects — house accountable, finance \
         executing, due 2026-01-14. On that reading operations' plan currently *understates* the \
         account by 30, and taking it is a correction.",
        Kind::RoadNotTaken,
    ),
    unhoused(
        "A selection is intention, not obligation. On that reading declining is a plan that does not \
         reason about that commitment, not a denial that it exists.",
        Kind::RoadNotTaken,
    ),
    exposition("Either way, **declining changes operations' plan, not the world**."),
    unclassified(
        "Which is why the confirmation below is a question to a party and not a computation.",
    ),
    // ---- Also: the attribution is a claim, not a finding -------------------------------------
    loss(
        "The premise of the whole exercise — that `791528…` is *finance's* line — is witnessed by \
         whoever wrote the record and by nothing else.",
        AUTHENTICITY,
    ),
    unhoused(
        "Demonstrated on a copy (`probe-attribution/`): change `by` from finance to market and the \
         record rebuilds, corroborates, and then answers that **market** decided `791528…`.",
        Kind::MethodLimit,
    ),
    housed(
        "What the check does buy is only that the named party had already been admitted — a `by` \
         naming a role id is refused",
        LINEAGE,
    ),
    // ---- So, in order --------------------------------------------------------------------------
    unclassified(
        "**Ask finance** — out of band, because the record cannot answer it — to confirm that \
         `791528…` is theirs, and that the −30 due 2026-01-14 is meant to stand in a plan where the \
         −20 due 2026-01-10 is gone. This is the only genuinely open question.",
    ),
    unclassified(
        "**Then fork** `1cd9af…`, introducing `06b94…`, omitting nothing. Check that you land on \
         `052eedc1…` and that the world reports no feasibility conflicts. **If the identity differs, \
         stop** — something moved underneath the report.",
    ),
    unclassified("**Record the provenance somewhere the fork is not.**"),
    // The plainest H4 sentence in the corpus, and an agent wrote it without the hypothesis existing.
    loss(
        "A fork states omitted and introduced and never says that another line of thinking is why; \
         `report.json` is not in the record either, and a question nobody acted on is not part of \
         one.",
        Standing::Tracked("lab/CHARTER.md"),
    ),
    unhoused(
        "The only surviving trail is that `791528…` is still there claiming finance — and §\"the \
         attribution is a claim\" is how much that is worth.",
        Kind::Qualification,
    ),
    unclassified("**Tell finance to advance its cut to 2026-01-07.**"),
    housed(
        "asking what operations' intention would be in finance's line comes back `conflicted` with \
         `HistoricalUnavailability` on `0df2ea…` (recorded 2026-01-07, finance's world knows only up \
         to 2026-01-06).",
        APPLICABILITY,
    ),
    housed(
        "Until finance advances, it is planning against a world in which operations' −60 does not \
         exist, and the exchange only runs one way.",
        Carrier::Derived(Derived::Cut),
    ),
    unclassified(
        "**Note the foreclosure from §\"the consequence\"** with the decision: after this fork the −20 \
         is not reinstatable.",
    ),
    // ---- What this was based on ----------------------------------------------------------------
    unhoused(
        "Every number above is printed by `cargo run` (`src/main.rs`); nothing here is computed by \
         hand.",
        Kind::MethodLimit,
    ),
    unhoused(
        "`probe-attribution/` and `probe-unknown-decider/` are copies of `repo/` with one field \
         changed, kept so the two attribution claims can be re-run.",
        Kind::MethodLimit,
    ),
];
