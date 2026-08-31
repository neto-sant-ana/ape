//! The three carvings, emitted from one run by one generator.
//!
//! *Same content, same substrate, same reader. Only the carving differs.* Written once so that no
//! carving can be quietly favoured by a better hand — which makes this module the place a preference
//! would hide, so every judgement is recorded here rather than in the result.
//!
//! ```text
//! A  FLAT          one document, reading order. Hypertext with no carving
//! B  PER ENTITY    a page per Agent, Commitment, Event, Thesis, plus arms and vocabulary
//! C  PER DECISION  a page per decided world, plus arms and vocabulary
//! ```
//!
//! # Run 1 broke the premise three ways, and the repairs are structural
//!
//! Observation 3 has the measurement. What changed here:
//!
//! **The run has three arms**, so an entity or a decision present in two of them is **one** page
//! carrying an `arms:` relation — an [`EntryId`] and a `ThesisId` are content, so the same entry in
//! two journals is the same entry. That is what makes *the two journals share nineteen* a thing a
//! reader derives rather than is told.
//!
//! **Custody is rendered**, on the arm pages. It is a claim about a journal's extent, so it belongs
//! to neither an entity nor a decision — which is what arm pages are for.
//!
//! **C carries every entry.** Run 1's vocabulary admitted an entry when it had no page of its own
//! *or* was an Agent; Commitment and Event have pages in B, so they fell out of both arms of that
//! condition and C rendered 16 of 21 entries. In C **nothing** has a page but a decision, so the
//! vocabulary is the whole journal, and that is the carving rather than an accident of it.
//!
//! # The judgements the protocol left open, fixed before the run
//!
//! **1. What is not a page in B.** The protocol names four page kinds and the record holds nine
//! entry kinds. Roles, resources, instances, actions, statements and eligibilities are
//! **vocabulary**: they introduce names and carry no reasoning.
//!
//! **2. Arm pages exist in B and C and not in A.** Some facts are about a journal and not about
//! anything in it — how many entries it holds, in what order, and which addresses it comes to. A
//! holds them by holding everything; B and C need somewhere to put them, and putting them on an
//! entity or a decision would be filing a fact under something that is not its subject.
//!
//! **3. Where a claim goes.** [`super::anchor`] says what a claim names; this says where that puts
//! it. B places it on the page of the thing named. C places it on every decision that reaches the
//! thing named — by being it, by naming it in a selection, or by having been taken by it. A claim
//! naming nothing goes to `overflow.md`.
//!
//! **4. A has no overflow, by construction**, and that is the null's whole advantage rather than an
//! oversight. Reported as such: A's overflow is zero because A is one document, not because its
//! carving placed anything.
//!
//! # What is identical across the three
//!
//! Markdown with YAML frontmatter and `[[wikilink]]` relations. Every claim verbatim, in the order
//! the testimony gave it. Every entry, every decision and every world of every arm, in [`ARMS`]
//! order — which a guard checks rather than this docstring promising.

use std::fmt::Write as _;

use crate::articulation::anchor::{self, Anchored};
use crate::articulation::record::{Entry, Record, Run, Taken, World};
use crate::classification::{Claim, Verdict};

/// Which unit the record is cut into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Carving {
    Flat,
    PerEntity,
    PerDecision,
}

impl Carving {
    pub const ALL: [Carving; 3] = [Carving::Flat, Carving::PerEntity, Carving::PerDecision];

    /// The directory this carving is written to.
    pub fn directory(&self) -> &'static str {
        match self {
            Self::Flat => "a-flat",
            Self::PerEntity => "b-per-entity",
            Self::PerDecision => "c-per-decision",
        }
    }
}

/// One page of a carving: where it goes, what it relates to, and what it says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page {
    pub name: String,
    pub frontmatter: Vec<(String, String)>,
    pub body: String,
}

