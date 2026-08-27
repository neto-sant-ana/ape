//! The contention experiment, run phase by phase.
//!
//! Every ordering here is a sequence of calls in one thread. Nothing races, and nothing needs a
//! scheduler: two handles on one directory, and the phase decides which call comes next. That is the
//! same move the coordination experiment used to produce a lost decision and the atomicity experiment
//! used to produce an interruption — an order is a value.
//!
//! Nothing is measured by the absence of an error. What a repository holds after an ordering is read
//! back off the files and compared **by value** against the base and against each party's whole
//! state, and a refusal is matched against the coordinate it names.
//!
//! Every literal is in the subject, written before the run.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use ape::kernel::entities::AgentId;

use ape_cli::converge;
use ape_cli::error::{ConvergeError, JournalError, LineageError, ReadingError, RepositoryError};
use ape_cli::reading;
use ape_cli::repository::Repository;
use ape_frontier::subject::contention::{
    self, Arranged, BASE_ENTRIES, BASE_WORLDS, File, INTENDED, MERGED_ENTRIES, MERGED_WORLDS,
    PARTY_ENTRIES, PARTY_WORLDS, Party,
};

/// A repository path no other process shares.
fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-contention-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// What a repository answers for, read off the files rather than off the process that wrote them.
///
/// Compared as one value. The levels are what say *whose* state this is — [`INTENDED`] gives the
/// base, the recruiter's and the buyer's three different numbers — and `by` is what says who claims
/// to have decided it. A phase that compared only the counts would read one party's state as the
/// other's.
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    entries: usize,
    decisions: usize,
    worlds: usize,
    intended: Vec<i128>,
    by: Vec<Option<AgentId>>,
}

fn state(repository: &Repository, arrangement: &Arranged) -> Result<State, String> {
    let rebuilt = reading::corroborated(repository).map_err(|refusal| refusal.to_string())?;

    let intended = rebuilt
        .lineage
        .decided()
        .iter()
        .map(|thesis| {
            contention::intended(rebuilt.canon.history(), thesis, arrangement.instance)
                .map_err(|refusal| refusal.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(State {
        entries: rebuilt.journal.len(),
        decisions: rebuilt.decisions.len(),
        worlds: rebuilt.lineage.decided().len(),
        intended,
        by: rebuilt.decisions.iter().map(|taken| taken.by).collect(),
    })
}

/// The base as every phase begins from it.
fn base() -> State {
    State {
        entries: BASE_ENTRIES,
        decisions: BASE_WORLDS,
        worlds: BASE_WORLDS,
        intended: vec![INTENDED[0]],
        by: vec![None],
    }
}

/// One party's whole state: the base, plus the one entry and the one world it added.
fn alone(arrangement: &Arranged, party: usize) -> State {
    State {
        entries: PARTY_ENTRIES,
        decisions: PARTY_WORLDS,
        worlds: PARTY_WORLDS,
        intended: vec![INTENDED[0], arrangement.parties[party].intends],
        by: vec![None, Some(arrangement.parties[party].agent)],
    }
}

/// Both parties' lines, which is what nothing being lost looks like.
///
/// `order` is which party's world comes first, and it is a parameter because the merge decides it
/// rather than the caller: a decision is placed by where in the journal it was taken, so whichever
/// party admitted first is the one whose world is second in the sequence. Both orders are the same
/// pair of lines, and a phase that hard-coded one would read the other as a state nobody wrote.
fn merged(arrangement: &Arranged, order: [usize; 2]) -> State {
    State {
        entries: MERGED_ENTRIES,
        decisions: MERGED_WORLDS,
        worlds: MERGED_WORLDS,
        intended: vec![
            INTENDED[0],
            arrangement.parties[order[0]].intends,
            arrangement.parties[order[1]].intends,
        ],
        by: vec![
            None,
            Some(arrangement.parties[order[0]].agent),
            Some(arrangement.parties[order[1]].agent),
        ],
    }
}

/// The three files as bytes, which is the only form in which two repositories are the same one.
///
/// Each file is checked to be non-empty here rather than by the phases that compare two of these:
/// two empty readings compare equal, so a scan that had broken would report agreement.
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

/// The generation a reader reads, derived from where the reader is sent rather than from a name.
fn live(repository: &Repository) -> PathBuf {
    repository
        .journal_path()
        .parent()
        .expect("a file a reader reads is in a directory")
        .to_path_buf()
}

/// Every generation on disk, whatever it is called.
fn generations(repository: &Repository) -> BTreeSet<PathBuf> {
    std::fs::read_dir(repository.root())
        .expect("the repository is a directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path())
        .collect()
}

/// A repository holding the base, and the arrangement it was built from.
fn founded(name: &str) -> (Repository, Arranged) {
    let repository = Repository::open(scratch(name));
    let arrangement = contention::arranged().expect("the arrangement holds");

    contention::found(&repository, &arrangement).expect("writable");

    (repository, arrangement)
}

/// A party that has read the base and planned against it.
fn party<'a>(repository: &Repository, arrangement: &'a Arranged, which: usize) -> Planned<'a> {
    let mut working = contention::read(repository).expect("the base reconstructs");
    let party = &arrangement.parties[which];

    contention::plan(&mut working, party).expect("the party plans");

    Planned { party, working }
}

struct Planned<'a> {
    party: &'a Party,
    working: ape_cli::reading::Corroborated,
}

impl Planned<'_> {
    fn files(&self) -> contention::Files {
        contention::files(&self.working)
    }
}

