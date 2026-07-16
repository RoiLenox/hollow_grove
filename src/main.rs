use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use hollow_grove::{Symptom, build_hollow_grove_foundation_verification_report, run_kernel_cycle};

const RUNTIME_BINARY_NAME: &str = "hollow_grove_runtime";
const BRIDGE_BINARY_NAME: &str = "hollow_grove_niri_bridge";
const BENCHMARK_BINARY_NAME: &str = "current_synthesis_benchmark";
const HUEMAN_SLICE_BINARY_NAME: &str = "hueman_slice_demo";
const CURRENT_SYNTHESIS_TUI_BINARY_NAME: &str = "current_synthesis_tui";

#[derive(Debug, Clone, PartialEq, Eq)]
enum MainCli {
    Help,
    Kernel,
    Runtime(Vec<String>),
    Bridge(Vec<String>),
    Desktop(Vec<String>),
    Benchmark(Vec<String>),
    HuemanSlice(Vec<String>),
    VerifyFoundation,
    CurrentSynthesisTui(Vec<String>),
}

fn parse_main_cli<I>(args: I) -> Result<MainCli, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        return Ok(MainCli::Kernel);
    };

    match command.as_str() {
        "--help" | "-h" | "help" => Ok(MainCli::Help),
        "kernel" => {
            if let Some(extra) = args.next() {
                Err(format!(
                    "kernel does not accept additional arguments: {extra}"
                ))
            } else {
                Ok(MainCli::Kernel)
            }
        }
        "runtime" => Ok(MainCli::Runtime(args.collect())),
        "bridge" => Ok(MainCli::Bridge(args.collect())),
        "desktop" | "launch" => Ok(MainCli::Desktop(args.collect())),
        "benchmark" => Ok(MainCli::Benchmark(args.collect())),
        "hueman-slice" => Ok(MainCli::HuemanSlice(args.collect())),
        "verify-foundation" => {
            if let Some(extra) = args.next() {
                Err(format!(
                    "verify-foundation does not accept additional arguments: {extra}"
                ))
            } else {
                Ok(MainCli::VerifyFoundation)
            }
        }
        "scenario" | "world" | "progression" | "point-squared" | "map" | "rule-of-twelve"
        | "manager-language" | "player-location" | "engine" | "bond" | "resource" | "player"
        | "npc" | "cleopatra" => {
            let mut forwarded = vec![command];
            forwarded.extend(args);
            Ok(MainCli::CurrentSynthesisTui(forwarded))
        }
        other => Err(format!("unknown command: {other}")),
    }
}

fn usage() -> &'static str {
    "Usage: hollow-grove [command] [args]\n\
     \n\
     Commands:\n\
       kernel            print the canonical witness (default when no command is given)\n\
       runtime [args]    run the Hollow Grove runtime loop\n\
       bridge [args]     run the Niri bridge against runtime memory\n\
       desktop [args]    launch the runtime loop with the Niri bridge attached\n\
       benchmark [args]  run the Current-Synthesis-style benchmark suite\n\
       hueman-slice      run the Hueman vertical-slice demo surface\n\
       verify-foundation print the Hollow Grove semantic foundation regression report\n\
       scenario ...      list or switch Current Synthesis scenarios\n\
       world ...         inspect authoritative Current Synthesis world context and alignment\n\
       progression ...   inspect Point and Point² progression state\n\
       point-squared ... inspect the canonical Point² ascension witness\n\
       map ...           inspect Ranina-centered world geometry\n\
       rule-of-twelve ... inspect the four-House twelve-position grammar\n\
       manager-language ... inspect Proxy / Moxy / Foxy manager semantics\n\
       player-location ... inspect derived player spatial interpretation\n\
       engine ...        inspect Current Synthesis engine state\n\
       bond ...          inspect bond candidates and traces\n\
       resource ...      inspect Aura, Current, and residue history\n\
       player ...        inspect or queue planned player actions\n\
       npc ...           inspect BLEP NPC state and history\n\
       cleopatra ...     tick and trace BLEP orchestration\n\
       help              print this help\n\
     \n\
     Examples:\n\
       hollow-grove\n\
       hollow-grove runtime --cycles 5 --interval-ms 1000\n\
       hollow-grove bridge --apply --watch\n\
       hollow-grove desktop --cycles 5 --interval-ms 1000\n\
       hollow-grove benchmark --warmup 5 --samples 25\n\
       hollow-grove hueman-slice walk\n\
       hollow-grove verify-foundation\n\
       hollow-grove scenario list\n\
       hollow-grove world context\n\
       hollow-grove world witness\n\
       hollow-grove world validate\n\
       hollow-grove progression witness\n\
       hollow-grove progression validate\n\
       hollow-grove point-squared witness\n\
       hollow-grove map witness\n\
       hollow-grove map validate\n\
       hollow-grove rule-of-twelve witness\n\
       hollow-grove rule-of-twelve validate\n\
       hollow-grove manager-language witness\n\
       hollow-grove manager-language validate\n\
       hollow-grove player-location witness\n\
       hollow-grove scenario use flooded_quarry_night_watch\n\
       hollow-grove engine status\n\
       hollow-grove bond list\n\
       hollow-grove resource history\n\
       hollow-grove player plan brace the intake ladder before dawn\n\
       hollow-grove player move cross the flooded rim\n\
       hollow-grove player decide signal the upper crew\n\
       hollow-grove npc inspect route_warden_04\n\
       hollow-grove cleopatra tick\n\
       hollow-grove cleopatra run 5"
}

