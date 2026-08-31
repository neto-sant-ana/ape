# Observation 01 — The corpus, fixed before it is read

Phase 0. **Nothing here has been classified.** This document exists so that the material is pinned
before anybody reads it for this purpose, and so that a later reader can check that the thing measured
is the thing described.

## The eight files

Pinned at `712161b`, the commit this experiment opened against.

```text
8d3002356fcda041e7b7c11ee91ef8a2ff3435a2261f331bdaf2c0cfbf03e12a   1617  01-single-agent/run-01
d42aff7bf5357c002a1f8a17295c4c0dc46d10fe4ac5b1ae7594115455960f6d   2377  02-hindsight/run-01
a4045464424817608762ce5d0de7407c9e1091280eb6232c3eca21c9d44def25   1469  03-narrative-mismatch/run-b
abb64b660080f6f077df0519baf254be60639179b2ce8cfa06b7722ac1bdfb17   1617  04-multiagent/run-a
737db4d5e8a6592b3491bdd3fd58b04ad9a8587e70a185afdcb721bb5088bf71   1595  04-multiagent/run-b-prime
b4fc8d68c4fa9acf4c433f504a4c1ad952a3c816aca3fc94311620c489298291   1698  04-multiagent/run-b
50f8d6eb6fedcee8b0c33163d35429ab5a9bcbb762f987316c925db668f77540   1136  04-multiagent/run-reader
e2f376b85f7bd3e3ebf4cceb8c55e6096b1674acd50b7767e0616e0506459c67   1873  05-reconciliation/run-a
                                                                   -----
                                                                   13382
```

Every path is under [`lab/agents`](../../agents). The digests are here for the same reason the agents
row publishes a briefing's digests before invoking: so that *the corpus was not edited to suit the
finding* is checkable rather than asserted.

## What each file is, and what it is not

**An `ANSWER.md` is the deliverable an agent was asked for**, in its own words, as part of a task whose
goal document is beside it. It is not a report about APE and was never written for this row.

```text
01-single-agent/run-01      in-memory boundary, engine db3f965. The first run, and the
                           only one whose boundary no longer exists
02-hindsight/run-01        in-memory. Asked what could have been known versus what was
                           consulted
03-narrative-mismatch/     in-memory. The run whose prose was sorted against the derived
run-b                      record and found wrong in specific ways
04-multiagent/run-a        the repository boundary. One of two parties
04-multiagent/run-b        the other party
04-multiagent/run-b-prime  a re-run of run-b after its arrangement was corrected
04-multiagent/run-reader   the only agent that READ rather than wrote
05-reconciliation/run-a    the repository with four files, and the row's first enforced
                           isolation
```

**Two boundaries and one reader**, and the protocol pre-registered why that matters: a kind that
appears only under the in-memory boundary is a property of that boundary, and a kind that appears only
in writers is a property of writing. `run-reader` is the control, and P5 is about it.

## What is deliberately not in the corpus

**The `GOAL.md` beside each answer**, which is the task rather than the testimony. It is read when a
claim's meaning depends on what was asked, and it is not classified.

**This laboratory's own result documents.** They are our reading of what happened; the corpus is the
agents' words. A `99-result.md` is the one part of the archive written by somebody who knew what it
would be used for, which is exactly what disqualifies it here.

**`04-multiagent/run-b` and `run-b-prime` are both kept**, though one supersedes the other as evidence
about its experiment. For this experiment they are two testimonies, and dropping one would be dropping
the only near-duplicate the corpus has — two runs of one task, which is the closest thing available to
asking whether a kind is a property of the task or of the agent.

## The order the reading happens in, and why it is fixed here

**The seven are classified before `05-reconciliation`.** Its five kinds are the candidate set, so
reading it first would let the order do the arguing — [`00-protocol.md`](00-protocol.md) lists that as
a failure condition rather than a preference, and this is where the order stops being a choice made
later.

Within the seven, the order is the one above: oldest boundary first, `run-reader` last of the
multiagent group. No claim about the corpus depends on that order, and it is recorded so that it is not
adjusted afterwards.
