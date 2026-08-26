# Candidate — Veracity

> **Answered.** Run as [experiment 11](../frontier/docs/11-veracity/00-protocol.md); the result is
> [`99-result.md`](../frontier/docs/11-veracity/99-result.md). The answer is **yes**, and it is the
> severe one: two parties learn the same fact on different days, record it truthfully, and merge — and
> the merged record answers a settled level of 0 for a world its decider had settled at 120, with every
> guard satisfied and nobody having lied. The mechanism is that no identity in the engine contains a
> recording instant, so two journals can be equal entry for entry and not be the same journal.
>
> What this page could not have supplied, and the experiment needed first, was a **definition of
> false**. It says so itself, below, and that turned out to be the whole reason 07 and 08 could not
> answer.
>
> This page is kept as it stood when the protocol was written. It is the material that was in front of
> whoever wrote it, and editing it afterwards to match what happened would remove the only evidence that
> the question was not decided in advance.

**This is not a protocol.** A protocol is written when its experiment begins, with the previous
findings in hand, and is not edited afterwards to match what happened. This is the material that would
be in front of whoever writes it.

It exists because two protocols asked the same question in almost the same words and neither answered
it, and because a third experiment had already measured an instance of it and filed the instance
somewhere else.

```text
07 atomicity   Is there a partial state that is neither refused nor silent — one that
               reconstructs and answers something FALSE?

08 contention  Is there an interleaving whose result is neither party's state and yet
               reconstructs, corroborates, and answers something FALSE?
```

Both were carried as open questions, both were excluded from their own boundary, and both went
unanswered because each experiment measured what its mechanism *does* leave rather than searching for
this. What they have in common is not the mechanism. It is the shape of the state they are hunting.

---

## The question

Corroboration proves that a repository agrees with itself. It does not prove that what the repository
says happened, happened.

> *Is there a state a record can be in that passes every check it has, is nobody's forgery, and answers
> a question wrongly?*

The interesting half is **nobody's forgery**. A record that agrees with itself and is wrong because
somebody edited it is the authenticity candidate, and the answer there has to come from outside the
record. This asks whether the same state is reachable **by accident** — by a process that stopped, by
two writers interleaving, by an admission landing in an order nobody intended — in which case there is
nothing outside the record to appeal to, because nobody lied.

---

## It already happened once, and was measured

Experiment 01, Observation 6, and it is the strongest material this candidate has. Two repositories
were tampered in one place each and both produced a world that every check accepted:

```text
intact     → 53e2b385…   head none   frozen 0   conflicts 1
tampered   → b5e38526…   head 7906cb…  frozen 1   conflicts 0
```

> Both produce the same world, and it is the diverged world Observation 1 measured — the overspend
> frozen, the refusal at −70 gone, the cascade carried into the advancement and the fork. Neither
> repository refuses. Neither reports anything.

And the reason, stated there in a form that is not about that subject:

```text
malformed record  →  detectable      (an address that is not there, a count past the end)
false record      →  undetectable    (an address that is there and is the wrong one)
```

A coordinate is a claim about the past and the record holds nothing to check it against. Which is the
whole candidate in one sentence, waiting for the question of whether *false* is reachable without a
hand doing the tampering.

---

## What each result contributes

**01 — the measured instance, and the boundary of internal checking.** A coordinate that is
well-formed and false is undetectable. That result filed it under *what the result does not cover*,
which is where it has sat since.

**02 — the deliberate face.** A consistent forgery is not refused: edit a repository, recompute every
derived value from what was written, and the refusal that mattered is simply gone. This is the same
state arrived at on purpose, and it is why the two candidates are neighbours.

**06 — the state that is byte-identical to a different history.** A repository that weighed twelve
candidates and one that never explored are the same bytes. That is not *false* — both readings are
true of what is there — but it is the sharpest available demonstration that the record's coherence
does not pin which history produced it.

**07 — asked, and the nearest thing found was not it.** The one partial state that reconstructs
answers for the previous repository's worlds, and every fact in it is true. The question survived.

**08 — asked again, and got closer.** One mixture of two writers' files reconstructs: *the first
writer's worlds over both writers' knowledge* — a repository **neither of them wrote**, answering for
intentions one of them never learned about. Every world in it was decided and every entry in it was
admitted, so it is still not false. It is the closest thing found in nine experiments, and what it
shows is that a state nobody wrote is reachable without anybody lying.

---

## The discriminator against authenticity

They are close enough that the protocol will have to say which it is doing.

```text
authenticity   who wrote this file?          the fact is not in the record and cannot be —
                                             the answer needs a key, or an anchor outside

veracity       does coherence imply truth?   the fact may or may not be in the record, and
                                             the first thing needed is a MEASUREMENT: does
                                             such a state exist, reachable by accident?
```

Authenticity's material is an argument that the answer is not in the record. This candidate's material
is one repository where the answer was not, and a search that two experiments started and neither
finished. If the search finds nothing, that is a result worth as much as finding something: it would
say that every state this application can reach by accident is either refused or true.

---

## What is not decided here

Whether the answer is a check, an anchor, a narrower claim about what corroboration promises, or the
finding that no such state is reachable. Naming a shape before the experiment runs is the thing the
programme's rules exist to prevent.

And one question of scope that belongs to whoever writes the protocol: *false* has not been defined
anywhere. Both protocols used the word and neither said what it would mean for a record to answer
falsely, given that a reading is derived from what is there and is therefore always true **of** what
is there. The candidate may turn out to be about the difference between *true of the record* and *true
of what happened* — and if it is, that difference is the finding.