/// Phase 0 — what the base answers, and what each party's commit would answer alone.
///
/// Nothing here is a finding. It is what every later phase is compared against, and it is recorded
/// for all three states the experiment can end in: the base, either party's, and both. A phase that
/// knew only the base could say something was lost and not whose.
#[test]
fn phase_0_what_the_base_answers_and_what_each_party_would() {
    let (repository, arrangement) = founded("phase-0");

    assert_eq!(
        state(&repository, &arrangement).expect("the base reconstructs"),
        base(),
        "one fund, one world, and a decision that claims nobody"
    );

    // The arrangement's own claim about the world both parties fork, against what a reader rebuilds.
    // A world is content-addressed, so these agree or the subject is describing a repository it did
    // not write.
    let rebuilt = reading::corroborated(&repository).expect("reconstructs");

    assert_eq!(
        rebuilt.lineage.decided().first().map(|world| world.id()),
        Some(arrangement.world),
        "the base's world is the one the arrangement names"
    );

    // Each party's whole state, alone, in a repository of its own. Neither is what the experiment
    // interrupts; they are what an outcome is compared against.
    for which in 0..arrangement.parties.len() {
        let planned = party(&repository, &arrangement, which);
        let alone_at = Repository::open(scratch(&format!("phase-0-alone-{which}")));

        contention::write(&alone_at, &planned.files()).expect("writable");

        assert_eq!(
            state(&alone_at, &arrangement).expect("one party's state reconstructs"),
            alone(&arrangement, which),
            "{}: the base, and one world intending {}",
            planned.party.label,
            planned.party.intends
        );
    }

    assert_ne!(
        alone(&arrangement, 0),
        alone(&arrangement, 1),
        "and the two are told apart by value, not by size"
    );
}

/// Phase 1 — serialized: the first party writes, and the second reads afterwards.
///
/// The ordering C4 predicted a refusal for, and there is none. Not because the guard failed: because
/// a party that read after another wrote holds a journal that **extends** what it found, which is
/// precisely the case the compare-and-append lets through. Both lines end up in the repository.
#[test]
fn phase_1_serialized_the_second_party_reads_what_the_first_left() {
    let (repository, arrangement) = founded("phase-1");

    let first = party(&repository, &arrangement, 0);

    converge::converge(&repository, &first.working).expect("the first party converges");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        alone(&arrangement, 0),
        "the first party's line is what is there"
    );

    // Read now, which is what makes this the serialized ordering.
    let second = party(&repository, &arrangement, 1);

    converge::converge(&repository, &second.working).expect("and so does the second");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        merged(&arrangement, [0, 1]),
        "both parties' knowledge and both parties' worlds, and nothing was refused"
    );
}

/// The same ordering without the merge, and it loses nothing either.
///
/// Which is what isolates the variable. A whole write is a swap with no comparison in it at all, and
/// under this ordering that costs nothing — the second party's own working copy already contains the
/// first party's line, because it read after the write. What a loss needs is a **stale read**, not a
/// write that does not compare.
#[test]
fn a_writer_that_read_afterwards_carries_the_other_line_with_it() {
    let (repository, arrangement) = founded("serialized-whole");

    let first = party(&repository, &arrangement, 0);

    contention::write(&repository, &first.files()).expect("writable");

    let second = party(&repository, &arrangement, 1);

    contention::write(&repository, &second.files()).expect("writable");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        merged(&arrangement, [0, 1]),
        "the second party wrote its own state, and its own state held both lines"
    );
}

/// Phase 2 — interleaved: both parties read the base, and then both write.
///
/// Through the application's coordination path, and this is the ordering the guard was built for
/// after all. Both parties hold journals that extend the *same* base and diverge from each other, so
/// the second one to write is refused by name at the entry where they disagree — and the refusal
/// carries the coordinate rather than a verdict.
#[test]
fn phase_2_interleaved_the_second_write_is_refused_at_the_entry() {
    let (repository, arrangement) = founded("phase-2");

    let first = party(&repository, &arrangement, 0);
    let second = party(&repository, &arrangement, 1);

    converge::converge(&repository, &first.working).expect("the first party converges");

    let was_there = bytes(&repository);

    match converge::converge(&repository, &second.working) {
        Err(ConvergeError::Diverged {
            position,
            expected,
            found,
            ..
        }) => {
            assert_eq!(
                position, BASE_ENTRIES,
                "at the entry after the base, which is the one each party added"
            );
            assert_ne!(
                expected, found,
                "and the two entries named are the two parties' own knowledge"
            );
        }
        Err(other) => panic!("refused, and for another reason: {other}"),
        Ok(_) => panic!("expected a party writing over a journal that moved, and it converged"),
    }

    assert_eq!(
        bytes(&repository),
        was_there,
        "and it left nothing behind, so the first party's line is intact"
    );

    // What the refused party does next, which the guard's own docstring names: it reads again and
    // plans again. Knowledge is not revisable, so admitting the same plan over more history is the
    // same fact.
    let again = party(&repository, &arrangement, 1);

    converge::converge(&repository, &again.working).expect("and converges");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        merged(&arrangement, [0, 1]),
        "both lines, and the refusal cost an attempt rather than a decision"
    );
}

