#!/usr/bin/env python3
"""Produce a read-only, address-free Hollow Grove machine enrollment audit."""

from __future__ import annotations

import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import tomllib


SCHEMA_VERSION = 1


class AuditError(RuntimeError):
    """The audit could not safely identify or inspect its target."""


def command(*args: str, env: dict[str, str] | None = None) -> dict:
    try:
        result = subprocess.run(
            args,
            check=False,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=30,
            env=env,
        )
        return {"exit_code": result.returncode, "output": result.stdout.strip()}
    except (OSError, subprocess.TimeoutExpired) as error:
        return {"exit_code": 127, "output": str(error)}


def load_profile(root: Path, machine_id: str) -> dict:
    if not re.fullmatch(r"[a-z0-9][a-z0-9-]*", machine_id):
        raise AuditError("unsafe machine identity")
    path = root / "distro/machines" / f"{machine_id}.toml"
    try:
        profile = tomllib.loads(path.read_text())
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise AuditError(f"cannot read machine profile {path}: {error}") from error
    if profile.get("schema_version") != SCHEMA_VERSION:
        raise AuditError("unsupported machine-profile schema")
    if profile.get("machine_id") != machine_id:
        raise AuditError("machine profile identity does not match its filename")
    return profile


def check(status: str, summary: str, evidence=None) -> dict:
    result = {"status": status, "summary": summary}
    if evidence not in (None, "", [], {}):
        result["evidence"] = evidence
    return result


def hostname() -> str:
    detected = command("hostnamectl", "--static")
    return detected["output"].splitlines()[0] if detected["exit_code"] == 0 else platform.node()


def systemd_state(*args: str) -> str:
    result = command("systemctl", *args)
    return result["output"].splitlines()[0] if result["output"] else "unavailable"


def installed_packages(names: list[str]) -> tuple[dict[str, str], list[str]]:
    versions = {}
    missing = []
    for name in names:
        result = command("pacman", "-Q", name)
        fields = result["output"].split()
        if result["exit_code"] == 0 and len(fields) >= 2 and fields[0] == name:
            versions[name] = fields[1]
        else:
            missing.append(name)
    return versions, missing


def niri_doctor() -> dict:
    service = {
        "enabled": systemd_state("--user", "is-enabled", "hollow-niri.service"),
        "active": systemd_state("--user", "is-active", "hollow-niri.service"),
    }
    environment = command("systemctl", "--user", "show-environment")
    niri_socket = None
    if environment["exit_code"] == 0:
        for line in environment["output"].splitlines():
            if line.startswith("NIRI_SOCKET="):
                niri_socket = line.partition("=")[2]
                break
    doctor_env = os.environ.copy()
    if niri_socket:
        doctor_env["NIRI_SOCKET"] = niri_socket
    doctor = command("/usr/lib/hollow-grove/bin/hollow-niri", "doctor", env=doctor_env)
    service["doctor_exit_code"] = doctor["exit_code"]
    service["doctor_summary"] = doctor["output"].splitlines()[:8]
    service["session_socket_available"] = bool(niri_socket)
    return service


def firewall_services() -> dict[str, str]:
    return {
        unit: systemd_state("is-active", unit)
        for unit in ("firewalld.service", "nftables.service", "ufw.service")
    }


def ssh_listener_scope() -> dict:
    result = command("ss", "-H", "-ltn")
    port_22 = []
    if result["exit_code"] == 0:
        for line in result["output"].splitlines():
            fields = line.split()
            if len(fields) >= 4 and fields[3].rsplit(":", 1)[-1] == "22":
                host = fields[3].rsplit(":", 1)[0].strip("[]")
                port_22.append("wildcard" if host in {"0.0.0.0", "::", "*"} else "scoped")
    return {
        "listening": bool(port_22),
        "wildcard": "wildcard" in port_22,
        "socket_count": len(port_22),
    }


def pacman_database_markers() -> list[str]:
    directory = Path("/var/lib/pacman/local")
    try:
        return sorted(
            entry.name
            for entry in directory.iterdir()
            if entry.is_file() and not entry.is_symlink() and entry.name != "ALPM_DB_VERSION"
        )
    except OSError as error:
        return [f"audit-unavailable: {error}"]


def repository_state(root: Path) -> dict:
    commit = command("git", "-C", str(root), "rev-parse", "HEAD")
    status = command(
        "git", "-C", str(root), "status", "--porcelain=v1", "--untracked-files=normal"
    )
    return {
        "commit": commit["output"] if commit["exit_code"] == 0 else None,
        "clean": status["exit_code"] == 0 and not status["output"],
    }


