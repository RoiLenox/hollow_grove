#!/usr/bin/env python3
"""Create checksummed source bundles and standalone Arch build directories."""

import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile
import tomllib

from source_boundary import audit_source


def git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args])


def desktop_dependencies(root):
    inventory = tomllib.loads(
        (root / "packaging/hollow-grove-general/etc/hollow-grove/applications.toml").read_text()
    )
    profile = tomllib.loads((root / "distro/profiles/desktop.toml").read_text())
    names = sorted(set(inventory["required_base"]["arch_packages"])
                   | set(profile["packages"])
                   | set(profile["external_packages"]["required"]))
    if any(not re.fullmatch(r"[a-z0-9][a-z0-9@+_.-]*", name) for name in names):
        raise ValueError("invalid package name in desktop inventory")
    return names


def prepare(root, output, allow_dirty=False, signed_tag=None):
    root, output = root.resolve(), output.resolve()
    private_material = audit_source(root)
    if private_material:
        raise ValueError('private material cannot be packaged: ' + '; '.join(private_material))
    dirty = bool(git(root, "status", "--porcelain=v1", "--untracked-files=normal").strip())
    if dirty and (not allow_dirty or signed_tag):
        raise ValueError("source is dirty; use --allow-dirty only for a local development snapshot")
    commit = git(root, "rev-parse", "HEAD").decode().strip()
    if signed_tag:
        subprocess.run(["git", "-C", str(root), "verify-tag", signed_tag], check=True)
        tagged = git(root, "rev-parse", f"{signed_tag}^{{commit}}").decode().strip()
        if tagged != commit:
            raise ValueError("signed tag does not identify the checked-out commit")
    version = tomllib.loads((root / "Cargo.toml").read_text())["package"]["version"]
    if not re.fullmatch(r"[0-9][a-zA-Z0-9.+_]*", version):
        raise ValueError("version must be suitable for an Arch pkgver")
    epoch = int(git(root, "show", "-s", "--format=%ct", "HEAD"))
    names = sorted(set(os.fsdecode(name) for name in
                       git(root, "ls-files", "--cached", "--others", "--exclude-standard", "-z").split(b"\0")
                       if name))
    files = []
    for name in names:
        path = root / name
        if not path.exists() and not path.is_symlink():
            continue  # A deletion in an explicitly requested development snapshot.
        if output == path or output in path.parents:
            raise ValueError("output must be outside the source inventory (use ignored dist/)")
        if not path.resolve().is_relative_to(root):
            raise ValueError(f"source symlink escapes repository: {name}")
        if not path.is_file():
            raise ValueError(f"unsupported source entry: {name}")
        files.append((name, path, path.read_bytes()))
    manifest = {
        "schema_version": 1, "version": version, "git_commit": commit,
        "dirty_worktree": dirty, "verified_tag": signed_tag,
        "release_status": "signed-source" if signed_tag else "development-candidate",
        "source_date_epoch": epoch,
        "files": {name: hashlib.sha256(data).hexdigest() for name, _, data in files},
    }
    manifest_bytes = (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode()
    output.mkdir(parents=True, exist_ok=False)
    prefix = f"hollow-grove-{version}"
    temporary_archive = output / "source.tar.gz"
    with temporary_archive.open("wb") as stream:
        with gzip.GzipFile(filename="", fileobj=stream, mode="wb", mtime=epoch) as compressed:
            with tarfile.open(fileobj=compressed, mode="w") as archive:
                for name, path, data in files:
                    # Materialize internal symlinks: extraction needs no checkout
                    # and cannot acquire a link to a developer's filesystem.
                    info = tarfile.TarInfo(f"{prefix}/{name}")
                    info.size, info.mtime = len(data), epoch
                    info.mode = 0o755 if path.stat().st_mode & 0o111 else 0o644
                    archive.addfile(info, io.BytesIO(data))
                info = tarfile.TarInfo(f"{prefix}/SOURCE-MANIFEST.json")
                info.size, info.mtime, info.mode = len(manifest_bytes), epoch, 0o644
                archive.addfile(info, io.BytesIO(manifest_bytes))
    digest = hashlib.sha256(temporary_archive.read_bytes()).hexdigest()
    archive_name = f"hollow-grove-source-{version}-{digest[:16]}.tar.gz"
    archive_path = temporary_archive.rename(output / archive_name)
    dependencies = desktop_dependencies(root)
    substitutions = {
        "@VERSION@": version, "@SOURCE_ARCHIVE@": archive_name,
        "@SOURCE_SHA256@": digest,
        "@DESKTOP_DEPENDENCIES@": "\n    " + "\n    ".join(f"'{p}'" for p in dependencies) + "\n",
    }
    packages = []
    for template in sorted((root / "packaging").glob("*/PKGBUILD.in")):
        recipe = template.read_text()
        for token, value in substitutions.items():
            recipe = recipe.replace(token, value)
        if re.search(r"@[A-Z_]+@", recipe):
            raise ValueError(f"unresolved token in {template.name}")
        destination = output / template.parent.name
        destination.mkdir()
        (destination / "PKGBUILD").write_text(recipe)
        build_script = destination / "build.sh"
        build_script.write_text(
            "#!/usr/bin/env bash\n"
            "set -euo pipefail\n"
            f"export SOURCE_DATE_EPOCH={epoch}\n"
            'exec makepkg --cleanbuild --check "$@"\n'
        )
        build_script.chmod(0o755)
        os.link(archive_path, destination / archive_name)
        srcinfo = subprocess.check_output(["makepkg", "--printsrcinfo"], cwd=destination)
        (destination / ".SRCINFO").write_bytes(srcinfo)
        packages.append(template.parent.name)
    (output / "SOURCE-MANIFEST.json").write_bytes(manifest_bytes)
    (output / "SHA256SUMS").write_text(f"{digest}  {archive_name}\n")
    (output / "BUILD.json").write_text(json.dumps({
        "source": archive_name, "sha256": digest, "packages": packages,
        "desktop_dependencies": dependencies, "source_status": manifest["release_status"],
        "source_date_epoch": epoch,
        "build_command": "./build.sh",
        "publication_ready": False,
    }, indent=2) + "\n")
    return archive_path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=Path("dist/arch-packages"))
    parser.add_argument("--allow-dirty", action="store_true",
                        help="include non-ignored worktree changes; mark as a development candidate")
    parser.add_argument("--signed-tag", help="verify a signed tag pointing to clean HEAD")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    try:
        archive = prepare(root, args.output, args.allow_dirty, args.signed_tag)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f"package preparation failed: {error}\n")
    print(f"Prepared {archive}")
    print("Build each generated directory with ./build.sh.")
    print("Publication still requires clean-chroot, signing, installation and upgrade gates.")


if __name__ == "__main__":
    main()
