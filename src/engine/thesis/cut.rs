//! `KnowledgeCut` — the two coordinates that together say what could be known.
//!
//! An Event Head is necessary and not sufficient. It delimits Events, and a Commitment never
//! enters the Event chain: its recording instant is its only knowledge-time coordinate. A cut
//! addressed by a head alone would therefore admit a Commitment recorded long after that
//! head, and the world it selects would contain intentions that were not yet knowledge —
//! anachronism at construction, which no rule about projection can undo afterwards.
//!
//! ```text
//! KnowledgeCut
//! ├── known_at    → the instant through which knowledge is recognized
//! └── event_head  → the chain recognized within it
//! ```
//!
//! The pair must agree, and agreeing is the type's own invariant: the recognized head must
//! have been recorded no later than `known_at`. Because that invariant names the head's
//! canonical record, it can only be settled by reading one — which is why a cut is declared
//! through the reader rather than assembled from two fields. A `KnowledgeCut` in hand is a
//! coherent cut, and nothing downstream re-establishes it.
//!
//! Coherence is not maximality. A cut may recognize a head earlier than the latest Event
//! recorded by `known_at`, including no head at all while Events exist — a plan drawn before
//! anything was observed, which is the ordinary case rather than the exception. What the cut
//! forbids is knowledge from *after* it, never ignorance of what came before.
//!
//! Resolution is the recording instant's: `known_at` is a civil date, so a cut is a day.
//! Anachronism is barred across days, not within one.

use super::ThesisError;

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
    /// Declare the cut a Thesis recognizes, refusing a head that was not yet recorded at
    /// `known_at`.
    pub fn declare<K: CanonicalKnowledge>(
        knowledge: &K,
        known_at: Date,
        event_head: Option<EventId>,
    ) -> Result<Self, ThesisError> {
        if let Some(head) = event_head {
            let record = knowledge
                .canonical_event(head)
                .ok_or(ThesisError::UnknownEvent(head))?;

            if !record.recorded_at().up_to(&known_at) {
                return Err(ThesisError::EventNotKnownAtCut {
                    event: head,
                    recorded_at: *record.recorded_at(),
                    known_at,
                });
            }
        }

        Ok(Self {
            known_at,
            event_head,
        })
    }

    pub fn known_at(&self) -> &Date {
        &self.known_at
    }

    pub fn event_head(&self) -> Option<EventId> {
        self.event_head
    }
}
