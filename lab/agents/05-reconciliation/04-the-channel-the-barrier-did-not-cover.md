# Observation 04 — The barrier held, and the channel beside it was open

Method, not result. It is here because the row's isolation claim just changed shape twice in one run
— once for the better and once for the worse — and both belong in the record.

## The barrier worked, and it is the first one this row has had

Every earlier run recorded the same honest weakness: *never handed a route, not technically prevented
from finding one.* This run closed the route with a permission bit, and the bit did what it was
measured to do. Nothing under `lab/` appears in the agent's list of paths read.

## The self-report was corroborated by something the agent does not control

Also a first. Every earlier run's *nothing was leaked* rested entirely on the agent saying so.

```text
r1.txt r2.txt r3.txt   last accessed 17:17:34–39, which is `git commit -F` reading them
mine-pristine/         written 17:30
mutation/              written 17:34
swap/                  written 17:36
```

The three files that describe this experiment were last touched thirteen minutes before the agent's
first write. Access times are a weak instrument in general — a `stat` from this session would have
disturbed the directory's own — but for these three files the question is only *was it after 17:30*,
and it was not.

## And the channel the barrier did not cover was open the whole time

**The agent used the session scratchpad**, outside the briefing directory, and said so plainly in its
path list. It wrote three working directories there: a pristine copy of `mine/` so the run could be
repeated, copies for the attempt to make its own guard refuse, and copies for running the parties
reversed. Every one of those is good practice and none of them is the problem.

The problem is what else is in that directory. It holds this session's own working files, including
the commit messages for the two commits that sealed this experiment — which name `MERGING.md`, state
the frictions, and say in as many words which one is the control. **An `ls` would have shown the
filenames. A `cat` of `r3.txt` would have ended the run.**

```text
what was protected     lab/                     by the kernel
what was not           the session scratchpad   by nothing
```

**The barrier was put where the laboratory is kept, and the laboratory had a second address.** Not a
copy anybody made on purpose — an artefact of how this session works, where commit bodies are written
to files before being pasted. The method never mentions it because until this run the agent was a
separate session with its own scratchpad.

## Recorded as a near miss

The run is not void: criterion 2 is met on the self-report and corroborated by the access times. But
the honest reading is that **the isolation that held is the one that was measured, and the one that
was not measured was open**, and it was the agent's own restraint that closed it — which is exactly
the thing this row stopped relying on when it reached for `chmod`.

For the next run, the barrier covers both, and the second is the cheaper of the two: the briefing
gets its own scratchpad, handed to the agent in the prompt, and the session's own stays unreadable
for the duration the same way `lab/` does.

**And the general lesson is the one the row keeps re-learning in new clothes.** A barrier protects a
path, not a fact. Before trusting one, the question is not *is the path closed* but *where else does
this fact live* — which is the question a duplicated truth always asks, arriving here as a truth with
two locations rather than two representations.
