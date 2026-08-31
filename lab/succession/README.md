# Succession

The third row, opened 29/08/2026. **What is a record worth to somebody who was not there?**

`frontier/` studies the join between what the engine means and what an application must carry.
`agents/` studies whether that meaning survives a caller the engine was not designed for. Both are
about the **writing**. This row is about what a later reader gets, and it is named for the handoff
rather than for the reader, because the pressure is applied where a record outlives the people and the
sessions that produced it.

```text
frontier/     what must an application do to carry that meaning across a process?
agents/       does the meaning survive a caller the engine was not designed for?
succession/   what is the record worth to whoever comes next?
```

The core states this row's premise as a consequence, not as a goal, and states it about a reader:

> *What reaches the record reaches it through a primitive — a label, a magnitude, an identity — so **a
> meaning that reaches no primitive is a meaning the next reader of that record does not have**.*
> — [`core/src/docs/01-ontology.md`](../../core/src/docs/01-ontology.md)

## Which hypotheses it serves

[`CHARTER.md`](../CHARTER.md) H3, H4 and H5. All three are about what the record **is** and what it is
**for**, which is why none of them fit the other two rows — a point
[`candidates/04-training.md`](../candidates/04-training.md) made in writing eleven experiments ago and
that nobody could act on, because there was nowhere to put it.

```text
H3   is the engine a substrate for training decision reasoning?
H4   can the record hold the material that reaches no primitive, and in what representation?
H5   is the result navigable by human and autonomous readers indiscriminately?
```

## What this row may ask of the crates, and what it may not

The two existing rows differ in what they may ask of `ape-cli`, and this one is a third case that has
to be stated rather than inherited.

* **It may not change the engine or the application**, like both other rows. An experiment that
  repaired what it was measuring would be measuring its own repair.
* **It produces requirements, not obligations.** `frontier/` produces obligations the application
  implements, with `cli/tests/pedigree.rs` refusing a claim whose experiment did not earn it. A
  succession finding is a statement about what a *representation* must hold — a requirement that some
  later experiment must turn into an obligation before anything is built on it. The distinction is not
  ceremony: a requirement derived from reading is weaker evidence than a measurement, and calling it an
  obligation would launder the difference.
* **It reads the other rows' artefacts as data.** That is its main instrument and its main hazard —
  see below.

## The hazard this row has that the others do not

**Its subject is prose, and prose is classified by a reader who has an interest in the outcome.**

`frontier/` measures a program: a guard is red or green, and a mutation either changes behaviour or
does not. `agents/` measures what an agent did, and its severe failure mode — leaking the experiment to
the agent — is procedural and can be closed with a permission bit. This row's severe failure mode is
that **the person classifying the material wants a particular answer**, and nothing about a
classification is red or green on its own.

Every protocol in this row carries a specific answer to that, and *being careful* is not one. What has
been used so far: derive the categories from a sample the finding is then tested against; commit the
classification as data before the reading of it is written; and put a second classifier who has not
read the hypothesis on a sample, so that agreement is measured rather than assumed.

## Experiments

```text
00-testimony    what did eight agents say that the record cannot hold?
                CONCLUDED — 423 claims, and the leftover is TWO bounded sets rather than
                one, decided by whether the reader of the testimony has a decision to make

01-articulation given hypertext, what is a page and what does a link mean?
                OPEN. The format turned out to be free — every structure holds a `why`,
                and markdown-with-wikilinks is HTML-with-anchors in other syntax. What is
                not free is the carving
```
