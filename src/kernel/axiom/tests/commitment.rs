//! Axiom tests: commitment admission.

use super::*;

#[test]
fn admits_a_consistent_discrete_commitment() {
    let f = discrete_graph();
    assert!(commit(&f).is_ok());
}

#[test]
fn rejects_commitment_with_instance_of_another_resource() {
    let mut f = discrete_graph();

    let other = f.store.add_resource(
        Resource::create(ResourceInput {
            label: ident("other-resource"),
            kind: ResourceKind::Discrete,
        })
        .unwrap(),
    );

    let alien = f.store.add_instance(
        ResourceInstance::create(ResourceInstanceInput {
            label: ident("alien-instance"),
            resource: other,
        })
        .unwrap(),
    );

    f.instance = alien;

    assert!(matches!(
        commit(&f),
        Err(AxiomError::ResourceInstanceMismatch { .. })
    ));
}

#[test]
fn rejects_discrete_commitment_carrying_a_value() {
    let f = discrete_graph();
    let mut input = commitment_input(&f);

    input.action_value = ActionValue::value(5.0).unwrap();

    let result = Axiom::new(&f.store).admit_commitment(input);

    assert!(matches!(result, Err(AxiomError::ActionValueMismatch)));
}

#[test]
fn accountable_needs_no_role_only_existence() {
    let mut f = discrete_graph();

    let bystander = f.store.add_agent(
        Agent::create(AgentInput {
            label: ident("bystander"),
            kind: AgentKind::Company,
        })
        .unwrap(),
    );

    f.accountable = bystander;

    assert!(commit(&f).is_ok());
}

#[test]
fn rejects_supersede_of_a_different_statement() {
    let mut f = discrete_graph();

    let action = f.store.add_action(
        Action::create(ActionInput {
            verb: ident("other-action"),
            kind: ActionKind::Discrete,
            resource: f.resource,
        })
        .unwrap(),
    );

    let other_statement = f.store.add_statement(
        Statement::create(StatementInput {
            participants: Participants::new([f.actor_role], [f.recipient_role]).unwrap(),
            action,
            settlement: Settlement::new([obs("Signed")], [obs("Cancelled")]).unwrap(),
        })
        .unwrap(),
    );

    let superseded_input = CommitmentInput {
        statement: other_statement,
        ..commitment_input(&f)
    };

    let superseded = f
        .store
        .add_commitment(Axiom::new(&f.store).admit_commitment(superseded_input).unwrap());

    let result = Axiom::new(&f.store).admit_commitment(CommitmentInput {
        supersedes: Some(superseded),
        ..commitment_input(&f)
    });

    assert!(matches!(result, Err(AxiomError::SupersedeStatementMismatch)));
}

#[test]
fn rejects_supersede_of_a_different_resource_instance() {
    let mut f = discrete_graph();

    let other_instance = f.store.add_instance(
        ResourceInstance::create(ResourceInstanceInput {
            label: ident("instance-2"),
            resource: f.resource,
        })
        .unwrap(),
    );

    let superseded = f.store.add_commitment(
        Axiom::new(&f.store)
            .admit_commitment(CommitmentInput {
                resource: other_instance,
                ..commitment_input(&f)
            })
            .unwrap(),
    );

    let result = Axiom::new(&f.store).admit_commitment(CommitmentInput {
        supersedes: Some(superseded),
        ..commitment_input(&f)
    });

    assert!(matches!(
        result,
        Err(AxiomError::SupersedeResourceInstanceMismatch)
    ));
}

#[test]
fn admits_supersede_revising_the_same_target() {
    let mut f = discrete_graph();

    let original = f.store.add_commitment(commit(&f).unwrap());

    let result = Axiom::new(&f.store).admit_commitment(CommitmentInput {
        supersedes: Some(original),
        term: Term::new(date(2026, 1, 1), date(2027, 6, 30)).unwrap(),
        ..commitment_input(&f)
    });

    assert!(result.is_ok());
}

#[test]
fn rejects_commitment_with_unknown_dependency() {
    let f = discrete_graph();

    let result = Axiom::new(&f.store).admit_commitment(CommitmentInput {
        dependencies: BTreeSet::from([CommitmentId::from([1u8; 32])]),
        ..commitment_input(&f)
    });

    assert!(matches!(result, Err(AxiomError::UnknownCommitment(_))));
}

#[test]
fn rejects_supersede_of_unknown_commitment() {
    let f = discrete_graph();

    let result = Axiom::new(&f.store).admit_commitment(CommitmentInput {
        supersedes: Some(CommitmentId::from([1u8; 32])),
        ..commitment_input(&f)
    });

    assert!(matches!(result, Err(AxiomError::UnknownCommitment(_))));
}

