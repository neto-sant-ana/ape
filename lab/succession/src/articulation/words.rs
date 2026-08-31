//! The words a carving renders in, so a reader is not doing translation instead of reading.
//!
//! # Why this exists, and what it costs
//!
//! P5 needs a human reader, and the readers available speak Portuguese. Handed the English
//! rendering, a Brazilian volunteer spends their attention on the language — and *how much English
//! a reader has* is not a property of any carving. The measurement would be about the wrong thing.
//!
//! So the carving is parameterised by language. **The English rendering is what three agents read
//! and does not move**; the Portuguese one exists for the human half, and the difference between
//! them is recorded rather than smoothed:
//!
//! > The human reader reads a **translated rendering** of the record. Their verdicts are therefore
//! > weaker evidence against the agents' than three agents' are against each other, and no
//! > conclusion may rest on the comparison of the two numbers alone. What P5 asks is about
//! > navigation — where a reader starts, and what they open — and that survives translation.
//!
//! # Three layers, and only two of them may be translated
//!
//! ```text
//! hex identities        addresses. Translating one would be inventing a record
//! journal labels        `finance`, `operations`, `cash`, `spend` — the record's OWN words,
//!                       written by the parties. Data, not presentation
//! field names and       `frozen`, `taken by`, "An Agent, labelled X" — the first is the
//!   connective prose    application's vocabulary, the second is this generator's. Both are
//!                       how the record is SHOWN, and neither is the record
//! ```
//!
//! The line is *is this in the file, or did the renderer put it there?* An `EntryId` and the label
//! `finance` are in the file. `- accountable:` is `ape-cli`'s field name and reaches the reader only
//! because this module prints it.

/// Which words a carving renders in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    /// What the agents read. Does not move.
    English,
    /// For a human reader who should be reading rather than translating.
    Portuguese,
}

/// Every word a carving prints that is not in the record.
///
/// A struct rather than a lookup by string, so a term added to a rendering **cannot** be forgotten
/// in the other language: the compiler asks for it.
pub struct Words {
    pub record: &'static str,
    pub arm: &'static str,
    pub vocabulary: &'static str,
    pub vocabulary_note_b: &'static str,
    pub vocabulary_note_c: &'static str,
    pub overflow: &'static str,
    pub overflow_note: &'static str,
    pub said: &'static str,
    pub in_arms: &'static str,
    pub journal_in_order: &'static str,
    pub custody_section: &'static str,
    pub decisions_in_order: &'static str,
    pub entries_in_full: &'static str,
    pub decisions_in_full: &'static str,
    pub counts: &'static str,
    pub an_agent: &'static str,
    pub a_commitment: &'static str,
    pub an_event: &'static str,
    pub a_kind: &'static str,
    pub accountable: &'static str,
    pub executors: &'static str,
    pub beneficiaries: &'static str,
    pub statement: &'static str,
    pub resource_instance: &'static str,
    pub committed_at: &'static str,
    pub due: &'static str,
    pub magnitude: &'static str,
    pub none: &'static str,
    pub dependencies: &'static str,
    pub recorded_at: &'static str,
    pub settles: &'static str,
    pub observation: &'static str,
    pub occurred_at: &'static str,
    pub produces_world: &'static str,
    pub known_at: &'static str,
    pub event_head: &'static str,
    pub frozen: &'static str,
    pub open: &'static str,
    pub taken_by: &'static str,
    pub nobody: &'static str,
    pub unnamed_agent: &'static str,
    pub taken_after: &'static str,
    pub witnessed: &'static str,
    pub extends: &'static str,
    pub selection: &'static str,
    pub omitted: &'static str,
    pub introduced: &'static str,
    pub held_by: &'static str,
    pub decision: &'static str,
    pub genesis: &'static str,
    pub advance: &'static str,
    pub fork: &'static str,
    pub decision_of: &'static str,
    pub producing: &'static str,
    pub after: &'static str,
    pub witnessing: &'static str,
    /// Page-name stems, which a reader sees before opening anything.
    pub page_agent: &'static str,
    pub page_commitment: &'static str,
    pub page_event: &'static str,
    pub page_thesis: &'static str,
    pub page_decision: &'static str,
    pub page_arm: &'static str,
    pub page_vocabulary: &'static str,
    pub page_overflow: &'static str,
    pub page_record: &'static str,
    /// Entry kinds, which name a section and a page.
    pub kinds: [(&'static str, &'static str); 9],
}

