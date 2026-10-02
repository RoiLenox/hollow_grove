#!/usr/bin/env python3
"""Audit Hollow Grove distribution policy and assemble fail-closed repositories."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import tomllib


SCHEMA_VERSION = 1
PACKAGE_SUFFIXES = (".pkg.tar.zst", ".pkg.tar.xz", ".pkg.tar.gz")
FINGERPRINT = re.compile(r"[0-9A-F]{40,64}")


class DistributionError(RuntimeError):
    """A distribution policy or artifact validation failed."""


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def read_toml(path: Path) -> dict:
    try:
        return tomllib.loads(path.read_text())
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise DistributionError(f"cannot read {path}: {error}") from error


def load_policy(root: Path) -> dict:
    policy = read_toml(root / "distro/channels.toml")
    if policy.get("schema_version") != SCHEMA_VERSION:
        raise DistributionError("unsupported channel-policy schema")
    if not re.fullmatch(r"[a-z0-9][a-z0-9_.-]*", policy.get("repository_name", "")):
        raise DistributionError("invalid repository name")
    if policy.get("target_architecture") != "x86_64":
        raise DistributionError("only the declared x86_64 foundation is supported")
    if not policy.get("package_signatures_required"):
        raise DistributionError("package signatures must remain required")
    if not policy.get("database_signatures_required"):
        raise DistributionError("repository database signatures must remain required")
    required = policy.get("packages", {}).get("required")
    if not isinstance(required, list) or not required or len(required) != len(set(required)):
        raise DistributionError("required package set must be a nonempty unique list")
    for name in required:
        if not isinstance(name, str) or not re.fullmatch(r"[a-z0-9][a-z0-9@+_.-]*", name):
            raise DistributionError(f"invalid required package name: {name!r}")
    channels = policy.get("channels", {})
    if set(channels) != {"testing", "stable"}:
        raise DistributionError("channel policy must define exactly testing and stable")
    sections = set()
    for name, channel in channels.items():
        section = channel.get("repository_section", "")
        if not re.fullmatch(r"[a-z0-9][a-z0-9_.-]*", section):
            raise DistributionError(f"invalid repository section for {name}")
        if section in sections:
            raise DistributionError("repository sections must be unique")
        sections.add(section)
        if not isinstance(channel.get("allow_unsigned_development"), bool):
            raise DistributionError(f"channel {name} lacks an unsigned-development policy")
    if channels["stable"]["allow_unsigned_development"]:
        raise DistributionError("stable may never allow unsigned development artifacts")
    if not policy.get("build_once_promote_unchanged"):
        raise DistributionError("package promotion must preserve built artifact bytes")
    return policy


def load_machine_profiles(root: Path) -> list[dict]:
    paths = sorted((root / "distro/machines").glob("*.toml"))
    if not paths:
        raise DistributionError("no machine profiles are declared")
    profiles = []
    seen_ids = set()
    for path in paths:
        profile = read_toml(path)
        if profile.get("schema_version") != SCHEMA_VERSION:
            raise DistributionError(f"unsupported machine-profile schema: {path}")
        machine_id = profile.get("machine_id", "")
        if not re.fullmatch(r"[a-z0-9][a-z0-9-]*", machine_id):
            raise DistributionError(f"invalid machine identity: {path}")
        if machine_id in seen_ids:
            raise DistributionError(f"duplicate machine identity: {machine_id}")
        seen_ids.add(machine_id)
        if profile.get("architecture") != "x86_64":
            raise DistributionError(f"unsupported machine architecture: {machine_id}")
        if profile.get("update_channel") not in {"testing", "stable"}:
            raise DistributionError(f"invalid update channel: {machine_id}")
        if profile.get("personal_configuration") != "machine-owned":
            raise DistributionError(f"personal configuration must remain machine-owned: {machine_id}")
        if profile.get("network_addresses_recorded") is not False:
            raise DistributionError(f"machine profile must not record network addresses: {machine_id}")
        profiles.append(profile)
    if not any(profile.get("build_authority") for profile in profiles):
        raise DistributionError("no build authority is declared")
    if any(profile.get("signing_authority") for profile in profiles):
        raise DistributionError("machine profiles must not silently claim signing authority")
    return profiles


def trusted_fingerprints(root: Path) -> set[str]:
    path = root / "distro/keys/trusted-fingerprints.txt"
    if not path.exists():
        return set()
    fingerprints = set()
    for number, raw in enumerate(path.read_text().splitlines(), 1):
        line = raw.strip().upper()
        if not line or line.startswith("#"):
            continue
        if not FINGERPRINT.fullmatch(line):
            raise DistributionError(f"invalid trusted fingerprint at {path}:{number}")
        fingerprints.add(line)
    return fingerprints


def package_paths(directory: Path) -> list[Path]:
    if not directory.is_dir():
        raise DistributionError(f"package directory does not exist: {directory}")
    packages = [
        path
        for path in sorted(directory.iterdir())
        if path.is_file() and path.name.endswith(PACKAGE_SUFFIXES)
    ]
    if not packages:
        raise DistributionError(f"no package artifacts found in {directory}")
    return packages


def archive_text(package: Path, member: str) -> str:
    try:
        return subprocess.check_output(
            ["bsdtar", "-xOf", str(package), member], stderr=subprocess.STDOUT
        ).decode()
    except (OSError, UnicodeDecodeError, subprocess.CalledProcessError) as error:
        raise DistributionError(f"cannot read {member} from {package.name}: {error}") from error


def archive_members(archive: Path) -> list[str]:
    try:
        return subprocess.check_output(
            ["bsdtar", "-tf", str(archive)], stderr=subprocess.STDOUT
        ).decode().splitlines()
    except (OSError, UnicodeDecodeError, subprocess.CalledProcessError) as error:
        raise DistributionError(f"cannot list {archive.name}: {error}") from error


def package_metadata(package: Path) -> dict[str, str]:
    fields = {}
    for line in archive_text(package, ".PKGINFO").splitlines():
        key, separator, value = line.partition(" = ")
        if separator and key in {"pkgname", "pkgver", "arch"}:
            fields[key] = value
    if set(fields) != {"pkgname", "pkgver", "arch"}:
        raise DistributionError(f"incomplete .PKGINFO in {package.name}")
    return fields


def source_manifest(package: Path) -> tuple[dict, str]:
    matches = [name for name in archive_members(package) if name.endswith("/SOURCE-MANIFEST.json")]
    if len(matches) != 1:
        raise DistributionError(f"{package.name} must contain exactly one SOURCE-MANIFEST.json")
    text = archive_text(package, matches[0])
    try:
        manifest = json.loads(text)
    except json.JSONDecodeError as error:
        raise DistributionError(f"invalid source manifest in {package.name}: {error}") from error
    return manifest, hashlib.sha256(text.encode()).hexdigest()


def verify_signature(package: Path, signature: Path, allowed: set[str]) -> str:
    if not signature.is_file():
        raise DistributionError(f"missing detached signature: {signature.name}")
    try:
        result = subprocess.run(
            ["gpg", "--batch", "--status-fd", "1", "--verify", str(signature), str(package)],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise DistributionError(f"signature verification failed for {package.name}") from error
    fingerprints = []
    for line in result.stdout.splitlines():
        parts = line.split()
        if len(parts) >= 3 and parts[:2] == ["[GNUPG:]", "VALIDSIG"]:
            fingerprints.append(parts[2].upper())
    if len(fingerprints) != 1 or fingerprints[0] not in allowed:
        raise DistributionError(f"{package.name} is not signed by an explicitly trusted key")
    return fingerprints[0]


def common_source_identity(records: list[dict]) -> dict:
    identities = {
        (
            record["source_manifest"].get("git_commit"),
            record["source_manifest"].get("dirty_worktree"),
            record["source_manifest"].get("release_status"),
            record["source_manifest"].get("verified_tag"),
            record["source_manifest_sha256"],
        )
        for record in records
    }
    if len(identities) != 1:
        raise DistributionError("packages do not carry one identical source identity")
    commit, dirty, status, verified_tag, manifest_sha = identities.pop()
    if not isinstance(commit, str) or not re.fullmatch(r"[0-9a-f]{40,64}", commit):
        raise DistributionError("package source manifest has an invalid Git identity")
    if not isinstance(dirty, bool) or not isinstance(status, str):
        raise DistributionError("package source manifest has invalid release state")
    return {
        "git_commit": commit,
        "dirty_worktree": dirty,
        "release_status": status,
        "verified_tag": verified_tag,
        "manifest_sha256": manifest_sha,
    }


def regular_artifacts(directory: Path) -> dict[str, str]:
    return {
        path.name: sha256(path)
        for path in sorted(directory.iterdir())
        if path.is_file() and not path.is_symlink() and path.name not in {"RELEASE.json", "SHA256SUMS"}
    }


def symlink_artifacts(directory: Path) -> dict[str, str]:
    return {
        path.name: os.readlink(path)
        for path in sorted(directory.iterdir())
        if path.is_symlink()
    }


def repository_database_records(database: Path) -> dict[str, dict[str, str]]:
    records = {}
    desc_members = [name for name in archive_members(database) if name.endswith("/desc")]
    for member in desc_members:
        lines = archive_text(database, member).splitlines()
        fields = {}
        for index, line in enumerate(lines[:-1]):
            if line in {"%NAME%", "%VERSION%", "%FILENAME%", "%SHA256SUM%"}:
                fields[line.strip("%").lower()] = lines[index + 1]
        required = {"name", "version", "filename", "sha256sum"}
        if set(fields) != required:
            raise DistributionError(f"incomplete repository record: {member}")
        if fields["name"] in records:
            raise DistributionError(f"duplicate repository database record: {fields['name']}")
        records[fields["name"]] = fields
    if not records:
        raise DistributionError(f"repository database has no package records: {database.name}")
    return records


def assemble_repository(
    root: Path,
    packages_dir: Path,
    output: Path,
    channel_name: str,
    unsigned_development: bool,
    signing_key: str | None,
) -> Path:
    policy = load_policy(root)
    channel = policy["channels"].get(channel_name)
    if channel is None:
        raise DistributionError(f"unknown channel: {channel_name}")
    if unsigned_development and not channel["allow_unsigned_development"]:
        raise DistributionError(f"{channel_name} forbids unsigned development repositories")
    if unsigned_development and signing_key:
        raise DistributionError("unsigned development mode cannot select a signing key")
    allowed = trusted_fingerprints(root)
    if not unsigned_development:
        if not signing_key:
            raise DistributionError("signed repository assembly requires --signing-key")
        signing_key = signing_key.upper()
        if signing_key not in allowed:
            raise DistributionError("selected signing key is not an explicit trusted fingerprint")

    packages_dir = packages_dir.resolve()
    output = output.resolve()
    if output.exists():
        raise DistributionError(f"output already exists: {output}")
    if output == packages_dir or packages_dir in output.parents:
        raise DistributionError("repository output cannot be inside the package input directory")

    records = []
    seen = set()
    paths = package_paths(packages_dir)
    for package in paths:
        metadata = package_metadata(package)
        name = metadata["pkgname"]
        if name in seen:
            raise DistributionError(f"duplicate package name: {name}")
        seen.add(name)
        if metadata["arch"] not in {"any", policy["target_architecture"]}:
            raise DistributionError(f"unsupported package architecture: {package.name}")
        manifest, manifest_sha = source_manifest(package)
        record = {
            "name": name,
            "version": metadata["pkgver"],
            "architecture": metadata["arch"],
            "filename": package.name,
            "sha256": sha256(package),
            "source_manifest": manifest,
            "source_manifest_sha256": manifest_sha,
        }
        signature = package.with_name(package.name + ".sig")
        if unsigned_development:
            if signature.exists():
                raise DistributionError(
                    f"unsigned development input contains an unverified signature: {signature.name}"
                )
            record["signature"] = None
        else:
            record["signature"] = {
                "filename": signature.name,
                "sha256": sha256(signature),
                "fingerprint": verify_signature(package, signature, allowed),
            }
        records.append(record)

    required = set(policy["packages"]["required"])
    if seen != required:
        missing = sorted(required - seen)
        extra = sorted(seen - required)
        raise DistributionError(f"package set mismatch; missing={missing}, extra={extra}")
    source = common_source_identity(records)
    if not unsigned_development:
        if source["dirty_worktree"] or source["release_status"] != "signed-source":
            raise DistributionError("signed repositories require a clean signed-source package set")
        if not isinstance(source["verified_tag"], str) or not source["verified_tag"]:
            raise DistributionError("signed repositories require a verified source tag")

    output.parent.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix=f".{output.name}.", dir=output.parent))
    try:
        copied = []
        for package, record in zip(paths, records, strict=True):
            destination = stage / package.name
            shutil.copy2(package, destination)
            copied.append(destination)
            if record["signature"]:
                shutil.copy2(package.with_name(record["signature"]["filename"]), stage)

        database = stage / f"{channel['repository_section']}.db.tar.zst"
        command = ["repo-add", "--nocolor", "--include-sigs"]
        if not unsigned_development:
            command.extend(["--sign", "--key", signing_key])
        command.append(str(database))
        command.extend(str(path) for path in copied)
        subprocess.run(command, check=True)

        if not unsigned_development:
            database_signature = database.with_name(database.name + ".sig")
            verify_signature(database, database_signature, allowed)

        release = {
            "schema_version": SCHEMA_VERSION,
            "repository": policy["repository_name"],
            "repository_section": channel["repository_section"],
            "channel": channel_name,
            "mode": "development-unsigned" if unsigned_development else "signed",
            "target_architecture": policy["target_architecture"],
            "build_once_promote_unchanged": policy["build_once_promote_unchanged"],
            "source": source,
            "packages": [
                {key: value for key, value in record.items() if key != "source_manifest"}
                for record in sorted(records, key=lambda item: item["name"])
            ],
            "artifacts": regular_artifacts(stage),
            "symlinks": symlink_artifacts(stage),
            "publication_ready": False,
        }
        (stage / "RELEASE.json").write_text(json.dumps(release, indent=2, sort_keys=True) + "\n")
        (stage / "SHA256SUMS").write_text(
            "".join(f"{digest}  {name}\n" for name, digest in sorted(release["artifacts"].items()))
        )
        stage.replace(output)
    except Exception:
        shutil.rmtree(stage, ignore_errors=True)
        raise
    return output


def verify_repository(root: Path, repository: Path) -> dict:
    policy = load_policy(root)
    repository = repository.resolve()
    release_path = repository / "RELEASE.json"
    try:
        release = json.loads(release_path.read_text())
    except (OSError, json.JSONDecodeError) as error:
        raise DistributionError(f"cannot read repository release manifest: {error}") from error
    if release.get("schema_version") != SCHEMA_VERSION:
        raise DistributionError("unsupported repository release schema")
    if release.get("repository") != policy["repository_name"]:
        raise DistributionError("repository identity does not match policy")
    if release.get("target_architecture") not in {None, policy["target_architecture"]}:
        raise DistributionError("repository architecture does not match policy")
    if release.get("build_once_promote_unchanged") not in {
        None,
        policy["build_once_promote_unchanged"],
    }:
        raise DistributionError("repository promotion policy does not match")
    channel_name = release.get("channel")
    channel = policy["channels"].get(channel_name)
    if channel is None or release.get("repository_section") != channel["repository_section"]:
        raise DistributionError("repository channel identity does not match policy")
    mode = release.get("mode")
    if mode == "development-unsigned":
        if not channel["allow_unsigned_development"]:
            raise DistributionError("unsigned repository violates its channel policy")
    elif mode != "signed":
        raise DistributionError("unknown repository trust mode")
    if release.get("publication_ready") is not False:
        raise DistributionError("foundation repositories must not claim publication readiness")
    packages = release.get("packages", [])
    if not isinstance(packages, list):
        raise DistributionError("repository package inventory is not a list")
    package_names = {item.get("name") for item in packages if isinstance(item, dict)}
    if package_names != set(policy["packages"]["required"]):
        raise DistributionError("repository manifest does not contain the required package set")
    artifacts = release.get("artifacts")
    if not isinstance(artifacts, dict) or not artifacts:
        raise DistributionError("repository manifest has no artifact inventory")
    for name, expected in artifacts.items():
        if Path(name).name != name or not re.fullmatch(r"[0-9a-f]{64}", expected):
            raise DistributionError(f"unsafe artifact record: {name!r}")
        path = repository / name
        if not path.is_file() or path.is_symlink() or sha256(path) != expected:
            raise DistributionError(f"repository artifact failed verification: {name}")
    actual_artifacts = regular_artifacts(repository)
    if actual_artifacts != artifacts:
        raise DistributionError("repository contains unrecorded or missing regular artifacts")
    symlinks = release.get("symlinks", {})
    if not isinstance(symlinks, dict):
        raise DistributionError("repository symlink inventory is not an object")
    for name, target in symlinks.items():
        if Path(name).name != name or Path(target).name != target:
            raise DistributionError(f"unsafe symlink record: {name!r}")
        path = repository / name
        if not path.is_symlink() or os.readlink(path) != target:
            raise DistributionError(f"repository symlink failed verification: {name}")
    if symlink_artifacts(repository) != symlinks:
        raise DistributionError("repository contains unrecorded or missing symlinks")
    sums = (repository / "SHA256SUMS").read_text()
    expected_sums = "".join(f"{digest}  {name}\n" for name, digest in sorted(artifacts.items()))
    if sums != expected_sums:
        raise DistributionError("SHA256SUMS does not match RELEASE.json")

    observed_packages = []
    for package in packages:
        filename = package.get("filename", "")
        if Path(filename).name != filename or not filename.endswith(PACKAGE_SUFFIXES):
            raise DistributionError("package record has an unsafe filename")
        path = repository / filename
        if filename not in artifacts or package.get("sha256") != artifacts[filename]:
            raise DistributionError(f"package digest record drifted for {filename}")
        metadata = package_metadata(path)
        expected_metadata = {
            "pkgname": package.get("name"),
            "pkgver": package.get("version"),
            "arch": package.get("architecture"),
        }
        if metadata != expected_metadata:
            raise DistributionError(f"package metadata drifted for {filename}")
        manifest, manifest_sha = source_manifest(path)
        if package.get("source_manifest_sha256") != manifest_sha:
            raise DistributionError(f"source manifest digest drifted for {filename}")
        observed_packages.append(
            {"source_manifest": manifest, "source_manifest_sha256": manifest_sha}
        )

    observed_source = common_source_identity(observed_packages)
    recorded_source = release.get("source")
    if not isinstance(recorded_source, dict):
        raise DistributionError("repository has no recorded source identity")
    source_keys = {"git_commit", "dirty_worktree", "release_status", "manifest_sha256"}
    if any(recorded_source.get(key) != observed_source[key] for key in source_keys):
        raise DistributionError("repository source identity drifted from its packages")

    database = repository / f"{channel['repository_section']}.db.tar.zst"
    if database.name not in artifacts:
        raise DistributionError("repository database is absent from the artifact inventory")
    database_records = repository_database_records(database)
    expected_database_records = {
        item["name"]: {
            "name": item["name"],
            "version": item["version"],
            "filename": item["filename"],
            "sha256sum": item["sha256"],
        }
        for item in packages
    }
    if database_records != expected_database_records:
        raise DistributionError("repository database drifted from its package inventory")

    if mode == "signed":
        if (
            observed_source["dirty_worktree"]
            or observed_source["release_status"] != "signed-source"
            or not isinstance(observed_source["verified_tag"], str)
            or not observed_source["verified_tag"]
            or recorded_source.get("verified_tag") != observed_source["verified_tag"]
        ):
            raise DistributionError("signed repository does not carry a verified clean source tag")
        allowed = trusted_fingerprints(root)
        if not allowed:
            raise DistributionError("signed repository has no configured trust root")
        for package in packages:
            filename = package.get("filename", "")
            signature = package.get("signature")
            if Path(filename).name != filename or not isinstance(signature, dict):
                raise DistributionError("signed package record is incomplete")
            signature_name = signature.get("filename", "")
            if Path(signature_name).name != signature_name:
                raise DistributionError("signed package record has an unsafe signature path")
            fingerprint = verify_signature(
                repository / filename, repository / signature_name, allowed
            )
            if fingerprint != signature.get("fingerprint"):
                raise DistributionError(f"signature identity drifted for {filename}")
        verify_signature(database, database.with_name(database.name + ".sig"), allowed)
    return release


def audit(root: Path) -> dict:
    policy = load_policy(root)
    profiles = load_machine_profiles(root)
    fingerprints = trusted_fingerprints(root)
    try:
        status = subprocess.check_output(
            ["git", "-C", str(root), "status", "--porcelain=v1", "--untracked-files=normal"]
        ).decode()
        commit = subprocess.check_output(["git", "-C", str(root), "rev-parse", "HEAD"]).decode().strip()
    except (OSError, UnicodeDecodeError, subprocess.CalledProcessError) as error:
        raise DistributionError(f"cannot inspect source repository: {error}") from error
    return {
        "schema_version": SCHEMA_VERSION,
        "repository": policy["repository_name"],
        "channels": sorted(policy["channels"]),
        "machines": sorted(profile["machine_id"] for profile in profiles),
        "trusted_signing_fingerprints": sorted(fingerprints),
        "source_commit": commit,
        "source_clean": not bool(status.strip()),
        "signed_release_ready": bool(fingerprints),
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("audit", help="validate distribution policy and machine roles")

    assemble = subparsers.add_parser("assemble", help="assemble a new package repository")
    assemble.add_argument("--packages", type=Path, required=True)
    assemble.add_argument("--output", type=Path, required=True)
    assemble.add_argument("--channel", choices=("testing", "stable"), required=True)
    assemble.add_argument("--unsigned-development", action="store_true")
    assemble.add_argument("--signing-key")

    verify = subparsers.add_parser("verify", help="verify a repository manifest and artifacts")
    verify.add_argument("--repository", type=Path, required=True)

    args = parser.parse_args()
    root = args.root.resolve()
    try:
        if args.command == "audit":
            result = audit(root)
        elif args.command == "assemble":
            result = {
                "repository": str(
                    assemble_repository(
                        root,
                        args.packages,
                        args.output,
                        args.channel,
                        args.unsigned_development,
                        args.signing_key,
                    )
                )
            }
        else:
            result = verify_repository(root, args.repository)
    except (DistributionError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"distribution operation failed: {error}\n")
    print(json.dumps(result, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