fn canonical_kernel_output() -> String {
    run_kernel_cycle(Symptom::origin()).to_string()
}

fn candidate_repo_root_from(path: &Path) -> Option<PathBuf> {
    let start = if path.is_dir() { path } else { path.parent()? };

    for ancestor in start.ancestors() {
        if ancestor.join("Cargo.toml").exists() {
            return Some(ancestor.to_path_buf());
        }
    }

    None
}

fn find_repo_root() -> Option<PathBuf> {
    if let Ok(current_dir) = env::current_dir()
        && let Some(root) = candidate_repo_root_from(&current_dir)
    {
        return Some(root);
    }

    if let Ok(current_exe) = env::current_exe() {
        return candidate_repo_root_from(&current_exe);
    }

    None
}

fn current_exe_parent() -> io::Result<PathBuf> {
    env::current_exe()?
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| io::Error::other("current executable has no parent directory"))
}

fn current_profile_dir_name(path: &Path) -> &'static str {
    match path.file_name().and_then(|name| name.to_str()) {
        Some("release") => "release",
        _ => "debug",
    }
}

fn latest_mtime_in_tree(path: &Path) -> io::Result<SystemTime> {
    let metadata = fs::metadata(path)?;
    let mut latest = metadata.modified()?;

    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_latest = latest_mtime_in_tree(&entry.path())?;
            if entry_latest > latest {
                latest = entry_latest;
            }
        }
    }

    Ok(latest)
}

fn latest_repo_source_mtime(repo_root: &Path) -> io::Result<SystemTime> {
    let mut latest = fs::metadata(repo_root.join("Cargo.toml"))?.modified()?;
    let src_latest = latest_mtime_in_tree(&repo_root.join("src"))?;
    if src_latest > latest {
        latest = src_latest;
    }
    Ok(latest)
}

fn binaries_are_fresh(bin_paths: &[PathBuf], repo_root: &Path) -> io::Result<bool> {
    let latest_source = latest_repo_source_mtime(repo_root)?;

    for bin_path in bin_paths {
        let binary_mtime = fs::metadata(bin_path)?.modified()?;
        if binary_mtime < latest_source {
            return Ok(false);
        }
    }

    Ok(true)
}

fn ensure_bins_available(bin_names: &[&str]) -> io::Result<Vec<PathBuf>> {
    let sibling_dir = current_exe_parent()?;
    let profile = current_profile_dir_name(&sibling_dir);
    let repo_root = find_repo_root();
    let sibling_paths = bin_names
        .iter()
        .map(|bin_name| sibling_dir.join(bin_name))
        .collect::<Vec<_>>();

    if sibling_paths.iter().all(|path| path.exists()) {
        match repo_root.as_deref() {
            Some(root) if binaries_are_fresh(&sibling_paths, root)? => return Ok(sibling_paths),
            None => return Ok(sibling_paths),
            Some(_) => {}
        }
    }

    let repo_root = repo_root.ok_or_else(|| {
        io::Error::other("could not locate the Hollow Grove repo root to build missing binaries")
    })?;
    let target_dir = repo_root.join("target").join(profile);
    let built_paths = bin_names
        .iter()
        .map(|bin_name| target_dir.join(bin_name))
        .collect::<Vec<_>>();

    if built_paths.iter().all(|path| path.exists()) && binaries_are_fresh(&built_paths, &repo_root)?
    {
        return Ok(built_paths);
    }

    let mut build = Command::new("cargo");
    build.current_dir(&repo_root).arg("build");
    if profile == "release" {
        build.arg("--release");
    }
    for bin_name in bin_names {
        build.args(["--bin", bin_name]);
    }

    let status = build.status()?;
    if !status.success() {
        return Err(io::Error::other(format!(
            "cargo build for integrated binaries exited with status {status}"
        )));
    }

    if built_paths.iter().all(|path| path.exists()) {
        Ok(built_paths)
    } else {
        Err(io::Error::other(
            "cargo build finished but one or more integrated binaries are still missing",
        ))
    }
}

