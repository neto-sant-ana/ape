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
use crate::articulation::words::{Lang, Words};
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
fn entity_page(id: &str, entry: &Entry, w: &Words) -> Option<String> {
    match entry {
        Entry::Agent { label, .. } => Some(format!("{}-{label}", w.page_agent)),
        Entry::Commitment { .. } => Some(format!("{}-{}", w.page_commitment, short(id))),
        Entry::Event { .. } => Some(format!("{}-{}", w.page_event, short(id))),
        _ => None,
    }
}

fn thesis_page(id: &str, w: &Words) -> String {
    format!("{}-{}", w.page_thesis, short(id))
}

fn decision_page(world: &World, w: &Words) -> String {
    format!("{}-{}", w.page_decision, short(&world.thesis))
}

fn arm_page(arm: &str, w: &Words) -> String {
    format!("{}-{arm}", w.page_arm)
}

/// Everything the record says about one entry, as lines.
fn entry_body(id: &str, entry: &Entry, w: &Words) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "`{id}`\n");

    match entry {
        Entry::Agent { label, .. } => {
            let _ = writeln!(out, "{} **{label}**.", w.an_agent);
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
            let _ = writeln!(out, "{}\n", w.a_commitment);
            let _ = writeln!(out, "- {}: `{accountable}`", w.accountable);
            let _ = writeln!(out, "- {}: {executors:?}", w.executors);
            let _ = writeln!(out, "- {}: {beneficiaries:?}", w.beneficiaries);
            let _ = writeln!(out, "- {}: `{statement}`", w.statement);
            let _ = writeln!(out, "- {}: `{resource}`", w.resource_instance);
            let _ = writeln!(
                out,
                "- {}: {committed_at}, {} {due_date}",
                w.committed_at, w.due
            );
            let _ = writeln!(
                out,
                "- {}: {}",
                w.magnitude,
                magnitude.as_deref().unwrap_or(w.none)
            );
            let _ = writeln!(out, "- {}: {dependencies:?}", w.dependencies);
        }
        Entry::Event {
            commitment,
            observation,
            occurred_at,
            ..
        } => {
            let _ = writeln!(out, "{}\n", w.an_event);
            let _ = writeln!(out, "- {}: `{commitment}`", w.settles);
            let _ = writeln!(out, "- {}: {observation}", w.observation);
            let _ = writeln!(out, "- {}: {occurred_at}", w.occurred_at);
        }
        other => {
            let _ = writeln!(out, "{} {}.\n", w.a_kind, w.kind(other.kind()));
            let _ = writeln!(
                out,
                "```json\n{}\n```",
                serde_json::to_string_pretty(other).unwrap_or_default()
            );
        }
    }

    let _ = writeln!(out, "- {}: {}", w.recorded_at, entry.recorded_at());
    out
}

