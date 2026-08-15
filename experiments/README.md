# Experiments

This directory holds questions asked *of* APE that no crate in the workspace owns.

`ape` defines operational meaning. `ape-cli` explores what an application must do to carry
that meaning across a process. Neither answers whether the meaning survives a different
kind of pressure — a caller the engine was not designed for, a use nobody had in mind when
the ontology was fixed.

An experiment exists to apply exactly one such pressure and report what happened.

---

# Rules

Every experiment here obeys the same three constraints.

**The engine is the subject, not the material.**

An experiment may not change `ape` to make itself succeed. If it needs a change, that need
*is* the result, and it belongs in the record before any code is written.

**Dependencies run one way, and never sideways.**

```text
experiment
    ↓
   ape
```

An experiment never depends on `ape-cli`, and the CLI never depends on an experiment. They
ask different questions and must be able to fail independently.

**A failed experiment is a result.**

The reason something did not work is worth more than a passing test obtained by relaxing
the question until it passed.

---

# Record

Each experiment is a directory holding its own protocol, its numbered observations as it
produces them, and its result.

The protocol is written before the experiment runs, and is not edited afterwards to match
what happened. Observations are appended. The result is written last.

Directories appear as their experiment begins. An experiment that has been planned but not
started has no directory — its absence is the honest signal that nothing has been observed
yet.

---

# Index

**[`agents/`](agents)** — whether an autonomous LLM agent can express, evaluate and later
reconstruct an operational decision through APE without extending its ontology.