/// The same interleaving through a whole write, which has no comparison in it.
///
/// C2's outcome: the live repository holds the second party's **three files entire** — not a mixture
/// — and both writers were told they succeeded. And the loser's whole state is still on disk, in the
/// generation the pointer no longer names, because the atomicity repair keeps whatever a write
/// replaces. What is *not* there any more is the base.
#[test]
fn phase_2_a_whole_write_loses_the_other_line_and_says_nothing() {
    let (repository, arrangement) = founded("phase-2-whole");

    let first = party(&repository, &arrangement, 0);
    let second = party(&repository, &arrangement, 1);

    contention::write(&repository, &first.files()).expect("writable");
    contention::write(&repository, &second.files()).expect("writable");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        alone(&arrangement, 1),
        "the second party's whole state, and the first party's line is not in it"
    );

    let elsewhere: Vec<State> = generations(&repository)
        .into_iter()
        .filter(|generation| *generation != live(&repository))
        .filter_map(|generation| state(&Repository::open(generation), &arrangement).ok())
        .collect();

    assert_eq!(
        elsewhere,
        vec![alone(&arrangement, 0)],
        "and the losing party's whole state is in the generation nothing points at"
    );
}

/// The interleaving one step finer, where both prepares collide.
///
/// C1, and it is unchanged by Part B: `prepare` chooses its target by reading the pointer, so two
/// writers that both prepare before either turns choose the **same** generation, and nothing refuses
/// it. The second one to prepare overwrites the first's three files there — which is measured here by
/// reading that generation, because it is a fact on disk before any pointer names it.
///
/// What Part B changed is the sentence that used to follow. Both turns named that generation and both
/// returned `Ok`, so the first writer published the second writer's repository; now the first
/// writer's turn is refused and the second writer's commits. The measurement of the old behaviour is
/// in Observation 2, against the commit that took it.
///
/// This is also where the losing party stops being recoverable: in the ordering above its state was
/// the previous generation, and here both writers were in one generation.
#[test]
fn phase_2_two_prepares_choose_one_generation() {
    let (repository, arrangement) = founded("phase-2-prepared");

    let first = party(&repository, &arrangement, 0);
    let second = party(&repository, &arrangement, 1);

    let (held, staged) = (first.files(), second.files());

    let one = contention::prepare(&repository, &held).expect("preparable");
    let other = contention::prepare(&repository, &staged).expect("preparable too");

    assert_eq!(
        one.generation(),
        other.generation(),
        "both writers prepared into the same generation, and nothing refused it"
    );
    assert_eq!(
        state(&Repository::open(one.generation()), &arrangement).expect("reconstructs"),
        alone(&arrangement, 1),
        "and what is in it is the second writer's whole state, over the first writer's"
    );
    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        base(),
        "while a reader is still reading the base, because nothing has turned"
    );

    match one.turn() {
        Err(RepositoryError::Contended { generation }) => {
            assert!(
                other.generation().ends_with(&generation),
                "the refusal names the generation the two writers met in"
            );
        }
        Err(other) => panic!("refused, and for another reason: {other}"),
        Ok(()) => panic!("expected a writer refused for publishing bytes it did not write"),
    }

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        base(),
        "and a refused turn leaves the repository as it was"
    );

    other.turn().expect("the writer that wrote it commits");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        alone(&arrangement, 1),
        "which is the same state a reader would have read, reached by the writer who meant it"
    );

    let elsewhere: Vec<State> = generations(&repository)
        .into_iter()
        .filter(|generation| *generation != live(&repository))
        .filter_map(|generation| state(&Repository::open(generation), &arrangement).ok())
        .collect();

    assert_eq!(
        elsewhere,
        vec![base()],
        "the base survives, and the losing writer's line is nowhere"
    );
}

/// A party's two acts, when what it uses to write is the merge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Step {
    Read(usize),
    Converge(usize),
}

/// Every ordering two parties' read and converge admit — six, the same shape as [`interleavings`].
fn readings() -> Vec<[Step; 4]> {
    let steps = [
        Step::Read(0),
        Step::Converge(0),
        Step::Read(1),
        Step::Converge(1),
    ];
    let mut orderings = Vec::new();

    for first in 0..steps.len() {
        for second in 0..steps.len() {
            for third in 0..steps.len() {
                for fourth in 0..steps.len() {
                    let taken = [first, second, third, fourth];

                    if BTreeSet::from(taken).len() != steps.len() {
                        continue;
                    }

                    let ordering = [steps[first], steps[second], steps[third], steps[fourth]];
                    let at = |step| ordering.iter().position(|held| *held == step);

                    if (0..2).all(|party| at(Step::Read(party)) < at(Step::Converge(party))) {
                        orderings.push(ordering);
                    }
                }
            }
        }
    }

    orderings
}

