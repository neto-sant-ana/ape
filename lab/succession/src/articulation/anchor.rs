//! Where a claim attaches to the record, decided mechanically and fixed before any agent runs.
//!
//! The protocol's method: *the reasoning that goes in comes from the classification, not from a
//! fresh reading — the 27 non-housed claims are placed by their kind and their anchor,
//! mechanically. Where a claim has no anchor under a carving, it goes to that carving's overflow.*
//!
//! **The classification holds no anchor**, which was found here rather than assumed: `Claim` carries
//! a run, a text, a verdict and a standing. So an anchor has to be *derived from the claim's own
//! words*, and the only honest derivations are the two the record itself supplies — its identities
//! and its labels. Anything else would be this laboratory reading the claims again and calling the
//! reading mechanical, which is the row's stated severe hazard.
//!
//! # The two derivations, and what each cost
//!
//! ```text
//! by identity   a backticked hex prefix that uniquely prefixes an identity the record holds
//!                 1 of the 27 non-housed claims
//! by label      a whole word that is a label the journal admits — `finance`, `operations`
//!                 9 of the 27, and it is what makes the experiment able to discriminate at all
//! ```
//!
//! Measured before the rule was chosen, and the first number is why the second exists: a rule
//! reading only identities would have put **26 of 27** in the overflow of both B and C, and two
//! carvings that differ by one claim answer nothing. That is recorded because choosing the rule
//! after seeing what it yields is exactly the move the protocol forbids between *runs* — so the
//! choosing happens here, in the open, before any agent is invoked.
//!
//! Only agent labels are taken as anchors, and not the roles, resources and verbs the journal also
//! labels. Those are vocabulary rather than parties, and a claim saying *spend* is not a claim about
//! the `spend` action — it is a sentence using a word. Fixed here rather than tuned later.
//!
//! # And 18 of 27 anchor to nothing, which the protocol already calls a result
//!
//! *The size of the overflow is itself a result.* It is reported per carving and not smoothed.

use std::collections::BTreeSet;

use crate::articulation::record::Run;

/// A claim's attachment points, in the record's own vocabulary.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Anchored {
    /// Identities the claim names, resolved against the record and full-length.
    pub identities: BTreeSet<String>,
    /// Agent identities the claim names by label.
    pub parties: BTreeSet<String>,
}

impl Anchored {
    pub fn is_empty(&self) -> bool {
        self.identities.is_empty() && self.parties.is_empty()
    }
}

/// A backticked hex prefix, which is how every testimony in this corpus abbreviates an identity.
///
/// Six is the floor because the corpus abbreviates to eight and a shorter prefix would start
/// matching by accident; uniqueness against the record is required on top, so a prefix that reaches
/// two identities anchors to neither and says so by anchoring to nothing.
fn prefixes(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = text;

    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else { break };
        let inner = &after[..close];
        let hex = inner.trim_end_matches('…');

        if hex.len() >= 6 && hex.chars().all(|glyph| glyph.is_ascii_hexdigit()) {
            found.push(hex);
        }
        rest = &after[close + 1..];
    }

    found
}

/// Whether `word` appears in `text` as a whole word, case-insensitively.
///
/// Whole-word rather than substring, because `house` is an agent here and *household* is not it.
///
/// **Protective and not load-bearing on this corpus**, which was measured rather than assumed:
/// replacing this with a plain `contains` changes no number in `tests/articulation.rs`, because
/// none of the four labels ever occurs inside a longer word in the Reconciliation testimony. It is
/// kept because the next testimony is not promised to be so tidy, and it is recorded as unexercised
/// so that nobody reads the boundary as a finding.
fn names(text: &str, word: &str) -> bool {
    let lowered = text.to_lowercase();
    let boundary = |glyph: Option<char>| glyph.is_none_or(|glyph| !glyph.is_alphanumeric());

    lowered.match_indices(word).any(|(at, _)| {
        boundary(lowered[..at].chars().next_back())
            && boundary(lowered[at + word.len()..].chars().next())
    })
}

/// What one claim attaches to, by the two derivations above, across every arm of the run.
pub fn of(text: &str, run: &Run) -> Anchored {
    let identities = run.identities();
    let mut anchored = Anchored::default();

    for prefix in prefixes(text) {
        let reached: Vec<&&str> = identities
            .iter()
            .filter(|id| id.starts_with(prefix))
            .collect();

        if let [only] = reached[..] {
            anchored.identities.insert((*only).to_owned());
        }
    }

    for (label, id) in run.labelled() {
        if run.is_agent(id) && names(text, &label) {
            anchored.parties.insert(id.to_owned());
        }
    }

    anchored
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_prefix_is_read_out_of_backticks_and_a_bare_word_is_not() {
        assert_eq!(
            prefixes("holds `4b8b9b88…` and `652a011d`"),
            ["4b8b9b88", "652a011d"]
        );
        assert!(prefixes("4b8b9b88 without backticks").is_empty());
        assert!(prefixes("`by` is not hex").is_empty());
        assert!(
            prefixes("`abcde`").is_empty(),
            "five is under the floor, so a short word cannot become an address"
        );
    }

    #[test]
    fn a_label_matches_as_a_word_and_not_inside_one() {
        assert!(names("Finance advanced to 2026-01-08", "finance"));
        assert!(names("the house is accountable", "house"));
        assert!(
            !names("a household of agents", "house"),
            "otherwise every carving anchors a claim to an agent it does not mention"
        );
        assert!(names("(finance)", "finance"), "punctuation is a boundary");
    }
}
