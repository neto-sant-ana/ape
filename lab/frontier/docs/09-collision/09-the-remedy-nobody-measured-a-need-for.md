# Observation 9 — Part B was not built, and the criterion is why

Phase 7. The criterion three experiments have now inherited:

> **It removes a state a reader can be misled by, and what the repair replaces survives.** A repair that
> only removes states already refused by name has removed nothing a reader needed.

Every state this experiment found was weighed against it, and none of them meets the first half.

```text
state                                        is a reader misled?
────────────────────────────────────────────────────────────────────────────────────────
a refused meeting                            no — refused by name, with the coordinate

a refusal between two repositories that      no — the caller is told; and the agreement is
agree about a world                          separately askable, read-only (Obs 7)

a merged repository recording one world      no — `worlds.json` holds the two records
twice                                        BYTE-IDENTICAL, identity included, and the
                                             archive holds one. The duplicate is in plain
                                             view rather than silent

a merged repository that does not say it     YES — and it is the authenticity family, named
met another                                  in Request 5 and excluded by the protocol
```

So the repair is not built, and that is the reportable result the protocol pre-registered.

## Which is a finding about the criterion, not only about this experiment

Nine experiments have applied that criterion and this is the first time it has come up empty — and the
reason is not that nothing is missing. Five needs are recorded in
[`08`](08-what-an-application-would-need.md), and every one of them is real.

They are all about what a **writer** can do.

```text
coordination    a decision was lost and nothing said so           a reader was misled
atomicity       a partial write reconstructed as a legitimate one  a reader was misled
contention      a turn published a superseded state cleanly        a reader was misled
collision       two repositories cannot be put together           a caller is refused, by name
```

The criterion selects for repairs that protect whoever reads. This experiment's gaps are about
capability: the record refuses correctly, says so precisely, and answers what two repositories share
when asked. Nothing about it deceives anybody — it merely cannot do a thing.

> **A missing capability and a misleading state are not the same defect, and a criterion built for the
> second will decline to fix the first.** That is the criterion working, and it is also the honest
> reason this experiment ends without a remedy.

## What the criterion would have accepted, and it belongs to something else

One state in the table does mislead a reader: a merged repository is indistinguishable from a repository
whose author wrote every line of it. The subject gains another record's lines and cannot say where they
came from; the party is untouched and cannot tell it was read.

That is the authenticity candidate's family, arriving with a new member — and the protocol excluded it
before the run, in writing, for exactly this reason: *a second repository is the first thing in nine
experiments that could be somebody else's.* Building the remedy here would have been answering that
candidate under another name.

## And the second condition never had to be applied

This protocol added one:

> A repair may not give one repository authority over another.

No candidate repair proposed one, and the severe failure condition it guards against — *if two
repositories can only meet by one being declared the real one, this record is centralized* — **was not
met**. Observation 2 is why: the common history is derived from content by both sides independently, so
there is no asymmetry for an authority to occupy. Whatever a meeting turns out to need, it does not need
one of them to be the real one.

That is the most valuable thing this experiment could have found, and it found it by not needing it.
