//! Axiom tests: commitment emission.

use super::*;

#[test]
fn accepts_a_consistent_discrete_commitment() {
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

    let result = Axiom::new(&f.store).emit_commitment(input);

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
fn rejects_commitment_with_unknown_dependency() {
    let f = discrete_graph();

    let result = Axiom::new(&f.store).emit_commitment(CommitmentInput {
        dependencies: BTreeSet::from([CommitmentId::from([1u8; 32])]),
        ..commitment_input(&f)
    });

    assert!(matches!(result, Err(AxiomError::UnknownCommitment(_))));
}