pub const ENGLISH: Words = Words {
    record: "The record",
    arm: "Arm",
    vocabulary: "Vocabulary",
    vocabulary_note_b: "The entries that introduce names.",
    vocabulary_note_c: "Every entry any arm of this record admitted.",
    overflow: "Overflow",
    overflow_note: "What was said about this record that is not about any one part of it.",
    said: "What was said about it",
    in_arms: "in",
    journal_in_order: "Journal, in the order it was admitted",
    custody_section: "Custody — every address this journal comes to",
    decisions_in_order: "Decisions, in the order taken",
    entries_in_full: "Entries in full",
    decisions_in_full: "Decisions in full",
    counts: "journal entries, {c} custody addresses, {d} decisions, {w} worlds.",
    an_agent: "An Agent, labelled",
    a_commitment: "A Commitment.",
    an_event: "An Event.",
    a_kind: "A",
    accountable: "accountable",
    executors: "executors",
    beneficiaries: "beneficiaries",
    statement: "statement",
    resource_instance: "resource instance",
    committed_at: "committed at",
    due: "due",
    magnitude: "magnitude",
    none: "none",
    dependencies: "dependencies",
    recorded_at: "recorded at",
    settles: "settles",
    observation: "observation",
    occurred_at: "occurred at",
    produces_world: "produces world",
    known_at: "known at",
    event_head: "event head",
    frozen: "frozen",
    open: "open",
    taken_by: "taken by",
    nobody: "nobody — the decision claims no party",
    unnamed_agent: "an agent this record does not name",
    taken_after: "taken after entry",
    witnessed: "witnessed entries",
    extends: "extends",
    selection: "selection",
    omitted: "omitted",
    introduced: "introduced",
    held_by: "held by",
    decision: "decision",
    genesis: "genesis",
    advance: "advance",
    fork: "fork",
    decision_of: "Decision",
    producing: "producing",
    after: "after",
    witnessing: "witnessing",
    page_agent: "agent",
    page_commitment: "commitment",
    page_event: "event",
    page_thesis: "thesis",
    page_decision: "decision",
    page_arm: "arm",
    page_vocabulary: "vocabulary",
    page_overflow: "overflow",
    page_record: "record",
    kinds: [
        ("role", "role"),
        ("agent", "agent"),
        ("eligibility", "eligibility"),
        ("resource", "resource"),
        ("resource-instance", "resource-instance"),
        ("action", "action"),
        ("statement", "statement"),
        ("commitment", "commitment"),
        ("event", "event"),
    ],
};

pub const PORTUGUESE: Words = Words {
    record: "O registro",
    arm: "Braço",
    vocabulary: "Vocabulário",
    vocabulary_note_b: "As entradas que introduzem nomes.",
    vocabulary_note_c: "Todas as entradas que qualquer braço deste registro admitiu.",
    overflow: "Avulsos",
    overflow_note: "O que foi dito sobre este registro que não é sobre nenhuma parte dele.",
    said: "O que se disse a respeito",
    in_arms: "em",
    journal_in_order: "Diário, na ordem em que foi admitido",
    custody_section: "Custódia — todos os endereços a que este diário chega",
    decisions_in_order: "Decisões, na ordem em que foram tomadas",
    entries_in_full: "Entradas por extenso",
    decisions_in_full: "Decisões por extenso",
    counts: "entradas no diário, {c} endereços de custódia, {d} decisões, {w} mundos.",
    an_agent: "Um Agente, rotulado",
    a_commitment: "Um Compromisso.",
    an_event: "Um Evento.",
    a_kind: "Um(a)",
    accountable: "responsável",
    executors: "executores",
    beneficiaries: "beneficiários",
    statement: "enunciado",
    resource_instance: "instância de recurso",
    committed_at: "comprometido em",
    due: "vence em",
    magnitude: "magnitude",
    none: "nenhuma",
    dependencies: "dependências",
    recorded_at: "registrado em",
    settles: "liquida",
    observation: "observação",
    occurred_at: "ocorreu em",
    produces_world: "produz o mundo",
    known_at: "conhecido até",
    event_head: "cabeça de eventos",
    frozen: "congelados",
    open: "abertos",
    taken_by: "tomada por",
    nobody: "ninguém — a decisão não reivindica parte alguma",
    unnamed_agent: "um agente que este registro não nomeia",
    taken_after: "tomada após a entrada",
    witnessed: "entradas testemunhadas",
    extends: "estende",
    selection: "seleção",
    omitted: "omitidos",
    introduced: "introduzidos",
    held_by: "sustentado por",
    decision: "decisão",
    genesis: "gênese",
    advance: "avanço",
    fork: "bifurcação",
    decision_of: "Decisão",
    producing: "produzindo",
    after: "após",
    witnessing: "testemunhando",
    page_agent: "agente",
    page_commitment: "compromisso",
    page_event: "evento",
    page_thesis: "tese",
    page_decision: "decisao",
    page_arm: "braco",
    page_vocabulary: "vocabulario",
    page_overflow: "avulsos",
    page_record: "registro",
    kinds: [
        ("role", "papel"),
        ("agent", "agente"),
        ("eligibility", "elegibilidade"),
        ("resource", "recurso"),
        ("resource-instance", "instância de recurso"),
        ("action", "ação"),
        ("statement", "enunciado"),
        ("commitment", "compromisso"),
        ("event", "evento"),
    ],
};

impl Lang {
    pub fn words(&self) -> &'static Words {
        match self {
            Self::English => &ENGLISH,
            Self::Portuguese => &PORTUGUESE,
        }
    }

    /// The suffix a carving's directory gets, so the two renderings never overwrite each other.
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::English => "",
            Self::Portuguese => "-pt",
        }
    }
}

impl Words {
    /// What this rendering calls an entry kind the file spells in English.
    pub fn kind(&self, spelled: &str) -> &'static str {
        self.kinds
            .iter()
            .find(|(en, _)| *en == spelled)
            .map(|(_, said)| *said)
            .unwrap_or("?")
    }

    /// What this rendering calls a decision kind — `genesis`, `advance`, `fork`.
    pub fn decides<'a>(&self, spelled: &'a str) -> &'a str
    where
        'static: 'a,
    {
        match spelled {
            "genesis" => self.genesis,
            "advance" => self.advance,
            "fork" => self.fork,
            other => other,
        }
    }

    /// The one line whose numbers are interleaved with words.
    pub fn counted(
        &self,
        entries: usize,
        custody: usize,
        decisions: usize,
        worlds: usize,
    ) -> String {
        format!(
            "{entries} {}",
            self.counts
                .replace("{c}", &custody.to_string())
                .replace("{d}", &decisions.to_string())
                .replace("{w}", &worlds.to_string())
        )
    }
}
