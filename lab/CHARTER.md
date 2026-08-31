# Charter

The standing hypotheses this laboratory exists to test. **Above the two rows**, which are instruments
rather than subjects: [`frontier/`](frontier) asks what an application must do to carry meaning,
[`agents/`](agents) asks whether an agent can express a decision through the primitives, and both of
them serve the list below.

**This file is not the queue.** [`QUEUE.md`](QUEUE.md) orders work *inside* a hypothesis. This one says
which hypotheses there are, where each came from, and what has been spent on it.

```text
H1  the core's gaps, concretized              frontier/            ~40%, swept below
H2  an auditable trail of a decision          agents/              four runs, then parked —
                                                                   and 00-testimony found the
                                                                   half it is missing
H3  a substrate for training                  succession/          material gathered, unrun
H4  a home for what reaches no primitive      succession/          FIRST HALF ANSWERED —
                                                                   two shapes, one anchored
H5  navigable by both kinds of reader         succession/          unblocked, and smaller than
                                                                   it looked
```

[`succession/`](succession) opened 29/08/2026 with this file, because H3, H4 and H5 fit neither
existing row — which [`candidates/04-training.md`](candidates/04-training.md) said in writing eleven
experiments before anybody could act on it.

---

## Why it was written on 29/08/2026, after twenty-three experiments

Because it should have existed before the first one, and the mechanism that hid it is worth writing
down rather than resolving to do better.

The queue orders candidates by **ripeness**, and says so plainly: *"how many concluded results named the
item, which is the only ripeness signal available"*. That metric is **closed under itself**. An item
enters only when a concluded experiment named it, so the queue can produce refinements of what already
ran and nothing else. **A founding hypothesis that no experiment has touched has ripeness zero,
permanently.**

What that produced is measurable rather than impressionistic:

```text
frontier 09–17    nine consecutive experiments in one neighbourhood — what a reference IS when
                  two records meet — ending in MERGING.md, which is the answer

agents 01–04      four runs on the row's founding claim, then a deliberate ten-day stand-by

agents 05         the row's first experiment after the stand-by, whose second clause its own
                  protocol describes as "borrowed from the other row"
```

The last line is the finding. The row whose declared burden is **auditability** spent its return
answering whether two repositories merge. Nobody decided that; ripeness did.

**So the selection rule changes.** Ripeness orders work *within* a hypothesis below. It never chooses
which hypothesis is attacked — that choice is the operator's, made against this file, and an experiment
that serves none of them needs a reason in its protocol saying so.

---

## H1 — Can the gaps the core explicitly leaves to the application be concretized?

> *Where a concept is not strictly necessary to represent coordination, it belongs to the application,
> which may compose it from the primitives that exist or wrap them in types of its own.*
> — [`core/src/docs/01-ontology.md`](../core/src/docs/01-ontology.md)

**Origin:** the core's own prose, from before the laboratory existed.

**Spent:** nine experiments, on **one** gap. `converge`, `custody` and the lineage's coordinate are the
reference/merge gap concretized, and [`MERGING.md`](frontier/docs/MERGING.md) is the assembled answer.

**Swept on 29/08/2026**, because this hypothesis had never had a number. The core delegates in **ten**
distinct places across five layer documents:

```text
✓  01-ontology:29        application vocabulary composed from the primitives
✓  04-canon:143          the recording instants, held by whoever wrote them
✓  06-thesis:309         proving when the application actually decided     ← the nine experiments
✓  07-synthesis:1341     moving mutable references such as `main`

◐  06-thesis:1314        constructing a Thesis derived from the target, from a resolved
   07-synthesis:1009     transfer — `transfer::applied` exists and no command reaches it

✗  01-ontology:33        the meaning that reaches no primitive                       → H4
✗  05-hermeneia:482      which hypothesis to ask, and what a failing verdict prevents
✗  06-thesis:900,906     the selection's representation, and structural sharing      → H4, cost
✗  07-synthesis:608      how a candidate is represented
✗  07-synthesis:1607     composing applicability with feasibility
```

**Four answered, one built and unreachable, five untouched.** So H1 is roughly 40% answered — and the
shape of what is left matters more than the fraction: three of the five untouched are the *feasibility
and candidate* cluster, and two are H4.

**And the sweep found something the fraction hides.** `cli` exposes **eighteen** public functions; the
application offers **three** commands — a level reading, `transfer`, and `decided`. `converge` has no
command. Neither does `holds`, nor `applied`. The nine experiments that answered `06-thesis:309`
concretized it **as a library**, and the only readers that have ever reached it are this laboratory and
one agent that was handed the crate.

> **Concretized has two meanings, and H1 has only been tested against one.** *A library function
> exists* is not *an application can reach it* — which is the same distinction H2 and H5 depend on,
> because an audit trail and a navigable asset are things a **reader** reaches.

---

## H2 — Can the application produce an auditable trail of an autonomous agent's decision process?

> *An immutable, content-addressed operational knowledge graph for making autonomous-agent decisions
> historically reconstructible, projectable and auditable.*
>
> **Auditable** *is the one that is not a restatement. A record can be perfectly reconstructible and
> still not settle whether a decision was defensible, because defensibility is a claim about the
> alternatives that existed and were not taken.*
> — [`agents/00-question`](agents/00-question/00-question.md)

**Origin:** the agents row's founding claim. It is H2, written before its first run.

**Spent:** four runs, all of which produced the pair this hypothesis needs — a decision expressible
through the primitives and a stated reason. `03-narrative-mismatch` sorted the second against the
derivation and found it wrong in specific ways, which is the audit happening by hand.

