//! An entity's content-addressed id: what decides it, and what that costs.
//!
//! `define_entity!` derives an id as `sha256` over each field's encoding, taken in **declaration
//! order**. So three things decide an identity, and only the third is not ours:
//!
//! ```text
//! the order the fields are declared in       ours
//! how each field's type encodes itself       ours — a set, a hex string, an option
//! the encoder's wire format                  postcard's, stable at 1.x by its author's commitment
//! ```
//!
//! The third is the one that looks like a risk and is not: the format is specified and frozen for
//! 1.x, and the next major is a differently named crate, so `cargo update` cannot reach it. The first
//! two are deliberate and revisable — and they had no alarm.
//!
//! A repository refers to knowledge **by identity**, so a change to any of the three does not make an
//! older repository disagree: it makes it stop replaying, because a reference resolves to nothing.
//!
//! # The one time this happened, and what it cost
//!
//! A bound stopped being `f64` and became `i128`, and a magnitude became `u128` — unsigned, because
//! the direction is the effect's. So the second of the three changed for the two families that hold
//! one: `Resource`, through its `Constraint`, and `Commitment`, through its `ActionValue`. Seven of
//! the nine pins below did not move, and the two that did moved for different reasons, which is why
//! the signedness of a magnitude is visible here at all.
//!
//! **That is not the blast radius on a repository, and reading it as one would be wrong.** These pins
//! are derived from synthetic references — `[1; 32]` is not a plausible identity — so a family moves
//! here only when a quantity is among its *own* fields. In a real history the references are real: an
//! instance names a resource, a commitment names an instance, an event names a commitment. One moved
//! `Resource` moves everything downstream of it, which is every identity in a history that has a
//! quantifiable resource in it.

define_error! {
    pub enum IdentityError {
        Serialization => "failed to serialize a field while computing the entity id",
    }
}

