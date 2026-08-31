---
kind: vocabulary
---

# Vocabulary

The entries that introduce names.

## role `3d359fe8`

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

## role `48845bf3`

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

## resource `9da39b54`

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

## eligibility `c3f30e2a`

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

## eligibility `6f7276c6`

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

## resource-instance `8098643f`

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

## action `f08e6708`

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

## action `5f5255e4`

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

## statement `dabb76d3`

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

## statement `1cb6093b`

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

## eligibility `23375b37`

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

## eligibility `8a9be0af`

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