def audit(root: Path, profile: dict) -> dict:
    checks = {}
    detected_host = hostname()
    checks["identity"] = check(
        "pass" if detected_host == profile["machine_id"] else "blocker",
        "detected hostname matches the declared machine identity"
        if detected_host == profile["machine_id"]
        else "detected hostname does not match the requested profile",
        detected_host,
    )

    architecture = platform.machine()
    checks["architecture"] = check(
        "pass" if architecture == profile["architecture"] else "blocker",
        "architecture matches profile"
        if architecture == profile["architecture"]
        else "architecture does not match profile",
        architecture,
    )

    root_fs_result = command("findmnt", "-n", "-o", "FSTYPE", "/")
    root_fs = root_fs_result["output"].splitlines()[0] if root_fs_result["output"] else "unknown"
    checks["root_filesystem"] = check(
        "pass" if root_fs == profile["root_filesystem"] else "blocker",
        "root filesystem matches profile"
        if root_fs == profile["root_filesystem"]
        else "root filesystem does not match profile",
        root_fs,
    )

    required_packages = [
        "hollow-grove-core",
        "hollow-grove-desktop",
        "hollow-grove-general",
        "hueman",
    ]
    versions, missing = installed_packages(required_packages)
    checks["packages"] = check(
        "pass" if not missing else "blocker",
        "all distribution packages are installed"
        if not missing
        else "required distribution packages are missing",
        {"versions": versions, "missing": missing},
    )
    integrity = command("pacman", "-Qkk", *required_packages) if not missing else None
    altered = []
    if integrity:
        altered = [line for line in integrity["output"].splitlines() if " altered file" in line and "0 altered" not in line]
    checks["package_integrity"] = check(
        "pass" if integrity and integrity["exit_code"] == 0 and not altered else "blocker",
        "installed distribution package files pass pacman integrity checks"
        if integrity and integrity["exit_code"] == 0 and not altered
        else "installed package integrity could not be established",
        altered,
    )

    source = repository_state(root)
    checks["source_checkout"] = check(
        "pass" if source["commit"] and source["clean"] else "attention",
        "source checkout is clean" if source["clean"] else "source checkout has local changes",
        source,
    )

    service = niri_doctor()
    service_ok = (
        service["enabled"] == "enabled"
        and service["active"] == "active"
        and service["doctor_exit_code"] == 0
    )
    checks["hollow_niri"] = check(
        "pass" if service_ok else "attention",
        "Hollow Niri is enabled, active and healthy"
        if service_ok
        else "Hollow Niri needs an active graphical-session health check",
        service,
    )

    firewalls = firewall_services()
    firewall_active = any(state == "active" for state in firewalls.values())
    checks["host_firewall"] = check(
        "pass" if firewall_active else "attention",
        "a recognized host firewall service is active"
        if firewall_active
        else "no recognized host firewall service is active",
        firewalls,
    )

    listener = ssh_listener_scope()
    checks["ssh_listener"] = check(
        "attention" if listener["wildcard"] else "pass",
        "SSH listens on wildcard addresses; constrain it with firewall policy"
        if listener["wildcard"]
        else "SSH listener is scoped or not active",
        listener,
    )

    markers = pacman_database_markers()
    checks["pacman_database_layout"] = check(
        "pass" if not markers else "attention",
        "pacman local database has no unexpected top-level files"
        if not markers
        else "pacman local database has unexpected top-level files",
        markers,
    )

    if root_fs == "btrfs":
        scrub = command("systemctl", "list-timers", "--all", "--no-legend", "--plain")
        scrub_enabled = "btrfs-scrub" in scrub["output"]
        recovery_evidence = {
            "strategy": profile.get("rollback_strategy"),
            "periodic_btrfs_scrub_visible": scrub_enabled,
        }
        recovery_status = "pass" if scrub_enabled else "attention"
        recovery_summary = (
            "Btrfs rollback and periodic scrub are visible"
            if scrub_enabled
            else "Btrfs rollback is declared but no periodic scrub timer is visible"
        )
    else:
        recovery_evidence = {
            "strategy": profile.get("rollback_strategy"),
            "verified_backup": False,
        }
        recovery_status = "attention"
        recovery_summary = "ext4 canary requires external backup evidence before deployment"
    checks["recovery"] = check(recovery_status, recovery_summary, recovery_evidence)

    boot = command("findmnt", "-n", "-o", "FSTYPE,OPTIONS", "/boot")
    boot_line = boot["output"].splitlines()[0] if boot["output"] else "not-mounted-separately"
    restrictive_vfat_mask = "umask=0077" in boot_line or (
        "fmask=0077" in boot_line and "dmask=0077" in boot_line
    )
    weak_vfat_mask = boot_line.startswith(("vfat ", "fat ")) and not restrictive_vfat_mask
    checks["boot_mount_permissions"] = check(
        "attention" if weak_vfat_mask else "pass",
        "VFAT boot mount permissions are broader than the distribution target"
        if weak_vfat_mask
        else "boot mount does not expose the known broad VFAT mask",
        boot_line,
    )

    counts = {
        status: sum(item["status"] == status for item in checks.values())
        for status in ("pass", "attention", "blocker")
    }
    overall = "blocked" if counts["blocker"] else "attention" if counts["attention"] else "pass"
    return {
        "schema_version": SCHEMA_VERSION,
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "machine_id": profile["machine_id"],
        "role": profile["role"],
        "update_channel": profile["update_channel"],
        "overall": overall,
        "counts": counts,
        "checks": checks,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--machine", required=True)
    parser.add_argument("--output", type=Path, help="write a new JSON evidence file")
    parser.add_argument("--strict", action="store_true", help="fail on attention findings")
    args = parser.parse_args()
    root = args.root.resolve()
    try:
        report = audit(root, load_profile(root, args.machine))
    except AuditError as error:
        parser.exit(2, f"machine audit failed: {error}\n")
    encoded = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.output:
        try:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            with args.output.open("x") as stream:
                stream.write(encoded)
        except OSError as error:
            parser.exit(2, f"cannot create audit evidence {args.output}: {error}\n")
    print(encoded, end="")
    if report["overall"] == "blocked" or (args.strict and report["overall"] != "pass"):
        raise SystemExit(1)


if __name__ == "__main__":
    main()