impl Page {
    /// The page as it reaches disk, which is also what a cost measurement counts.
    pub fn rendered(&self) -> String {
        let mut out = String::from("---\n");

        for (key, value) in &self.frontmatter {
            let _ = writeln!(out, "{key}: {value}");
        }
        out.push_str("---\n\n");
        out.push_str(&self.body);

        out
    }
}

/// The short form every page and every link uses for an identity.
pub fn short(id: &str) -> &str {
    &id[..8.min(id.len())]
}

fn link(name: &str) -> String {
    format!("[[{name}]]")
}

/// A relation to several pages, as a comma-separated list of links and nothing else.
///
/// Deliberately not a YAML flow sequence: `[[[a]], [[b]]]` is unreadable to a person and ambiguous
/// to anything scanning for `[[`, which the link guard found by tripping over it.
fn links<'a>(names: impl IntoIterator<Item = &'a str>) -> String {
    let joined: Vec<String> = names.into_iter().map(link).collect();

    if joined.is_empty() {
        "none".to_owned()
    } else {
        joined.join(", ")
    }
}

struct Placed<'a> {
    claim: &'a Claim,
    at: Anchored,
}

/// The claims a carving has to place: everything the record does not already house.
fn to_place<'a>(claims: &'a [Claim], run: &Run) -> Vec<Placed<'a>> {
    claims
        .iter()
        .filter(|claim| !matches!(claim.verdict, Verdict::Housed(_)))
        .map(|claim| Placed {
            claim,
            at: anchor::of(claim.text, run),
        })
        .collect()
}

fn quoted(claim: &Claim) -> String {
    let kind = match &claim.verdict {
        Verdict::Housed(_) => "housed".to_owned(),
        Verdict::Unhoused(Some(kind)) => format!("{kind:?}").to_lowercase(),
        Verdict::Unhoused(None) => "unclassified".to_owned(),
        Verdict::Exposition => "exposition".to_owned(),
    };

    format!("- *({kind})* {}\n", claim.text.replace('\n', " "))
}

/// The name of the page an entity gets under [`Carving::PerEntity`].
fn entity_page(id: &str, entry: &Entry) -> Option<String> {
    match entry {
        Entry::Agent { label, .. } => Some(format!("agent-{label}")),
        Entry::Commitment { .. } => Some(format!("commitment-{}", short(id))),
        Entry::Event { .. } => Some(format!("event-{}", short(id))),
        _ => None,
    }
}

fn thesis_page(id: &str) -> String {
    format!("thesis-{}", short(id))
}

fn decision_page(world: &World) -> String {
    format!("decision-{}", short(&world.thesis))
}

fn arm_page(arm: &str) -> String {
    format!("arm-{arm}")
}

/// Everything the record says about one entry, as lines.
fn entry_body(id: &str, entry: &Entry) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "`{id}`\n");

    match entry {
        Entry::Agent { label, .. } => {
            let _ = writeln!(out, "An Agent, labelled **{label}**.");
        }
        Entry::Commitment {
            accountable,
            executors,
            beneficiaries,
            statement,
            resource,
            committed_at,
            due_date,
            magnitude,
            dependencies,
            ..
        } => {
            let _ = writeln!(out, "A Commitment.\n");
            let _ = writeln!(out, "- accountable: `{accountable}`");
            let _ = writeln!(out, "- executors: {executors:?}");
            let _ = writeln!(out, "- beneficiaries: {beneficiaries:?}");
            let _ = writeln!(out, "- statement: `{statement}`");
            let _ = writeln!(out, "- resource instance: `{resource}`");
            let _ = writeln!(out, "- committed at: {committed_at}, due {due_date}");
            let _ = writeln!(
                out,
                "- magnitude: {}",
                magnitude.as_deref().unwrap_or("none")
            );
            let _ = writeln!(out, "- dependencies: {dependencies:?}");
        }
        Entry::Event {
            commitment,
            observation,
            occurred_at,
            ..
        } => {
            let _ = writeln!(out, "An Event.\n");
            let _ = writeln!(out, "- settles: `{commitment}`");
            let _ = writeln!(out, "- observation: {observation}");
            let _ = writeln!(out, "- occurred at: {occurred_at}");
        }
        other => {
            let _ = writeln!(out, "A {}.\n", other.kind());
            let _ = writeln!(
                out,
                "```json\n{}\n```",
                serde_json::to_string_pretty(other).unwrap_or_default()
            );
        }
    }

    let _ = writeln!(out, "- recorded at: {}", entry.recorded_at());
    out
}

