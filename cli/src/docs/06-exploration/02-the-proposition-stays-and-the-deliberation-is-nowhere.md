# Observation 2 — The proposition stays and the deliberation is nowhere

E2 predicted both halves and both are measured. What the numbers add is that the second half is not
a saving.

## The measurement

Twelve candidates, each admitted, forked in memory, interpreted under `FinalState`, scored, and
dropped. Measured off the files after each one:

```text
                       entries   lineage bytes   worlds   watermark
founded                     14            1305        1   2026-01-03
after candidate 1           15            1305        1   2026-01-04
after candidate 2           16            1305        1   2026-01-04
   …                         …            1305        1   2026-01-04
after candidate 12          26            1305        1   2026-01-04
```

Twelve worlds were weighed. Ten were admissible, two were refused, one was chosen. The record grew
by twelve propositions and by nothing else, and not one of the twelve world identities appears
anywhere in it.

The watermark advanced **once** for twelve candidates, which the protocol had already checked and
which is carried forward rather than found: they are all recorded at one instant, so the second
candidate moves nothing.

The floor did the refusing, and named what it found:

```text
spend 110   OutOfBounds { instance: account, level: -10.0 }
spend 120   OutOfBounds { instance: account, level: -20.0 }
```

Which is the first measurement of the division Phase 0 could only read off its own literals: ten and
two, as predicted, and the objective's answer is the candidate that spends the opening exactly,
leaving the account at `0.0`.

## Unauditable because unrecorded, and unprunable because unidentifiable

The second half is measured by an absence, and the protocol warned about exactly that:

> Arrangement A is what an application does by accident. It has no code to inspect and no record to
> read, so measuring it means measuring an absence — and an absence is the easiest thing to report as
> whatever the author expected.

So the first half is measured **positively** instead. Every commitment admission the journal holds is
read back out of the file, and their field names are compared:

```text
commitment admissions in journal.json      13   ← the opening and twelve candidates
distinct sets of field names among them     1
```

One shape across thirteen entries. Nothing in the journal says which twelve were only weighed, and
that is a statement about what the file contains rather than about what a reader failed to find. The
count is asserted alongside it, because a scan that matched nothing would otherwise report one shape
across zero entries and read as agreement.

## The lineage's zero is not a saving

The journal is the other side of the same arrangement:

```text
journal bytes   3430  ->  11089        ~638 per candidate, permanent
lineage bytes   1305  ->   1305
```

Arrangement A paid 7,659 bytes to weigh twelve candidates and bought nothing that can be read. The
zero in the second row is not the deliberation being cheap; it is the deliberation not existing.

## The baseline E3 will be read against

One number here belongs to E3 rather than E2, and it is available now:

```text
one decision's record        1305 bytes
its witness alone             982 bytes        75%
entries witnessed              14
```

Three quarters of the lineage file is the sequence witness, at fourteen entries and with nothing
about exploration recorded at all. E3 predicts the witness is the dominant term under arrangement B.
Phase 1 says it is already the dominant term at one decision — so what Phase 3 measures is how the
term grows, not whether it leads.
