---
kind: decision (fork)
world: b939289591aaf9360606e06f1889314e4c7a5ff4d6762508325b801df73a08cb
arms: [[arm-operations]], [[arm-merged]]
extends: [[decision-1f093bfa]]
taken by: 326993e96a20964d9b4317764638aea39d48c5ee990495221aecc4f437e0f8a3
about: ["2f54506ad11b1279c35b86b82a84df617d7d7fc740a92b5294a590b48a309ec2", "4b8b9b8890365fc386a67977ccd312256ad1ece7ba0b5d1e00451379756609cd"]
claims: 3
---

# decision-b9392895

`b939289591aaf9360606e06f1889314e4c7a5ff4d6762508325b801df73a08cb`

A **fork**, held by: operations, merged.

- produces world: `b939289591aaf9360606e06f1889314e4c7a5ff4d6762508325b801df73a08cb`
- known at: 2026-01-07
- event head: Some("6d336a99e589182e6450905e02070dfbaa4febcdce6134829b166e570b1a7096")
- frozen: ["3167ccd390cf517c6ddd17ef553772d970784ed8f53d1754392386f604714f4c"]
- open: ["4b8b9b8890365fc386a67977ccd312256ad1ece7ba0b5d1e00451379756609cd"]
- taken by: `326993e96a20964d9b4317764638aea39d48c5ee990495221aecc4f437e0f8a3` (operations)
- taken after entry: `4b8b9b8890365fc386a67977ccd312256ad1ece7ba0b5d1e00451379756609cd`
- witnessed entries: 20
- extends: `1f093bfa4767c65232f55b6f98c534bac595d44715fd8a76ab8439c62c8ffbd3`
- omitted: ["2f54506ad11b1279c35b86b82a84df617d7d7fc740a92b5294a590b48a309ec2"]
- introduced: ["4b8b9b8890365fc386a67977ccd312256ad1ece7ba0b5d1e00451379756609cd"]

## What was said about it

- *(methodlimit)* I measured the other order too — the Canon rejects it outright […] Running the whole program with the roles swapped fails at exactly that point, which means operations was the only side of this pair that could host the merge at all.
- *(roadnottaken)* Reconciling them would take two decisions neither party made — operations advancing its own tip to finance's instant, and then somebody answering what becomes of `2f54506a…`, which operations omitted and finance kept. That answer is not derivable from either record, so I left it open.
- *(want)* The record has no way to distinguish *finance took this against its own prefix* from *operations retook finance's intention against the merged prefix*
