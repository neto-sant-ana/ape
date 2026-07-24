//! Axiom tests: eligibility admission and its temporal (as-of) derivation.

use super::*;

#[test]
fn admits_a_valid_eligibility_assignment() {
    let mut store = Store::default();
    let role = store.add_role(Role::create(RoleInput { label: ident("role") }).unwrap());

    let agent = store.add_agent(
        Agent::create(AgentInput {
            label: ident("agent"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );

    let axiom = Axiom::new(&store);

    assert!(
        axiom
            .admit_eligibility_assignment(EligibilityAssignmentInput {
                agent,
                roles: BTreeSet::from([role]),
                effective_from: date(2026, 1, 1),
            })
            .is_ok()
    );
}

#[test]
fn rejects_eligibility_assignment_for_unknown_agent() {
    let mut store = Store::default();
    let role = store.add_role(Role::create(RoleInput { label: ident("role") }).unwrap());
    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.admit_eligibility_assignment(EligibilityAssignmentInput {
            agent: AgentId::from([9u8; 32]),
            roles: BTreeSet::from([role]),
            effective_from: date(2026, 1, 1),
        }),
        Err(AxiomError::UnknownAgent(_))
    ));
}

#[test]
fn rejects_eligibility_assignment_for_unknown_role() {
    let mut store = Store::default();

    let agent = store.add_agent(
        Agent::create(AgentInput {
            label: ident("agent"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );

    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.admit_eligibility_assignment(EligibilityAssignmentInput {
            agent,
            roles: BTreeSet::from([RoleId::from([9u8; 32])]),
            effective_from: date(2026, 1, 1),
        }),
        Err(AxiomError::UnknownRole(_))
    ));
}

#[test]
fn rejects_executor_without_eligibility_for_an_actor_role() {
    let mut f = discrete_graph();

    let bad = f.store.add_agent(
        Agent::create(AgentInput {
            label: ident("bad-executor"),
            kind: AgentKind::Individual,
        })
        .unwrap(),
    );

    f.executor = bad;

    assert!(matches!(
        commit(&f),
        Err(AxiomError::AgentNotEligibleForRole(_))
    ));
}

#[test]
fn admits_an_empty_eligibility_assignment_as_a_withdrawal() {
    let mut store = Store::default();

    let agent = store.add_agent(
        Agent::create(AgentInput {
            label: ident("agent"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );

    let axiom = Axiom::new(&store);

    assert!(
        axiom
            .admit_eligibility_assignment(EligibilityAssignmentInput {
                agent,
                roles: BTreeSet::new(),
                effective_from: date(2026, 1, 1),
            })
            .is_ok()
    );
}

#[test]
fn eligibility_takes_effect_on_its_own_effective_from() {
    let mut f = discrete_graph();

    let sameday = f.store.add_agent(
        Agent::create(AgentInput {
            label: ident("sameday"),
            kind: AgentKind::Individual,
        })
        .unwrap(),
    );

    f.store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: sameday,
            roles: BTreeSet::from([f.actor_role]),
            effective_from: date(2026, 1, 1),
        })
        .unwrap(),
    );

    f.executor = sameday;

    assert!(commit(&f).is_ok());
}

#[test]
fn eligibility_recorded_after_committed_at_is_not_yet_in_effect() {
    let mut f = discrete_graph();

    let latecomer = f.store.add_agent(
        Agent::create(AgentInput {
            label: ident("latecomer"),
            kind: AgentKind::Individual,
        })
        .unwrap(),
    );

    f.store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: latecomer,
            roles: BTreeSet::from([f.actor_role]),
            effective_from: date(2026, 6, 1),
        })
        .unwrap(),
    );

    f.executor = latecomer;

    assert!(matches!(
        commit(&f),
        Err(AxiomError::AgentNotEligibleForRole(_))
    ));
}

#[test]
fn a_later_empty_assignment_withdraws_eligibility() {
    let mut f = discrete_graph();

    f.store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: f.executor,
            roles: BTreeSet::new(),
            effective_from: date(2025, 6, 1),
        })
        .unwrap(),
    );

    assert!(matches!(
        commit(&f),
        Err(AxiomError::AgentNotEligibleForRole(_))
    ));
}

#[test]
fn a_withdrawal_after_committed_at_does_not_apply_retroactively() {
    let mut f = discrete_graph();

    f.store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: f.executor,
            roles: BTreeSet::new(),
            effective_from: date(2027, 1, 1),
        })
        .unwrap(),
    );

    assert!(commit(&f).is_ok());
}

#[test]
fn a_tie_on_effective_from_resolves_by_id_deterministically() {
    let mut store = Store::default();

    let agent = store.add_agent(
        Agent::create(AgentInput {
            label: ident("agent"),
            kind: AgentKind::Individual,
        })
        .unwrap(),
    );
    let role_a = store.add_role(Role::create(RoleInput { label: ident("a") }).unwrap());
    let role_b = store.add_role(Role::create(RoleInput { label: ident("b") }).unwrap());

    let effective_from = date(2025, 1, 1);
    let one = EligibilityAssignment::create(EligibilityAssignmentInput {
        agent,
        roles: BTreeSet::from([role_a]),
        effective_from,
    })
    .unwrap();
    let other = EligibilityAssignment::create(EligibilityAssignmentInput {
        agent,
        roles: BTreeSet::from([role_b]),
        effective_from,
    })
    .unwrap();

    let winner = one.id().max(other.id());
    store.add_eligibility(one);
    store.add_eligibility(other);

    assert_eq!(
        store.eligibility_at(agent, &date(2026, 1, 1)).map(|e| e.id()),
        Some(winner),
    );
}

#[test]
fn an_assignment_carrying_several_roles_satisfies_any_of_them() {
    let mut f = discrete_graph();

    let extra_role = f
        .store
        .add_role(Role::create(RoleInput { label: ident("extra") }).unwrap());

    let multi = f.store.add_agent(
        Agent::create(AgentInput {
            label: ident("multi"),
            kind: AgentKind::Individual,
        })
        .unwrap(),
    );

    f.store.add_eligibility(
        EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: multi,
            roles: BTreeSet::from([extra_role, f.actor_role]),
            effective_from: date(2025, 1, 1),
        })
        .unwrap(),
    );

    f.executor = multi;

    assert!(commit(&f).is_ok());
}