/// Through the merge, no ordering of two writers loses a line.
///
/// The question the experiment asks, answered for the path the application actually has. Every
/// ordering of two parties' read and converge, with the refused ones doing what the guard's own
/// docstring says — reading again and deciding again — ends holding **both** lines. And the refusal
/// happens in exactly one place: where both parties read before either wrote.
///
/// A `converge` is one call, so two of them cannot interleave without a thread. That is why the
/// comparison inside it always sees the repository as it is at the moment of writing, and why the
/// stale reading a party may be holding costs it an attempt rather than a decision.
#[test]
fn through_the_merge_no_ordering_of_two_writers_loses_a_line() {
    let orderings = readings();

    assert_eq!(orderings.len(), 6, "two parties, two acts each, ordered");

    for (index, ordering) in orderings.iter().enumerate() {
        let (repository, arrangement) = founded(&format!("merged-{index}"));
        let mut held: [Option<_>; 2] = [None, None];
        let mut refused = 0;

        for step in ordering {
            match step {
                Step::Read(which) => {
                    held[*which] = Some(party(&repository, &arrangement, *which).working);
                }
                Step::Converge(which) => {
                    let working = held[*which].take().expect("a party reads before it writes");

                    match converge::converge(&repository, &working) {
                        Ok(_) => {}
                        Err(ConvergeError::Diverged { position, .. }) => {
                            refused += 1;

                            assert_eq!(
                                position, BASE_ENTRIES,
                                "{ordering:?}: at the entry each party added"
                            );

                            // What the refusal asks for, and the whole of it.
                            let again = party(&repository, &arrangement, *which);

                            converge::converge(&repository, &again.working)
                                .expect("and the party that read again converges");
                        }
                        Err(other) => panic!("{ordering:?}: refused for another reason: {other}"),
                    }
                }
            }
        }

        let held = state(&repository, &arrangement).expect("reconstructs");

        assert!(
            held == merged(&arrangement, [0, 1]) || held == merged(&arrangement, [1, 0]),
            "{ordering:?}: both lines are there, in whichever order the journal placed them"
        );

        let both_read_first = ordering
            .iter()
            .take(2)
            .all(|step| matches!(step, Step::Read(_)));

        assert_eq!(
            refused,
            usize::from(both_read_first),
            "{ordering:?}: one refusal where both parties read before either wrote, and none otherwise"
        );
    }
}

/// One of the four operations an ordering is made of, and which writer performs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Prepare(usize),
    Turn(usize),
}

/// Every ordering two writers' two operations admit.
///
/// Four operations, and the only constraint is that a writer prepares before it turns — so this is
/// the four-item permutations filtered by that, which is six. Enumerated rather than sampled,
/// because Phase 3 claims something about *every* interleaving.
fn interleavings() -> Vec<[Op; 4]> {
    let ops = [Op::Prepare(0), Op::Turn(0), Op::Prepare(1), Op::Turn(1)];
    let mut orderings = Vec::new();

    for first in 0..ops.len() {
        for second in 0..ops.len() {
            for third in 0..ops.len() {
                for fourth in 0..ops.len() {
                    let taken = [first, second, third, fourth];

                    if BTreeSet::from(taken).len() != ops.len() {
                        continue;
                    }

                    let ordering = [ops[first], ops[second], ops[third], ops[fourth]];
                    let at = |op| ordering.iter().position(|held| *held == op);

                    if (0..2).all(|writer| at(Op::Prepare(writer)) < at(Op::Turn(writer))) {
                        orderings.push(ordering);
                    }
                }
            }
        }
    }

    orderings
}

/// Whose state a repository holds, named rather than described.
///
/// The four a writer could have put there, and two more: `Unwritten` for a state that reconstructs
/// and is nobody's, `Refused` for one that does not reconstruct at all. Phase 3 expects neither, and
/// says so by their absence from its table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Whose {
    Base,
    First,
    Second,
    Merged,
    Unwritten,
    Refused,
}

fn whose(repository: &Repository, arrangement: &Arranged) -> Whose {
    match state(repository, arrangement) {
        Err(_) => Whose::Refused,
        Ok(held) if held == base() => Whose::Base,
        Ok(held) if held == alone(arrangement, 0) => Whose::First,
        Ok(held) if held == alone(arrangement, 1) => Whose::Second,
        Ok(held) if held == merged(arrangement, [0, 1]) => Whose::Merged,
        Ok(held) if held == merged(arrangement, [1, 0]) => Whose::Merged,
        Ok(_) => Whose::Unwritten,
    }
}

/// What one ordering left: what a reader reads, what is on disk beside it, and who was refused.
///
/// `refused` is a set of writers rather than a count, because Part B made the identity of the refused
/// writer the point: the one that is told no is the one whose prepared generation somebody else wrote
/// over, and a count could not say that.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Left {
    live: Whose,
    elsewhere: Vec<Whose>,
    refused: BTreeSet<usize>,
}

/// Run one ordering over a fresh repository, and read off what it left.
fn ordered(name: &str, ordering: [Op; 4]) -> (Left, Arranged) {
    let (repository, arrangement) = founded(name);

    let held = [
        party(&repository, &arrangement, 0).files(),
        party(&repository, &arrangement, 1).files(),
    ];
    let mut prepared: [Option<_>; 2] = [None, None];
    let mut refused = BTreeSet::new();

    for op in ordering {
        match op {
            Op::Prepare(writer) => match contention::prepare(&repository, &held[writer]) {
                Ok(staged) => prepared[writer] = Some(staged),
                Err(_) => {
                    refused.insert(writer);
                }
            },
            Op::Turn(writer) => match prepared[writer]
                .take()
                .expect("a writer turns what it prepared")
                .turn()
            {
                Ok(()) => {}
                Err(_) => {
                    refused.insert(writer);
                }
            },
        }
    }

    let live_at = live(&repository);
    let elsewhere = generations(&repository)
        .into_iter()
        .filter(|generation| *generation != live_at)
        .map(|generation| whose(&Repository::open(generation), &arrangement))
        .collect();

    (
        Left {
            live: whose(&repository, &arrangement),
            elsewhere,
            refused,
        },
        arrangement,
    )
}

