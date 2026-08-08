# Observation 2 — An adapter's contract coverage is the suite's coverage

The application needs a canonical history of its own before it can persist one, and the
engine ships conformance suites so that an adapter can prove it honors the port. This is
the first adapter written from outside the engine, and therefore the first time that proof
is exercised the way an application exercises it.

The adapter was written with a deliberate deviation, to observe what the suite would say
about it. The recording watermark advanced before the event head was checked, so an append
refused for a stale head still moved `recorded_through` — a write that never happened
leaving the mark that it had.

The suite passed.

Both refusal cases assert on the head, on the commitment index and on the by-id read.
Neither asserts on `recorded_through`, and the events each case constructs share a single
recording instant, so the watermark has no distinct value that could be observed moving.
The gap is the same in the single-threaded and the contended halves.

The property itself is not in doubt. It is stated, and where it is stated is the finding:

```text
port documentation   → describes what a refusal leaves untouched, without the watermark
conformance suite    → does not assert it
reference adapter    → states it, in a private method behind an optional feature
```

An adapter author reads the port and runs the suite. The one place the property is written
is a private method of the implementation an application is told not to build upon. This
experiment reached it only by reading the engine's source directly, which is not a step
the boundary asks of an application.

Nor can the application close the gap itself. Exercising the property needs a
`Canonical<Event>` carrying a stale head and a later instant, and constructing a canonical
record is not an operation the public API offers. That is the boundary of Observation 1
met from the other side: there it prevented hydrating a record while reading, here it
prevents an adapter from proving what the suite leaves untested.

> *Through the engine's current public boundary, an adapter's contract coverage is exactly
> the suite's coverage.*

The reach is narrow, and the narrowness is circumstantial rather than structural.
`Canon::admit_*` reads the head immediately before appending, so a single-process
application does not produce a stale head at all; contending callers ordinarily share a
recording instant, at which a losing racer moves the watermark to the value the winner
sets anyway. Neither condition is enforced. `recorded_at` is supplied by the caller, and
monotonic recording is what the watermark exists to require rather than something it may
assume.

---

## Consequences to carry

* An application implementing a port cannot read a passing suite as a proven contract, and
  has no public means of proving the remainder itself.
* Where a port's promise is documented only inside the reference implementation, an
  adapter written against the public boundary cannot be expected to honor it.
* This is pressure on an engine interface rather than on the repository model, and the
  correction belongs to the engine rather than to this experiment.

The deviation has been corrected in the adapter, against the structure of the reference
implementation rather than against a guess.
