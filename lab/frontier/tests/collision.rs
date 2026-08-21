//! The collision experiment, run phase by phase.
//!
//! Two whole repositories in one process, and the instrument is **which one is read as a working
//! copy**. Nothing here needs a network, a clone or a thread: a meeting is a call, and which
//! repository is the subject of that call is a value the phase chooses.
//!
//! Nothing is measured by the absence of an error. A refusal is matched against the coordinate it
//! names, a meeting is compared **by value** against what each repository held alone, and two worlds
//! are compared **by identity** rather than by what they answer — because two worlds that agree about
//! a level are not thereby the same world.
//!
//! Every literal is in the subject, written before the run.

use std::path::PathBuf;

use ape::kernel::entities::ResourceInstanceId;

use ape_cli::converge;
use ape_cli::error::ConvergeError;
use ape_cli::reading;
use ape_cli::repository::Repository;
use ape_frontier::subject::collision::{
    self, Arranged, BASE_ENTRIES, Founding, INTENDED, Relation, SIDE_ENTRIES, SIDE_WORLDS, Side,
};

/// A repository path no other process shares.
fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir()
        .join(format!("ape-collision-{}", std::process::id()))
        .join(name);
    let _ = std::fs::remove_dir_all(&path);
    path
}

/// What a repository answers for, read off the files rather than off the process that wrote them.
///
/// The levels are what say *whose* state this is: the base intends 300, the left's world 260 and the
/// right's 230, which are three different numbers written before the run.
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    entries: usize,
    decisions: usize,
    worlds: usize,
    intended: Vec<i128>,
}