/// Phase 3 — every interleaving the two operations admit.
///
/// Six orderings, and the table is asserted as one value: a phase that checked six facts separately
/// would leave whichever it forgot unmeasured. What comes out of it is one rule — **the live
/// repository is the state of whoever prepared last** — and the pointer's turn does not appear in it.
///
/// The other column is what the loser leaves behind, and it is not constant: where the two prepares
/// were separated by a turn, the loser's whole state is the previous generation; where they collided,
/// the previous generation is the base and the loser is nowhere. Nothing in the repository says which
/// of the two happened.
///
/// The third column is Part B's. Before it, nothing refused any of the twenty-four calls and both
/// writers were told they had committed; now the writer whose prepared generation was written over is
/// refused, which is every writer except the last to prepare. The rule about *who* a reader reads did
/// not change — what changed is that the other writer now knows.
#[test]
fn phase_3_every_interleaving_the_two_operations_admit() {
    let orderings = interleavings();

    assert_eq!(
        orderings.len(),
        6,
        "four operations, two of them ordered against the other two"
    );

    let mut table: BTreeMap<Vec<Op>, Left> = BTreeMap::new();

    for (index, ordering) in orderings.iter().enumerate() {
        let (left, _) = ordered(&format!("phase-3-{index}"), *ordering);

        table.insert(ordering.to_vec(), left);
    }

    assert_eq!(
        table.len(),
        orderings.len(),
        "and no two of them are the same sequence"
    );

    // The rule, read off the table rather than argued: the live state is the last writer to prepare,
    // and the other one is refused wherever there was a collision to be refused about.
    for (ordering, left) in &table {
        let prepares: Vec<usize> = ordering
            .iter()
            .filter_map(|op| match op {
                Op::Prepare(writer) => Some(*writer),
                Op::Turn(_) => None,
            })
            .collect();
        let last = *prepares.last().expect("every ordering prepares");
        let collided = ordering[..2].iter().all(|op| matches!(op, Op::Prepare(_)));

        assert_eq!(
            left.live,
            [Whose::First, Whose::Second][last],
            "{ordering:?}: what a reader reads is what the last prepare wrote"
        );
        assert_eq!(
            left.refused,
            if collided {
                BTreeSet::from([1 - last])
            } else {
                BTreeSet::new()
            },
            "{ordering:?}: refused exactly where a writer's prepared generation was written over"
        );
    }

    // And what is beside it. Two orderings are serialized — a writer turns before the other prepares
    // — and those are the two that keep the loser.
    let serialized: BTreeSet<Vec<Op>> = table
        .iter()
        .filter(|(_, left)| {
            left.elsewhere == vec![Whose::First] || left.elsewhere == vec![Whose::Second]
        })
        .map(|(ordering, _)| ordering.clone())
        .collect();

    assert_eq!(
        serialized,
        BTreeSet::from([
            vec![Op::Prepare(0), Op::Turn(0), Op::Prepare(1), Op::Turn(1)],
            vec![Op::Prepare(1), Op::Turn(1), Op::Prepare(0), Op::Turn(0)],
        ]),
        "the loser's state survives exactly where the two prepares were separated by a turn"
    );
    assert!(
        table
            .iter()
            .filter(|(ordering, _)| !serialized.contains(*ordering))
            .all(|(_, left)| left.elsewhere == vec![Whose::Base]),
        "and where they collided, what survives beside the winner is the base"
    );

    // Nothing in the six is a mixture, and nothing in the six holds both lines.
    assert!(
        table
            .values()
            .all(|left| left.live != Whose::Unwritten && left.live != Whose::Refused),
        "a whole write is one call, so no ordering of two of them leaves a state nobody wrote"
    );
    assert!(
        table.values().all(|left| left.live != Whose::Merged),
        "and none of them merges: a write that does not compare cannot notice the other line"
    );
}

/// A prepared write held across another writer's commits cannot turn the pointer backwards.
///
/// Outside Phase 3's closed set, which is closed over *two* operations each — this takes five, and
/// finding it is what says the set is closed as stated rather than closed absolutely. The first
/// writer prepares and does not turn; the other commits twice, the second time holding both lines;
/// and then the first writer turns.
///
/// It was the one outcome that misled a **reader** rather than a writer: the turn put back the state
/// the first of those two commits had left, reconstructing and corroborating and carrying no fact
/// that said it was a rollback. Observation 5 measured it, and it is what earned Part B — which is
/// what this now asserts instead, at the same five calls.
#[test]
fn a_turn_cannot_publish_a_state_that_was_already_replaced() {
    let (repository, arrangement) = founded("resurrection");

    let one = party(&repository, &arrangement, 0);
    let staged = contention::prepare(&repository, &one.files()).expect("preparable");

    // The other writer commits, which overwrites the generation the first one prepared into.
    let other = party(&repository, &arrangement, 1);

    contention::write(&repository, &other.files()).expect("writable");

    // And commits again, this time holding both lines: it reads what is there and converges.
    let both = party(&repository, &arrangement, 0);

    converge::converge(&repository, &both.working).expect("the second commit converges");

    let was_there = bytes(&repository);

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        merged(&arrangement, [1, 0]),
        "two commits in, and nothing has been lost"
    );

    assert!(
        matches!(staged.turn(), Err(RepositoryError::Contended { .. })),
        "a handle from before those two commits does not get to name what a reader reads"
    );
    assert_eq!(
        bytes(&repository),
        was_there,
        "and the repository is the one the second commit left, byte for byte"
    );
}

/// Two writers holding whole states, and how their journals are related.
///
/// The variable Phase 4 turns out to be about. `Divergent` is two parties that both read the base:
/// their journals are the same length and differ at the entry each added. `Extending` is the state
/// the sequence actually reaches when a refused party re-reads — one writer still holding what it
/// wrote, the other holding the merge of both — and there one journal is a prefix of the other.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Related {
    Divergent,
    Extending,
}