fn decision_body(taken: &Taken, world: &World, run: &Run, holders: &[&'static str]) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "`{}`\n", world.thesis);
    let _ = writeln!(
        out,
        "A **{}**, held by: {}.\n",
        taken.decides,
        holders.join(", ")
    );
    let _ = writeln!(out, "- produces world: `{}`", world.thesis);
    let _ = writeln!(out, "- known at: {}", world.known_at);
    let _ = writeln!(out, "- event head: {:?}", world.event_head);
    let _ = writeln!(out, "- frozen: {:?}", world.frozen);
    let _ = writeln!(out, "- open: {:?}", world.open);
    let _ = writeln!(
        out,
        "- taken by: {}",
        taken
            .by
            .as_deref()
            .map(|by| {
                let named = run
                    .entries()
                    .into_iter()
                    .find(|(at, _, _)| *at == by)
                    .and_then(|(_, entry, _)| entry.label())
                    .unwrap_or("an agent this record does not name")
                    .to_owned();
                format!("`{by}` ({named})")
            })
            .unwrap_or_else(|| "nobody — the decision claims no party".to_owned())
    );
    let _ = writeln!(out, "- taken after entry: `{}`", taken.after);
    let _ = writeln!(out, "- witnessed entries: {}", taken.witness.len());

    if let Some(extends) = &taken.extends {
        let _ = writeln!(out, "- extends: `{extends}`");
    }
    for (name, set) in [
        ("selection", &taken.selection),
        ("omitted", &taken.omitted),
        ("introduced", &taken.introduced),
    ] {
        if !set.is_empty() {
            let _ = writeln!(out, "- {name}: {set:?}");
        }
    }

    out
}

/// What an arm holds, in the order it holds it, and the addresses it comes to.
///
/// The facts that belong to a journal rather than to anything in it. Custody is here for that
/// reason: it is a claim about the journal's extent, and no entity is its subject.
fn arm_body(arm: &Record) -> String {
    let mut out = format!("# {}\n\n", arm.arm);
    let _ = writeln!(
        out,
        "{} journal entries, {} custody addresses, {} decisions, {} worlds.\n",
        arm.journal.len(),
        arm.custody.len(),
        arm.lineage.len(),
        arm.worlds.len()
    );

    out.push_str("## Journal, in the order it was admitted\n\n");
    for (position, (id, entry)) in arm.addressed().into_iter().enumerate() {
        let _ = writeln!(
            out,
            "{}. `{}` — {} recorded {}",
            position + 1,
            id,
            entry.kind(),
            entry.recorded_at()
        );
    }

    out.push_str("\n## Custody — every address this journal comes to\n\n");
    for id in &arm.custody {
        let _ = writeln!(out, "- `{id}`");
    }

    out.push_str("\n## Decisions, in the order taken\n\n");
    for (position, (taken, world)) in arm.decided().into_iter().enumerate() {
        let _ = writeln!(
            out,
            "{}. {} producing `{}`, after `{}`, witnessing {} entries",
            position + 1,
            taken.decides,
            world.thesis,
            taken.after,
            taken.witness.len()
        );
    }

    out
}

/// Cut the run and the claims into pages.
pub fn carve(run: &Run, claims: &[Claim], carving: Carving) -> Vec<Page> {
    let placed = to_place(claims, run);

    match carving {
        Carving::Flat => flat(run, &placed),
        Carving::PerEntity => per_entity(run, &placed),
        Carving::PerDecision => per_decision(run, &placed),
    }
}

/// A — one document, reading order, and nothing overflows because nothing was placed.
fn flat(run: &Run, placed: &[Placed<'_>]) -> Vec<Page> {
    let mut body = String::from("# The record\n\n");

    for arm in &run.arms {
        let _ = writeln!(body, "# Arm: {}\n", arm.arm);
        body.push_str(&arm_body(arm));
        body.push_str("\n## Entries in full\n\n");

        for (id, entry) in arm.addressed() {
            let _ = writeln!(body, "### {} `{}`\n", entry.kind(), short(id));
            body.push_str(&entry_body(id, entry));
            body.push('\n');
        }

        body.push_str("## Decisions in full\n\n");
        for (taken, world) in arm.decided() {
            let _ = writeln!(body, "### `{}`\n", short(&world.thesis));
            body.push_str(&decision_body(taken, world, run, &[arm.arm]));
            body.push('\n');
        }
    }

    body.push_str("# What was said about it\n\n");
    for entry in placed {
        body.push_str(&quoted(entry.claim));
    }

    vec![Page {
        name: "record".to_owned(),
        frontmatter: vec![
            ("kind".to_owned(), "record".to_owned()),
            (
                "arms".to_owned(),
                run.arms
                    .iter()
                    .map(|arm| arm.arm)
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
        ],
        body,
    }]
}

/// The arm pages, which B and C both carry.
fn arms(run: &Run) -> Vec<Page> {
    run.arms
        .iter()
        .map(|arm| Page {
            name: arm_page(arm.arm),
            frontmatter: vec![
                ("kind".to_owned(), "arm".to_owned()),
                ("entries".to_owned(), arm.journal.len().to_string()),
                ("custody".to_owned(), arm.custody.len().to_string()),
                ("decisions".to_owned(), arm.lineage.len().to_string()),
                ("worlds".to_owned(), arm.worlds.len().to_string()),
            ],
            body: arm_body(arm),
        })
        .collect()
}

/// B — a page per Agent, Commitment, Event and Thesis, plus arms and vocabulary.
fn per_entity(run: &Run, placed: &[Placed<'_>]) -> Vec<Page> {
    let mut pages = arms(run);
    let mut vocabulary = String::from("# Vocabulary\n\nThe entries that introduce names.\n\n");

    for (id, entry, holders) in run.entries() {
        let Some(name) = entity_page(id, entry) else {
            let _ = writeln!(
                vocabulary,
                "## {} `{}` — in {}\n",
                entry.kind(),
                short(id),
                holders.join(", ")
            );
            vocabulary.push_str(&entry_body(id, entry));
            vocabulary.push('\n');
            continue;
        };

        let mut body = format!("# {name}\n\n{}", entry_body(id, entry));
        let mine: Vec<&Placed<'_>> = placed
            .iter()
            .filter(|at| at.at.identities.contains(id) || at.at.parties.contains(id))
            .collect();

        if !mine.is_empty() {
            body.push_str("\n## What was said about it\n\n");
            for entry in &mine {
                body.push_str(&quoted(entry.claim));
            }
        }

        let mut frontmatter = vec![
            ("kind".to_owned(), entry.kind().to_owned()),
            ("identity".to_owned(), id.to_owned()),
            (
                "arms".to_owned(),
                links(
                    holders
                        .iter()
                        .map(|arm| arm_page(arm))
                        .collect::<Vec<_>>()
                        .iter()
                        .map(String::as_str),
                ),
            ),
        ];

        match entry {
            Entry::Agent { .. } => {
                let decided: Vec<String> = run
                    .decisions()
                    .into_iter()
                    .filter(|(_, taken, _)| taken.by.as_deref() == Some(id))
                    .map(|(world, _, _)| thesis_page(&world.thesis))
                    .collect();

                frontmatter.push((
                    "decided".to_owned(),
                    links(decided.iter().map(String::as_str)),
                ));
            }
            _ => {
                let held_by: Vec<String> = run
                    .decisions()
                    .into_iter()
                    .filter(|(world, _, _)| {
                        world
                            .frozen
                            .iter()
                            .chain(&world.open)
                            .any(|held| held == id)
                    })
                    .map(|(world, _, _)| thesis_page(&world.thesis))
                    .collect();

                frontmatter.push((
                    "held_by".to_owned(),
                    links(held_by.iter().map(String::as_str)),
                ));
            }
        }

        frontmatter.push(("claims".to_owned(), mine.len().to_string()));
        pages.push(Page {
            name,
            frontmatter,
            body,
        });
    }

    for (world, taken, holders) in run.decisions() {
        let name = thesis_page(&world.thesis);
        let mut body = format!("# {name}\n\n");
        body.push_str(&decision_body(taken, world, run, &holders));

        let mine: Vec<&Placed<'_>> = placed
            .iter()
            .filter(|at| at.at.identities.contains(&world.thesis))
            .collect();

        if !mine.is_empty() {
            body.push_str("\n## What was said about it\n\n");
            for entry in &mine {
                body.push_str(&quoted(entry.claim));
            }
        }

        let holds: Vec<String> = world
            .frozen
            .iter()
            .chain(&world.open)
            .filter_map(|held| {
                run.entries()
                    .into_iter()
                    .find(|(at, _, _)| at == held)
                    .and_then(|(at, entry, _)| entity_page(at, entry))
            })
            .collect();

        pages.push(Page {
            name,
            frontmatter: vec![
                ("kind".to_owned(), "thesis".to_owned()),
                ("identity".to_owned(), world.thesis.clone()),
                (
                    "arms".to_owned(),
                    links(
                        holders
                            .iter()
                            .map(|arm| arm_page(arm))
                            .collect::<Vec<_>>()
                            .iter()
                            .map(String::as_str),
                    ),
                ),
                (
                    "parent".to_owned(),
                    world
                        .thesis_parent
                        .as_deref()
                        .map(|id| link(&thesis_page(id)))
                        .unwrap_or_else(|| "none".to_owned()),
                ),
                ("holds".to_owned(), links(holds.iter().map(String::as_str))),
                ("claims".to_owned(), mine.len().to_string()),
            ],
            body,
        });
    }

    pages.push(Page {
        name: "vocabulary".to_owned(),
        frontmatter: vec![("kind".to_owned(), "vocabulary".to_owned())],
        body: vocabulary,
    });
    pages.push(overflow(placed, |at| at.is_empty()));

    pages
}

/// C — a page per decided world, plus arms and a vocabulary of **every** entry.
///
/// Nothing but a decision has a page here, so the vocabulary is the whole journal — which is what
/// *entities are linked and not owned* means, and what Run 1 got wrong by dropping the five entries
/// that happened to have pages in B.
fn per_decision(run: &Run, placed: &[Placed<'_>]) -> Vec<Page> {
    let mut pages = arms(run);
    let mut vocabulary =
        String::from("# Vocabulary\n\nEvery entry any arm of this record admitted.\n\n");

    for (id, entry, holders) in run.entries() {
        let _ = writeln!(
            vocabulary,
            "## {} `{}` — in {}\n",
            entry.kind(),
            short(id),
            holders.join(", ")
        );
        vocabulary.push_str(&entry_body(id, entry));
        vocabulary.push('\n');
    }

    for (world, taken, holders) in run.decisions() {
        let name = decision_page(world);
        let mut body = format!("# {name}\n\n");
        body.push_str(&decision_body(taken, world, run, &holders));

        let mine: Vec<&Placed<'_>> = placed
            .iter()
            .filter(|at| reaches(taken, world, &at.at))
            .collect();

        if !mine.is_empty() {
            body.push_str("\n## What was said about it\n\n");
            for entry in &mine {
                body.push_str(&quoted(entry.claim));
            }
        }

        pages.push(Page {
            name,
            frontmatter: vec![
                ("kind".to_owned(), format!("decision ({})", taken.decides)),
                ("world".to_owned(), world.thesis.clone()),
                (
                    "arms".to_owned(),
                    links(
                        holders
                            .iter()
                            .map(|arm| arm_page(arm))
                            .collect::<Vec<_>>()
                            .iter()
                            .map(String::as_str),
                    ),
                ),
                (
                    "extends".to_owned(),
                    taken
                        .extends
                        .as_deref()
                        .and_then(|id| {
                            run.decisions()
                                .into_iter()
                                .find(|(world, _, _)| world.thesis == id)
                                .map(|(world, _, _)| link(&decision_page(world)))
                        })
                        .unwrap_or_else(|| "none — a genesis".to_owned()),
                ),
                (
                    "by".to_owned(),
                    taken.by.clone().unwrap_or_else(|| "nobody".to_owned()),
                ),
                ("about".to_owned(), format!("{:?}", taken.commitments())),
                ("claims".to_owned(), mine.len().to_string()),
            ],
            body,
        });
    }

    pages.push(Page {
        name: "vocabulary".to_owned(),
        frontmatter: vec![("kind".to_owned(), "vocabulary".to_owned())],
        body: vocabulary,
    });
    pages.push(overflow(placed, |at| {
        !run.decisions()
            .into_iter()
            .any(|(world, taken, _)| reaches(taken, world, at))
    }));

    pages
}

/// Whether a decision reaches what a claim names — by being it, holding it, or having taken it.
fn reaches(taken: &Taken, world: &World, at: &Anchored) -> bool {
    let names_world = at.identities.contains(&world.thesis);
    let names_commitment = taken
        .commitments()
        .into_iter()
        .any(|held| at.identities.contains(held));
    let taken_by = taken
        .by
        .as_deref()
        .is_some_and(|by| at.parties.contains(by));

    names_world || names_commitment || taken_by
}

/// The claims a carving could not place, and the protocol says its size is a result.
fn overflow(placed: &[Placed<'_>], unplaced: impl Fn(&Anchored) -> bool) -> Page {
    let mine: Vec<&Placed<'_>> = placed.iter().filter(|at| unplaced(&at.at)).collect();
    let mut body = String::from(
        "# Overflow\n\nWhat was said about this record that is not about any one part of it.\n\n",
    );

    for entry in &mine {
        body.push_str(&quoted(entry.claim));
    }

    Page {
        name: "overflow".to_owned(),
        frontmatter: vec![
            ("kind".to_owned(), "overflow".to_owned()),
            ("claims".to_owned(), mine.len().to_string()),
        ],
        body,
    }
}

/// How many claims each carving placed, and how many it could not.
pub fn placement(run: &Run, claims: &[Claim], carving: Carving) -> Placement {
    let pages = carve(run, claims, carving);
    let overflowed = pages
        .iter()
        .find(|page| page.name == "overflow")
        .and_then(|page| {
            page.frontmatter
                .iter()
                .find(|(key, _)| key == "claims")
                .and_then(|(_, count)| count.parse().ok())
        })
        .unwrap_or(0);

    let to_place = to_place(claims, run).len();

    Placement {
        pages: pages.len(),
        bytes: pages.iter().map(|page| page.rendered().len()).sum(),
        to_place,
        overflowed,
        placed: to_place - overflowed,
    }
}

/// What one carving costs and reaches, in the units the protocol asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    pub pages: usize,
    pub bytes: usize,
    pub to_place: usize,
    pub placed: usize,
    pub overflowed: usize,
}
