//! The atomicity experiment, run phase by phase.
//!
//! Each phase is a test rather than a program, for the reason the reconstruction experiment gives: a
//! comparison has to fail loudly. What is different here is that nothing is measured by the absence
//! of an error — a partial state that reconstructs is compared **by value** against the whole
//! repository, and a partial state that is refused is matched against the refusal it was predicted
//! to raise, naming the coordinate.
//!
//! Every literal is in the subject, written before the run.

use ape::canon::CanonicalHistory;

use ape_cli::error::{JournalError, LineageError, ReadingError};
use ape_cli::journal::EntryId;
use ape_cli::reading::{self, Reading};
use ape_cli::repository::Repository;
use ape_frontier::subject::atomicity::{
    self, AFTER_ENTRIES, AFTER_WORLDS, Arranged, BEFORE_ENTRIES, BEFORE_WORLDS, File, INTENDED,
    ORDER, UNWITNESSED,
};

/// A repository path no other process shares.
fn scratch(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-atomicity-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// What a repository holds, read off the files rather than off the process that wrote them.
///
/// Compared as one value, because a phase asserting these field by field would pass while quietly
/// leaving one of them unmeasured.
#[derive(Debug, PartialEq)]
struct Held {
    entries: usize,
    decisions: usize,
    worlds: usize,
}

fn held(repository: &Repository) -> Held {
    Held {
        entries: repository.read_journal().expect("readable").len(),
        decisions: repository.read_lineage().expect("readable").len(),
        worlds: repository.read_worlds().expect("readable").len(),
    }
}

/// The three files as bytes, which is the only form in which two repositories are the same one.
///
/// Read rather than re-encoded. A comparison of two `Vec<Admission>` would be a comparison of what
/// this process parsed; the experiment is about what is on disk.
///
/// Each file is checked to be non-empty here rather than by the phases that compare two of these.
/// Two empty readings compare equal, so a scan that had broken would report agreement — which is the
/// one way this family of guard passes while measuring nothing.
fn bytes(repository: &Repository) -> Vec<Vec<u8>> {
    let read: Vec<Vec<u8>> = [
        repository.journal_path(),
        repository.lineage_path(),
        repository.worlds_path(),
    ]
    .iter()
    .map(|path| std::fs::read(path).expect("the file is on disk"))
    .collect();

    assert_eq!(
        read.len(),
        File::ALL.len(),
        "three files, or the scan broke"
    );
    assert!(
        read.iter().all(|file| !file.is_empty()),
        "and none of them is empty, or the comparison is between two nothings"
    );

    read
}

/// Every world a repository answers for, and what each intends the account to hold.
///
/// The pair is what a phase compares: the reading says which world it is, and the level says what it
/// was for. A phase comparing only readings would report a lost intention as a shorter list.
fn answers(
    repository: &Repository,
    arrangement: &Arranged,
) -> Result<Vec<(Reading, i128)>, String> {
    let rebuilt = reading::corroborated(repository).map_err(|refusal| refusal.to_string())?;

    rebuilt
        .lineage
        .decided()
        .iter()
        .map(|thesis| {
            let reading = reading::of(
                rebuilt.canon.history(),
                thesis,
                arrangement.instance,
                &atomicity::asked_at(),
            )
            .map_err(|refusal| refusal.to_string())?;
            let intended =
                atomicity::intended(rebuilt.canon.history(), thesis, arrangement.instance)
                    .map_err(|refusal| refusal.to_string())?;

            Ok((reading, intended))
        })
        .collect()
}

/// The watermark, which is the one thing a reading does not carry and knowledge moves.
fn watermark(repository: &Repository) -> Option<String> {
    Some(
        reading::corroborated(repository)
            .expect("the repository reconstructs")
            .canon
            .history()
            .recorded_through()?
            .to_iso(),
    )
}

/// Whichever refusal a partial state raises, named.
fn refusal(repository: &Repository) -> ReadingError {
    match reading::corroborated(repository) {
        Err(refusal) => refusal,
        Ok(_) => panic!("the partial state reconstructs, and it was predicted to be refused"),
    }
}

/// A repository holding `before` whole, and a commit over it that stopped after `reached` files.
fn interrupted(name: &str, order: [File; 3], reached: usize) -> (Repository, Arranged) {
    let repository = Repository::open(scratch(name));
    let arrangement = atomicity::arranged().expect("the arrangement holds");

    atomicity::interrupted(&repository, &arrangement, order, reached).expect("writable");

    (repository, arrangement)
}

/// Phase 0 — what the whole repository answers.
///
/// Nothing here is a finding. It is the state every later phase is compared against, and it is
/// recorded for **both** whole repositories: the one that existed before the write, and the one the
/// interrupted commit would have produced. A phase that knew only the first could say a partial state
/// answers less, and not what it was supposed to answer instead.
#[test]
fn phase_0_what_the_whole_repository_answers() {
    let (repository, arrangement) = interrupted("phase-0", ORDER, 0);

    assert_eq!(
        held(&repository),
        Held {
            entries: BEFORE_ENTRIES,
            decisions: BEFORE_WORLDS,
            worlds: BEFORE_WORLDS,
        },
        "a write that never began leaves the repository as it was"
    );

    // The tail nothing decided about, which is what makes recovering a journal a question rather
    // than an identity. Derived from the two files, so the arrangement cannot claim one it lacks.
    let tail = arrangement.unwitnessed();

    assert_eq!(tail.len(), UNWITNESSED, "one entry no decision witnesses");
    assert_eq!(
        watermark(&repository).as_deref(),
        Some("2026-01-05"),
        "and the watermark stands where it was recorded"
    );

    let before = answers(&repository, &arrangement).expect("the whole repository reconstructs");

    assert_eq!(before.len(), BEFORE_WORLDS, "two worlds were decided");
    assert_eq!(
        before.iter().map(|(_, level)| *level).collect::<Vec<_>>(),
        INTENDED[..BEFORE_WORLDS].to_vec(),
        "the pledge, and the pledge with one outflow proposed against it"
    );

    // The commit the writer intended, read once so that a partial state can be told from it. Written
    // to a repository of its own: this is not a state the experiment interrupts, it is the state the
    // interruption is on its way to.
    let intended = Repository::open(scratch("phase-0-intended"));

    atomicity::write(&intended, &arrangement.after, ORDER).expect("writable");

    assert_eq!(
        held(&intended),
        Held {
            entries: AFTER_ENTRIES,
            decisions: AFTER_WORLDS,
            worlds: AFTER_WORLDS,
        },
        "one entry admitted and one decision taken"
    );

    let after = answers(&intended, &arrangement).expect("the finished commit reconstructs");

    assert_eq!(
        after.iter().map(|(_, level)| *level).collect::<Vec<_>>(),
        INTENDED.to_vec(),
        "and the world it adds is the one that intends the rest of the account"
    );

    // The two whole states agree about every world they share. Which is what makes the third world
    // the whole of what a commit adds, and the whole of what an interruption can lose.
    assert_eq!(
        before,
        after[..BEFORE_WORLDS].to_vec(),
        "a commit that appends does not move what was already answered"
    );
}

/// Phase 1 — interrupt after the journal.
///
/// A1 says this one reconstructs and A2 says it is the silent one. Both are measured positively: the
/// worlds are compared by value against Phase 0, the omission is named, and the silence is shown as
/// **agreement with a legitimate repository** rather than as the absence of an error.
#[test]
fn phase_1_interrupt_after_the_journal() {
    let (repository, arrangement) = interrupted("phase-1", ORDER, 1);

    assert_eq!(
        held(&repository),
        Held {
            entries: AFTER_ENTRIES,
            decisions: BEFORE_WORLDS,
            worlds: BEFORE_WORLDS,
        },
        "the journal is the commit's, and the other two are the previous repository's"
    );

    let torn = answers(&repository, &arrangement).expect("the partial state reconstructs");

    // What it answers: every world Phase 0 answered, identically, and not the one the commit was
    // for. Compared as whole values, so a world that came back subtly different is not read as the
    // same world.
    let (whole, _) = {
        let intended = Repository::open(scratch("phase-1-intended"));
        atomicity::write(&intended, &arrangement.after, ORDER).expect("writable");
        (
            answers(&intended, &arrangement).expect("reconstructs"),
            intended,
        )
    };

    assert_eq!(
        torn,
        whole[..BEFORE_WORLDS].to_vec(),
        "the worlds that survive answer exactly what they answered before"
    );
    assert_eq!(
        torn.len(),
        AFTER_WORLDS - 1,
        "and the world the commit was for is not among them"
    );
    assert_eq!(
        whole.last().expect("three worlds").1,
        INTENDED[AFTER_WORLDS - 1],
        "what was lost is an intention, and it is a number"
    );

    // The knowledge, however, arrived. So the repository is not the previous one either: it holds an
    // entry the previous one did not, and its watermark says so.
    assert_eq!(
        held(&repository).entries,
        AFTER_ENTRIES,
        "the outflow the commit admitted is in the journal"
    );
    assert_eq!(
        watermark(&repository).as_deref(),
        Some("2026-01-06"),
        "and the watermark moved with it"
    );

    // The silence, measured positively. A writer that admitted the same outflow and chose to decide
    // nothing about it commits three files, finishing every write it began — and the two directories
    // are byte-identical. The format has nowhere to say that one of them was interrupted.
    let deciding_nothing = Repository::open(scratch("phase-1-deciding-nothing"));

    atomicity::write(
        &deciding_nothing,
        &atomicity::Files {
            journal: arrangement.after.journal.clone(),
            lineage: arrangement.before.lineage.clone(),
            worlds: arrangement.before.worlds.clone(),
        },
        ORDER,
    )
    .expect("writable");

    assert_eq!(
        bytes(&repository),
        bytes(&deciding_nothing),
        "an interrupted commit and a finished one that decided nothing are the same repository"
    );
}

/// Phase 2 — interrupt after the lineage.
///
/// A1 says this one is refused at the length, and which refusal it is matters as much as that there
/// was one: a phase satisfied by any error would accept a repository refused for a reason this
/// experiment is not about.
#[test]
fn phase_2_interrupt_after_the_lineage() {
    let (repository, _) = interrupted("phase-2", ORDER, 2);

    assert_eq!(
        held(&repository),
        Held {
            entries: AFTER_ENTRIES,
            decisions: AFTER_WORLDS,
            worlds: BEFORE_WORLDS,
        },
        "two of the three files are the commit's, and the witnesses are the previous ones"
    );

    match refusal(&repository) {
        ReadingError::LineageLengthDisagrees { derived, recorded } => {
            assert_eq!(derived, AFTER_WORLDS, "the decisions produce three worlds");
            assert_eq!(recorded, BEFORE_WORLDS, "and two were recorded");
        }
        other => panic!("the length is what disagrees, and this says {other}"),
    }
}

/// Phase 2's mirror — interrupt after the lineage, with the journal never written.
///
/// A1's second case, and the one the current write order cannot reach: a lineage whose last decision
/// addresses an entry the journal it sits beside does not hold. Reached here by writing the lineage
/// first, which is Phase 4's variable and is used here only to produce the case A1 predicted.
#[test]
fn a_lineage_whose_journal_never_arrived_is_refused_at_the_entry() {
    let order = [File::Lineage, File::Journal, File::Worlds];
    let (repository, arrangement) = interrupted("lineage-first", order, 1);

    assert_eq!(
        held(&repository),
        Held {
            entries: BEFORE_ENTRIES,
            decisions: AFTER_WORLDS,
            worlds: BEFORE_WORLDS,
        },
        "the lineage is the commit's, and the journal it addresses is the previous one"
    );

    match refusal(&repository) {
        ReadingError::Lineage(LineageError::Journal(JournalError::UnknownEntry(entry))) => {
            assert_eq!(
                entry,
                arrangement.appended(),
                "the entry named is the one the lost decision was taken after"
            );
        }
        other => panic!("the entry is what is missing, and this says {other}"),
    }
}

/// What a partial state makes of itself, named rather than counted.
///
/// Three cases, because the experiment predicted three. A phase that reported *refused* without the
/// coordinate would put two different faults in one cell.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Reconstructs { worlds: usize },
    RefusedAtEntry { entry: EntryId },
    RefusedAtLength { derived: usize, recorded: usize },
}

