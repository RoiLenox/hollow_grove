# Hollow Grove distribution keys

Only public distribution keys and revocation material belong here. Private
keys, passphrases, agent sockets, exported secret packets and recovery secrets
must never enter the repository, source archives, build hosts or ISO profile.

The initial key ceremony must choose an offline or hardware-backed primary
identity, a replaceable package-signing subkey, an expiry/rotation policy and
an independently stored revocation certificate. Record the verified full
fingerprint in `trusted-fingerprints.txt` only after that ceremony.

`trusted-fingerprints.txt` is absent until a real trust root exists. Repository
tooling treats that absence as a hard failure for signed and stable assembly.
