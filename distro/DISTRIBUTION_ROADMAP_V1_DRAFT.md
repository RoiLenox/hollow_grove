# Hollow Grove OS distribution roadmap

Status: proposal for discussion; implementation and release gates remain pending.
Prepared: 2026-10-12.
Inspection baseline: `os/package-foundation`, commit `d51f506d3a4d9f00381d5e4d44471b316f7a5c1c`, including the existing uncommitted desktop work.

The recommended destination is an identifiable, installable and maintained
Arch-derived Hollow Grove OS: its own desktop experience, signed packages,
update channels, installation media, recovery procedure and release ownership.
The proposed sequence is a private release for the two declared machines,
followed by a public alpha. This is a planning assumption pending the owner's
preference, not a decision to publish.

Arch supplies the base system and package manager. Hollow Grove owns its
integration, defaults, selected applications, distribution packages, release
validation and user experience. The existing Niri desktop, keyboard language,
constitutional palette and Hueman companion provide the product identity.

**What the current checkout already provides**

| Area | Evidence and remaining gap |
| --- | --- |
| Product architecture | `ARCHITECTURE.md` establishes an Arch-derived OS, four package layers and separate development/update/administration/application responsibilities. |
| Package inputs | `../packaging/` defines General, Desktop, Core and Hueman. `../tools/prepare-arch-packages.py` produces deterministic source bundles and provenance. Clean binary builds and independent rebuild results still need release evidence. |
| Desktop | Niri, Waybar, Walker, keyboard discovery, diagnostics and hardware-neutral defaults exist. Complete clean-install and everyday desktop testing remains a release gate. |
| Repositories | `../tools/hg-distribution.py` implements `audit`, `assemble` and `verify`. There is no promotion command or publication workflow tied to canary evidence. |
| Trust | The read-only audit reports `signed_release_ready: false`, no trusted signing fingerprints and a dirty source tree. Its readiness field currently measures key presence alone, not all release conditions. |
| CI | `../.github/workflows/foundation.yml` checks Rust, policy, source preparation and recipe syntax. Its Arch job does not build/install the complete package set. |
| Devices | Policy declares `noahsarch` as forge/stable and `occamsrazer` as canary. These are recorded roles, not a fresh audit of either installed machine. |
| Installation | The foundation explicitly supplies no OS image, keyring, update endpoint or replacement OS identity. |
| Licensing | `../LICENSE` currently reserves project rights, including use and distribution, absent separate permission. Public release terms need an explicit decision. |

The policy audit was run for this plan. No package build, machine enrollment,
installation, recovery rehearsal or remote inspection was performed. Historical
build reports are useful references, but do not establish readiness of this
checkout and its current dependency set.

**1. Define the first supported product**

Adopt one x86_64 desktop edition, using the existing Niri shell and bundled
Core/Hueman local functionality. Use an upstream Arch kernel as the initial
installation baseline, with a tested LTS recovery option. Record laptop and
desktop hardware support from actual tests; preserve machine-specific graphics
and display configuration as overrides.