**Where it stopped, and it is not for lack of material.** The row parked, and resumed on a question
belonging to the other row. What every run left behind is recorded in
[`candidates/04-training.md`](candidates/04-training.md) in one line, and it is the hinge to H4:

> Every run produced a decision expressible through the primitives and **a justification in prose that
> the record has no place for**.

---

## H3 — Is the engine a substrate for training decision reasoning?

**Origin:** experiment 06 was written to begin it. The material is gathered in
[`candidates/04-training.md`](candidates/04-training.md), which names three readings and says which is
cheapest and which has four runs of prior art.

**Spent:** nothing. Eleven frontier experiments ran while it sat, and the reason is now known and
fixed: it *"belongs to neither row"* by its own first page, and until [`succession/`](succession)
existed that sentence was a place it could not be put rather than a thing to act on.

**The constraint it carries**, from the agents row's own written exclusions: a *harness* is not a
*benchmark of judgment*, and **an argument that APE is for AI** is explicitly not what this laboratory
does. The protocol has to be written so that **no** is a real outcome, or it is advocacy with an
experiment attached.

---

## H4 — Is there a representation that holds the entities, their relations, **and** the reasoning that produced them?

**Origin: emergent.** It was not thought of before the work; it arrived out of the development, some
time before 29/08/2026, and it is the first hypothesis here with that provenance. It is recorded as
emergent because how a hypothesis arrived changes how much the corpus can be trusted to have prepared
for it — H1 and H2 were written before their evidence, and this one after.

**The representation is the free variable, not the premise.** It was first described as
*"markdown-like, frontmatter holding the entities and their relations by wikilink, the body holding
what is not the entity's"* — and that was **an analogy chosen to explain it**, not a specification. A
directory of files, a relational schema, a document store or something else is what the experiment
discovers. Writing the analogy into the hypothesis would decide the experiment in the protocol.

Stated without it:

> **Can the record hold, beside each entity, the material that reaches no primitive — the hypothesis,
> the motivation, why an alternative was weighed and dropped — related to the entity without being one?
> And in what representation, at what cost?**

**Why it is not a swerve, and this is the part the queue could not see.** Four agent runs measured that
this prose exists, matters, and is unhoused. The core states the consequence of leaving it unhoused, in
the ontology:

> *A meaning that reaches no primitive is a meaning the next reader of that record does not have.*

And [`06-exploration`](frontier/docs/06-exploration/99-result.md) measured that **eleven of thirteen**
decisions in a record are roads not taken — so the material this hypothesis wants a home for is the
majority of what the record holds, and the one thing the record cannot currently say about any of it is
which was **weighed** rather than **meant**.

**It carries cost with it, and that is the unlock.** The queue's oldest deferral reads:

> *Deferred by all eighteen: cost. No experiment has measured it and every one of them says so … **What
> is missing is not a method — it is a reason to want the number.***

H4 is the reason. *Which representation* is not answerable without measuring one, and the core has
already delegated exactly this decision — [`06-thesis.md`](../core/src/docs/06-thesis.md) states the
current selection's costs as `O(n)`, calls it *"a first implementation choice, not a semantic
requirement"*, and says that reducing both storage and derivation cost *"requires a hierarchical hash
structure, such as a persistent Merkle tree"* whose adoption *"would also require an explicit decision
about validation strategy"*.

**So H4 and H1 meet here**, and the cost deferral is the thing they meet through. The tree behind a
Thesis's selection is a gap the core left to the application, to be decided on cost, and nobody has
measured the cost.

**[`succession/00-testimony`](succession/00-testimony/99-result.md) — Phase 2 complete, Phase 4
outstanding.** The leftover is bounded and there are **two** sets of it, decided by whether the reader
of the testimony has a decision to make: *motivation, want, loss, qualification, method limit* for a
reader who will check, and *accountability, recommendation, own reasoning, evaluation* for one who
will choose. Only the first set has an anchor in the record — motivation attaches to one object and
explains that object — so H4's next question is sharper than the one it was given: **a representation
built for the anchored shape has no place for the other four.**

It answers the first half and refuses the second. A representation cannot be compared before the thing to be represented is
known, and the analogy H4 arrived in already presumes an answer. Its corpus is the eight `ANSWER.md`
files eight agents wrote across five experiments — **13,382 words produced before this hypothesis
existed**, by sessions that could not have shaped them to fit it, which is the strongest available
correction for H4 being emergent.

---

## H5 — Does that representation produce an asset navigable by human and autonomous readers indiscriminately?

**Origin: emergent**, with H4 and downstream of it — there is nothing to navigate until H4 has an
answer.

**The word that carries the weight is *indiscriminately*.** Two representations can both be navigable
and be navigable by different readers, and a result that measures one reader has measured half a
hypothesis. What makes it falsifiable is a task a human and an agent can both be given against the same
artefact, where the two answers are comparable.

**Spent:** nothing, and it is correctly blocked rather than neglected. It lives in
[`succession/`](succession) with H3 and H4.

---

## How a sixth arrives

H4 and H5 emerged from the work, and more will. The rule is the one this laboratory applies to every
other finding:

* **A hypothesis that lives in a conversation does not exist.** It is added here when it is recognized,
  with its origin marked *emergent* and dated, not when it is ready to be worked on.
* **Emergent is a property worth keeping, not an apology.** A hypothesis written after its evidence is
  in a different epistemic position from one written before, and the difference matters when a result
  seems to confirm it.
* **Adding one is not scheduling one.** This file records what the laboratory is for; the operator
  chooses what is attacked.
