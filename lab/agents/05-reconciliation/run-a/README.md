# Run A — operations, handed finance's record

**Published before the agent was invoked.** Everything below — the digests, the state the agent will
meet, and the deviations — is the record of the briefing as it stood with nobody having read it.

```text
date            2026-08-27
engine + cli    ae3fbc0   (core/ and cli/ clean against it)
agent           a subagent of the session that assembled the briefing, given the briefing
                and nothing else
task            act for operations; end up holding one record
```

## What the agent will meet, measured

Written by `ape-agents`' `parted` binary and printed before the briefing was sealed:

```text
base entries shared by both  19
mine    20 entries, 3 decisions
theirs  20 entries, 3 decisions

converge  Diverged at 19, 19 entries in common
```

So the two records agree for the whole base and part at the first entry after it, and the operation a
caller reaches for refuses them while saying how much they hold in common. That refusal carrying a
second number is three days old and this is its first outside reader.

## The briefing, and the proof it was not edited

```text
e517842f83658b0f889510839efad2a1ba35eafbd9f7d5a86ea4f4778a500182  GOAL.md
39c4c51ecb16e4b559f79eabbed4943f37b5a6ee14354dab30c60551bbdc2485  Cargo.toml
a4253230ac0c136037a7e690a6c048263e2dc8255cac32e4a771295914feba32  src/main.rs
ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb  mine/current
52c4eb6a0ca9147e8bd9a6711da55535a876e5d563ba03c4a51c4dc0b6943997  mine/a/custody.json
45ae1170e48a29972ce3fe8330e6d724b69dbedd7414e2c4b51c64d0c4b58ce3  mine/a/journal.json
f5b28b94741679f589db9e4518fb61093e091861151481c0103c3d187f414592  mine/a/lineage.json
9079edbaafa7c981fc0e1b36ae767747e512d49ada4c61349358efcb85251b81  mine/a/worlds.json
ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb  theirs/current
c1b94efd20048f3daeb8b77924820057a9ae0a17594dc13730b0441ef09469ad  theirs/a/custody.json
f23f10f24950e668c78c8a90caaf1fe813250f3a45e0f4dfb196cf3ad5bb40da  theirs/a/journal.json
4e5fe964a859fc45009956fdd2ff6f99a6b7f1435bdaf951b5079deaece31158  theirs/a/lineage.json
c63e822bc160f2792628a8f52d81a45a5020ff21981fc4c02d9ec55ec7a23edf  theirs/a/worlds.json
```

The briefing was built and compiled before the agent was invoked, which is the first run's mistake
turned into a step.

## The vendored crates, and the eight routes that had to go

Both crates were copied in, so nothing in the briefing refers to a path outside it. What is new since
experiment 04 is that the application's docstrings had begun **citing the laboratory by path** — eight
of them, added as the frontier row earned obligations into `cli/`. Every one is a route to the
experiment, which is the thing the method's isolation forbids by construction.

Removed, with the prose they sat in kept. Six files were edited and their digests are here for the
same reason 04 published its one:

```text
f6e3e9aad3c662618933e292d04ad2426c61a37fe49c33a89c3ef8c0b5fa2686  cli/src/lib.rs
51826628251000210af3ac8ead5a7fdfa9d21c00881b8bf2b5e548e47eb38085  cli/src/error.rs
5b5c9dc8eb34766f4d40e71b9fe11948a669e3e5ec5c318d25e6d2dafa290d41  cli/src/lineage.rs
16320b9831619f347d18a7d607e4735ba97f662d9ef1d927516395dc0bf9ad90  cli/src/converge.rs
e18fe29620b6e1d55be157c185a070969e214f48e7752a760cdb83717340228d  cli/src/repository.rs
```

**And both crates' `README.md` were excluded entirely**, which is a deviation and not a tidy-up.
`cli/README.md` links to the laboratory's directory and to its results, and explains the pedigree
guard: it is a route map rather than a docstring. Excluding it means the agent is a **weaker caller
than a real one** — a real caller reads the README — and the exclusion is in the experiment's favour,
which is why it is here rather than folded in.

`cli/tests/` and `core/tests/` were excluded on 04's reasoning, unchanged.

## What the library still tells the agent, and is not a leak

Four things, measured out of the vendored crates and written into
[the protocol](../00-protocol.md) before the frictions were:

* that the two records are **divergent rather than incompatible** — the refusal's own message says so;
* the recovery and its order — read again and admit again, earlier-dated first;
* that `by` is the party that **took** a decision;
* that a custody claim exists, what it is, and what it costs.

That is the boundary documenting itself, and 04 established it is legitimate: a library that says why
it is shaped as it is is what a real caller has. It is recorded here because the first draft of this
experiment's predictions was about things the briefing answers, and the second draft is built on the
difference.

**What the crates do not say anywhere** is that another party's decisions can be *retaken* in this
record's frame. That is the experiment.

## Isolation, and this is the first run that was enforced rather than arranged

Every earlier run recorded the same honest weakness: *the agent was never handed a route to the
experiment; it was not technically prevented from looking for one.* This one starts worse and ends
better, and both halves belong here.

**Worse, because a subagent begins in the repository.** A separate session begins in the briefing
directory; this one's working directory is the repository root, so a directory listing reaches the
laboratory. That is a route nobody had to be handed.

**Better, because the route was closed.** `lab/` was made unreadable for the duration of the run:

```text
chmod 000 lab      before invoking
chmod 755 lab      after
```

Measured before it was relied on, as a non-root user against a directory of the same ownership:

```text
cat inside        Permission denied
ls inside         Permission denied
grep -r from above  cannot open directory: Permission denied
```

**It is the kernel and not the harness**, which is the whole reason it was chosen. The alternative
considered was a `deny` rule in the session's permissions, and it was rejected: those match a tool name
and a path pattern, and a shell is a language — `cat $(echo lab)/…` matches no pattern. A deny rule is
advice. `chmod` is a barrier.

The other alternative, running the subagent in a git worktree, is worse rather than better: a worktree
is a second copy of the repository, laboratory included.

**What it still does not do.** The agent's listing shows `core/`, `cli/` and a `lab/` it cannot open,
so it knows a laboratory exists. That much the vendored crates' prose already implies and it was kept
deliberately. What it cannot do is read one.

**And the cost, stated.** For the duration of the run the session that assembled the briefing cannot
read `lab/` either, which is irrelevant while it waits — and if the run dies mid-flight the directory
is left unreadable until somebody restores it. One command, and it is named above so that the next
person does not have to work it out.

The agent is also asked to list every path it read. That is self-reported and is now a **check on the
barrier** rather than the barrier itself. If the list holds anything under `lab/`, the run is void and
is recorded as void — fixed in advance so that the question afterwards is not how much it saw.

---

## Everything above was written before the agent existed. What happened is elsewhere

The run is recorded in [`01`](../01-what-came-back.md) and judged from
[`02`](../02-the-documented-carried-and-the-undocumented-happened.md) onward, so that this page stays
what it claims to be.

`GOAL.md`, `main.rs`, `ANSWER.md`, `output.txt` and both records as the run left them are in this
directory. The run is not void — nothing under `lab/` is in the agent's path list, and the access times
of this session's own files corroborate it. **The barrier had a gap beside it**, and that is
[`04`](../04-the-channel-the-barrier-did-not-cover.md).