fn holding(
    name: &str,
    related: Related,
) -> (
    Repository,
    Arranged,
    contention::Files,
    contention::Files,
    Whose,
) {
    let (repository, arrangement) = founded(name);

    let one = party(&repository, &arrangement, 0).files();

    match related {
        Related::Divergent => {
            let other = party(&repository, &arrangement, 1).files();

            (repository, arrangement, one, other, Whose::Second)
        }
        Related::Extending => {
            // The first writer's line is put there, and the second reads it — so what the second
            // holds is the merge, and its journal extends the first's rather than diverging from it.
            contention::write(&repository, &one).expect("writable");

            let other = party(&repository, &arrangement, 1).files();

            (repository, arrangement, one, other, Whose::Merged)
        }
    }
}

/// What a mixture of two writers' files makes of itself, named rather than counted.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Mixed {
    Reconstructs { entries: usize, worlds: usize },
    RefusedAtEntry,
    RefusedAtLength { derived: usize, recorded: usize },
    RefusedAtWorld { position: usize, coordinate: String },
}

fn mixed(repository: &Repository) -> Mixed {
    match reading::corroborated(repository) {
        Ok(rebuilt) => Mixed::Reconstructs {
            entries: rebuilt.journal.len(),
            worlds: rebuilt.lineage.decided().len(),
        },
        Err(ReadingError::LineageLengthDisagrees { derived, recorded }) => {
            Mixed::RefusedAtLength { derived, recorded }
        }
        Err(ReadingError::WorldDisagrees {
            position,
            coordinate,
        }) => Mixed::RefusedAtWorld {
            position,
            coordinate: coordinate.to_owned(),
        },
        Err(ReadingError::Lineage(LineageError::Journal(JournalError::UnknownEntry(_)))) => {
            Mixed::RefusedAtEntry
        }
        Err(other) => {
            panic!("a mixture refused for a reason this experiment predicted none of: {other}")
        }
    }
}

/// Every mixture of two writers' three files, reached inside a generation they both prepared.
///
/// The first writer prepares — three files, where nothing reads them — and the second writes some of
/// its own over them, one file at a time. That is the finer grain the atomicity experiment used and
/// reported as the door its repair left open. Then the first writer turns, and what it publishes is
/// the mixture.
fn mixtures(related: Related) -> BTreeMap<BTreeSet<File>, Mixed> {
    let mut states = BTreeMap::new();

    for (index, order) in File::orders().into_iter().enumerate() {
        for reached in 1..File::ALL.len() {
            let (repository, _, one, other, _) =
                holding(&format!("mix-{related:?}-{index}-{reached}"), related);

            let staged = contention::prepare(&repository, &one).expect("preparable");
            let into = Repository::open(staged.generation());

            for file in &order[..reached] {
                contention::put(&into, &other, *file).expect("writable");
            }

            let state = order[..reached].iter().copied().collect::<BTreeSet<_>>();

            assert!(
                matches!(staged.turn(), Err(RepositoryError::Contended { .. })),
                "and the writer that prepared it does not get to publish it"
            );

            // A consumer breaking, recorded rather than absorbed. Experiment 16 gave the whole write
            // a fourth file — the addresses the journal comes to — so a prepared generation now
            // carries the first writer's claim about a journal the second writer overwrote, and every
            // mixture is refused by it. That is a real narrowing of the door this phase reports as
            // left open, and it is measured in 16's own suite; classifying a mixture with the claim
            // present would be measuring THAT repair instead of the state this phase enumerates,
            // which is the same reason the turn is weighed before the reading rather than after.
            //
            // So the generation is reduced to the three files the application wrote when this ran.
            std::fs::remove_file(into.custody_path()).expect("a whole write put one there");

            // Read where it is rather than where a pointer would send a reader. The mixture is a fact
            // on disk as soon as the second writer's file lands, and Part B refuses the turn that
            // would publish it — so classifying it through the pointer would be measuring the repair
            // instead of the state.
            let outcome = mixed(&into);

            if let Some(seen) = states.get(&state) {
                assert_eq!(
                    *seen, outcome,
                    "one mixture, reached by two orders, and they disagree about what it is"
                );
            }

            states.insert(state, outcome);
        }
    }

    states
}

