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
//! ├── known_at    → the latest recording instant a Commitment may be selected from
//! └── event_head  → the factual chain the Thesis interprets
//! ```
//!
//! The pair must agree, and agreeing is the type's own invariant: the recognized head must
//! have been recorded no later than `known_at`. Because that invariant names the head's
//! canonical record, it can only be settled by reading one — which is why a cut is declared
//! through the reader rather than assembled from two fields. A `KnowledgeCut` in hand is a
//! coherent cut, and nothing downstream re-establishes it.
//!
//! What `declare` proves is local, and one-sided: nothing recognized here was recorded after
//! `known_at`. It does not prove the converse — that everything recorded by `known_at` is
//! recognized. Settling that would mean knowing the latest Event recorded no later than the
//! instant, a question only canonical history can answer and this port cannot ask.
//!
//! So a cut naming a head earlier than that one remains constructible, and keeping the two
//! coordinates describing the same moment is the application's part. The distinction matters
//! more than it looks: a Thesis whose head is old *because the past was short when it was made*
//! is a Thesis that fell behind, which the model names and permits, while one whose instant is
//! current and whose head is old has set aside facts that were already known. Only the second
//! is a retraction, and only the second is what this constructor cannot yet tell apart.
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