fn decision_body(
    taken: &Taken,
    world: &World,
    run: &Run,
    holders: &[&'static str],
    w: &Words,
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "`{}`\n", world.thesis);
    let _ = writeln!(
        out,
        "{} **{}**, {}: {}.\n",
        w.a_kind,
        w.decides(&taken.decides),
        w.held_by,
        holders.join(", ")
    );
    let _ = writeln!(out, "- {}: `{}`", w.produces_world, world.thesis);
    let _ = writeln!(out, "- {}: {}", w.known_at, world.known_at);
    let _ = writeln!(out, "- {}: {:?}", w.event_head, world.event_head);
    let _ = writeln!(out, "- {}: {:?}", w.frozen, world.frozen);
    let _ = writeln!(out, "- {}: {:?}", w.open, world.open);
    let _ = writeln!(
        out,
        "- {}: {}",
        w.taken_by,
        taken
            .by
            .as_deref()
            .map(|by| {
                let named = run
                    .entries()
                    .into_iter()
                    .find(|(at, _, _)| *at == by)
                    .and_then(|(_, entry, _)| entry.label())
                    .unwrap_or(w.unnamed_agent)
                    .to_owned();
                format!("`{by}` ({named})")
            })
            .unwrap_or_else(|| w.nobody.to_owned())
    );
    let _ = writeln!(out, "- {}: `{}`", w.taken_after, taken.after);
    let _ = writeln!(out, "- {}: {}", w.witnessed, taken.witness.len());

    if let Some(extends) = &taken.extends {
        let _ = writeln!(out, "- {}: `{extends}`", w.extends);
    }
    for (name, set) in [
        (w.selection, &taken.selection),
        (w.omitted, &taken.omitted),
        (w.introduced, &taken.introduced),
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
fn arm_body(arm: &Record, w: &Words) -> String {
    let mut out = format!("# {}\n\n", arm.arm);
    let _ = writeln!(
        out,
        "{}\n",
        w.counted(
            arm.journal.len(),
            arm.custody.len(),
            arm.lineage.len(),
            arm.worlds.len()
        )
    );

    let _ = writeln!(out, "## {}\n", w.journal_in_order);
    for (position, (id, entry)) in arm.addressed().into_iter().enumerate() {
        let _ = writeln!(
            out,
            "{}. `{}` — {} {} {}",
            position + 1,
            id,
            w.kind(entry.kind()),
            w.recorded_at,
            entry.recorded_at()
        );
    }

    let _ = writeln!(out, "\n## {}\n", w.custody_section);
    for id in &arm.custody {
        let _ = writeln!(out, "- `{id}`");
    }

    let _ = writeln!(out, "\n## {}\n", w.decisions_in_order);
    for (position, (taken, world)) in arm.decided().into_iter().enumerate() {
        let _ = writeln!(
            out,
            "{}. {} {} `{}`, {} `{}`, {} {} {}",
            position + 1,
            w.decides(&taken.decides),
            w.producing,
            world.thesis,
            w.after,
            taken.after,
            w.witnessing,
            taken.witness.len(),
            w.witnessed
        );
    }

    out
}

/// Cut the run and the claims into pages.
pub fn carve(run: &Run, claims: &[Claim], carving: Carving) -> Vec<Page> {
    carve_in(run, claims, carving, Lang::English)
}

/// The same, in the words a given reader reads. See [`crate::articulation::words`].
pub fn carve_in(run: &Run, claims: &[Claim], carving: Carving, lang: Lang) -> Vec<Page> {
    let placed = to_place(claims, run);
    let w = lang.words();

    match carving {
        Carving::Flat => flat(run, &placed, w),
        Carving::PerEntity => per_entity(run, &placed, w),
        Carving::PerDecision => per_decision(run, &placed, w),
    }
}

/// A — one document, reading order, and nothing overflows because nothing was placed.
fn flat(run: &Run, placed: &[Placed<'_>], w: &Words) -> Vec<Page> {
    let mut body = format!("# {}\n\n", w.record);

    for arm in &run.arms {
        let _ = writeln!(body, "# {}: {}\n", w.arm, arm.arm);
        body.push_str(&arm_body(arm, w));
        let _ = writeln!(body, "\n## {}\n", w.entries_in_full);

        for (id, entry) in arm.addressed() {
            let _ = writeln!(body, "### {} `{}`\n", w.kind(entry.kind()), short(id));
            body.push_str(&entry_body(id, entry, w));
            body.push('\n');
        }

        let _ = writeln!(body, "## {}\n", w.decisions_in_full);
        for (taken, world) in arm.decided() {
            let _ = writeln!(body, "### `{}`\n", short(&world.thesis));
            body.push_str(&decision_body(taken, world, run, &[arm.arm], w));
            body.push('\n');
        }
    }

    let _ = writeln!(body, "# {}\n", w.said);
    for entry in placed {
        body.push_str(&quoted(entry.claim));
    }

    vec![Page {
        name: w.page_record.to_owned(),
        frontmatter: vec![
            ("kind".to_owned(), w.page_record.to_owned()),
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
fn arms(run: &Run, w: &Words) -> Vec<Page> {
    run.arms
        .iter()
        .map(|arm| Page {
            name: arm_page(arm.arm, w),
            frontmatter: vec![
                ("kind".to_owned(), w.page_arm.to_owned()),
                ("entries".to_owned(), arm.journal.len().to_string()),
                ("custody".to_owned(), arm.custody.len().to_string()),
                ("decisions".to_owned(), arm.lineage.len().to_string()),
                ("worlds".to_owned(), arm.worlds.len().to_string()),
            ],
            body: arm_body(arm, w),
        })
        .collect()
}

/// B — a page per Agent, Commitment, Event and Thesis, plus arms and vocabulary.
fn per_entity(run: &Run, placed: &[Placed<'_>], w: &Words) -> Vec<Page> {
    let mut pages = arms(run, w);
    let mut vocabulary = format!("# {}\n\n{}\n\n", w.vocabulary, w.vocabulary_note_b);

    for (id, entry, holders) in run.entries() {
        let Some(name) = entity_page(id, entry, w) else {
            let _ = writeln!(
                vocabulary,
                "## {} `{}` — {} {}\n",
                w.kind(entry.kind()),
                short(id),
                w.in_arms,
                holders.join(", ")
            );
            vocabulary.push_str(&entry_body(id, entry, w));
            vocabulary.push('\n');
            continue;
        };

        let mut body = format!("# {name}\n\n{}", entry_body(id, entry, w));
        let mine: Vec<&Placed<'_>> = placed
            .iter()
            .filter(|at| at.at.identities.contains(id) || at.at.parties.contains(id))
            .collect();

        if !mine.is_empty() {
            let _ = writeln!(body, "\n## {}\n", w.said);
            for entry in &mine {
                body.push_str(&quoted(entry.claim));
            }
        }

        let mut frontmatter = vec![
            ("kind".to_owned(), w.kind(entry.kind()).to_owned()),
            ("identity".to_owned(), id.to_owned()),
            (
                "arms".to_owned(),
                links(
                    holders
                        .iter()
                        .map(|arm| arm_page(arm, w))
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
                    .map(|(world, _, _)| thesis_page(&world.thesis, w))
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
                    .map(|(world, _, _)| thesis_page(&world.thesis, w))
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
        let name = thesis_page(&world.thesis, w);
        let mut body = format!("# {name}\n\n");
        body.push_str(&decision_body(taken, world, run, &holders, w));

        let mine: Vec<&Placed<'_>> = placed
            .iter()
            .filter(|at| at.at.identities.contains(&world.thesis))
            .collect();

        if !mine.is_empty() {
            let _ = writeln!(body, "\n## {}\n", w.said);
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
                    .and_then(|(at, entry, _)| entity_page(at, entry, w))
            })
            .collect();

        pages.push(Page {
            name,
            frontmatter: vec![
                ("kind".to_owned(), w.page_thesis.to_owned()),
                ("identity".to_owned(), world.thesis.clone()),
                (
                    "arms".to_owned(),
                    links(
                        holders
                            .iter()
                            .map(|arm| arm_page(arm, w))
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
                        .map(|id| link(&thesis_page(id, w)))
                        .unwrap_or_else(|| "none".to_owned()),
                ),
                ("holds".to_owned(), links(holds.iter().map(String::as_str))),
                ("claims".to_owned(), mine.len().to_string()),
            ],
            body,
        });
    }

    pages.push(Page {
        name: w.page_vocabulary.to_owned(),
        frontmatter: vec![("kind".to_owned(), w.page_vocabulary.to_owned())],
        body: vocabulary,
    });
    pages.push(overflow(placed, w, |at| at.is_empty()));

    pages
}

/// C — a page per decided world, plus arms and a vocabulary of **every** entry.
///
/// Nothing but a decision has a page here, so the vocabulary is the whole journal — which is what
/// *entities are linked and not owned* means, and what Run 1 got wrong by dropping the five entries
/// that happened to have pages in B.
fn per_decision(run: &Run, placed: &[Placed<'_>], w: &Words) -> Vec<Page> {
    let mut pages = arms(run, w);
    let mut vocabulary = format!("# {}\n\n{}\n\n", w.vocabulary, w.vocabulary_note_c);

    for (id, entry, holders) in run.entries() {
        let _ = writeln!(
            vocabulary,
            "## {} `{}` — {} {}\n",
            w.kind(entry.kind()),
            short(id),
            w.in_arms,
            holders.join(", ")
        );
        vocabulary.push_str(&entry_body(id, entry, w));
        vocabulary.push('\n');
    }

    for (world, taken, holders) in run.decisions() {
        let name = decision_page(world, w);
        let mut body = format!("# {name}\n\n");
        body.push_str(&decision_body(taken, world, run, &holders, w));

        let mine: Vec<&Placed<'_>> = placed
            .iter()
            .filter(|at| reaches(taken, world, &at.at))
            .collect();

        if !mine.is_empty() {
            let _ = writeln!(body, "\n## {}\n", w.said);
            for entry in &mine {
                body.push_str(&quoted(entry.claim));
            }
        }

        pages.push(Page {
            name,
            frontmatter: vec![
                (
                    "kind".to_owned(),
                    format!("{} ({})", w.decision, w.decides(&taken.decides)),
                ),
                ("world".to_owned(), world.thesis.clone()),
                (
                    "arms".to_owned(),
                    links(
                        holders
                            .iter()
                            .map(|arm| arm_page(arm, w))
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
                                .map(|(world, _, _)| link(&decision_page(world, w)))
                        })
                        .unwrap_or_else(|| format!("{} — {}", w.none, w.genesis)),
                ),
                (
                    w.taken_by.to_owned(),
                    taken.by.clone().unwrap_or_else(|| w.none.to_owned()),
                ),
                ("about".to_owned(), format!("{:?}", taken.commitments())),
                ("claims".to_owned(), mine.len().to_string()),
            ],
            body,
        });
    }

    pages.push(Page {
        name: w.page_vocabulary.to_owned(),
        frontmatter: vec![("kind".to_owned(), w.page_vocabulary.to_owned())],
        body: vocabulary,
    });
    pages.push(overflow(placed, w, |at| {
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
fn overflow(placed: &[Placed<'_>], w: &Words, unplaced: impl Fn(&Anchored) -> bool) -> Page {
    let mine: Vec<&Placed<'_>> = placed.iter().filter(|at| unplaced(&at.at)).collect();
    let mut body = format!("# {}\n\n{}\n\n", w.overflow, w.overflow_note);

    for entry in &mine {
        body.push_str(&quoted(entry.claim));
    }

    Page {
        name: w.page_overflow.to_owned(),
        frontmatter: vec![
            ("kind".to_owned(), w.page_overflow.to_owned()),
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
