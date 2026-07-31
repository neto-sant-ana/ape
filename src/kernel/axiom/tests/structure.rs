//! Axiom tests: structural emission of the definitional layer.

use super::*;

use crate::kernel::value_objects::Effect;

#[test]
fn rejects_action_on_missing_resource() {
    let store = Store::default();
    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.emit_action(ActionInput {
            verb: ident("sign"),
            kind: ActionKind::Discrete,
            resource: ResourceId::from([9u8; 32]),
        }),
        Err(AxiomError::UnknownResource(_))
    ));
}

#[test]
fn rejects_action_kind_not_matching_resource_kind() {
    let mut store = Store::default();

    let resource = store.add_resource(
        Resource::create(ResourceInput {
            label: ident("resource"),
            kind: ResourceKind::Discrete,
        })
        .unwrap(),
    );

    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.emit_action(ActionInput {
            verb: ident("increase"),
            kind: ActionKind::Quantifiable(Effect::Increase),
            resource,
        }),
        Err(AxiomError::ActionResourceKindMismatch)
    ));
}

#[test]
fn rejects_statement_referencing_unknown_action() {
    let mut store = Store::default();
    let role = store.add_role(
        Role::create(RoleInput {
            label: ident("role"),
        })
        .unwrap(),
    );
    let axiom = Axiom::new(&store);

    assert!(matches!(
        axiom.emit_statement(StatementInput {
            participants: Participants::new([role], [role]).unwrap(),
            action: ActionId::from([3u8; 32]),
            settlement: Settlement::new([obs("Signed")], [obs("Cancelled")]).unwrap(),
        }),
        Err(AxiomError::UnknownAction(_))
    ));
}