fn outcome(repository: &Repository) -> Outcome {
    match reading::corroborated(repository) {
        Ok(rebuilt) => Outcome::Reconstructs {
            worlds: rebuilt.lineage.decided().len(),
        },
        Err(ReadingError::LineageLengthDisagrees { derived, recorded }) => {
            Outcome::RefusedAtLength { derived, recorded }
        }
        Err(ReadingError::Lineage(LineageError::Journal(JournalError::UnknownEntry(entry)))) => {
            Outcome::RefusedAtEntry { entry }
        }
        Err(other) => panic!(
            "a partial state refused for a reason this experiment predicted none of: {other}"
        ),
    }
}

/// Which of the three files the interrupted commit had replaced.
///
/// This is what a partial state *is*. Two orders that stop at different points can leave the same
/// three files, and the experiment is about the states rather than about the schedules that reach
/// them.
fn replaced(order: [File; 3], reached: usize) -> std::collections::BTreeSet<File> {
    order[..reached].iter().copied().collect()
}

/// Every state an interruption can leave, under every order the three files could be written in.
///
/// Six orders and two interruptible points each. What comes back is keyed by the state rather than by
/// the schedule, so a state two orders both reach is one entry and the phases can say so.
fn every_state() -> std::collections::BTreeMap<std::collections::BTreeSet<File>, (Outcome, usize)> {
    let mut states: std::collections::BTreeMap<_, (Outcome, usize)> = Default::default();

    for (index, order) in File::orders().into_iter().enumerate() {
        for reached in 1..File::ALL.len() {
            let (repository, _) = interrupted(&format!("state-{index}-{reached}"), order, reached);
            let outcome = outcome(&repository);
            let state = replaced(order, reached);

            if let Some((seen, _)) = states.get(&state) {
                assert_eq!(
                    *seen, outcome,
                    "one state, reached by two orders, and they disagree about what it is"
                );
            }

            states
                .entry(state)
                .and_modify(|(_, reached_by)| *reached_by += 1)
                .or_insert((outcome, 1));
        }
    }

    states
}

