# Cross-House clinical authorization policy 2

Stonebend and Sandmanor's governance `Manifest` commands previously accepted
an active Prima Donna officeholder as the independent clearance signer. New
commands require a separately certified clinical authorization. Civic office,
Form mastery, election administration and Court review confer no clinical
credential authority.

## Explicit transition and evidence boundary

The receiving House records `UpdateClinicalAuthorization` in its existing
governance journal. The shared implementation is
`src/world/glaushouse/clinical_authorization.rs`. It reuses Glaushouse governance
`Attestation`, the existing House identity, stable identity validation and the
shared version-1 event archive. It does not change Glaushouse's governance rules
or serialize its private clinical registry.

1. `AdoptPolicy` supplies version `2`, a fresh authenticated mandate signed by
   the receiving governance policy's authorizer, and explicitly appointed
   clinical credential authorities. There are no default authorities. The
   original election mandate cannot be reused. Adoption cannot be replaced,
   repeated as a new event or downgraded.
2. `Register` records an external clinical authorization, independently
   certified by an appointed clinical credential authority. The certifier must
   differ from both subject and reviewer. The authorization binds the reviewer
   (clearance signer), subject, exact House/Form manifestation procedure,
   validity interval, separate clearance and separate personal consent. All
   four record references must be distinct, attached and unused. The trusted
   adapter must establish that the credential, clearance and consent each cover
   this scope and the entire recorded interval; it must not expand their scope
   or round their validity outward.
3. `Manifest` retains its existing command encoding and Form/reversion gates.
   Its clearance and consent must exactly match a registered, unrevoked decision
   for this subject and procedure, valid at the command's in-world year.
4. `Revoke` disables that authorization permanently. A clinical credential
   authority can revoke the credential, its reviewer can withdraw clearance,
   and its subject can withdraw consent. A fresh authenticated proof is needed.
   A replacement authorization requires new records; an old record cannot be
   overwritten or resurrected.

Validity is `[valid_from, valid_until)` in the receiving adapters' existing
in-world year ordinals. No real-time, day-to-year or clinical-clock conversion
is invented. Finer-grained decisions need an explicit time adapter before use.
Revocation prevents subsequent procedures; it does not rewrite completed
manifestations. Automatic ordinary reversion and mastery remain governed by
their existing rules, without an additional clinical check.

These are authenticated external **decision references**, not newly issued
medical credentials. As with existing governance attestations, the caller must
authenticate actors and evidence before invoking the adapter, and promptly
record revocation/withdrawal before admitting subsequent commands. Signer
strings and the archive's FNV checksum are not digital signatures. A forged
claim cannot substitute for a registered decision, but this module cannot
authenticate an entirely forged input archive or an unauthenticated caller.

No constitutional, Agency, Factory, enterprise, private-chart, Hueman identity
or Threefold Memory permissions are added. No clinical gate is added to other
transformation APIs, mastery recognition, or safe reversion.

## Historical compatibility

Only archive decoding can use the original office-based clinical rule before
the explicit policy-2 event. This preserves accepted historical manifestations
and byte-for-byte archive round trips. Live `execute` refuses new manifestations
before adoption with `RequiresLegalParameter("clinical authorization policy v2")`.
Retrying an already accepted identical event remains a non-mutating idempotent
operation. After adoption, replay and live commands both use policy 2.

The archive schema, aggregate IDs, rule-set strings, initial snapshot and
existing events are unchanged. There is no data rewrite or inferred credential
for a historical officeholder. Consumers running older code will not understand
the new clinical update action; readers must be upgraded before appending it.

`tests/fixtures/clinical_authorization/{stonebend,sandmanor}-v1.json` were
captured with the pre-change workspace library and simulated governance test
fixtures. They include accepted office-only manifestations. Tests verify exact
re-encoding, preserved event prefixes and state, explicit transition without
invented credentials, and refusal to append new office-only manifestations.