/// The derivation, pinned: known inputs against known identities.
///
/// # This is a pin, not a prediction
///
/// The literals were read off a run and frozen here. That is the opposite of the rule this project
/// applies to *measurements*, where a literal written after the numbers were seen is not a
/// measurement — and it is right for the same reason: these do not claim an identity ought to be
/// anything. They claim it is what it was, so that changing it has to be said out loud.
///
/// # One family each, and why not one entity
///
/// A change to the macro or to the encoder moves every identity, so any single pin would catch it.
/// What needs one pin per family is a change to **one** declaration — reordering two fields of
/// `Statement` leaves the other eight untouched, and a guard watching only `Role` would pass.
///
/// Measured, by doing exactly that: this turns red, and so do the seven suites elsewhere in the
/// workspace that read a committed repository. Everything else stays green — 307 tests of 315 pass
/// through a change that moves every identity in the kernel.
///
/// `Thesis` is pinned in its own module's suite, where the constructor its `Selection` needs is
/// visible.
#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::super::{
        Action, ActionId, ActionInput, Agent, AgentId, AgentInput, Commitment, CommitmentId,
        CommitmentInput, EligibilityAssignment, EligibilityAssignmentInput, Event, EventId,
        EventInput, Resource, ResourceId, ResourceInput, ResourceInstance, ResourceInstanceId,
        ResourceInstanceInput, Role, RoleId, RoleInput, Statement, StatementId, StatementInput,
    };

    use crate::kernel::value_objects::{
        ActionKind, ActionValue, Assignment, Constraint, Date, Effect, Identifier, Observation,
        Participants, ResourceKind, Settlement, Term,
    };

    fn ident(value: &str) -> Identifier {
        Identifier::new(value).expect("a pinned label is well formed")
    }

    fn obs(value: &str) -> Observation {
        Observation::new(value).expect("a pinned observation is well formed")
    }

    fn day(day: u8) -> Date {
        Date::from_ymd(2026, 1, day).expect("a pinned date is real")
    }

    /// Every family's identity, from inputs fixed here.
    ///
    /// The inputs are arbitrary and obviously so — `[1; 32]` is not a plausible identity. Nothing
    /// about the values matters except that they never change.
    fn derived() -> Vec<(&'static str, String)> {
        let role = Role::create(RoleInput {
            label: ident("role"),
        })
        .expect("derivable");

        let agent = Agent::create(AgentInput {
            label: ident("agent"),
        })
        .expect("derivable");

        let eligibility = EligibilityAssignment::create(EligibilityAssignmentInput {
            agent: AgentId::from([1; 32]),
            roles: BTreeSet::from([RoleId::from([2; 32]), RoleId::from([3; 32])]),
            effective_from: day(1),
        })
        .expect("derivable");

        let resource = Resource::create(ResourceInput {
            label: ident("resource"),
            kind: ResourceKind::Quantifiable(
                Constraint::between(0, 100).expect("a pinned constraint is well formed"),
            ),
        })
        .expect("derivable");

        let instance = ResourceInstance::create(ResourceInstanceInput {
            label: ident("instance"),
            resource: ResourceId::from([4; 32]),
        })
        .expect("derivable");

        let action = Action::create(ActionInput {
            verb: ident("act"),
            kind: ActionKind::Quantifiable(Effect::Increase),
            resource: ResourceId::from([5; 32]),
        })
        .expect("derivable");

        let statement = Statement::create(StatementInput {
            participants: Participants::new([RoleId::from([6; 32])], [RoleId::from([7; 32])])
                .expect("a pinned participation is well formed"),
            action: ActionId::from([8; 32]),
            settlement: Settlement::new([obs("Done")], [obs("Void")])
                .expect("a pinned settlement is well formed"),
        })
        .expect("derivable");

        let commitment = Commitment::create(CommitmentInput {
            assignment: Assignment::new(
                AgentId::from([9; 32]),
                [AgentId::from([10; 32])],
                [AgentId::from([11; 32])],
            )
            .expect("a pinned assignment is well formed"),
            statement: StatementId::from([12; 32]),
            resource: ResourceInstanceId::from([13; 32]),
            term: Term::new(day(1), day(20)).expect("a pinned term is well formed"),
            action_value: ActionValue::value(10).expect("a pinned value is well formed"),
            dependencies: BTreeSet::from([CommitmentId::from([14; 32])]),
        })
        .expect("derivable");

        let event = Event::create(EventInput {
            commitment_id: CommitmentId::from([15; 32]),
            observation: obs("Done"),
            previous_event: Some(EventId::from([16; 32])),
            occurred_at: day(3),
        })
        .expect("derivable");

        vec![
            ("Role", role.id().to_string()),
            ("Agent", agent.id().to_string()),
            ("EligibilityAssignment", eligibility.id().to_string()),
            ("Resource", resource.id().to_string()),
            ("ResourceInstance", instance.id().to_string()),
            ("Action", action.id().to_string()),
            ("Statement", statement.id().to_string()),
            ("Commitment", commitment.id().to_string()),
            ("Event", event.id().to_string()),
        ]
    }

    /// The identity of every kernel family, unchanged.
    ///
    /// Compared as one table rather than one assertion each, so a change that moved every identity
    /// reports every family at once instead of stopping at the first.
    #[test]
    fn the_identity_derivation_is_unchanged() {
        let pinned: Vec<(&str, &str)> = vec![
            (
                "Role",
                "b51462c88504a22db538d7847ad9456159278656316af30b12dbc31f189c3c51",
            ),
            (
                "Agent",
                "5ed91bbebb6fb63275b05909cd34c5da52d93ae528eabc5572c6d4d0e7bd0ef9",
            ),
            (
                "EligibilityAssignment",
                "7db65528ffff7fa9df4778e9a22829b1532262845f489be9433ac996dfcbf41a",
            ),
            (
                "Resource",
                "258c8675534a0590531bba61c80c9d87f008a0b694b7ed8f0595c35f8ba64acd",
            ),
            (
                "ResourceInstance",
                "8ef2f381d4aadd150fd0636762b3974c482139c0ceaacfe3233de12c3a41aef6",
            ),
            (
                "Action",
                "58f552bf447dbffb65c1ca0cc67173da9823463c93324102c7525774f1a6e925",
            ),
            (
                "Statement",
                "578bf4d61f641da2a776ace4589e93bd26f7482d0986278f66f3b2bd371f1698",
            ),
            (
                "Commitment",
                "7b81eda5291a70fa8b3368c7c51a4a41b622a238c41417a0f3aae2597a900832",
            ),
            (
                "Event",
                "ba5c4021a5d4b3e1039c89afe1655c1025e5c506754cc71ca884e9c4136eaaa7",
            ),
        ];

        let derived = derived();
        let found: Vec<(&str, &str)> = derived
            .iter()
            .map(|(family, id)| (*family, id.as_str()))
            .collect();

        assert_eq!(
            found, pinned,
            "the identity derivation moved. If that was deliberate, update these literals and say \
             what it breaks: a repository refers to knowledge by identity, so one written before the \
             change stops replaying rather than merely disagreeing"
        );
    }
}
