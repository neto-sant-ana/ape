# Observation 4 — What crosses by identity

**S4 confirmed, both halves.** Closed over all nine families.

```text
                      into a record whose      into one whose chain
                      chain left the base's    never left it
  role                     crosses                  crosses
  agent                    crosses                  crosses
  eligibility              crosses                  crosses
  resource                 crosses                  crosses
  resource-instance        crosses                  crosses
  action                   crosses                  crosses
  statement                crosses                  crosses
  commitment               crosses                  crosses
  event                    DOES NOT                 crosses
```

*Crosses* means the address the entry had there is the address admitting it produces here. Eight
families keep it because their identities contain no chain; the Event does not, because an `EventId`
contains its predecessor and the two chains are `E → Eh` and `E → Eg`. Admitted here, their Event is
another assertion settling the same commitment.

**The right-hand column is what makes the table say anything.** The same entry, the same receiver in
every other respect, and one difference: this record never admitted an Event of its own, so its chain
is still a prefix of theirs and the address survives. The Event row is about the **chain**, not about
Events.

Seen red by taking the receiving record's own Event away, at which point the left column reports
`("event", true)` and names the row.

## What it means and what it does not

**A merge of two sets and a merge of two chains are different operations.** A chain is an ordering that
is part of identity, so two chains that diverged cannot exchange members and stay themselves. That is
what a chain *is*, and reading it as something missing would be reading a definition as a defect.

It also bounds this experiment's answer without contradicting it. A record can take another's Events —
it just cannot take them **as the same entries**. The commitment they settle crosses by identity, the
observation and the instant of occurrence cross as content, and what does not cross is the position in
a line that only one of the two records was standing in.

Which is the same sentence experiment 14 arrived at from the other side, and the two are worth reading
together: an address must stay a function of the entity it names, and for an Event the entity includes
where in the chain it is.

## The commitment family arrives twice, and the row holds

`there` committed twice — one settled, one open — so the table has ten entries and nine rows. The phase
asserts that a family arriving twice crosses both times or neither, because a row reporting whichever
of the two the loop saw last would be a row about the loop.
