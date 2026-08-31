---
kind: vocabulary
---

# Vocabulary

Every entry any arm of this record admitted.

## role `3d359fe8` — in operations, finance, merged

`3d359fe8c7ff74044b22d66f9a5bf29cf848739401b16456a69ccf64829d6f4f`

A role.

```json
{
  "admits": "role",
  "label": "spender",
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## role `48845bf3` — in operations, finance, merged

`48845bf3932457367beba3b252731695cbef505e4c5b6bce1acf708444564626`

A role.

```json
{
  "admits": "role",
  "label": "counterparty",
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## agent `fe0e80f6` — in operations, finance, merged

`fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562`

An Agent, labelled **house**.
- recorded at: 2026-01-01

## agent `0d3a24e8` — in operations, finance, merged

`0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace`

An Agent, labelled **market**.
- recorded at: 2026-01-01

## resource `9da39b54` — in operations, finance, merged

`9da39b547d3ee483bdbbd98aee6e1508639cb3f4c37c3511c43dc23843fa949a`

A resource.

```json
{
  "admits": "resource",
  "label": "cash",
  "kind": {
    "between": {
      "lower": "0",
      "upper": "1000"
    }
  },
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## eligibility `c3f30e2a` — in operations, finance, merged

`c3f30e2ad08cfc63e35ea0450be520dea705fb93b6130bd9527ac23512e97ca9`

A eligibility.

```json
{
  "admits": "eligibility",
  "agent": "fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562",
  "roles": [
    "3d359fe8c7ff74044b22d66f9a5bf29cf848739401b16456a69ccf64829d6f4f"
  ],
  "effective_from": "2026-01-01",
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## eligibility `6f7276c6` — in operations, finance, merged

`6f7276c6013b0d533edcc0868ef65748efd55f437e9bc928061f78ce83e38c8c`

A eligibility.

```json
{
  "admits": "eligibility",
  "agent": "0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace",
  "roles": [
    "48845bf3932457367beba3b252731695cbef505e4c5b6bce1acf708444564626"
  ],
  "effective_from": "2026-01-01",
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## resource-instance `8098643f` — in operations, finance, merged

`8098643f890f456c60ef73ea3ef46c9e7bbf9715d52ca29437907427a5174100`

A resource-instance.

```json
{
  "admits": "resource-instance",
  "label": "account",
  "resource": "9da39b547d3ee483bdbbd98aee6e1508639cb3f4c37c3511c43dc23843fa949a",
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## action `f08e6708` — in operations, finance, merged

`f08e670889cebb746387802e39f869ca2f76c9c29d9a61a83a3d74fa364ba859`

A action.

```json
{
  "admits": "action",
  "verb": "receive",
  "kind": {
    "quantifiable": "increase"
  },
  "resource": "9da39b547d3ee483bdbbd98aee6e1508639cb3f4c37c3511c43dc23843fa949a",
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## action `5f5255e4` — in operations, finance, merged

`5f5255e453e4eaff04dc1546aa57c86a036a1e6daad4df76b2508639237e6600`

A action.

```json
{
  "admits": "action",
  "verb": "spend",
  "kind": {
    "quantifiable": "decrease"
  },
  "resource": "9da39b547d3ee483bdbbd98aee6e1508639cb3f4c37c3511c43dc23843fa949a",
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## statement `dabb76d3` — in operations, finance, merged

`dabb76d32d2548e014a176f70c72f2bdf0d2ace3e5dcd65220a3ed07e2231666`

A statement.

```json
{
  "admits": "statement",
  "actors": [
    "48845bf3932457367beba3b252731695cbef505e4c5b6bce1acf708444564626"
  ],
  "recipients": [
    "3d359fe8c7ff74044b22d66f9a5bf29cf848739401b16456a69ccf64829d6f4f"
  ],
  "action": "f08e670889cebb746387802e39f869ca2f76c9c29d9a61a83a3d74fa364ba859",
  "fulfills": [
    "Settled"
  ],
  "cancels": [
    "Cancelled"
  ],
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## statement `1cb6093b` — in operations, finance, merged

`1cb6093bb3776703e8e19199a07367dabf4a349793cdf7027ea7cdb5e5c25fe3`

A statement.

```json
{
  "admits": "statement",
  "actors": [
    "3d359fe8c7ff74044b22d66f9a5bf29cf848739401b16456a69ccf64829d6f4f"
  ],
  "recipients": [
    "48845bf3932457367beba3b252731695cbef505e4c5b6bce1acf708444564626"
  ],
  "action": "5f5255e453e4eaff04dc1546aa57c86a036a1e6daad4df76b2508639237e6600",
  "fulfills": [
    "Settled"
  ],
  "cancels": [
    "Cancelled"
  ],
  "recorded_at": "2026-01-01"
}
```
- recorded at: 2026-01-01

## commitment `3167ccd3` — in operations, finance, merged

`3167ccd390cf517c6ddd17ef553772d970784ed8f53d1754392386f604714f4c`

A Commitment.

- accountable: `0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace`
- executors: ["0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace"]
- beneficiaries: ["fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562"]
- statement: `dabb76d32d2548e014a176f70c72f2bdf0d2ace3e5dcd65220a3ed07e2231666`
- resource instance: `8098643f890f456c60ef73ea3ef46c9e7bbf9715d52ca29437907427a5174100`
- committed at: 2026-01-01, due 2026-01-02
- magnitude: 100
- dependencies: []
- recorded at: 2026-01-01

## event `6d336a99` — in operations, finance, merged

`6d336a99e589182e6450905e02070dfbaa4febcdce6134829b166e570b1a7096`

An Event.

- settles: `3167ccd390cf517c6ddd17ef553772d970784ed8f53d1754392386f604714f4c`
- observation: Settled
- occurred at: 2026-01-02
- recorded at: 2026-01-02

## agent `326993e9` — in operations, finance, merged

`326993e96a20964d9b4317764638aea39d48c5ee990495221aecc4f437e0f8a3`

An Agent, labelled **operations**.
- recorded at: 2026-01-03

## agent `10807723` — in operations, finance, merged

`108077234acde7911af42ac2a820bcdb2e770e9c923608f9454a2eec6c71970b`

An Agent, labelled **finance**.
- recorded at: 2026-01-03

## eligibility `23375b37` — in operations, finance, merged

`23375b3765fd8e21a5f638f5bd0b4e47853763fc8d070d87cd3b1cd449a5bb67`

A eligibility.

```json
{
  "admits": "eligibility",
  "agent": "326993e96a20964d9b4317764638aea39d48c5ee990495221aecc4f437e0f8a3",
  "roles": [
    "3d359fe8c7ff74044b22d66f9a5bf29cf848739401b16456a69ccf64829d6f4f"
  ],
  "effective_from": "2026-01-03",
  "recorded_at": "2026-01-03"
}
```
- recorded at: 2026-01-03

## eligibility `8a9be0af` — in operations, finance, merged

`8a9be0af9c5f0016ce00305211f29d191b2ad03ea9ff0d8493f7e145f6cde742`

A eligibility.

```json
{
  "admits": "eligibility",
  "agent": "108077234acde7911af42ac2a820bcdb2e770e9c923608f9454a2eec6c71970b",
  "roles": [
    "3d359fe8c7ff74044b22d66f9a5bf29cf848739401b16456a69ccf64829d6f4f"
  ],
  "effective_from": "2026-01-03",
  "recorded_at": "2026-01-03"
}
```
- recorded at: 2026-01-03

## commitment `2f54506a` — in operations, finance, merged

`2f54506ad11b1279c35b86b82a84df617d7d7fc740a92b5294a590b48a309ec2`

A Commitment.

- accountable: `fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562`
- executors: ["fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562"]
- beneficiaries: ["0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace"]
- statement: `1cb6093bb3776703e8e19199a07367dabf4a349793cdf7027ea7cdb5e5c25fe3`
- resource instance: `8098643f890f456c60ef73ea3ef46c9e7bbf9715d52ca29437907427a5174100`
- committed at: 2026-01-05, due 2026-01-10
- magnitude: 20
- dependencies: []
- recorded at: 2026-01-05

## commitment `4b8b9b88` — in operations, merged

`4b8b9b8890365fc386a67977ccd312256ad1ece7ba0b5d1e00451379756609cd`

A Commitment.

- accountable: `fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562`
- executors: ["fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562"]
- beneficiaries: ["0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace"]
- statement: `1cb6093bb3776703e8e19199a07367dabf4a349793cdf7027ea7cdb5e5c25fe3`
- resource instance: `8098643f890f456c60ef73ea3ef46c9e7bbf9715d52ca29437907427a5174100`
- committed at: 2026-01-07, due 2026-01-20
- magnitude: 60
- dependencies: []
- recorded at: 2026-01-07

## commitment `652a011d` — in finance, merged

`652a011d545fddb2f76257420dcfac55c0db58e433dd3f887923ddcc4e1d1ebe`

A Commitment.

- accountable: `0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace`
- executors: ["0d3a24e8704bd9f36112a65e0e7cb15fe596d542dc5212ba5b8efeeb66c24ace"]
- beneficiaries: ["fe0e80f6960e5f133afe9f98b8857fad8636865ad320e8baaa099de8285dd562"]
- statement: `dabb76d32d2548e014a176f70c72f2bdf0d2ace3e5dcd65220a3ed07e2231666`
- resource instance: `8098643f890f456c60ef73ea3ef46c9e7bbf9715d52ca29437907427a5174100`
- committed at: 2026-01-08, due 2026-01-12
- magnitude: 40
- dependencies: []
- recorded at: 2026-01-08

