---
kind: decision (advance)
world: 558f991d1bd39c3d3578cf226f14940920d6e919c37d2ea4bcf670f68b066e96
arms: [[arm-finance]], [[arm-merged]]
extends: [[decision-74a6a53e]]
by: 108077234acde7911af42ac2a820bcdb2e770e9c923608f9454a2eec6c71970b
about: []
claims: 8
---

# decision-558f991d

`558f991d1bd39c3d3578cf226f14940920d6e919c37d2ea4bcf670f68b066e96`

A **advance**, held by: finance, merged.

- produces world: `558f991d1bd39c3d3578cf226f14940920d6e919c37d2ea4bcf670f68b066e96`
- known at: 2026-01-08
- event head: Some("6d336a99e589182e6450905e02070dfbaa4febcdce6134829b166e570b1a7096")
- frozen: ["3167ccd390cf517c6ddd17ef553772d970784ed8f53d1754392386f604714f4c"]
- open: ["2f54506ad11b1279c35b86b82a84df617d7d7fc740a92b5294a590b48a309ec2"]
- taken by: `108077234acde7911af42ac2a820bcdb2e770e9c923608f9454a2eec6c71970b` (finance)
- taken after entry: `652a011d545fddb2f76257420dcfac55c0db58e433dd3f887923ddcc4e1d1ebe`
- witnessed entries: 20
- extends: `74a6a53e4e4d559ff967697523bb44138973421c1d17e718694b8b9eb4fa0905`

## What was said about it

- *(exposition)* An `EntryId` is derived from what admitting produced, so finance's entry is the same knowledge in either record.
- *(qualification)* Had either side's extra entry been an Event, finance's cut would have resolved differently and its two worlds would have been lost by the retake.
- *(roadnottaken)* Reconciling them would take two decisions neither party made — operations advancing its own tip to finance's instant, and then somebody answering what becomes of `2f54506a…`, which operations omitted and finance kept. That answer is not derivable from either record, so I left it open.
- *(loss)* **I did not preserve finance's original witnesses.** This is the one claim from finance's record that the merged record does not carry, and it is a real loss rather than a technicality.
- *(qualification)* The witnesses now on those two records are true claims about the merged record's history; they are not finance's claims about its own.
- *(want)* The record has no way to distinguish *finance took this against its own prefix* from *operations retook finance's intention against the merged prefix*
- *(roadnottaken)* I chose the attribution that keeps the provenance of the intention, and the cost is that the record slightly overstates what finance witnessed.
- *(methodlimit)* I tried to make it refuse by editing a copy of finance's record, but the crate catches a tampered record first — `reading::corroborated` refuses it before my guard is reached, which is the crate working correctly and my guard still unexercised.
