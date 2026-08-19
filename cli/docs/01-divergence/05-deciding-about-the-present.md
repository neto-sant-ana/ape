# Observation 5 — An ordering discipline is sound, and it forbids deciding about the present

The protocol left it open whether an application could avoid the divergence by refusing to
admit knowledge within an instant a decision names, and noted that such an answer would be a
constraint on how an application may behave rather than a claim about what a repository
records.

It is both, and the constraint is larger than the phrasing suggests.

---

## The rule, and why it is the only one available

Canonical history refuses an admission recorded before its watermark and admits one recorded
*at* it. Writing `w` for the watermark and `d` for the instant a decision names:

```text
every future admission has recorded_at ≥ w
head_as_of(d) can still move   ⟺   an admission may land in [w, d]
that window is empty           ⟺   d < w
```

So the discipline is one strict comparison, and it is the weakest sound one. Anything that
admits `d ≥ w` admits an instant the engine would still accept an Event within, and an Event
within it moves the cut that instant resolves. `d = w` is not a borderline case: it is the day
in progress.

The mirror rule — decide freely, and refuse to admit at an instant already decided — closes
the same window from the other side and is refused by the same subject one phase later. What
the arrangement defeats is not a variant; it is the family.

---

## What it costs, measured

Every decision the subject takes names an instant later than the watermark, so the discipline
refuses all of them at the moment they are taken. A refused decision is not lost — an instant
seals as soon as recording passes it, so the honest behaviour is to hold the decision and try
again when knowledge arrives. That is what the run does, and this is what it gets:

```text
recorded through 01-05   genesis   at 01-10   refused
recorded through 01-10   genesis   at 01-10   refused   ← reaching is not passing
recorded through 01-11   genesis   at 01-10   taken
recorded through 01-11   advance   at 01-15   refused   ← and forever
                         fork                 unreachable
```

One world of three. And it is not one of the three: waiting for the tenth to close means
waiting past the Event recorded within it, so the world that can be taken is the world the
cancellation produced — the same cut, partition and absence of a verdict that Observation 1
measured as the divergence. The refusal at −70 is not reproduced; it is unreachable.

The two decisions behind it are unreachable for reasons that compound rather than repeat. The
advancement names the fifteenth and nothing in the journal is ever recorded after it, so no
moment of this history seals that instant. The fork inherits its parent's cut, and the
alternative was recorded on the eleventh — a day a cut at the tenth cannot know, which the
engine refuses as anachronism. Together:

> *A disciplined application can only reach worlds at sealed instants, and an intention
> admitted after the last sealed instant can never be selected by one.*

An application that decides only about closed days cannot select what was admitted today.

---

## What is written down does not change, and that is the problem

The repository is byte-for-byte the shape Observation 2 pinned: a decision records an instant
and an intention, and nothing that places it. The closed field set in Phase 4 is unchanged,
which is the honest form of "records nothing new".

It also means the discipline leaves no trace. What a reader gets is one-sided:

```text
no admission recorded after d   →  d was never sealed  →  the decision broke the rule
some admission recorded after d →  d is sealed now     →  disciplined, or lucky
```

Run against the repository this experiment writes, the audit catches the advancement at
`2026-01-15` and misses the genesis at `2026-01-10`, which broke the discipline exactly as
badly — the eleventh was recorded after the tenth, but only after the decision was taken. The
audit is a necessary condition, and the sufficient one is the ordering between the two
sequences, which is precisely what no file holds.

So a reader of the repository alone cannot tell a faithful reconstruction from a lucky one.
The discipline moves correctness out of the record and into the conduct of a process that is
no longer running.

---

## Consequences to carry

* The discipline is real and cheap — one comparison against a watermark the engine already
  maintains — and it is enforced where a decision is taken rather than where one is applied.
  What it cannot do is be checked afterwards.
* It is not compatible with either subject this laboratory has. The reconstruction
  experiment's decisions are refused by it too, at the same place and for the same reason, and
  its repository fails the audit at its advancement. That experiment reconstructed correctly
  regardless — which is the definition of lucky, and the reason the distinction matters.
* An application that reasons about today is outside the rule, and reasoning about today is
  not an edge case. Both subjects decide at an instant ahead of what is recorded, because that
  is what deciding means when knowledge is still arriving.
* Whatever closes this without forbidding the present has to place the decision in the
  sequence of admissions, which is a change to what a repository records rather than to when
  an application may act. This observation is the measurement of the alternative, not an
  argument against it.
