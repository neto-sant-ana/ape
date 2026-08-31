//! The three carvings, emitted from one record by one generator.
//!
//! *Same content, same substrate, same reader. Only the carving differs.* Written once so that no
//! carving can be quietly favoured by a better hand — which makes this module the place a preference
//! would hide, so every judgement is recorded here rather than in the result.
//!
//! ```text
//! A  FLAT          one document, reading order. Hypertext with no carving
//! B  PER ENTITY    a page per Agent, Commitment, Event, Thesis — plus vocabulary
//! C  PER DECISION  a page per `Taken` — plus vocabulary
//! ```
//!
//! # The four judgements the protocol left open, fixed here and before any run
//!
//! **1. What is *not* a page in B.** The protocol names four page kinds and the record holds nine
//! entry kinds. The other five — role, resource, resource-instance, action, statement — plus the
//! eligibilities are **vocabulary**: they introduce names and carry no reasoning. Twelve of the
//! twenty-one entries. They go on one `vocabulary.md`, rather than becoming twelve more pages that
//! no claim anchors to.
//!
//! **2. C gets that same page.** The protocol says C is five decisions, and it would be cheating C's
//! cost to let it drop the vocabulary B has to carry. One page each, identical content, so the
//! page-count asymmetry P4 predicts is between **15 and 6** and not between 15 and 5.
//!
//! **3. Where a claim goes.** [`super::anchor`] says what a claim names; this says where that puts
//! it. B places it on the page of the thing named. C places it on the page of every decision that
//! reaches the thing named — by being it, by naming it in a selection, or by having been taken by
//! it. A claim naming nothing goes to `overflow.md`.
//!
//! **4. A has no overflow, by construction**, and that is the null's whole advantage rather than an
//! oversight. It is reported as such: A's overflow is zero because A is one document, not because
//! its carving placed anything.
//!
//! # What is identical across the three
//!
//! Every page is markdown with YAML frontmatter and `[[wikilink]]` relations, including A's single
//! document — the substrate is held constant and the protocol says choosing markdown is arbitrary.
//! Every claim appears **verbatim**, from the classification, in the order the testimony gave it.

use std::fmt::Write as _;

use crate::articulation::anchor::{self, Anchored};
use crate::articulation::record::{Entry, Record, Taken, World};
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

    /// The directory this carving is written to, and the name an agent is given.
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
    /// `key: value` lines, in the order they are written. Relations are `[[wikilinks]]`.
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
///
/// Eight hex digits, which is what the corpus itself abbreviates to — so a reader moving between the
/// testimony and a carving sees the same string. Uniqueness is checked by the guard rather than
/// assumed here.
pub fn short(id: &str) -> &str {
    &id[..8.min(id.len())]
}

fn link(name: &str) -> String {
    format!("[[{name}]]")
}

/// A relation to several pages, as a comma-separated list of links and nothing else.
///
/// Deliberately not wrapped in a YAML flow sequence: `[[[a]], [[b]]]` is unreadable to a person and
/// ambiguous to anything scanning for `[[`, which the link guard found by tripping over it. An empty
/// relation says `none` rather than `[]`, because a reader should not have to know which of the two
/// brackets is the list.
fn links<'a>(names: impl IntoIterator<Item = &'a str>) -> String {
    let joined: Vec<String> = names.into_iter().map(link).collect();

    if joined.is_empty() {
        "none".to_owned()
    } else {
        joined.join(", ")
    }
}

/// A claim, and everything it attaches to.
struct Placed<'a> {
    claim: &'a Claim,
    at: Anchored,
}

