//! Axiom tests: event admission.

use super::*;

#[test]
fn admits_a_recognized_event_and_rejects_an_unrecognized_one() {
    let mut f = discrete_graph();
    let commitment = f.store.add_commitment(commit(&f).unwrap());
    let axiom = Axiom::new(&f.store);

    assert!(
        axiom
            .admit_event(EventInput {
                commitment_id: commitment,
                observation: obs("Signed"),
                previous_event: None,
                occurred_at: date(2026, 6, 1),
            })
            .is_ok()
    );

    assert!(matches!(
        axiom.admit_event(EventInput {
            commitment_id: commitment,
            observation: obs("Unrelated"),
            previous_event: None,
            occurred_at: date(2026, 6, 1),
        }),
        Err(AxiomError::ObservationNotSettling)
    ));
}