fn state(repository: &Repository, instance: ResourceInstanceId) -> Result<State, String> {
    let rebuilt = reading::corroborated(repository).map_err(|refusal| refusal.to_string())?;

    let intended = rebuilt
        .lineage
        .decided()
        .iter()
        .map(|thesis| {
            collision::intended(rebuilt.canon.history(), thesis, instance)
                .map_err(|refusal| refusal.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(State {
        entries: rebuilt.journal.len(),
        decisions: rebuilt.decisions.len(),
        worlds: rebuilt.lineage.decided().len(),
        intended,
    })
}

/// One side's whole state, alone: the base's world and its own.
///
/// `which` is 1 for the left and 2 for the right, which are their places in [`INTENDED`].
fn alone(which: usize) -> State {
    State {
        entries: SIDE_ENTRIES,
        decisions: SIDE_WORLDS,
        worlds: SIDE_WORLDS,
        intended: vec![INTENDED[0], INTENDED[which]],
    }
}

/// Put one side's repository on disk, the way an application puts one there.
fn found(name: &str, side: &Side) -> Repository {
    let repository = Repository::open(scratch(name));

    collision::write(&repository, &side.files).expect("writable");

    repository
}

/// Both repositories on disk, in the relation asked for.
fn both(name: &str, founding: Founding) -> (Repository, Repository, Arranged) {
    let arrangement = collision::arranged(founding).expect("the arrangement holds");
    let left = found(&format!("{name}-left"), &arrangement.left);
    let right = found(&format!("{name}-right"), &arrangement.right);

    (left, right, arrangement)
}

/// Phase 0 — what each repository answers alone.
///
/// Nothing here is a finding. It is what every later phase is compared against, and it is recorded
/// for both, because a phase that knew only one of them could say a meeting changed something and
/// not into what.
#[test]
fn phase_0_what_each_repository_answers_alone() {
    let (left, right, arrangement) = both("phase-0", Founding::shared());

    assert_eq!(
        state(&left, arrangement.left.instance).expect("the left repository reconstructs"),
        alone(1),
        "the fund, and one plan of 40 against it"
    );
    assert_eq!(
        state(&right, arrangement.right.instance).expect("the right repository reconstructs"),
        alone(2),
        "the fund, and one plan of 70 against it"
    );
    assert_ne!(alone(1), alone(2), "and the two are told apart by value");

    // The arrangement's own claim about the worlds each side decided, against what a reader rebuilds.
    // A world is content-addressed, so these agree or the subject is describing repositories it did
    // not write.
    for (repository, side) in [(&left, &arrangement.left), (&right, &arrangement.right)] {
        let rebuilt = reading::corroborated(repository).expect("reconstructs");

        assert_eq!(
            rebuilt
                .lineage
                .decided()
                .iter()
                .map(|world| world.id())
                .collect::<Vec<_>>(),
            side.worlds,
            "the worlds on disk are the worlds the arrangement names"
        );
    }
}

/// What one meeting came to, named rather than described.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Met {
    /// The journals diverge, at the entry named.
    RefusedAt { position: usize },
    /// The merge went through, and the subject holds this many entries and worlds.
    Merged { entries: usize, worlds: usize },
}

/// Hand one repository's reading to the other, and say what came of it.
///
/// The subject is written on success and the party is not, which is C5 and is asserted by the phase
/// rather than here.
fn met(subject: &Repository, party: &Repository) -> Met {
    let held = collision::as_party(party).expect("a repository read back is a working copy");

    match converge::converge(subject, &held) {
        Ok(merged) => Met::Merged {
            entries: merged.journal.len(),
            worlds: merged.lineage.decided().len(),
        },
        Err(ConvergeError::Diverged { position, .. }) => Met::RefusedAt { position },
        Err(other) => panic!("refused for a reason this experiment predicted none of: {other}"),
    }
}

/// Phase 1 — the probe, in three relations and both directions.
///
/// C1 and C5. There is no operation whose subject is two repositories, so a meeting is expressed by
/// handing one repository's reading to the other — which is what `converge` takes, because a
/// repository read back **is** a working copy.
///
/// The coordinate is the measurement. Two repositories founded independently diverge at **0**, and
/// two that admitted the same base diverge at the entry after it — so the position of the refusal is
/// the size of what they happened to share, and a refusal becomes an answer rather than an obstacle.
#[test]
fn phase_1_the_probe_in_three_relations() {
    for (relation, expected) in [
        (Relation::Disjoint, Met::RefusedAt { position: 0 }),
        (
            Relation::Shared,
            Met::RefusedAt {
                position: BASE_ENTRIES,
            },
        ),
    ] {
        let name = format!("phase-1-{relation:?}");
        let (left, right, arrangement) = both(&name, Founding::of(relation));

        let before = (
            state(&left, arrangement.left.instance).expect("reconstructs"),
            state(&right, arrangement.right.instance).expect("reconstructs"),
        );

        assert_eq!(
            met(&left, &right),
            expected,
            "{relation:?}: left as subject, right as party"
        );
        assert_eq!(
            met(&right, &left),
            expected,
            "{relation:?}: and the same the other way round, which is what makes it about the journals"
        );

        assert_eq!(
            (
                state(&left, arrangement.left.instance).expect("reconstructs"),
                state(&right, arrangement.right.instance).expect("reconstructs")
            ),
            before,
            "{relation:?}: a refused meeting leaves both repositories as they were"
        );
    }
}

/// The third relation, and it is the one that is not a meeting.
///
/// Where one journal **extends** the other, nothing is refused: the merge is defined for exactly this
/// case, because it is what a party that read late looks like. So the third of the three relations
/// turns out to be the case the application already had, and calling it a meeting between two
/// repositories is what would be new about it.
///
/// C5 is measured here rather than in Phase 1, because an asymmetry can only be seen where something
/// succeeds: the subject holds both lines afterwards, and the party holds what it always held and has
/// no way to find out.
#[test]
fn phase_1_extending_is_the_case_the_merge_was_built_for() {
    let (left, right, arrangement) = both("phase-1-extending", Founding::of(Relation::Extending));

    let party_held = state(&right, arrangement.right.instance).expect("reconstructs");

    assert_eq!(
        met(&left, &right),
        Met::Merged {
            entries: SIDE_ENTRIES + 1,
            worlds: SIDE_WORLDS * 2,
        },
        "the longer journal is taken whole, and every decision either side holds is kept"
    );

    // What the subject holds afterwards, by value. Four decisions produce three distinct worlds — the
    // base is decided by both sides and recorded twice, because the merge unions the RECORDS of
    // decisions and not the worlds they produce.
    assert_eq!(
        state(&left, arrangement.left.instance).expect("the merged repository reconstructs"),
        State {
            entries: SIDE_ENTRIES + 1,
            decisions: SIDE_WORLDS * 2,
            worlds: SIDE_WORLDS * 2,
            intended: vec![INTENDED[0], INTENDED[0], INTENDED[1], INTENDED[2]],
        },
        "and the world both sides decided is answered for twice"
    );

    assert_eq!(
        state(&right, arrangement.right.instance).expect("reconstructs"),
        party_held,
        "the party is untouched, and nothing in it says a meeting happened"
    );

    // And the other direction, over two fresh repositories, because criterion 5 asks for both and
    // because the two directions reach the merged journal by different halves of the same comparison:
    // one takes what it holds, the other takes what it found.
    let (left, right, arrangement) =
        both("phase-1-extending-back", Founding::of(Relation::Extending));
    let party_held = state(&left, arrangement.left.instance).expect("reconstructs");

    assert_eq!(
        met(&right, &left),
        Met::Merged {
            entries: SIDE_ENTRIES + 1,
            worlds: SIDE_WORLDS * 2,
        },
        "the outcome is the same value whichever repository is the subject"
    );
    assert_eq!(
        state(&right, arrangement.right.instance).expect("the merged repository reconstructs"),
        State {
            entries: SIDE_ENTRIES + 1,
            decisions: SIDE_WORLDS * 2,
            worlds: SIDE_WORLDS * 2,
            intended: vec![INTENDED[0], INTENDED[0], INTENDED[1], INTENDED[2]],
        },
        "by value, and it is the same repository either way"
    );
    assert_eq!(
        state(&left, arrangement.left.instance).expect("reconstructs"),
        party_held,
        "and it is the subject that gains, which is the whole of the asymmetry"
    );
}

/// The entries two journals share, counted from the front.
fn shared_prefix(one: &[ape_cli::journal::EntryId], other: &[ape_cli::journal::EntryId]) -> usize {
    one.iter().zip(other).take_while(|(a, b)| a == b).count()
}

/// Phase 2 — the base nobody copied.
///
/// C2, and it is the prediction that would make a merge-base derived rather than found. Both
/// repositories are built by one construction, so where they share a base they share it **because the
/// admissions were the same**. Nothing was cloned; no operation ran between them.
///
/// And the assertion that ties this to Phase 1: the length of the shared prefix, measured here from
/// the journals, is the position the refusal named there. A refusal's coordinate is a measurement of
/// what two repositories have in common.
#[test]
fn phase_2_the_base_nobody_copied() {
    for (relation, shared) in [
        (Relation::Disjoint, 0),
        (Relation::Shared, BASE_ENTRIES),
        (Relation::Extending, SIDE_ENTRIES),
    ] {
        let arrangement =
            collision::arranged(Founding::of(relation)).expect("the arrangement holds");

        let (left, right) = (
            collision::entries(&arrangement.left.files).expect("the left journal admits"),
            collision::entries(&arrangement.right.files).expect("the right journal admits"),
        );

        assert_eq!(
            shared_prefix(&left, &right),
            shared,
            "{relation:?}: the shared prefix is the one the relation promises"
        );

        // Disjoint means disjoint: not merely a different first entry, but nothing in common at all.
        let common = left.iter().filter(|entry| right.contains(entry)).count();

        assert_eq!(
            common,
            match relation {
                Relation::Disjoint => 0,
                Relation::Shared | Relation::Twinned => BASE_ENTRIES,
                Relation::Extending => SIDE_ENTRIES,
            },
            "{relation:?}: and what they share as a set is what they share as a prefix"
        );
    }

    // The same shared base, after each side has gone on to admit something of its own. A prefix that
    // stopped being shared once either side grew would be a shared prefix in name only.
    let arrangement = collision::arranged(Founding::shared()).expect("the arrangement holds");
    let (left, right) = (
        collision::entries(&arrangement.left.files).expect("admits"),
        collision::entries(&arrangement.right.files).expect("admits"),
    );

    assert_eq!(left.len(), SIDE_ENTRIES, "each side grew by its own plan");
    assert_eq!(right.len(), SIDE_ENTRIES);
    assert_eq!(
        shared_prefix(&left, &right),
        BASE_ENTRIES,
        "and the base they never exchanged is still the base they both hold"
    );
    assert_ne!(
        left[BASE_ENTRIES], right[BASE_ENTRIES],
        "diverging at the first entry each admitted on its own"
    );
}

/// The worlds a repository decided, rebuilt from its files.
fn decided(repository: &Repository) -> Vec<ape::engine::thesis::Thesis> {
    reading::corroborated(repository)
        .expect("reconstructs")
        .lineage
        .decided()
        .to_vec()
}

/// Phase 3 — one decision, taken twice.
///
/// C3. Both sides take the same genesis at the same instant, with nothing between them, and the two
/// worlds are compared **by identity**. They are the same world — a `Thesis` is identified by its
/// parent, its cut and its selection, and the arrangement made all three equal without either side
/// being asked.
///
/// Then the condition is removed, and it is removed at the cut rather than at the decision: one side
/// admits the Event that settles the fund. The instant is the same, the selection is the same, and the
/// worlds are different — because a cut is `(known_at, event_head)` and the head is **resolved against
/// the Event chain that stood**.
#[test]
fn phase_3_one_decision_taken_twice() {
    let (left, right, _) = both("phase-3-agree", Founding::shared());
    let (agreeing, other) = (decided(&left), decided(&right));

    assert_eq!(
        agreeing[0].id(),
        other[0].id(),
        "two repositories decided one world, and nothing coordinated it"
    );
    assert_eq!(
        agreeing[0].cut().known_at(),
        other[0].cut().known_at(),
        "at the same instant"
    );
    assert_eq!(
        (agreeing[0].cut().event_head(), other[0].cut().event_head()),
        (None, None),
        "and the instant resolved to no head on either side, because neither had an Event"
    );
    assert_ne!(
        agreeing[1].id(),
        other[1].id(),
        "and what they each went on to intend is not one world"
    );

    // The same decision, over knowledge that differs by one Event. Nothing about the decision moved.
    let (settled, unsettled, _) = both(
        "phase-3-cut",
        Founding {
            relation: Relation::Shared,
            settling: collision::Settling::LeftOnly,
            unselected: collision::Unselected::Neither,
        },
    );
    let (settled, unsettled) = (decided(&settled), decided(&unsettled));

    assert_eq!(
        settled[0].cut().known_at(),
        unsettled[0].cut().known_at(),
        "the same instant is named"
    );
    assert!(
        settled[0].cut().event_head().is_some() && unsettled[0].cut().event_head().is_none(),
        "and it resolves to a head on the side that had an Event, and to none on the side that did not"
    );
    assert_ne!(
        settled[0].id(),
        unsettled[0].id(),
        "so the same decision produced two worlds, and the difference is in the knowledge under them"
    );
}

/// Phase 4 — what a world's identity does not pin.
///
/// C4. One side admits a Commitment no world selects, so the journals differ and nothing a world is
/// identified by does. Every world identity on that side is the one it had without the extra
/// knowledge — which makes the closed set of blind spots a rule rather than a list:
///
/// > **A world's identity is blind to every admission except the ones its selection names and the
/// > Events its cut resolves against.**
///
/// Three positions of that one rule are measured: an unselected Commitment (blind), an Event
/// (`known_at` unchanged, head moved), and a selected Commitment (the selection itself).
///
/// And then the consequence, which is C1 meeting C3: two repositories that agree about a world by
/// identity are still refused a meeting, because what the merge compares is the journal.
#[test]
fn phase_4_what_a_worlds_identity_does_not_pin() {
    let plain = collision::arranged(Founding::shared()).expect("the arrangement holds");
    let carrying = collision::arranged(Founding {
        relation: Relation::Shared,
        settling: collision::Settling::Neither,
        unselected: collision::Unselected::LeftOnly,
    })
    .expect("the arrangement holds");

    assert_eq!(
        collision::entries(&carrying.left.files)
            .expect("admits")
            .len(),
        SIDE_ENTRIES + 1,
        "the left journal holds one entry more than it did"
    );
    assert_eq!(
        carrying.left.worlds, plain.left.worlds,
        "and every world it decided is the world it decided without that entry"
    );

    // The blind spot is not merely *this* entry: it is that a world names a parent, a cut and a
    // selection, and a journal is none of those. The two edges of the rule, measured.
    let blind = collision::entries(&carrying.left.files).expect("admits");
    let seen = collision::entries(&plain.left.files).expect("admits");

    assert_ne!(blind, seen, "the journals differ");
    assert_eq!(
        carrying.left.worlds.len(),
        SIDE_WORLDS,
        "and the lineage does not"
    );

    // C1 with C3, which is the experiment — and it needs the one relation where the two sides have
    // nothing of their own. Under `Shared` a refusal proves nothing about unnamed knowledge, because
    // each side admitted a plan of its own and the journals would diverge at 13 without any help.
    // `Twinned` removes that: the same base, the same plan, so the two lineages are the same lineage.
    let twinned = collision::arranged(Founding {
        relation: Relation::Twinned,
        settling: collision::Settling::Neither,
        unselected: collision::Unselected::LeftOnly,
    })
    .expect("the arrangement holds");

    assert_eq!(
        twinned.left.worlds, twinned.right.worlds,
        "the two repositories agree about every world either of them holds"
    );

    let left = found("phase-4-left", &twinned.left);
    let right = found("phase-4-right", &twinned.right);

    assert_eq!(
        agreed(&right, &twinned.left.worlds),
        SIDE_WORLDS,
        "and each holds all of the other's, asked by identity"
    );
    assert_eq!(
        met(&left, &right),
        Met::RefusedAt {
            position: BASE_ENTRIES
        },
        "and they are refused a meeting anyway"
    );
    assert_eq!(
        met(&right, &left),
        Met::RefusedAt {
            position: BASE_ENTRIES
        },
        "in both directions"
    );

    // What is at the position that refuses them: on one side the inflow no world names, and on the
    // other the plan both sides made. So the refusal is not about a *kind* of entry — it is that the
    // journals say different things in the same place, and one of the two things is invisible to
    // every world both repositories hold.
    let (holding, plain) = (
        collision::entries(&twinned.left.files).expect("admits"),
        collision::entries(&twinned.right.files).expect("admits"),
    );

    assert_eq!(
        holding.len(),
        plain.len() + 1,
        "one side knows one thing more"
    );
    assert_ne!(
        holding[BASE_ENTRIES], plain[BASE_ENTRIES],
        "and at the entry after the base the two journals say different things"
    );
    assert_eq!(
        holding[BASE_ENTRIES + 1],
        plain[BASE_ENTRIES],
        "because the extra knowledge shifted the plan they agree about by one place"
    );
}

/// Phase 4's neighbour — why the merge does not simply admit both and keep going.
///
/// The refusal Phase 4 measures is `ConvergeError::Diverged`, raised in the application before any
/// admission is attempted. So the obvious question is what the engine would say if it were asked: the
/// union of two twinned repositories' knowledge **is** one admissible journal — base, then the inflow
/// no world names, then the plan both sides made — and one of them already holds exactly it.
///
/// This does the natural thing by hand, and the refusal moves one layer down rather than going away.
#[test]
fn the_natural_merge_is_refused_one_layer_down() {
    let twinned = collision::arranged(Founding {
        relation: Relation::Twinned,
        settling: collision::Settling::Neither,
        unselected: collision::Unselected::LeftOnly,
    })
    .expect("the arrangement holds");

    // The union of their knowledge, which needs no interleaving: the left already holds every entry
    // the right holds, and one more in the middle.
    let journal = &twinned.left.files.journal;
    let entries = collision::entries(&twinned.left.files).expect("admits");
    let mut decisions: Vec<_> = twinned
        .left
        .files
        .lineage
        .iter()
        .chain(twinned.right.files.lineage.iter())
        .cloned()
        .collect();

    // Ordered the way the merge orders, because a rebuild admits the journal in step with the lineage
    // and a decision cannot be applied after the replay has passed the entry it was taken at. This is
    // `converge`'s own linearization: by where in the journal each decision was taken, then by what the
    // decision itself says.
    decisions.sort_by_key(|taken| {
        (
            entries.iter().position(|entry| *entry == taken.after),
            taken.clone(),
        )
    });

    assert_eq!(
        journal.len(),
        SIDE_ENTRIES + 1,
        "one journal holds both sides' knowledge, and it is admissible — the left admits it"
    );

    let mut canon = ape::canon::Canon::new(ape_cli::history::ResidentHistory::new());

    match ape_cli::lineage::rebuild(&mut canon, journal, &decisions) {
        Err(ape_cli::error::LineageError::UnwitnessedKnowledge { entry }) => {
            assert_eq!(
                entry, entries[BASE_ENTRIES],
                "the entry named is the one admitted between the base and the plan"
            );
        }
        Err(other) => panic!("refused, and for another reason: {other}"),
        Ok(_) => panic!("expected the union to be refused, and it rebuilt"),
    }
}

/// What two repositories agree about, asked of each by the other's identities.
fn agreed(one: &Repository, worlds: &[ape::engine::thesis::ThesisId]) -> usize {
    worlds
        .iter()
        .filter(|world| converge::holds(one, **world).expect("readable"))
        .count()
}

/// Phase 5 — what the application can express, and what it cannot.
///
/// C5, and the answer is not the one the prediction gave. The prediction was that the application can
/// only express a meeting by making one repository a party, which is true of **acting** — `converge`
/// takes a repository and a working copy, writes the subject and leaves the party untouched.
///
/// But it turns out to have a symmetric **question** already, and nobody built it for this.
/// `converge::holds` asks whether a repository holds a world, by identity. The coordination experiment
/// added it so a party could find out whether what it decided survived. Pointed at another
/// repository's world identity, it answers what two repositories agree about — read-only, in both
/// directions, with neither of them told.
///
/// So the record can **say** what two repositories have in common and cannot **act** on it.
#[test]
fn phase_5_what_the_application_can_express() {
    // One agreed world for both relations that share a base, and none for the one that does not.
    //
    // `Extending` was expected to agree about **two**, and that expectation was wrong in a way worth
    // keeping: extension is a relation between *journals*, and the right side extends the left's
    // knowledge while deciding nothing about the part it carries. So agreement about worlds is not a
    // function of the journal relation — C3 says agreeing about a world requires agreeing about the
    // knowledge under it, and this is the other direction of that arrow, which does not hold.
    for (relation, agreement) in [
        (Relation::Disjoint, 0),
        (Relation::Shared, 1),
        (Relation::Extending, 1),
    ] {
        let name = format!("phase-5-{relation:?}");
        let (left, right, arrangement) = both(&name, Founding::of(relation));

        assert_eq!(
            agreed(&right, &arrangement.left.worlds),
            agreement,
            "{relation:?}: what the right repository holds of the left's worlds"
        );
        assert_eq!(
            agreed(&left, &arrangement.right.worlds),
            agreement,
            "{relation:?}: and the question is symmetric, which the merge is not"
        );

        // Asking changed nothing. Which is the half that makes it a question rather than an operation:
        // the two repositories are exactly as they were, and neither can tell it was asked.
        assert_eq!(
            state(&left, arrangement.left.instance).expect("reconstructs"),
            alone(1),
            "{relation:?}: and the left repository is untouched by having been asked"
        );
    }

    // The agreement Phase 3 measured by identity is the one this answers, and the whole of it: one
    // world out of two, for two repositories that share a base and nothing else.
    let (left, right, arrangement) = both("phase-5-scope", Founding::shared());

    assert!(
        converge::holds(&right, arrangement.left.worlds[0]).expect("readable"),
        "the world both sides decided is in both"
    );
    assert!(
        !converge::holds(&right, arrangement.left.worlds[1]).expect("readable"),
        "and what the left went on to intend is in neither but the left"
    );

    // And then the operation that would act on it, over the same two repositories, refuses.
    assert_eq!(
        met(&left, &right),
        Met::RefusedAt {
            position: BASE_ENTRIES
        },
        "so the record answers what they share and refuses to put it together"
    );
}