/// The claims a carving has to place: everything the record does not already house.
///
/// The housed ones are the baseline and are not reasoning — a carving that also wrote them down
/// would be measuring whether a reader prefers to read the same fact twice.
fn to_place<'a>(claims: &'a [Claim], record: &Record) -> Vec<Placed<'a>> {
    claims
        .iter()
        .filter(|claim| !matches!(claim.verdict, Verdict::Housed(_)))
        .map(|claim| Placed {
            claim,
            at: anchor::of(claim.text, record),
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

    format!("- *({kind})* {}\n", claim.claim_text())
}

impl Claim {
    /// The claim's words, with the newlines a rendered page needs.
    fn claim_text(&self) -> String {
        self.text.replace('\n', " ")
    }
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

/// The page a decision gets, named by the world it produced rather than by its kind.
///
/// `Taken::decides` is the variant tag, so naming a page from it gives `decision-2-fork` — three of
/// which would collide in a record with three forks. The world identity is the decision's own name
/// and is what a claim can reach.
fn decision_page(position: usize, world: &World) -> String {
    format!("decision-{}-{}", position + 1, short(&world.thesis))
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

fn decision_body(position: usize, taken: &Taken, world: &World, record: &Record) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "`{}`\n", world.thesis);
    let _ = writeln!(
        out,
        "Decision {} of {}, a **{}**.\n",
        position + 1,
        record.lineage.len(),
        taken.decides
    );
    let _ = writeln!(out, "- produces world: `{}`", world.thesis);
    let _ = writeln!(out, "- known at: {}", world.known_at);
    let _ = writeln!(out, "- frozen: {:?}", world.frozen);
    let _ = writeln!(out, "- open: {:?}", world.open);
    let _ = writeln!(
        out,
        "- taken by: {}",
        taken
            .by
            .as_deref()
            .map(|by| {
                let named = record
                    .addressed()
                    .into_iter()
                    .find(|(at, _)| *at == by)
                    .and_then(|(_, entry)| entry.label())
                    .unwrap_or("an agent this record does not name");
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

/// Cut the record and the claims into pages.
pub fn carve(record: &Record, claims: &[Claim], carving: Carving) -> Vec<Page> {
    let placed = to_place(claims, record);

    match carving {
        Carving::Flat => flat(record, &placed),
        Carving::PerEntity => per_entity(record, &placed),
        Carving::PerDecision => per_decision(record, &placed),
    }
}

/// A — one document, reading order, and nothing overflows because nothing was placed.
fn flat(record: &Record, placed: &[Placed<'_>]) -> Vec<Page> {
    let mut body = String::from("# The record\n\n## Journal\n\n");

    for (id, entry) in record.addressed() {
        let _ = writeln!(body, "### {} `{}`\n", entry.kind(), short(id));
        body.push_str(&entry_body(id, entry));
        body.push('\n');
    }

    body.push_str("## Lineage\n\n");
    for (position, (taken, world)) in record.decided().into_iter().enumerate() {
        let _ = writeln!(body, "### Decision {}\n", position + 1);
        body.push_str(&decision_body(position, taken, world, record));
        body.push('\n');
    }

    body.push_str("## Worlds\n\n");
    for world in &record.worlds {
        let _ = writeln!(body, "### `{}`\n", short(&world.thesis));
        let _ = writeln!(body, "- known at: {}", world.known_at);
        let _ = writeln!(body, "- parent: {:?}", world.thesis_parent);
        let _ = writeln!(body, "- frozen: {:?}", world.frozen);
        let _ = writeln!(body, "- open: {:?}\n", world.open);
    }

    body.push_str("## What was said about it\n\n");
    for entry in placed {
        body.push_str(&quoted(entry.claim));
    }

    vec![Page {
        name: "record".to_owned(),
        frontmatter: vec![
            ("carving".to_owned(), "flat".to_owned()),
            ("pages".to_owned(), "1".to_owned()),
        ],
        body,
    }]
}

/// B — a page per Agent, Commitment, Event and Thesis, plus one for the vocabulary.
fn per_entity(record: &Record, placed: &[Placed<'_>]) -> Vec<Page> {
    let mut pages = Vec::new();
    let mut vocabulary = String::from("# Vocabulary\n\nThe entries that introduce names.\n\n");

    for (id, entry) in record.addressed() {
        let Some(name) = entity_page(id, entry) else {
            let _ = writeln!(vocabulary, "## {} `{}`\n", entry.kind(), short(id));
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
        ];

        // A relation only where the record has one to state. An Agent is in no world's selection, so
        // emitting an always-empty `held_by` on every agent page would tell a reader that agents
        // relate to nothing — which is a claim the record does not make.
        match entry {
            // In B a decision has no page of its own — the world it produced does. So an agent's
            // relation is to the theses it decided, and not to a `decision-*` page that exists only
            // in C. Every link a carving writes has to land inside that carving.
            Entry::Agent { .. } => {
                let decided: Vec<String> = record
                    .decided()
                    .into_iter()
                    .filter(|(taken, _)| taken.by.as_deref() == Some(id))
                    .map(|(_, world)| thesis_page(&world.thesis))
                    .collect();

                frontmatter.push((
                    "decided".to_owned(),
                    links(decided.iter().map(String::as_str)),
                ));
            }
            _ => {
                let held_by: Vec<String> = record
                    .worlds
                    .iter()
                    .filter(|world| {
                        world
                            .frozen
                            .iter()
                            .chain(&world.open)
                            .any(|held| held == id)
                    })
                    .map(|world| thesis_page(&world.thesis))
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

    for (position, world) in record.worlds.iter().enumerate() {
        let name = thesis_page(&world.thesis);
        let mut body = format!("# {name}\n\n`{}`\n\n", world.thesis);
        let _ = writeln!(body, "- known at: {}", world.known_at);
        let _ = writeln!(body, "- event head: {:?}", world.event_head);
        let _ = writeln!(body, "- frozen: {:?}", world.frozen);
        let _ = writeln!(body, "- open: {:?}", world.open);

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
                record
                    .addressed()
                    .into_iter()
                    .find(|(at, _)| at == held)
                    .and_then(|(at, entry)| entity_page(at, entry))
            })
            .collect();

        pages.push(Page {
            name,
            frontmatter: vec![
                ("kind".to_owned(), "thesis".to_owned()),
                ("identity".to_owned(), world.thesis.clone()),
                (
                    "parent".to_owned(),
                    world
                        .thesis_parent
                        .as_deref()
                        .map(|id| link(&thesis_page(id)))
                        .unwrap_or_else(|| "none".to_owned()),
                ),
                ("holds".to_owned(), links(holds.iter().map(String::as_str))),
                ("position".to_owned(), (position + 1).to_string()),
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
        at.identities.is_empty() && at.parties.is_empty()
    }));

    pages
}

/// C — a page per decision, plus the same vocabulary page B carries.
fn per_decision(record: &Record, placed: &[Placed<'_>]) -> Vec<Page> {
    let mut pages = Vec::new();
    let mut vocabulary = String::from("# Vocabulary\n\nThe entries that introduce names.\n\n");

    for (id, entry) in record.addressed() {
        if entity_page(id, entry).is_none() || matches!(entry, Entry::Agent { .. }) {
            let _ = writeln!(vocabulary, "## {} `{}`\n", entry.kind(), short(id));
            vocabulary.push_str(&entry_body(id, entry));
            vocabulary.push('\n');
        }
    }

    for (position, (taken, world)) in record.decided().into_iter().enumerate() {
        let name = decision_page(position, world);
        let mut body = format!("# Decision {}\n\n", position + 1);
        body.push_str(&decision_body(position, taken, world, record));

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

        let about: Vec<&str> = taken.commitments();

        pages.push(Page {
            name,
            frontmatter: vec![
                ("kind".to_owned(), format!("decision ({})", taken.decides)),
                ("world".to_owned(), world.thesis.clone()),
                (
                    "extends".to_owned(),
                    taken
                        .extends
                        .as_deref()
                        .and_then(|id| {
                            let at = record.worlds.iter().position(|world| world.thesis == id)?;

                            Some(link(&decision_page(at, &record.worlds[at])))
                        })
                        .unwrap_or_else(|| "none — a genesis".to_owned()),
                ),
                (
                    "by".to_owned(),
                    taken.by.clone().unwrap_or_else(|| "nobody".to_owned()),
                ),
                ("about".to_owned(), format!("{about:?}")),
                ("position".to_owned(), (position + 1).to_string()),
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
        !record
            .decided()
            .into_iter()
            .any(|(taken, world)| reaches(taken, world, at))
    }));

    pages
}

/// Whether a decision reaches what a claim names — by being it, holding it, or having taken it.
///
/// Three ways rather than one, because C's own description is *entities are linked and not owned*: a
/// claim about a commitment belongs where that commitment was decided, not nowhere. Fixed before the
/// run, and it is the rule that makes C's reach comparable to B's rather than smaller by
/// construction.
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
        "# Overflow\n\nWhat was said about this record that attaches to no page of this carving.\n\n",
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
///
/// Reported together because a carving that placed everything by having one page is not the same
/// achievement as one that placed everything across many.
pub fn placement(record: &Record, claims: &[Claim], carving: Carving) -> Placement {
    let pages = carve(record, claims, carving);
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

    let to_place = to_place(claims, record).len();

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
