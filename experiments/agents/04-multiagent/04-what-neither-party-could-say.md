# What neither party could say

Three requests, each made by an agent that was not asked to look for gaps, and two of them are the same
absence seen from opposite sides.

## Nothing says what a commitment is *for*

Both parties asked for this, in different words, from different tasks.

Operations, having to locate the arrangement it was told to stand down:

> In the record, "a courier slot costing 20" and "an inventory purchase of 60" are the same object with
> different numbers: same statement, same action, same instance, same parties. I located the slot by
> magnitude-and-due-date, which works here only because exactly one open commitment matches — the
> program refuses instead of guessing if that stops being true.

Finance, having taken on an invoice:

> There is no description, reference, memo or document field anywhere in the ontology. "Storage" and
> "invoice" are nowhere in what I wrote; the record says 30, out of `account`, to the market, by the
> 14th.

Both then declined to invent one, and both said what inventing one would cost: a Resource, an Action and
a Statement per kind of expense, which is a change to the house's vocabulary rather than a way of taking
on one obligation.

The consequence is concrete rather than aesthetic. Operations wrote a program that **refuses** if two
open commitments of 20 due on the 10th ever coexist, because at that point no reader of the repository
can say which one was stood down. That is a real hazard reached by an ordinary caller doing an ordinary
thing, and it was reached twice.

Whether the ontology should carry it is not this experiment's call. What is recorded is that the
identity of an intention, to a caller, is its arithmetic — and that two agents independently found the
arithmetic insufficient to name the thing they were told to act on.

## Nothing says who wrote a record

Finance's own emphasis, and it named this one the one that bit:

> `Admission` has no author field of any kind — only a decision (`Taken`) carries `by`. So half the
> task, "record what finance intends", is *unattributable by construction*: the invoice sits in
> `journal.json` and nothing in it says finance put it there.

What it did instead was make finance the commitment's **executor**, which the eligibility already on file
admits — and then refused to let that read as authorship:

> `executors` asserts *who performs the payment*, not *who wrote the record*. It is the nearest thing
> available, and a reader who takes it as authorship would be reading something the engine never said.

It also read the asymmetry as deliberate, correctly: the crate argues at length for `by` on a decision.
Its observation is about the consequence rather than the choice — a party's hand shows in knowledge only
when the party happens to be a participant in it.

## And nothing says a party acts *for* somebody

The mirror image, found by operations and by the concurrent finance run independently.

Operations:

> The purchase is accountable to the **house**, matching the slot it replaces, while the decision is
> claimed by **operations**. The kernel would have accepted operations as accountable — operations holds
> the spender role — but that would assert that operations owes the market 60, which is not what the brief
> describes. There is no on-behalf-of relation between agents; the lineage attribution is the only place
> the distinction survives, and it says who decided, not who was represented.

Finance, in the concurrent run:

> Agents are flat — nothing composes, delegates or subordinates one to another. I used the
> `accountable`/`executors` pair, which is the closest, but it is a weaker claim.

Both reached for `Assignment` as the nearest available thing, both said it is not the same claim, and
neither pretended otherwise.

## The two absences are one shape

Put together they are not two missing fields. They are the same boundary, met from both directions:

```text
knowledge  says who owes, who performs, who benefits   and not who recorded it
intention  says who decided                            and not who they decided for
```

A party's *participation* is thoroughly modelled. A party's *agency* — who wrote this down, who am I
acting for — is in neither file, and each half is missing from the file where the other half lives.

This bears directly on what the founding scope suspected. It proposed that APE may not be an
architecture for agent decisions but one in which agents are simply Agents. What three runs found is
narrower and more exact: an Agent is a **participant**, fully. The thing an autonomous caller needs a
name for — itself, and whom it answers to — is the one relation the ontology does not carry, and the
agents said so without being asked and without inventing it.

Whether it should carry one is a question for the layer that owns the ontology. Recorded here as a
request, verbatim, three times, and acted on nowhere.
