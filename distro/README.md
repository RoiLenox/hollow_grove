# Hollow Grove OS distribution foundation

Hollow Grove OS is an Arch-based operating system. Hueman is its separately
versioned, bundled companion, intended for multiplayer collaboration through
shared spaces and cooperative work. The current client supports the local
Hollow Grove Core seam; remote collaboration remains a planned capability.

The distribution trust, machine-role and promotion model is defined by
[Distribution Architecture](ARCHITECTURE.md). Repository channels live in
`channels.toml`; hardware-neutral product policy remains in
`profiles/desktop.toml`; machine declarations under `machines/` contain no
addresses, credentials or personal configuration.

## Ownership

| Component | Owner |
| --- | --- |
| Desktop defaults, keyboard language, native Niri helper | `hollow-grove-general` |
| Complete desktop dependency set and product profile | `hollow-grove-desktop` |
| Authoritative application/world service and local integration launcher | `hollow-grove-core` |
| Presentation, controls and companion application entry | `hueman` |
| Display/GPU policy, device names, personal accounts and files | Individual machine/user |

The desktop dependency list is generated from the existing
`applications.toml` required-base inventory plus `profiles/desktop.toml`.
Walker and Elephant need project-maintained, signed packages before a stock
Arch image can resolve the entire profile. Installed third-party repositories
are not silently treated as distribution dependencies.

## Build source packages

From the repository root:

```sh
python3 tools/prepare-arch-packages.py --output dist/arch-packages
# For local review of uncommitted work only:
python3 tools/prepare-arch-packages.py --allow-dirty --output dist/arch-snapshot
```

The tool writes one deterministic source archive, a per-file provenance
manifest, checksums, and independent `PKGBUILD` / `.SRCINFO` directories. The
archive is included in each build directory; none of the generated recipes
access a neighboring checkout. Copying one directory is sufficient to provide
its authored inputs. The `.in` files are templates, not runnable recipes.

Rust dependencies are explicitly fetched from `Cargo.lock` during `prepare()`;
build and check then use `--frozen`. The build host provides Cargo/Rust and
Godot through declared dependencies. Record these versions with validation
results; a fixed source archive alone is not proof of reproducible binaries.
For an offline rebuild, retain the fetched Cargo registry along with the source.

Run `./build.sh` inside a generated directory. The wrapper pins makepkg's
`SOURCE_DATE_EPOCH` to the source commit and always requests a clean build with
checks. Additional makepkg arguments may follow it; for example,
`./build.sh --config /absolute/path/to/distro/makepkg-x86_64.conf`. For release,
use Arch devtools in a clean chroot with the selected repository snapshot,
build the four packages, lint the result, and exercise install/upgrade/remove
in disposable machines. A signed source candidate can be prepared with
`--signed-tag TAG`; public release additionally needs package/repository/image
signatures, resolved dependency provenance and installation/recovery evidence.

For local package validation, pass the portable makepkg configuration shown
above to use an x86-64 CPU baseline with checks enabled. It deliberately does
not inherit the reference laptop's x86-64-v3 or disabled-check settings. This
does not replace the clean-chroot release gate.

No OS image, keyring, public update endpoint, or replacement `/etc/os-release`
is installed by this foundation. Those belong to the next release milestone.
The profile's release gates remain false until actual evidence exists.

## Verification

```sh
python3 tools/test_prepare_arch_packages.py
python3 tools/test_hg_distribution.py
python3 tools/test_hg_machine_audit.py
python3 tools/hg-distribution.py audit
cargo test --workspace --locked --offline
cargo test --workspace --features dev-tools --locked --offline
./verify-godot.sh
./verify-playable-lifecycle.sh
```

Run a read-only, address-free enrollment audit on a declared machine with:

```sh
python3 tools/hg-machine-audit.py --machine "$(hostnamectl --static)"
```

Pass `--output /path/to/new-report.json` to preserve immutable enrollment
evidence. The command refuses to overwrite an existing report.

Use distinct empty target directories for the feature configurations when
validating a clean build. This catches undeclared sibling-binary dependencies.
Package `check()` covers its actual shipped surface; broad workspace and
two-process lifecycle checks additionally gate the release.

## Two-machine rollout

The user has authorized migration to the main PC, `noahsarch` / `archie`,
after local testing and cleanup. Resolve the actual SSH host/user using the
local connection inventory before connecting. An older device-sync entry is
not proof that it identifies that PC.

1. Validate local packages and record their source/binary checksums.
2. Inventory the peer OS, packages, free space, source status and hardware.
3. Back up peer package/configuration manifests and Hueman data; verify the
   backup and preserve any newer/divergent peer work.
4. Stage the exact tested artifacts and verify checksums before installation.
5. Apply package-owned structure using the peer's package manager. Keep its
   graphics, display geometry and personal configuration machine-owned.
6. Test desktop/launcher/health and save continuity, retaining a working
   fallback until a subsequent login and recovery check succeed.

Never use a raw home-directory mirror or the laptop's NVIDIA configuration as
the distribution installation procedure. The SSH rollout is system deployment,
not a substitute for Hueman's future authenticated multiplayer protocol.
