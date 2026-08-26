# Observation 3 — What it cost, in the two places the protocol looked

## N4 — two records stop sharing a journal, and go on sharing a world

```text
                                        addresses in common
                                        content   composite
  here and later                           13         0
  (every entry learned a day apart)

  here and late                            13        12
  (one instant differs)
```

**Both halves of N4 confirmed.** The first row is the extreme and the second says the change is
proportional rather than categorical: two records lose exactly the entries they disagree about, and
two that disagree about everything lose everything.

And the half that survives, measured the same way: `here` and `later` decide **one world by identity**
under both shapes, with nothing copied between them. A `ThesisId` is derived from a cut and a
selection, and neither carries a recording instant — so nothing this change touches reaches it.

**N5 confirmed, as the reason.** `converge::holds` is the only operation a second repository is
answerable to and it takes an identity; everything else that crosses between two records crosses as an
address. So the surviving agreement is the one nothing calls, and the one that dies is the one
`converge` is built on.

It is worth being exact about what dies, because *`converge` refuses them either way* is true and is
not the point. Today two records founded apart hold a common prefix that can be **intersected** — and
that intersection is the raw material of the item five results have named, a partial meeting between
two records that share a base. Under an address that says *when*, two records a day apart have nothing
to intersect. The cost lands on the capability the queue holds, not on an operation that exists.

## The blast radius, counted

Predicted before the build and measured after.

```text
compilation      25 call sites, in five concluded experiments' suites
                   witness 14, exploration 5, corroboration 4, convergence 1,
                   commensurability 1

at runtime       all 7 tests in lab/agents/tests/merge.rs, against four committed
                   repositories, each refused with UnknownEntry naming a bare address

untouched        every test the application has of its own
```

The runtime half is what `core/src/kernel/entities/identification.rs` already says happens when what
decides an identity changes: an older repository does not disagree, it **stops replaying**. Written
about the kernel, and it turns out to describe the application's own address exactly.

Those four repositories are a concluded agents-row experiment's published artefacts — its result is
what they are evidence for — and `lab/README.md` says a concluded experiment keeps its own. So the
second condition on keeping the change fires here, before anything about the trade is weighed:

> A cost that lands on a published result is not a cost this experiment may pay.

## And a note on where compilation costs land

Twenty-five sites in five suites is a number that invites the wrong reading. Test suites are not the
product, and the application compiled untouched — so on the face of it this is the laboratory paying
for its own experiment.

The next observation is why that reading is wrong, and it is the finding none of the five predictions
had.