/// Phase 4 — the same prefixes, written in every other order.
///
/// A3, and enumerated rather than sampled: three files admit six orders, each with two interruptible
/// points, and the twelve schedules leave six distinct states. The claim is about which outcomes are
/// **reachable**, so a phase that measured one reversal would be reporting a sample as a closed set.
#[test]
fn phase_4_the_same_prefixes_in_every_other_order() {
    let arrangement = atomicity::arranged().expect("the arrangement holds");
    let states = every_state();

    assert_eq!(
        states.len(),
        2usize.pow(File::ALL.len() as u32) - 2,
        "every mixture of the two repositories' files is reachable, and nothing else is"
    );

    // The closed set, as one value. Stated here in full rather than probed field by field, because
    // criterion 5 asks what is reachable and a phase that asserted six facts separately would leave
    // whichever it forgot unmeasured.
    let refused_at_entry = Outcome::RefusedAtEntry {
        entry: arrangement.appended(),
    };
    let lineage_outran_its_witness = Outcome::RefusedAtLength {
        derived: AFTER_WORLDS,
        recorded: BEFORE_WORLDS,
    };
    let witness_outran_its_lineage = Outcome::RefusedAtLength {
        derived: BEFORE_WORLDS,
        recorded: AFTER_WORLDS,
    };

    assert_eq!(
        states
            .iter()
            .map(|(state, (outcome, _))| (state.clone(), outcome.clone()))
            .collect::<std::collections::BTreeMap<_, _>>(),
        std::collections::BTreeMap::from([
            (
                std::collections::BTreeSet::from([File::Journal]),
                Outcome::Reconstructs {
                    worlds: BEFORE_WORLDS
                }
            ),
            (
                std::collections::BTreeSet::from([File::Lineage]),
                refused_at_entry.clone()
            ),
            (
                std::collections::BTreeSet::from([File::Worlds]),
                witness_outran_its_lineage.clone()
            ),
            (
                std::collections::BTreeSet::from([File::Journal, File::Lineage]),
                lineage_outran_its_witness.clone()
            ),
            (
                std::collections::BTreeSet::from([File::Journal, File::Worlds]),
                witness_outran_its_lineage
            ),
            (
                std::collections::BTreeSet::from([File::Lineage, File::Worlds]),
                refused_at_entry
            ),
        ]),
        "six states, and three outcomes among them"
    );
    assert!(
        states.values().all(|(_, reached_by)| *reached_by == 2),
        "each state is reached by exactly two of the six orders"
    );

    // The silent state, and it is one of the six. Reachable only where the journal is the sole file
    // the commit had replaced — which is to say only where the journal was written first.
    let silent: Vec<_> = states
        .iter()
        .filter(|(_, (outcome, _))| matches!(outcome, Outcome::Reconstructs { .. }))
        .map(|(state, _)| state.clone())
        .collect();

    assert_eq!(
        silent,
        vec![std::collections::BTreeSet::from([File::Journal])],
        "one state reconstructs, and it is the one where only the journal is new"
    );
    assert_eq!(
        states[&silent[0]].0,
        Outcome::Reconstructs {
            worlds: BEFORE_WORLDS
        },
        "answering for the previous repository's worlds and not the commit's"
    );

    // And the application's own order is one of the two that reach it. Which is what makes the order
    // a variable rather than a detail: the same three writes, reordered, put the silent state out of
    // reach without changing anything else about the application.
    assert!(
        ORDER[0] == File::Journal,
        "the application writes the journal first"
    );

    let reachable_under = |order: [File; 3]| {
        (1..File::ALL.len())
            .map(|reached| states[&replaced(order, reached)].0.clone())
            .collect::<Vec<_>>()
    };

    assert_eq!(
        reachable_under(ORDER),
        vec![
            Outcome::Reconstructs {
                worlds: BEFORE_WORLDS
            },
            Outcome::RefusedAtLength {
                derived: AFTER_WORLDS,
                recorded: BEFORE_WORLDS
            },
        ],
        "under the application's order an interruption is silent or refused at the length"
    );
    assert_eq!(
        reachable_under([File::Lineage, File::Journal, File::Worlds]),
        vec![
            Outcome::RefusedAtEntry {
                entry: arrangement.appended()
            },
            Outcome::RefusedAtLength {
                derived: AFTER_WORLDS,
                recorded: BEFORE_WORLDS
            },
        ],
        "and with two lines reordered, neither of them is silent"
    );
}