fn run_child_binary(bin_name: &str, args: &[String]) -> io::Result<()> {
    let binary_path = ensure_bins_available(&[bin_name])?
        .into_iter()
        .next()
        .ok_or_else(|| io::Error::other("missing child binary path after resolution"))?;
    let status = Command::new(&binary_path).args(args).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "{bin_name} exited with status {status}"
        )))
    }
}

fn run_desktop_launcher(runtime_args: &[String]) -> io::Result<()> {
    let bin_paths = ensure_bins_available(&[BRIDGE_BINARY_NAME, RUNTIME_BINARY_NAME])?;
    let bridge_path = &bin_paths[0];
    let runtime_path = &bin_paths[1];

    let mut bridge = Command::new(bridge_path)
        .args(["--apply", "--watch", "--quiet"])
        .spawn()?;
    let runtime_result = Command::new(runtime_path).args(runtime_args).status();

    let _ = bridge.kill();
    let _ = bridge.wait();

    let runtime_status = runtime_result?;
    if runtime_status.success() {
        let bridge_status = Command::new(bridge_path)
            .args(["--apply", "--cycles", "1", "--quiet"])
            .status()?;

        if bridge_status.success() {
            Ok(())
        } else {
            Err(io::Error::other(format!(
                "{BRIDGE_BINARY_NAME} exited with status {bridge_status}"
            )))
        }
    } else {
        Err(io::Error::other(format!(
            "{RUNTIME_BINARY_NAME} exited with status {runtime_status}"
        )))
    }
}

