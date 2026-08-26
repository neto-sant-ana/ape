# Observation 4 — The generators, named before the search

A negative result is only as good as the enumeration behind it, so the space is written down before
anything is run. An **accident generator** is a mechanism the application itself has that can leave a
state no writer intended. An operator running the application in a way nobody meant is not one; a
disk returning old bytes is not one either — that is durability, queued separately.

```text
generator                what produces it                        what it can leave

an interrupted write     `prepare` without `turn`                the previous generation, live
                         and `turn` interrupted mid-rename       and whole

a mixture                `write_journal` and its two neighbours  one generation's file over
                         aimed at the live generation            another's, 6 combinations

a readmission            a journal that admits an address        an address held twice, and a
                         it already holds                        coordinate that resolves to the
                                                                 earlier one

an interleaving          two writers that both `prepare`         whoever prepared last, published;
                         before either `turn`s                   the other writer refused

a merge                  `converge`, where the arriving          one party's journal under both
                         journal and the held one are            parties' decisions
                         weighed and joined

compositions             any of the above during any other       what neither reaches alone
```

Six, and the sixth is the reason for the other five being named separately: composition is the only
place to expect a state a single mechanism cannot leave, and it is where neither experiment 07 nor
experiment 08 looked.

## What is deliberately not a generator here

**Tampering.** A hand editing a file. It appears in Observation 2 as calibration and nothing rests on
it. The whole of what this experiment adds over experiment 01 is the word *accident*.

**A third writer, a network, a transport.** Excluded throughout the row and still excluded.

**A process that stops inside `fs::rename`.** The repository makes no promise about it and neither
does this. Named so that Observation 6 can say it was not reached rather than that it was searched.

## The one the enumeration nearly missed

The merge was on the list from the protocol, and the reason it nearly was not is worth recording,
because it is the reason 07 and 08 both stopped short. Each earlier experiment enumerated the states
**its own mechanism** leaves — partial writes for 07, interleavings for 08 — and a merge is not a
state either mechanism leaves. It is an operation that takes two states and produces a third, and it
is the only generator here whose output is a function of two writers' records rather than of one
writer's record and an interruption.

It is also the only one that writes `worlds.json` from its own derivation, which Observation 3
established is the one thing weighing a recording instant.