Use Arch's rolling base with testing/stable channels for Hollow Grove packages.
Identify installer snapshots separately from application versions. A coherent
full system update is the supported path: Arch explicitly does not support
partial upgrades. Record the complete upstream package set used for build and
validation; revalidate or rebuild when upstream ABI changes invalidate that
evidence. Holding a few base libraries indefinitely would not create a stable
distribution. A fully curated frozen base would require a separate maintenance
commitment. [Arch maintenance guidance](https://wiki.archlinux.org/title/System_maintenance#Partial_upgrades_are_unsupported)

Separate OS release gates from future authenticated Hueman multiplayer. Require
the shipped local companion to launch, save, reload and preserve data across
updates. Track remote collaboration in an application capability gate so the OS
can release with an accurate description of its current functionality.

Completion: one release-policy document defines architecture, edition, kernel,
supported installation modes, versioning, update model, hardware scope and the
first-release acceptance matrix. Existing desktop changes are reviewed and
captured deliberately before selecting the release source commit.

**2. Make the complete desktop build and install from declared inputs**

Extend CI or a dedicated forge runner to build every shipped package in clean
Arch chroots. Record toolchain versions, upstream repository state, source
hashes, `.BUILDINFO`, logs, package lint results and binary hashes. Compare an
independent rebuild and investigate differences before claiming reproducibility.
Arch devtools provides clean-chroot and rebuild tools for this work.
[Arch devtools](https://man.archlinux.org/man/devtools.7.en)

Resolve the complete desktop dependency graph against the selected Arch
snapshot. The profile currently flags Walker and Elephant as external-package
blockers: verify their availability at release time, and maintain pinned,
reviewed project builds for dependencies that remain outside the selected
official repositories. Users should receive those binaries through the signed
repository, without a first-boot dependency on an AUR helper.

Generalize repository provenance before adding those packages. The current
assembler requires exactly the four configured packages and one identical
Hollow Grove source manifest. Preserve that consistency for packages built
from the shared source while allowing independently versioned upstream
packages to carry their own verified source identities. Bind the complete
selection in a release manifest. Package generation also currently derives
all versions from the root Cargo package; implement independent component
versioning when the first component actually needs its own release cadence.

Completion: a fresh disposable Arch system installs the entire desktop from
declared repositories, with no sibling checkout, personal package cache or
undeclared third-party repository. Installation, upgrade and removal respect
file ownership and retain user data.

**3. Establish Hollow Grove identity and update trust**

Add the already planned `hollow-grove-keyring`, `hollow-grove-mirrorlist`,
`hollow-grove-release` and `hollow-grove-base` packages. Assign ownership for
branding, release metadata, base dependencies and repository configuration.
Implement Hollow Grove OS identification through a tested package ownership
strategy, including `ID_LIKE=arch`; resolve ownership of `os-release` with
Arch's `filesystem` package before shipping. The upstream specification defines
the identity fields and `/etc` versus `/usr/lib` precedence.
[os-release specification](https://man.archlinux.org/man/os-release.5.en)

Carry the existing visual identity through boot menus, login/session selection,
the desktop, an About view, installation instructions and a small first-run
guide. Explain keyboard navigation, networking, application choices, updates
and recovery in ordinary user language.

Prepare the keyring, fingerprint verification instructions and rotation/revocation
procedure. The owner performs the deliberate signing-key ceremony required by
the existing architecture; private signing material stays outside source and
build artifacts. Require package and repository database signatures for routine
enrollment. `repo-add` supports signed repository databases.
[Repository signing](https://man.archlinux.org/man/repo-add.8.en)

Serve immutable release directories from a backed-up repository endpoint.
Implement evidence-gated testing-to-stable promotion: verify the full candidate,
bind validation records to exact package hashes, preserve package bytes,
authenticate release metadata and publish the channel change atomically.
Repository databases can differ by channel; promoted package bytes must match.
Retain previous releases and test key rotation and rejected signatures.

Completion: a fresh test installation can establish trust, identify itself as
Hollow Grove OS, consume testing, and receive the verified candidate through
stable. Missing or mismatched evidence blocks promotion.

**4. Prove upgrades and recovery on installed systems**

Make pacman the installed OS update path. Document the existing
`hollow-grove-update.sh` as development/state transfer tooling and give OS
updates an unambiguous entry point with visible progress and failure guidance.
Review package-owned defaults and pacman backup handling; `/etc/skel` alone
does not propagate new defaults to existing users. Use versioned defaults and
explicit migrations that preserve overrides, monitor settings and Hueman saves.

Run VM tests first, then enroll the canary using freshly collected machine
inventory and a verified backup. Follow the existing machine-role architecture
only after the current installations and recovery paths have been checked.
Promote the tested package bytes to the stable machine after acceptance.

Rehearse failed downloads, interrupted updates, an unusable graphical session,
a previous-kernel boot and a save/configuration migration failure. For Btrfs,
restore a consistent package database, root filesystem and matching boot
artifacts; account for EFI/boot files outside snapshots. For ext4, test the
documented backup restoration path. Verify an off-device restoration as well.

Completion: two successive release candidates upgrade through the same process
on both machines, preserve user state, and have recorded recovery results.
Updating and recovering must work without the development checkout.

**5. Produce and validate installation media**

Create `distro/archiso/` using the tested packages and release manifest. Archiso
supports custom profiles and generates installation images; the ISO should
consume the same signed package artifacts used by installed machines.
[Archiso profiles](https://github.com/archlinux/archiso/blob/master/docs/README.profile.rst)

Start with a version-pinned Archinstall integration and a documented guided
installation flow. Maintain the selected version's configuration schema and
test it in VMs. Archinstall already provides guided installation and supports
configuration-driven use. [Archinstall upstream](https://github.com/archlinux/archinstall)

The flow covers networking, locale, keyboard, accounts, disk selection,
filesystem, encryption, bootloader and the Hollow Grove session. Show the disk
operation summary before destructive installation. Recommend a new-install
Btrfs layout with a tested recovery procedure; define encrypted and unencrypted
support explicitly. Retain the architecture's UEFI and legacy boot test gates
unless support policy is deliberately revised. Secure Boot support needs its
own implementation and test evidence.

Test live boot, installation, reboot without the medium, initial account setup,
full update and recovery. Exercise login/logout, portals and screen sharing,
lock/suspend/resume, audio, networking, Bluetooth where supported, brightness,
external monitors and keyboard/help behavior. Verify the image and installed
system contain no developer identity, credentials or personal state.

Completion: another person can install onto a blank VM from the published
instructions, reach a usable desktop, update it and follow the recovery guide.
Publish image checksums, detached signatures and the exact package manifest
when the corresponding release is authorized.

**6. Operate a public alpha as a maintained product**

Choose explicit public-use and distribution terms for Hollow Grove-authored
software and assets, keeping component boundaries visible. Inventory shipped
licenses, notices and source obligations; provide the corresponding source
required by each applicable license alongside the release process. GPL binary
distribution has specific source provision requirements.
[GNU licensing FAQ](https://www.gnu.org/licenses/gpl-faq.en.html#RedistributedBinariesGetSource)

Prepare a stable download location, release notes, known issues, hardware
requirements, installation/recovery documentation, bug reporting and a security
contact. Assign ownership for package maintenance, upstream security monitoring,
emergency releases, repository availability, signing-key renewal and backups.
Define a realistic support policy and update cadence before inviting users.

Completion: an external tester completes the installation/update/recovery path;
the maintainer can release a fix through the signed channel; the supported
scope, release terms and operational responsibilities are published. Community
listings and broader promotion can follow this working alpha.

**First implementation batch**

1. Add `distro/RELEASE_POLICY.md` and a release-evidence schema; split OS and
   Hueman capability gates without declaring any current gate passed.
2. Reconcile the desktop dependency inventory and current uncommitted wrappers;
   define upstream provenance for every external dependency.
3. Extend package preparation/repository validation for the full package set,
   and add a clean-chroot build runner with preserved evidence.
4. Add VM installation/upgrade smoke checks, beginning with the existing four
   packages and then the complete desktop dependency set.
5. Prepare the new identity/keyring/mirrorlist/base recipes and promotion
   interface for review; activate signing and endpoints in the later trust step.

The first milestone is a fresh VM running the complete packaged desktop from
declared inputs, with recorded build and install evidence. The critical path is
that result, trusted updates, verified recovery, then the installer alpha.
Schedule estimates should follow the first clean build and VM cycle, where the
actual packaging and hardware work becomes measurable.