fn main() -> io::Result<()> {
    let cli = parse_main_cli(env::args().skip(1))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;

    match cli {
        MainCli::Help => {
            println!("{}", usage());
            Ok(())
        }
        MainCli::Kernel => {
            println!("{}", canonical_kernel_output());
            Ok(())
        }
        MainCli::Runtime(args) => run_child_binary(RUNTIME_BINARY_NAME, &args),
        MainCli::Bridge(args) => run_child_binary(BRIDGE_BINARY_NAME, &args),
        MainCli::Desktop(args) => run_desktop_launcher(&args),
        MainCli::Benchmark(args) => run_child_binary(BENCHMARK_BINARY_NAME, &args),
        MainCli::HuemanSlice(args) => run_child_binary(HUEMAN_SLICE_BINARY_NAME, &args),
        MainCli::VerifyFoundation => {
            println!("{}", build_hollow_grove_foundation_verification_report()?);
            Ok(())
        }
        MainCli::CurrentSynthesisTui(args) => {
            run_child_binary(CURRENT_SYNTHESIS_TUI_BINARY_NAME, &args)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use super::{MainCli, binaries_are_fresh, canonical_kernel_output, parse_main_cli, usage};
    use hollow_grove::CANONICAL_WITNESS;

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nonce}"))
    }

    #[test]
    fn main_cli_defaults_to_kernel_output() {
        let cli = parse_main_cli(std::iter::empty::<String>()).expect("cli should parse");
        assert_eq!(cli, MainCli::Kernel);
        assert_eq!(canonical_kernel_output(), CANONICAL_WITNESS);
    }

    #[test]
    fn main_cli_supports_integrated_commands() {
        assert_eq!(
            parse_main_cli([String::from("runtime"), String::from("--help")])
                .expect("runtime cli should parse"),
            MainCli::Runtime(vec![String::from("--help")])
        );
        assert_eq!(
            parse_main_cli([String::from("bridge"), String::from("--help")])
                .expect("bridge cli should parse"),
            MainCli::Bridge(vec![String::from("--help")])
        );
        assert_eq!(
            parse_main_cli([
                String::from("desktop"),
                String::from("--cycles"),
                String::from("2")
            ])
            .expect("desktop cli should parse"),
            MainCli::Desktop(vec![String::from("--cycles"), String::from("2")])
        );
        assert_eq!(
            parse_main_cli([
                String::from("benchmark"),
                String::from("--samples"),
                String::from("3")
            ])
            .expect("benchmark cli should parse"),
            MainCli::Benchmark(vec![String::from("--samples"), String::from("3")])
        );
        assert_eq!(
            parse_main_cli([String::from("hueman-slice"), String::from("status")])
                .expect("hueman-slice cli should parse"),
            MainCli::HuemanSlice(vec![String::from("status")])
        );
        assert_eq!(
            parse_main_cli([String::from("verify-foundation")])
                .expect("verify-foundation cli should parse"),
            MainCli::VerifyFoundation
        );
        assert_eq!(
            parse_main_cli([String::from("scenario"), String::from("list")])
                .expect("scenario cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("scenario"), String::from("list")])
        );
        assert_eq!(
            parse_main_cli([String::from("world"), String::from("context")])
                .expect("world cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("world"), String::from("context")])
        );
        assert_eq!(
            parse_main_cli([String::from("progression"), String::from("witness")])
                .expect("progression cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("progression"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("point-squared"), String::from("witness")])
                .expect("point-squared cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("point-squared"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("engine"), String::from("status")])
                .expect("engine cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("engine"), String::from("status")])
        );
        assert_eq!(
            parse_main_cli([String::from("rule-of-twelve"), String::from("witness")])
                .expect("rule-of-twelve cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("rule-of-twelve"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("manager-language"), String::from("witness")])
                .expect("manager-language cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("manager-language"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("player-location"), String::from("witness")])
                .expect("player-location cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("player-location"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("player"), String::from("status")])
                .expect("player cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("player"), String::from("status")])
        );
        assert_eq!(
            parse_main_cli([String::from("cleopatra"), String::from("tick")])
                .expect("cleopatra cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("cleopatra"), String::from("tick")])
        );
    }

    #[test]
    fn main_cli_supports_help_and_reports_unknown_commands() {
        assert_eq!(
            parse_main_cli([String::from("--help")]).expect("help should parse"),
            MainCli::Help
        );
        let error = parse_main_cli([String::from("unknown")]).expect_err("unknown should fail");
        assert_eq!(error, "unknown command: unknown");
    }

    #[test]
    fn usage_reports_integrated_entrypoints() {
        let usage = usage();
        assert!(usage.contains("hollow-grove"));
        assert!(usage.contains("runtime [args]"));
        assert!(usage.contains("bridge [args]"));
        assert!(usage.contains("desktop [args]"));
        assert!(usage.contains("benchmark [args]"));
        assert!(usage.contains("hueman-slice"));
        assert!(usage.contains("verify-foundation"));
        assert!(usage.contains("scenario list"));
        assert!(usage.contains("world context"));
        assert!(usage.contains("progression witness"));
        assert!(usage.contains("progression validate"));
        assert!(usage.contains("point-squared witness"));
        assert!(usage.contains("map witness"));
        assert!(usage.contains("map validate"));
        assert!(usage.contains("rule-of-twelve witness"));
        assert!(usage.contains("rule-of-twelve validate"));
        assert!(usage.contains("manager-language witness"));
        assert!(usage.contains("manager-language validate"));
        assert!(usage.contains("player-location witness"));
        assert!(usage.contains("engine status"));
        assert!(usage.contains("player plan"));
        assert!(usage.contains("cleopatra tick"));
    }

    #[test]
    fn launcher_rebuild_check_detects_stale_child_binary() {
        let repo_root = unique_temp_dir("hollow-grove-main-freshness");
        let src_dir = repo_root.join("src");
        let bin_path = repo_root
            .join("target")
            .join("debug")
            .join("current_synthesis_benchmark");

        fs::create_dir_all(&src_dir).expect("src dir should create");
        fs::create_dir_all(bin_path.parent().expect("bin parent should exist"))
            .expect("target dir should create");
        fs::write(
            repo_root.join("Cargo.toml"),
            "[package]\nname = \"stub\"\nversion = \"0.0.0\"\n",
        )
        .expect("cargo manifest should write");
        fs::write(src_dir.join("main.rs"), "fn main() {}\n").expect("source should write");
        std::thread::sleep(Duration::from_millis(20));
        fs::write(&bin_path, "binary").expect("binary should write");

        assert!(
            binaries_are_fresh(&[bin_path.clone()], &repo_root).expect("freshness should check")
        );

        std::thread::sleep(Duration::from_millis(20));
        fs::write(
            src_dir.join("main.rs"),
            "fn main() { println!(\"new\"); }\n",
        )
        .expect("source should rewrite");

        assert!(
            !binaries_are_fresh(&[bin_path.clone()], &repo_root)
                .expect("freshness should detect stale binary")
        );

        fs::remove_file(&bin_path).expect("binary should remove");
        fs::remove_file(src_dir.join("main.rs")).expect("source should remove");
        fs::remove_file(repo_root.join("Cargo.toml")).expect("manifest should remove");
        fs::remove_dir_all(repo_root).expect("temp repo should remove");
    }
}