/// What a reader holding only a partial state can put back, using rules the format supplies.
///
/// Two rules, and both are read off the repository rather than invented for this phase:
///
/// * **the worlds file is derived**, so it is recomputed from the two files that produce it;
/// * **the lineage appends, and the worlds file records how many worlds the previous one produced**,
///   so a lineage longer than the record beside it is truncated to that record.
///
/// There is no third rule, and the absence is the measurement. Nothing on disk records how long the
/// journal was, so nothing here can put a journal back.
///
/// The rules are applied **blind** — without being told which file the commit had replaced — because
/// that is the only reader there is. A procedure told which file to fix would be measuring what this
/// experiment knows rather than what the repository holds.
fn repaired(repository: &Repository, into: &Repository) -> Result<(), String> {
    let journal = repository.read_journal().map_err(|why| why.to_string())?;
    let mut decisions = repository.read_lineage().map_err(|why| why.to_string())?;
    let worlds = repository.read_worlds().map_err(|why| why.to_string())?;

    if decisions.len() > worlds.len() {
        decisions.truncate(worlds.len());
    }

    let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());
    let (rebuilt, _) = ape_cli::lineage::rebuild(&mut canon, &journal, &decisions)
        .map_err(|refusal| refusal.to_string())?;

    into.write_journal(&journal)
        .map_err(|why| why.to_string())?;
    into.write_lineage(&decisions)
        .map_err(|why| why.to_string())?;
    into.write_worlds(&atomicity::worlds(&rebuilt))
        .map_err(|why| why.to_string())?;

    Ok(())
}

