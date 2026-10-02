# Hollow Grove package repository

Repository channels are defined in `distro/channels.toml`. Assemble a private
development repository from an existing package directory with:

```sh
python3 tools/hg-distribution.py assemble \
  --channel testing \
  --packages /path/to/packages \
  --output /path/to/new/testing-repository \
  --unsigned-development
python3 tools/hg-distribution.py verify \
  --repository /path/to/new/testing-repository
```

The output directory must not exist. The command inventories packages with
pacman, requires the complete configured package set, creates a repository
database with `repo-add`, and records every regular artifact in `RELEASE.json`
and `SHA256SUMS`.

Unsigned mode is deliberately limited to the private testing channel. Stable
assembly requires detached package signatures from a trusted fingerprint and
signs the repository database with the same explicit key. A signing key is
never discovered implicitly or copied by this tool.

Promotion will consume verified testing artifacts and preserve their package
bytes. Do not publish or install the development-unsigned repository as a
trusted system source.
