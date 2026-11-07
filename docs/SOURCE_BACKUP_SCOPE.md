# Portable source and private machine state

The GitHub project contains Hollow Grove source, lore, approved art assets,
tests, build recipes and product defaults. The named hardware profiles remain
`noahsarch` for AMD and `occamsrazer` for NVIDIA. These describe build and test
roles; they contain no login keys, passwords or network addresses.

Live game saves, SSH/GPG/cloud account directories, environment credentials,
personal desktop settings and generated caches are outside the source backup.
The checked-in `packaging/**/etc/skel` files are product defaults for new users,
not copies of a developer's home configuration.

Run `python3 tools/source_boundary.py` before checkpointing or packaging work.
It checks both staged bytes and the working source inventory, including new
non-ignored files. It rejects recognized private account paths, live saves,
common credential formats and source symlinks that escape the repository.
Diagnostics report filenames and categories without echoing matched secrets.
Normal edit/integration checks, CI and source/release preparation use this gate.
Pattern checks cannot detect every possible secret or replace a human review.
They do not rewrite historical commits or change Git author metadata.

Public machine names do not authenticate an SSH connection. Live SSH policy,
authorized keys and network filtering are administered separately on each host.
Source publication does not install accounts, start SSH, or open firewall ports.
An administrator can require public-key authentication and deny password/root
login after verifying a fresh authorized key connection and a recovery path.
Keep any resulting audit receipts and administrative backups outside this repo.