/// Phase 3 — what is left of what was there.
///
/// A4, for every state Phase 4 enumerated. The answer is not one answer: two of the six states put
/// the previous repository back byte for byte, and four do not — so *refused* and *safe* turn out to
/// be independent of each other rather than the same thing said twice.
#[test]
fn phase_3_what_is_left_of_what_was_there() {
    let previous = Repository::open(scratch("phase-3-previous"));
    let arrangement = atomicity::arranged().expect("the arrangement holds");

    atomicity::write(&previous, &arrangement.before, ORDER).expect("writable");

    let was_there = bytes(&previous);
    let mut survives: std::collections::BTreeMap<std::collections::BTreeSet<File>, bool> =
        Default::default();

    for (index, order) in File::orders().into_iter().enumerate() {
        for reached in 1..File::ALL.len() {
            let (torn, _) = interrupted(&format!("left-{index}-{reached}"), order, reached);
            let put_back = Repository::open(scratch(&format!("put-back-{index}-{reached}")));

            let recovered = repaired(&torn, &put_back).is_ok() && bytes(&put_back) == was_there;

            survives.insert(replaced(order, reached), recovered);
        }
    }

    // The two states the previous repository survives: the lineage replaced with the witness that
    // records its previous length intact, and the witness replaced with the two files that derive it
    // intact. Which is one rule — a replaced file comes back where what survived determines it — and
    // three of the six states fail it, including the one where *both* appending files were replaced.
    let recovered: Vec<_> = survives
        .iter()
        .filter(|(_, put_back)| **put_back)
        .map(|(state, _)| state.clone())
        .collect();

    assert_eq!(
        recovered,
        vec![
            std::collections::BTreeSet::from([File::Lineage]),
            std::collections::BTreeSet::from([File::Worlds]),
        ],
        "the previous repository survives where what was replaced is determined by what was not"
    );
    assert!(
        !survives[&std::collections::BTreeSet::from([File::Lineage, File::Worlds])],
        "and the lineage's previous length is witnessed by the worlds file alone, so losing both loses it"
    );

    // And it does not survive wherever the journal was — including the state that reconstructs. So
    // the one partial state a reader is not warned about is a state that lost something.
    assert!(
        survives
            .iter()
            .filter(|(state, _)| state.contains(&File::Journal))
            .all(|(_, put_back)| !*put_back),
        "a replaced journal is a previous journal nothing can put back"
    );
    assert!(
        !survives[&std::collections::BTreeSet::from([File::Journal])],
        "and that is the silent one"
    );

    // What a replaced journal loses is exactly the tail no decision witnessed. The witness of the
    // last surviving decision is a floor, and nothing on disk says whether the floor is the length.
    let floor = arrangement
        .before
        .lineage
        .last()
        .expect("the previous repository decided something")
        .witness
        .len();

    assert_eq!(
        floor + UNWITNESSED,
        BEFORE_ENTRIES,
        "the previous journal is pinned to within its unwitnessed tail, and no closer"
    );
}