/// Phase 4 — the finer grain, and whether it is a new failure.
///
/// C3 said the mixtures would be the same six states the atomicity experiment enumerated. They are,
/// and only when one writer's journal **extends** the other's — which is the same condition the
/// compare-and-append tests. Where the two writers diverge, no mixture is silent: every one of the
/// six is refused, four of them at the entry a decision names and two at a world.
///
/// So the door the repair left open leads back to the old failures, and it leads there through
/// exactly the parties the guard would have refused.
#[test]
fn phase_4_the_finer_grain_and_whether_it_is_a_new_failure() {
    let divergent = mixtures(Related::Divergent);
    let extending = mixtures(Related::Extending);

    for (related, states) in [
        (Related::Divergent, &divergent),
        (Related::Extending, &extending),
    ] {
        assert_eq!(
            states.len(),
            2usize.pow(File::ALL.len() as u32) - 2,
            "{related:?}: every mixture of the two writers' files is reachable, and nothing else is"
        );
    }

    let journal = BTreeSet::from([File::Journal]);
    let lineage = BTreeSet::from([File::Lineage]);
    let worlds = BTreeSet::from([File::Worlds]);
    let at_world = Mixed::RefusedAtWorld {
        position: 1,
        coordinate: "what it still proposes".to_owned(),
    };

    assert_eq!(
        divergent,
        BTreeMap::from([
            (journal.clone(), Mixed::RefusedAtEntry),
            (lineage.clone(), Mixed::RefusedAtEntry),
            (worlds.clone(), at_world.clone()),
            (
                BTreeSet::from([File::Journal, File::Lineage]),
                at_world.clone()
            ),
            (
                BTreeSet::from([File::Journal, File::Worlds]),
                Mixed::RefusedAtEntry
            ),
            (
                BTreeSet::from([File::Lineage, File::Worlds]),
                Mixed::RefusedAtEntry
            ),
        ]),
        "two writers that diverge leave six mixtures and no silent one"
    );

    assert_eq!(
        extending,
        BTreeMap::from([
            (
                journal.clone(),
                Mixed::Reconstructs {
                    entries: MERGED_ENTRIES,
                    worlds: PARTY_WORLDS,
                }
            ),
            (lineage.clone(), Mixed::RefusedAtEntry),
            (
                worlds.clone(),
                Mixed::RefusedAtLength {
                    derived: PARTY_WORLDS,
                    recorded: MERGED_WORLDS,
                }
            ),
            (
                BTreeSet::from([File::Journal, File::Lineage]),
                Mixed::RefusedAtLength {
                    derived: MERGED_WORLDS,
                    recorded: PARTY_WORLDS,
                }
            ),
            (
                BTreeSet::from([File::Journal, File::Worlds]),
                Mixed::RefusedAtLength {
                    derived: PARTY_WORLDS,
                    recorded: MERGED_WORLDS,
                }
            ),
            (
                BTreeSet::from([File::Lineage, File::Worlds]),
                Mixed::RefusedAtEntry
            ),
        ]),
        "and two whose journals extend leave the atomicity experiment's six, silent state included"
    );

    // The silent one, and what it is: the first writer's worlds over both writers' knowledge. A
    // repository neither of them wrote, answering for intentions one of them never learned about.
    let silent: Vec<_> = extending
        .iter()
        .filter(|(_, outcome)| matches!(outcome, Mixed::Reconstructs { .. }))
        .map(|(state, _)| state.clone())
        .collect();

    assert_eq!(
        silent,
        vec![journal],
        "one mixture reconstructs, and it is the one where only the journal is the other writer's"
    );
    assert!(
        divergent
            .values()
            .all(|outcome| !matches!(outcome, Mixed::Reconstructs { .. })),
        "and where the journals diverge there is none, because a decision names an entry that is not there"
    );
}

/// The commitment a party's plan admits, derived by replaying that party's own journal.
///
/// Derived rather than carried, because a party's plan is held unadmitted until the party admits it
/// — and what a phase needs to look for in a surviving repository is the identity, which only a
/// replay produces.
fn planned(files: &contention::Files) -> ape::kernel::entities::CommitmentId {
    let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());

    *ape_cli::journal::replay(&mut canon, &files.journal)
        .expect("a party's journal admits")
        .commitments
        .last()
        .expect("a party's plan is a commitment")
}

/// Phase 5 — what is left of the losing party.
///
/// For every interleaving: nothing of its line, and the record still says the party exists. Both
/// halves are measured positively — the query that would find its worlds is asked and comes back
/// empty, and the agent it claims is resolved out of the journal that survived.
///
/// The gap has a shape coordination already named for a different reason: `decided_by` cannot tell
/// *this party decided nothing* from *this party's decisions are gone*. There it was the cost of an
/// optional field; here it is the cost of a write that does not compare.
#[test]
fn phase_5_what_is_left_of_the_losing_party() {
    for (index, ordering) in interleavings().iter().enumerate() {
        let repository = Repository::open(scratch(&format!("phase-5-{index}")));
        let arrangement = contention::arranged().expect("the arrangement holds");

        contention::found(&repository, &arrangement).expect("writable");

        let (left, _) = ordered(&format!("phase-5-run-{index}"), *ordering);
        let winner = match left.live {
            Whose::First => 0,
            Whose::Second => 1,
            other => panic!("{ordering:?}: the live state is {other:?}"),
        };
        let loser = 1 - winner;

        // Re-run the same ordering here, so the repository and the arrangement are the ones this
        // phase asks about rather than the ones Phase 3 read.
        let held = [
            party(&repository, &arrangement, 0).files(),
            party(&repository, &arrangement, 1).files(),
        ];
        let mut prepared: [Option<_>; 2] = [None, None];

        for op in ordering {
            match op {
                Op::Prepare(writer) => {
                    prepared[*writer] =
                        Some(contention::prepare(&repository, &held[*writer]).expect("preparable"));
                }
                // A refused turn is an outcome and not a failure of the arrangement: since Part B,
                // the writer whose generation was written over is told so, and what this phase asks
                // about is what is left afterwards either way.
                Op::Turn(writer) => {
                    let _ = prepared[*writer]
                        .take()
                        .expect("a writer turns what it prepared")
                        .turn();
                }
            }
        }

        let party_at = |which: usize| arrangement.parties[which].agent;

        assert_eq!(
            reading::decided_by(&repository, party_at(loser)).expect("readable"),
            BTreeSet::new(),
            "{ordering:?}: nothing the losing party decided is there"
        );
        assert_eq!(
            reading::decided_by(&repository, party_at(winner))
                .expect("readable")
                .len(),
            1,
            "and the winner's one world is"
        );

        // And the party itself. Its agent was admitted in the base, so the knowledge that says it
        // exists is knowledge no writer could have dropped — which is what makes the empty answer
        // above unreadable: a party that exists and decided nothing looks exactly like this.
        let rebuilt = reading::corroborated(&repository).expect("reconstructs");

        assert!(
            rebuilt.admitted.agents.contains(&party_at(loser)),
            "the record still says the losing party exists"
        );
        assert!(
            !rebuilt
                .admitted
                .commitments
                .contains(&planned(&held[loser])),
            "and says nothing about what it knew"
        );
    }
}

