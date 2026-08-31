//! What an agent is given: one carving, the same 46 questions, and nothing else.
//!
//! *One agent per carving, none of which has read this protocol, each asked the same question set
//! and each reporting what it opened.*
//!
//! # What must not reach the agent, and why each one would end the run
//!
//! ```text
//! the protocol         it names the three carvings and the predictions. An agent that knows
//!                      B is expected to win on motivation is not a reader, it is a confederate
//! the classification   it carries the verdict on every claim — the answers, in a file
//! the baseline         19 and 17. An agent told the number can land on it
//! the other carvings   a reader comparing two is measuring something this does not ask
//! the observations     01 says nine of twenty-seven anchor. That is the result
//! ```
//!
//! So a briefing is **staged outside the repository**, and `lab/` is made unreadable for the
//! duration of the run — kernel-enforced rather than promised, which is the procedure
//! `agents/05-reconciliation` established and the reason its isolation is believable.
//!
//! # The questions are the claims, and the order is the testimony's
//!
//! Each of the 46 claims becomes the same question: *can this be established from the record you
//! have?* Verbatim, in the order the testimony gave them, identical across the three briefings —
//! which a guard checks, because three agents asked slightly different questions produce three
//! numbers nobody can put beside each other.
//!
//! **Housed and unhoused are not marked**, and neither is the kind. That is the whole measurement:
//! the agent is not being asked to agree with a classification, it is being asked what it can get.

use std::fmt::Write as _;

use crate::articulation::carving::Page;
use crate::classification::Claim;

/// The instructions, **identical** for the three agents — the carving is not named in them.
///
/// Naming it would say there is more than one way to cut this record, which is the experiment. The
/// only thing that varies is what is in `record/`, and a guard asserts the three texts are equal
/// byte for byte.
///
/// It also says nothing about how many answers should come back established, or what the record is
/// for.
pub fn instructions(questions: usize) -> String {
    format!(
        "# What you are doing

You have a record of an organisation's decisions, written by somebody else, in `record/`. You were
not there and nobody is available to ask.

`questions.md` holds {questions} statements that were made about this record. For each one, say
whether **the record you have establishes it**.

## What establishes means

A statement is established if a careful reader with these files, and nothing else, could show it
is true of this record. Not *plausible*, not *consistent with* — shown.

- If the files say it, it is established.
- If the files let you derive it, it is established. Say what you derived it from.
- If believing it needs anything the files do not contain, it is **not** established. That is a
  perfectly ordinary answer and a great many of them may be correct.

Do not treat a statement as established because it sounds like something the record would say, or
because a page repeats it as a remark somebody made. **A remark on a page is somebody's claim, not
the record's evidence for it** — you may still find the evidence elsewhere in the files, and then
it is established by that.

## What to report

Write `answers.md`, with one section per question, numbered as in `questions.md`:

```
## 12
verdict: established | not established
opened: <the files you actually opened to answer this, comma-separated>
because: <one or two sentences — what in the files settles it, or what is missing>
```

Then finish with:

```
## totals
opened at least once: <files>
answered established: <n>
```

**Report the files you actually opened**, not the ones you think you should have. If you answered
from something already read, say so and name it. If you opened a file and it turned out to be
useless, it still counts as opened.

## Rules

- Answer from `record/` only. Do not look anywhere else in the filesystem.
- Do not modify anything.
- Answer all {questions} questions, in order. If you cannot decide, say `not established` and say
  why.
"
    )
}

/// One briefing: the instructions, the questions, and the carving's pages.
pub struct Briefing {
    pub instructions: String,
    pub questions: String,
    pub pages: Vec<Page>,
}

/// The question set, verbatim and in the testimony's order.
///
/// Every claim, housed or not — the housed ones are the baseline and leaving them out would tell an
/// agent which half it was holding.
pub fn questions(claims: &[Claim]) -> String {
    let mut out = String::from(
        "# Statements made about this record\n\nFor each, say whether the record you have \
         establishes it.\n\n",
    );

    for (position, claim) in claims.iter().enumerate() {
        let _ = writeln!(
            out,
            "## {}\n\n{}\n",
            position + 1,
            claim.text.replace('\n', " ")
        );
    }

    out
}

/// Everything one agent is handed.
pub fn brief(pages: Vec<Page>, claims: &[Claim]) -> Briefing {
    Briefing {
        instructions: instructions(claims.len()),
        questions: questions(claims),
        pages,
    }
}
