//! `KnowledgeCut` — the two coordinates that together say what could be known.
//!
//! An Event Head is necessary and not sufficient. It delimits Events, and a Commitment never
//! enters the Event chain: its recording instant is its only knowledge-time coordinate. A cut
//! addressed by a head alone would therefore admit a Commitment recorded long after that head,
//! and the world it selects would hold intentions that were not yet knowledge — anachronism at
//! construction, which no rule about projection can undo afterwards.
//!
//! ```text
//! KnowledgeCut
//! ├── known_at    → the instant the cut is taken at
//! └── event_head  → the chain that was current at it
//! ```
//!
//! The instant is what an application supplies; the head is **resolved** from it. That is what
//! makes the pair describe one moment rather than two. A cut cannot hold a current instant beside
//! an old head, so a Thesis whose head is old is one that fell behind — never one that set aside
//! facts it already knew. Retraction is not something a cut can express.
//!
//! Naming a head directly stays available, and stays a refinement rather than an escape. Several
//! Events may share a recording instant, and an instant addresses the last of them; a finer cut
//! names an earlier one *within that same instant*. Two things are required of it, and neither
//! implies the other: it must share the instant's recording date, or it would leave out Events the
//! instant recognizes; and it must lie on the chain ending at the head that instant addresses, or
//! it would recognize a reach of history that is merely contemporaneous.
//!
//! Resolution is the recording instant's: `known_at` is a civil date, so a cut is a day, and the
//! group a day addresses is every Event recorded on it.

use super::ThesisError;
use super::frozen::lies_on_chain_to;

use crate::canon::CanonicalKnowledge;

use crate::kernel::entities::EventId;

use crate::kernel::value_objects::Date;

define_value_object! {
    pub struct KnowledgeCut {
        known_at: Date,
        event_head: Option<EventId>,
    }
}
impl KnowledgeCut {
    /// The cut an instant addresses: the chain that was current at `known_at`.
    ///
    /// Nothing can fail here. Every instant addresses exactly one cut — the last Event recorded no
    /// later than it, or none where none had been — so a cut is not something an application can
    /// get wrong, only something it can choose.
    pub fn at<K: CanonicalKnowledge>(knowledge: &K, known_at: Date) -> Self {
        Self {
            event_head: knowledge.head_as_of(&known_at),
            known_at,
        }
    }

    /// A finer cut within the same instant, naming an Event of the group that instant addresses.
    pub fn within<K: CanonicalKnowledge>(
        knowledge: &K,
        known_at: Date,
        event_head: EventId,
    ) -> Result<Self, ThesisError> {
        let named = knowledge
            .canonical_event(event_head)
            .ok_or(ThesisError::UnknownEvent(event_head))?;

        if !named.recorded_at().up_to(&known_at) {
            return Err(ThesisError::EventNotKnownAtCut {
                event: event_head,
                recorded_at: *named.recorded_at(),
                known_at,
            });
        }

        let addressed = knowledge.head_as_of(&known_at);
        let addressed_at = match addressed {
            None => None,
            Some(id) => Some(
                *knowledge
                    .canonical_event(id)
                    .ok_or(ThesisError::UnknownEvent(id))?
                    .recorded_at(),
            ),
        };

        if addressed_at != Some(*named.recorded_at()) {
            return Err(ThesisError::HeadPrecedesCut {
                named: event_head,
                addressed,
            });
        }

        let addressed = addressed.expect("an instant that admits a head addresses one");

        if !lies_on_chain_to(knowledge, event_head, addressed)? {
            return Err(ThesisError::HeadDoesNotBelongToCut {
                named: event_head,
                addressed,
            });
        }

        Ok(Self {
            known_at,
            event_head: Some(event_head),
        })
    }

    pub fn known_at(&self) -> &Date {
        &self.known_at
    }

    pub fn event_head(&self) -> Option<EventId> {
        self.event_head
    }
}