/// Phase 7 — the repair, and the states it puts out of reach.
///
/// Part B. Its shape was decided by Phases 1 to 5 and by the two criteria: it must remove a state a
/// reader can be misled by while what it replaces survives, and — this experiment's own addition — if
/// it serializes writers it must say what a writer that waits is owed.
///
/// Both are measured here rather than argued.
///
/// **A state a reader can be misled by.** Three of them, and each has its own test above: a turn that
/// published another writer's whole state, a turn that published a mixture of two writers' files, and
/// a turn that put back a state two commits old. All three came from one cause — a turn was a claim
/// about a name and the name's contents were nobody's business — so all three are answered by one
/// comparison. What is asserted below is the second half: that the six mixtures Phase 4 enumerated
/// cannot reach a reader through the write, and that a repository answers what it answered before
/// through every one of them.
///
/// **What it replaces survives**, which was already the atomicity repair's promise and is measured
/// again here because this changes the operation that keeps it.
///
/// **Nothing waits.** The second criterion is satisfied by construction rather than by design effort,
/// and saying so is the honest form: no writer is held, nothing is claimed, nothing can time out, and
/// a refused writer's recovery is the one the coordination experiment established. That is asserted
/// too — the refused party reads again, decides again, converges, and both lines are there.
#[test]
fn phase_7_a_write_that_compares_before_it_swaps() {
    let (repository, arrangement) = founded("phase-7");

    let was_there = bytes(&repository);
    let replaced = live(&repository);

    // Every mixture Phase 4 enumerated, produced the way Phase 4 produces them — in a generation one
    // writer prepared and another wrote into — and none of them reaches a reader.
    for order in File::orders() {
        for reached in 1..=File::ALL.len() {
            let one = party(&repository, &arrangement, 0);
            let other = party(&repository, &arrangement, 1);
            let staged = contention::prepare(&repository, &one.files()).expect("preparable");
            let into = Repository::open(staged.generation());

            for file in &order[..reached] {
                contention::put(&into, &other.files(), *file).expect("writable");
            }

            assert!(
                matches!(staged.turn(), Err(RepositoryError::Contended { .. })),
                "no prefix of another writer's state can be published as a commit"
            );
            assert_eq!(
                bytes(&repository),
                was_there,
                "and the repository is untouched by the attempt"
            );
            assert_eq!(
                state(&repository, &arrangement).expect("reconstructs"),
                base(),
                "answering what it answered before, by value"
            );
        }
    }

    // The collision itself: two writers, one generation, and the one whose bytes are still there is
    // the one that commits. What it replaces survives, which is the criterion's second half.
    let one = party(&repository, &arrangement, 0);
    let other = party(&repository, &arrangement, 1);

    let mine = contention::prepare(&repository, &one.files()).expect("preparable");
    let theirs = contention::prepare(&repository, &other.files()).expect("preparable too");

    assert!(
        matches!(mine.turn(), Err(RepositoryError::Contended { .. })),
        "the writer that was written over is the one told so"
    );

    theirs.turn().expect("and the other one commits");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        alone(&arrangement, 1),
        "the commit landed, and it landed whole"
    );
    assert_eq!(
        bytes(&Repository::open(&replaced)),
        was_there,
        "and the repository it replaced survives it, byte for byte"
    );

    // The refused writer's way back, and there is nothing to release first. It reads what is there,
    // decides again, and puts both lines back through the merge — which is what a refusal has meant
    // since the coordination experiment, and is why no writer here waits for anything.
    let again = party(&repository, &arrangement, 0);

    converge::converge(&repository, &again.working).expect("the refused party converges");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        merged(&arrangement, [1, 0]),
        "and both lines are there, at the cost of an attempt"
    );
}

/// What the repair does not reach, kept as a green test rather than as a sentence.
///
/// A whole write compares against the generation it prepared, so it cannot see a writer holding an old
/// *reading*: that writer overwrites nobody, prepares a legitimate state of its own, and loses the
/// other line in silence. Phase 2 measured it, and it measures the same thing after Part B — which is
/// the point, and is why the two comparisons are two.
#[test]
fn a_stale_writer_is_still_not_something_a_write_can_see() {
    let (repository, arrangement) = founded("residue");

    let first = party(&repository, &arrangement, 0);
    let second = party(&repository, &arrangement, 1);

    // The second party's reading is now old, and the write it makes is whole, legitimate, and alone.
    contention::write(&repository, &first.files()).expect("writable");
    contention::write(&repository, &second.files()).expect("writable");

    assert_eq!(
        state(&repository, &arrangement).expect("reconstructs"),
        alone(&arrangement, 1),
        "the first party's line is gone, and nothing refused anything"
    );

    // And through the merge, the same two parties in the same order lose nothing. The difference is
    // not the ordering; it is which comparison the writer went through.
    let (merging, arrangement) = founded("residue-merged");
    let one = party(&merging, &arrangement, 0);
    let other = party(&merging, &arrangement, 1);

    converge::converge(&merging, &one.working).expect("converges");

    match converge::converge(&merging, &other.working) {
        Err(ConvergeError::Diverged { position, .. }) => {
            assert_eq!(position, BASE_ENTRIES, "at the entry each party added")
        }
        Err(other) => panic!("refused, and for another reason: {other}"),
        Ok(_) => panic!("expected the stale party to be told"),
    }
}