/// Being refused and being safe are independent, which is the experiment's question.
///
/// Read off the two phases above rather than argued: three of the five refused states lost the
/// previous repository and two kept it, and the one state nothing refuses lost it. A table with three
/// of its four cells occupied is not a correlation.
#[test]
fn a_refusal_says_nothing_about_what_survived() {
    let previous = Repository::open(scratch("independent-previous"));
    let arrangement = atomicity::arranged().expect("the arrangement holds");

    atomicity::write(&previous, &arrangement.before, ORDER).expect("writable");

    let was_there = bytes(&previous);
    let mut cells: std::collections::BTreeSet<(bool, bool)> = Default::default();

    for (index, order) in File::orders().into_iter().enumerate() {
        for reached in 1..File::ALL.len() {
            let (torn, _) = interrupted(&format!("cell-{index}-{reached}"), order, reached);
            let put_back = Repository::open(scratch(&format!("cell-back-{index}-{reached}")));

            let refused = !matches!(outcome(&torn), Outcome::Reconstructs { .. });
            let survived = repaired(&torn, &put_back).is_ok() && bytes(&put_back) == was_there;

            cells.insert((refused, survived));
        }
    }

    assert_eq!(
        cells,
        std::collections::BTreeSet::from([(true, true), (true, false), (false, false)]),
        "refused-and-safe, refused-and-lost, and unrefused-and-lost all occur"
    );
    assert!(
        !cells.contains(&(false, true)),
        "and the one cell nothing reaches is the reassuring one: no state both reconstructs and keeps what was there"
    );
}

