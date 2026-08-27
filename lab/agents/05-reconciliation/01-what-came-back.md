# Observation 01 — What came back, before any of it is judged

Phase 3. Everything here is the run as it happened; the frictions are answered in
[`02`](02-the-documented-carried-and-the-undocumented-happened.md) and after.

## The run

```text
invoked         2026-08-27, one subagent, one prompt
the prompt      the briefing's absolute path, "read GOAL.md there and do what it says,
                work only inside that directory", and nothing else
returned        after 62 tool calls
artifacts       run-a/main.rs, run-a/ANSWER.md, run-a/output.txt, and the two records
```

## The barrier held, and so did the channel it did not cover

`lab/` was unreadable for the duration. The agent's own list of paths read holds nothing under it,
which is criterion 2 met — but a self-report is the weakest kind of evidence and this run has a
second kind.

**It read the session scratchpad**, which the barrier did not cover and the protocol did not
anticipate: it wrote `mine-pristine/`, `mutation/` and `swap/` there and says so. That directory also
holds this session's own working files, three of which — the commit messages for the two commits
that sealed this experiment — describe the frictions, name `MERGING.md`, and would have ruined the
run.

Measured by access time rather than taken on trust:

```text
r1.txt r2.txt r3.txt   last read 17:17:34–39, which is `git commit -F` reading them
the agent's own dirs   written 17:30, 17:34, 17:36
```

Every experiment-describing file in that directory was last touched **thirteen minutes before the
agent's first write**, and none was touched during the run. So the self-report is corroborated by
something the agent does not control, which is the first time this row has had that.

**It is recorded as a near miss rather than as a clean result.** The barrier was put on `lab/` because
that is where the laboratory lives. The scratchpad is a second copy of part of it, it was open, and
nothing but the agent's own restraint kept it out. See
[`04`](04-the-channel-the-barrier-did-not-cover.md).

## What it did

```text
1  read both records, and printed what they do and do not hold in common
2  called converge(mine, theirs) and printed the refusal — measured, not assumed
3  spliced the journals: operations' 20, then finance's 1
4  measured that the other order is refused by the Canon, and printed that refusal too
5  carried finance's two Decisions VERBATIM and re-witnessed them against the merged prefix
6  checked every world either record claimed comes back identically, and made the write
   conditional on it
7  wrote the merged record into mine/ through converge
8  re-read mine/ from disk and printed what a reader told nothing would see
9  asked Synthesis what the two tips do to each other, and printed the answer without deciding it
```

## What `mine/` holds at the end

```text
mine/current   b
mine/a         untouched, byte-identical to the briefing
mine/b         21 entries, 21 custody addresses, 5 decisions, 5 worlds

74a6a53e4e4d  parent -              by (nobody)     both parties decided this one
1f093bfa4767  parent 74a6a53e4e4d   by operations
b939289591aa  parent 1f093bfa4767   by operations
558f991d1bd3  parent 74a6a53e4e4d   by finance      retaken
dd201a84e58e  parent 558f991d1bd3   by finance      retaken
```

`theirs/` is byte-identical to the briefing, all five files.

## Three things checked rather than believed

**The result reproduces.** Restoring a copy to the pre-merge state — digests re-checked against the
ones published before the agent existed — and running its program again produces `mine/b` byte for
byte:

```text
4aa8a4bddf04  custody.json      b97b133d71d2  journal.json
de50d9d5046f  lineage.json      7f5d61ba76e4  worlds.json
```

**Its guard goes red, and names the world.** It reports having verified this; inverting the filter in
`unreproduced` confirms it, with the message it claims:

```text
assertion `left == right` failed: the world that did not come back is the one reported
  left: ["aa"]   right: ["cc"]
```

**Its account of what it could not prove is accurate.** It says the repository-level wiring of that
guard is unexercised because `reading::corroborated` refuses a tampered record before the guard is
reached. That is what the application does, and the honest half is that it said so rather than
letting two green tests imply the write was gated by something anybody had watched refuse.