/// Phase 6 — the repair, and the states it puts out of reach.
///
/// Part B. Its shape was decided by Phases 1 to 4 and by the criterion the coordination experiment
/// set: *it removes a state a reader can be misled by, and the repository before an interrupted write
/// survives.* Both halves are measured here, and the first one is measured by producing every state
/// Phase 4 enumerated and finding that none of them is visible.
///
/// The prefixes are produced in the generation the pointer does not name, using the same three
/// single-file writes Phases 1 to 4 used — because a prefix of a write is still a prefix, and what has
/// changed is not that the files stop being written one at a time but *where* they land.
#[test]
fn phase_6_a_write_that_is_whole() {
    let repository = Repository::open(scratch("phase-6"));
    let arrangement = atomicity::arranged().expect("the arrangement holds");

    // The previous repository, put there the way an application puts one there.
    let first = repository
        .prepare(atomicity::input(&arrangement.before))
        .expect("preparable");
    let replaced = first.generation();

    first.turn().expect("the pointer turns");

    let was_there = bytes(&repository);
    let answered = answers(&repository, &arrangement).expect("the whole repository reconstructs");

    assert_eq!(
        answered.len(),
        BEFORE_WORLDS,
        "two worlds, as Phase 0 read them"
    );

    // Where the commit that follows would land, and it is not where a reader looks.
    let staged = repository
        .prepare(atomicity::input(&arrangement.after))
        .expect("preparable");
    let pending = staged.generation();

    assert_ne!(
        pending, replaced,
        "a whole write puts its files somewhere other than the generation being read"
    );
    assert_eq!(
        bytes(&repository),
        was_there,
        "and a staged write, complete and unturned, is invisible"
    );

    // Every state Phase 4 enumerated, produced in the pending generation. Twelve schedules, six
    // states, and the repository answers Phase 0 through all of them.
    for order in File::orders() {
        for reached in 1..=File::ALL.len() {
            let _ = std::fs::remove_dir_all(&pending);

            let into = Repository::open(&pending);

            for file in &order[..reached] {
                atomicity::put(&into, &arrangement.after, *file).expect("writable");
            }

            assert_eq!(
                bytes(&repository),
                was_there,
                "no prefix of a whole write reaches a reader"
            );
            assert_eq!(
                answers(&repository, &arrangement).expect("the repository reconstructs"),
                answered,
                "and it answers what it answered before, by value"
            );
        }
    }

    // The turn, and the other half of the criterion: what the write replaces is still on disk, whole.
    repository
        .prepare(atomicity::input(&arrangement.after))
        .expect("preparable")
        .turn()
        .expect("the pointer turns");

    assert_eq!(
        answers(&repository, &arrangement)
            .expect("the repository reconstructs")
            .len(),
        AFTER_WORLDS,
        "the commit landed, and it landed whole"
    );
    assert_eq!(
        bytes(&Repository::open(&replaced)),
        was_there,
        "and the repository it replaced survives it, byte for byte"
    );
}

/// The instrument writes what the application writes, and nothing else.
///
/// A guard on the arrangement rather than a measurement. Every partial state in this experiment is
/// produced by calling the same three methods in some order and stopping, so a state reached by any
/// other means would not be an interruption.
#[test]
fn a_write_that_reached_every_file_is_the_finished_commit() {
    let (interrupted_at_three, arrangement) = interrupted("reached-three", ORDER, 3);
    let finished = Repository::open(scratch("finished"));

    atomicity::write(&finished, &arrangement.after, ORDER).expect("writable");

    assert_eq!(
        bytes(&interrupted_at_three),
        bytes(&finished),
        "stopping after the last write is not stopping"
    );
}

/// The addresses the arrangement claims are the addresses the journal produces.
///
/// The entry a lost decision was taken after is a value the phases above match a refusal against, so
/// an arrangement that reported an address the journal never held would make those phases pass
/// against the wrong fault.
#[test]
fn the_appended_entry_is_the_last_one_the_commit_admitted() {
    let arrangement = atomicity::arranged().expect("the arrangement holds");

    let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());
    let replayed = ape_cli::journal::replay(&mut canon, &arrangement.after.journal)
        .expect("the commit's journal admits");

    assert_eq!(
        replayed.entries.last().cloned(),
        Some(arrangement.appended()),
        "the decision was taken after the entry the commit admitted last"
    );
    assert_eq!(
        arrangement.before.journal.len(),
        BEFORE_ENTRIES,
        "and the previous journal is one entry shorter"
    );

    let previous: Vec<EntryId> = {
        let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());
        ape_cli::journal::replay(&mut canon, &arrangement.before.journal)
            .expect("the previous journal admits")
            .entries
    };

    assert!(
        !previous.contains(&arrangement.appended()),
        "which is why a lineage written before its journal names an entry that is not there"
    );
}
