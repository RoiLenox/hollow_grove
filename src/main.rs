use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use hollow_grove::{
    APPLICATION_REGISTRY_ARTIFACT_PATH, Symptom, build_hollow_grove_application_registry_json,
    build_hollow_grove_application_witness, build_hollow_grove_foundation_verification_report,
    canonical_hollow_grove_application_registry, run_kernel_cycle, write_text_artifact,
};

const RUNTIME_BINARY_NAME: &str = "hollow_grove_runtime";
const BRIDGE_BINARY_NAME: &str = "hollow_grove_niri_bridge";
const DEV_BINARY_NAME: &str = "hg-dev";
const BENCHMARK_BINARY_NAME: &str = "current_synthesis_benchmark";
const HUEMAN_SLICE_BINARY_NAME: &str = "hueman_slice_demo";
const CURRENT_SYNTHESIS_TUI_BINARY_NAME: &str = "current_synthesis_tui";
const HOLLOW_DECK_BINARY_NAME: &str = "hollow_deck";
const SEMANTIC_BINARY_NAME: &str = "hollow_grove_semantic";

#[derive(Debug, Clone, PartialEq, Eq)]
enum MainCli {
    Help,
    Kernel,
    Runtime(Vec<String>),
    Bridge(Vec<String>),
    Desktop(Vec<String>),
    Benchmark(Vec<String>),
    App(Vec<String>),
    HuemanSlice(Vec<String>),
    HollowDeck(Vec<String>),
    Lore(Vec<String>),
    Architecture(Vec<String>),
    Stonebend(Vec<String>),
    Sandmanor(Vec<String>),
    Glaushouse(Vec<String>),
    VerifyFoundation,
    CurrentSynthesisTui(Vec<String>),
    SemanticTools(Vec<String>),
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
        "app" | "application" => Ok(MainCli::App(args.collect())),
        "hueman-slice" => Ok(MainCli::HuemanSlice(args.collect())),
        "deck" | "hollow-deck" => Ok(MainCli::HollowDeck(args.collect())),
        "lore" => Ok(MainCli::Lore(args.collect())),
        "architecture" => Ok(MainCli::Architecture(args.collect())),
        "stonebend" => Ok(MainCli::Stonebend(args.collect())),
        "sandmanor" => Ok(MainCli::Sandmanor(args.collect())),
        "glaushouse" => Ok(MainCli::Glaushouse(args.collect())),
        "semantic" | "rampart" | "federation" | "model" | "dataset" => {
            let forwarded_command = if command == "federation" {
                "rampart".to_owned()
            } else {
                command
            };
            let mut forwarded = vec![forwarded_command];
            forwarded.extend(args);
            Ok(MainCli::SemanticTools(forwarded))
        }
        "verify-foundation" => {
            if let Some(extra) = args.next() {
                Err(format!(
                    "verify-foundation does not accept additional arguments: {extra}"
                ))
            } else {
                Ok(MainCli::VerifyFoundation)
            }
        }
        "guides"
        | "scenario"
        | "world"
        | "progression"
        | "point-squared"
        | "map"
        | "rule-of-twelve"
        | "manager-language"
        | "player-location"
        | "being-object"
        | "move"
        | "civic-body"
        | "civic-crisis"
        | "flow-glow"
        | "embodied-action"
        | "current-inheritance"
        | "grip"
        | "aura-polarity"
        | "light-aura"
        | "dark-aura"
        | "aura-route"
        | "foundation-checkpoint"
        | "stanislavski"
        | "falloutman"
        | "marquise"
        | "engine"
        | "bond"
        | "resource"
        | "player"
        | "npc"
        | "cleopatra" => {
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
       app ...           inspect, launch, or attach a Hollow Grove-managed application\n\
       hueman-slice      run the Hueman vertical-slice demo surface\n\
       deck [args]      open or query the canonical Hollow Deck\n\
       lore [args]       inspect lore contracts and replay-backed gameplay consequence\n\
       architecture [args] audit architecture source links and lore execution coverage\n\
       stonebend [args]   inspect current paths or replay a Stonebend governance archive\n\
       sandmanor [args]   inspect current paths or replay a Sandmanor governance archive\n\
       glaushouse [args]  inspect current seats, Forms or a Glaushouse governance archive\n\
       semantic ...      inspect witnessed semantic positions and history\n\
       rampart ...       run or serve the authoritative transformer-assisted semantic cycle\n\
       model ...         validate an untrusted model response\n\
       dataset ...       validate witnessed semantic JSONL\n\
       verify-foundation print the Hollow Grove semantic foundation regression report\n\
       guides status     present Roï and Rosaliña through the outside TUI\n\
       guides audit      audit sessions and durable crossing journals\n\
       guides schema     print the versioned GuideContextV1 JSON Schema\n\
       scenario ...      list or switch Current Synthesis scenarios\n\
       world ...         inspect authoritative Current Synthesis world context and alignment\n\
       progression ...   inspect Point and Point² progression state\n\
       point-squared ... inspect the canonical Point² ascension witness\n\
       map ...           inspect Ranina-centered world geometry\n\
       rule-of-twelve ... inspect the four-House twelve-position grammar\n\
       manager-language ... inspect Proxy / Moxy / Foxy manager semantics\n\
       player-location ... inspect derived player spatial interpretation\n\
       being-object ... inspect the Being / Object root ontology\n\
       move ...          inspect canonical move resolution over Being and Object\n\
       civic-body ...    inspect the civic-body correspondence layer\n\
       civic-crisis ...  inspect the canonical civic breach-response witness\n\
       flow-glow ...     inspect embodied Flow / Glow gesture grammar\n\
       embodied-action ... inspect typed embodied action fixtures\n\
       current-inheritance ... inspect the inherited Current Grip lineage\n\
       grip ...          inspect full Grip inheritance fixtures\n\
       aura-polarity ... inspect Aura polarity evaluation and directional witnesses\n\
       light-aura ...    inspect canonical Light Aura fixtures\n\
       dark-aura ...     inspect canonical Dark Aura fixtures\n\
       aura-route ...    inspect route-level Aura polarity tendencies\n\
       foundation-checkpoint ... inspect the Embodied World Foundation V1 freeze checkpoint\n\
       stanislavski ...  inspect typed Stanislavski action-beat generation and evaluation\n\
       falloutman ...    inspect Falloutman dramatic response presentation over Stanislavski V2\n\
       marquise ...      inspect canonical Flynt maritime Mystery Men lore\n\
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
       hollow-grove app list\n\
       hollow-grove app describe chroma_cord\n\
       hollow-grove app launch chroma_cord\n\
       hollow-grove app attach chroma_cord\n\
       hollow-grove hueman-slice walk\n\
       hollow-grove deck\n\
       hollow-grove deck registrar\n\
       hollow-grove deck registrar show WR-006\n\
       hollow-grove deck nature gargoyle\n\
       hollow-grove deck compare gargoyle medusa\n\
       hollow-grove semantic inspect semantic-witnesses-v1.jsonl object.support-column.041 structural-repair\n\
       hollow-grove rampart run point.hueman.origin structural-repair \"Restore the supporting frame.\"\n\
       hollow-grove rampart serve --listen 127.0.0.1:47820\n\
       hollow-grove model validate-response response.json\n\
       hollow-grove dataset validate-semantic semantic-witnesses-v1.jsonl\n\
       hollow-grove verify-foundation\n\
       hollow-grove guides status\n\
       hollow-grove guides audit\n\
       hollow-grove guides schema\n\
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
       hollow-grove being-object witness\n\
       hollow-grove being-object validate\n\
       hollow-grove move witness\n\
       hollow-grove civic-body witness\n\
       hollow-grove civic-body validate\n\
       hollow-grove civic-crisis witness\n\
       hollow-grove flow-glow witness\n\
       hollow-grove flow-glow validate\n\
       hollow-grove embodied-action witness\n\
       hollow-grove current-inheritance witness\n\
       hollow-grove current-inheritance validate\n\
       hollow-grove grip witness\n\
       hollow-grove aura-polarity witness\n\
       hollow-grove aura-polarity validate\n\
       hollow-grove light-aura witness\n\
       hollow-grove dark-aura witness\n\
       hollow-grove aura-route witness\n\
       hollow-grove foundation-checkpoint witness\n\
       hollow-grove foundation-checkpoint validate\n\
       hollow-grove stanislavski witness\n\
       hollow-grove stanislavski validate\n\
       hollow-grove falloutman witness\n\
       hollow-grove falloutman validate\n\
       hollow-grove falloutman menu witness\n\
       hollow-grove falloutman beat witness\n\
       hollow-grove falloutman hidden-wound witness\n\
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
    let dev_build_enabled =
        env::var_os("HOLLOW_GROVE_DEV_BUILD").is_some_and(|value| value == "1" || value == "true");
    let sibling_paths = bin_names
        .iter()
        .map(|bin_name| sibling_dir.join(bin_name))
        .collect::<Vec<_>>();

    if sibling_paths.iter().all(|path| path.exists()) && !dev_build_enabled {
        return Ok(sibling_paths);
    }

    if sibling_paths.iter().all(|path| path.exists()) {
        match repo_root.as_deref() {
            Some(root) if binaries_are_fresh(&sibling_paths, root)? => return Ok(sibling_paths),
            None => return Ok(sibling_paths),
            Some(_) => {}
        }
    }

    if !dev_build_enabled {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "required release binaries are missing beside {}; run install-local-release.sh or set HOLLOW_GROVE_DEV_BUILD=1",
                sibling_dir.display()
            ),
        ));
    }

    let repo_root = repo_root.ok_or_else(|| {
        io::Error::other("could not locate the Hollow Grove repo root to build missing binaries")
    })?;
    let target_root = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join("target"));
    // Cargo resolves an environment-provided relative target path against
    // its working directory, which is the repository root below.
    let target_root = if target_root.is_absolute() {
        target_root
    } else {
        repo_root.join(target_root)
    };
    let target_dir = target_root.join(profile);
    let built_paths = bin_names
        .iter()
        .map(|bin_name| target_dir.join(bin_name))
        .collect::<Vec<_>>();

    if built_paths.iter().all(|path| path.exists()) && binaries_are_fresh(&built_paths, &repo_root)?
    {
        return Ok(built_paths);
    }

    let mut build = Command::new("cargo");
    build
        .current_dir(&repo_root)
        .args(["build", "--features", "dev-tools"]);
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

fn run_dev_tool(tool_name: &str, args: &[String]) -> io::Result<()> {
    let sibling_dir = current_exe_parent()?;
    let consolidated_binary = sibling_dir.join(DEV_BINARY_NAME);
    let compatibility_alias = sibling_dir.join(tool_name);
    let (binary_path, tool_argument) = if consolidated_binary.is_file() {
        (consolidated_binary, Some(tool_name))
    } else if compatibility_alias.is_file() {
        (compatibility_alias, None)
    } else {
        let binary_path = ensure_bins_available(&[DEV_BINARY_NAME])?
            .into_iter()
            .next()
            .ok_or_else(|| io::Error::other("missing hg-dev path after resolution"))?;
        (binary_path, Some(tool_name))
    };

    let mut command = Command::new(&binary_path);
    if let Some(tool_argument) = tool_argument {
        command.arg(tool_argument);
    }
    let status = command.args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "{tool_name} exited with status {status}"
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

fn application_usage() -> &'static str {
    "Usage: hollow-grove app <command> [application] [args]\n\
     \n\
     Commands:\n\
       list                         list Hollow Grove-managed applications\n\
       describe chroma_cord         print the typed ownership and attachment witness\n\
       registry                     refresh and print the machine-readable registry\n\
       launch chroma_cord [args]    open the native managed application window\n\
       attach chroma_cord           attach the focused managed window through the existing pair control\n"
}

fn sync_application_registry_artifact(repo_root: &Path) -> io::Result<()> {
    write_text_artifact(
        &repo_root.join(APPLICATION_REGISTRY_ARTIFACT_PATH),
        &build_hollow_grove_application_registry_json(),
    )?;
    Ok(())
}

fn managed_application(name: &str) -> io::Result<hollow_grove::ApplicationDefinition> {
    canonical_hollow_grove_application_registry()
        .application(name)
        .cloned()
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown managed application: {name}"),
            )
        })
}

fn run_application_cli(args: &[String]) -> io::Result<()> {
    let Some(command) = args.first().map(String::as_str) else {
        println!("{}", application_usage());
        return Ok(());
    };
    match command {
        "--help" | "-h" | "help" => {
            println!("{}", application_usage());
            Ok(())
        }
        "list" => {
            for application in canonical_hollow_grove_application_registry().applications {
                println!(
                    "{}\t{}\t{}\t{}",
                    application.canonical_name,
                    application.kind.as_str(),
                    application.world_anchor.node_name,
                    application.window_app_id
                );
            }
            Ok(())
        }
        "describe" | "witness" => {
            let name = args.get(1).map(String::as_str).unwrap_or("chroma_cord");
            managed_application(name)?;
            println!("{}", build_hollow_grove_application_witness());
            Ok(())
        }
        "registry" => {
            let repo_root = find_repo_root().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "could not locate Hollow Grove repo root",
                )
            })?;
            sync_application_registry_artifact(&repo_root)?;
            print!("{}", build_hollow_grove_application_registry_json());
            Ok(())
        }
        "launch" => {
            let name = args.get(1).map(String::as_str).unwrap_or("chroma_cord");
            let application = managed_application(name)?;
            let repo_root = find_repo_root().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "could not locate Hollow Grove repo root",
                )
            })?;
            sync_application_registry_artifact(&repo_root)?;
            let launcher = repo_root.join(&application.launch_entrypoint);
            if !launcher.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "managed application launcher not found: {}",
                        launcher.display()
                    ),
                ));
            }
            Command::new(&launcher)
                .args(args.get(2..).unwrap_or_default())
                .spawn()?;
            println!(
                "launched {} as {}; use the existing pair binding to attach it to {}",
                application.canonical_name,
                application.window_app_id,
                application.world_anchor.node_name
            );
            Ok(())
        }
        "attach" => {
            let name = args.get(1).map(String::as_str).unwrap_or("chroma_cord");
            let application = managed_application(name)?;
            let repo_root = find_repo_root().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "could not locate Hollow Grove repo root",
                )
            })?;
            sync_application_registry_artifact(&repo_root)?;
            let attach_script = repo_root.join("rebind-hueman-pair.sh");
            let status = Command::new(&attach_script)
                .env(
                    "HOLLOW_GROVE_REQUIRE_APPLICATION",
                    &application.canonical_name,
                )
                .status()?;
            if !status.success() {
                return Err(io::Error::other(format!(
                    "{} attachment rejected; focus its managed window and try again",
                    application.canonical_name
                )));
            }
            println!(
                "attached {} to {} under Hollow Grove control",
                application.canonical_name, application.world_anchor.node_name
            );
            Ok(())
        }
        other => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unknown app command: {other}"),
        )),
    }
}

fn run_lore_cli(args: &[String]) -> io::Result<()> {
    use hollow_grove::gameplay::{GameApplicationService, LoreInspection};

    let mut save = None;
    let mut json = false;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!(
                    "Usage: hollow-grove lore [--save GAMEPLAY_ARCHIVE] [--json]\nReads and replays an existing save without writing it. Without --save, reports authored definitions only."
                );
                return Ok(());
            }
            "--save" if save.is_none() => {
                save = Some(args.next().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "--save requires a gameplay archive path",
                    )
                })?);
            }
            "--json" if !json => json = true,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unknown or repeated lore argument: {arg}"),
                ));
            }
        }
    }
    let application = save
        .map(|path| {
            let encoded = fs::read_to_string(path)?;
            GameApplicationService::from_archive(&encoded)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))
        })
        .transpose()?;
    let inspection =
        LoreInspection::from_runtime(application.as_ref().map(GameApplicationService::runtime))
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    if json {
        println!("{}", serde_json::to_string_pretty(&inspection)?);
    } else {
        print!("{}", inspection.witness_markdown());
    }
    Ok(())
}

fn run_architecture_cli(args: &[String]) -> io::Result<()> {
    use hollow_grove::architecture_audit::ArchitectureCoverageReport;
    use hollow_grove::gameplay::GameApplicationService;
    let mut root = hollow_grove::source_inventory::default_source_root();
    let mut save = None;
    let mut handoff = None;
    let mut json = false;
    let mut root_seen = false;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!(
                    "Usage: hollow-grove architecture [--source-root CHECKOUT] [--save GAMEPLAY_ARCHIVE] [--handoff BACKEND_ARCHIVE] [--json]\nRead-only source crosswalk and replay-backed coverage. Declared proof files are not test execution."
                );
                return Ok(());
            }
            "--source-root" if !root_seen => {
                root = args
                    .next()
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "--source-root needs a checkout",
                        )
                    })?
                    .into();
                root_seen = true;
            }
            "--save" if save.is_none() => {
                save = Some(args.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "--save needs an archive")
                })?);
            }
            "--handoff" if handoff.is_none() => {
                handoff = Some(args.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "--handoff needs an archive")
                })?);
            }
            "--json" if !json => json = true,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("unknown or repeated architecture argument: {arg}"),
                ));
            }
        }
    }
    let application = save
        .map(|path| {
            GameApplicationService::from_archive(&fs::read_to_string(path)?)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))
        })
        .transpose()?;
    let report = ArchitectureCoverageReport::inspect(
        &root,
        application.as_ref().map(GameApplicationService::runtime),
    )
    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    let report = if let Some(path) = handoff {
        let service =
            hollow_grove::world::lore_handoffs::HandoffService::decode(&fs::read_to_string(path)?)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
        report.with_handoff(&service)
    } else {
        report
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        print!("{}", report.witness_markdown());
    }
    if !report.source_links_passed {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "architecture source-link audit failed; see violations",
        ));
    }
    Ok(())
}

fn run_stonebend_cli(args: &[String]) -> io::Result<()> {
    use hollow_grove::world::stonebend::governance::{Governance, Office, PATHS, RULE_SET};
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] | ["--help" | "-h"] => {
            println!(
                "Usage: hollow-grove stonebend paths\nhollow-grove stonebend inspect ARCHIVE [--json]\nRead-only inspection; archives are replayed and validated before reporting authority."
            );
        }
        ["paths"] => {
            for (name, stages) in PATHS {
                println!("{name}: {}", stages.join(" → "));
            }
            println!(
                "Form mastery is separate from office. High Freemason is the elected Freemason's seal function."
            );
        }
        ["inspect", path] | ["inspect", path, "--json"] => {
            let bytes = fs::read(path)?;
            let governance = Governance::decode(&bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            // Validate the authority projection as well as the archive itself.
            governance
                .world_session()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            if args.len() == 3 {
                println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                    "rule_set": RULE_SET, "revision": governance.revision(), "year": governance.year(),
                    "current_offices": Office::ALL.map(|office| serde_json::json!({"office": office, "tenure": governance.incumbent(office)})),
                    "active_seal": governance.active_seal(), "tenure_history": governance.tenures(),
                })).map_err(io::Error::other)?);
            } else {
                println!(
                    "Stonebend Revision 100 — year {}, {} accepted events",
                    governance.year(),
                    governance.revision()
                );
                for office in Office::ALL {
                    println!(
                        "{}: {}",
                        office.id(),
                        governance
                            .incumbent(office)
                            .map_or("vacant", |t| t.holder.as_str())
                    );
                }
                println!(
                    "Active High Freemason seal: {}",
                    governance.active_seal().unwrap_or("none")
                );
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected stonebend paths or stonebend inspect ARCHIVE [--json]",
            ));
        }
    }
    Ok(())
}

fn run_sandmanor_cli(args: &[String]) -> io::Result<()> {
    use hollow_grove::world::sandmanor::governance::{FORM_PATHS, Governance, Office, RULE_SET};
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] | ["--help" | "-h"] => println!(
            "Usage: hollow-grove sandmanor paths\nhollow-grove sandmanor inspect ARCHIVE [--json]\nRead-only inspection; archives are replayed and validated before reporting authority."
        ),
        ["paths"] => {
            for (region, forms) in FORM_PATHS {
                println!("{region}: {}", forms.join(" → "));
            }
            println!(
                "The Mino and The Cent are elected for four years. Only the two sitting qualified governors enter the Contest of Improvement. Form mastery and Agency rank do not grant office."
            );
        }
        ["inspect", path] | ["inspect", path, "--json"] => {
            let governance = Governance::decode(&fs::read(path)?)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            governance
                .world_session()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            if args.len() == 3 {
                println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                    "rule_set": RULE_SET, "revision": governance.revision(), "year": governance.year(),
                    "current_offices": Office::ALL.map(|office| serde_json::json!({"office":office,"tenure":governance.incumbent(office)})),
                    "tenure_history": governance.tenures(), "contests": governance.contests(),
                })).map_err(io::Error::other)?);
            } else {
                println!(
                    "Sandmanor Revision 100 — year {}, {} accepted events",
                    governance.year(),
                    governance.revision()
                );
                for office in Office::ALL {
                    println!(
                        "{}: {}",
                        office.id(),
                        governance
                            .incumbent(office)
                            .map_or("vacant", |t| t.holder.as_str())
                    );
                }
                for contest in governance.contests().values() {
                    println!(
                        "{}: {:?} (attempt {})",
                        contest.id, contest.status, contest.attempt
                    );
                }
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected sandmanor paths or sandmanor inspect ARCHIVE [--json]",
            ));
        }
    }
    Ok(())
}

fn run_glaushouse_cli(args: &[String]) -> io::Result<()> {
    use hollow_grove::world::glaushouse::governance::{
        CARE_ROLES, CallStage, Governance, MuseHouse, ORDINARY_FORMS, Office, RULE_SET,
    };
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] | ["--help" | "-h"] => println!(
            "Usage: hollow-grove glaushouse paths\nhollow-grove glaushouse seats\nhollow-grove glaushouse inspect ARCHIVE [--json]\nRead-only inspection; archives are replayed and validated before reporting authority."
        ),
        ["paths"] => {
            println!(
                "Ordinary Forms: {}",
                ORDINARY_FORMS
                    .map(|form| {
                        let mut chars = form.chars();
                        format!(
                            "{}{}",
                            chars.next().expect("Form name").to_uppercase(),
                            chars.as_str()
                        )
                    })
                    .join(" → ")
            );
            println!(
                "Advanced Persephone: post-Muse mastery and a current elected seat. Prima Donna: current sovereign only."
            );
            for (role, path, material) in CARE_ROLES {
                println!("{role}: {path} / {material}; independently licensed care role.");
            }
            println!(
                "The Call: {}",
                CallStage::ALL.map(CallStage::name).join(" → ")
            );
            println!(
                "Six of nine seat votes; nine-year sovereign term. Nightingale citizenship, mastery and office grant no clinical license."
            );
        }
        ["seats"] => {
            println!("House of Persephone — nine equal Muse-House seats:");
            for house in MuseHouse::ALL {
                println!("Persephone de {}: {}", house.name(), house.office_id());
            }
            println!(
                "Six-year peer-elected terms, three cohorts two years apart. Historical Deck affiliations require explicit reconciliation."
            );
        }
        ["inspect", path] | ["inspect", path, "--json"] => {
            let g = Governance::decode(&fs::read(path)?)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            g.world_session()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            if args.len() == 3 {
                let current = Office::all()
                    .map(|office| serde_json::json!({"office":office,"tenure":g.incumbent(office)}))
                    .collect::<Vec<_>>();
                println!("{}",serde_json::to_string_pretty(&serde_json::json!({"rule_set":RULE_SET,"revision":g.revision(),"date":g.date(),
                    "current_offices":current,"tenure_history":g.tenures(),"elections":g.elections(),"calls":g.calls()})).map_err(io::Error::other)?);
            } else {
                println!(
                    "Glaüshouse Revision 100 — year {}, day {}, {} accepted events",
                    g.date().year,
                    g.date().day,
                    g.revision()
                );
                for office in Office::all() {
                    println!(
                        "{}: {}",
                        office.id(),
                        g.incumbent(office).map_or("vacant", |t| t.holder.as_str())
                    );
                }
                for call in g.calls().values() {
                    println!(
                        "{}: {} ({} ballots)",
                        call.id,
                        call.stage.name(),
                        call.rounds
                    );
                }
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected glaushouse paths, seats, or inspect ARCHIVE [--json]",
            ));
        }
    }
    Ok(())
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
        MainCli::Benchmark(args) => run_dev_tool(BENCHMARK_BINARY_NAME, &args),
        MainCli::App(args) => run_application_cli(&args),
        MainCli::HuemanSlice(args) => run_dev_tool(HUEMAN_SLICE_BINARY_NAME, &args),
        MainCli::HollowDeck(args) => run_child_binary(HOLLOW_DECK_BINARY_NAME, &args),
        MainCli::Lore(args) => run_lore_cli(&args),
        MainCli::Architecture(args) => run_architecture_cli(&args),
        MainCli::Stonebend(args) => run_stonebend_cli(&args),
        MainCli::Sandmanor(args) => run_sandmanor_cli(&args),
        MainCli::Glaushouse(args) => run_glaushouse_cli(&args),
        MainCli::SemanticTools(args) => run_child_binary(SEMANTIC_BINARY_NAME, &args),
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
            parse_main_cli([String::from("app"), String::from("list")])
                .expect("application cli should parse"),
            MainCli::App(vec![String::from("list")])
        );
        assert_eq!(
            parse_main_cli([String::from("hueman-slice"), String::from("status")])
                .expect("hueman-slice cli should parse"),
            MainCli::HuemanSlice(vec![String::from("status")])
        );
        assert_eq!(
            parse_main_cli([
                String::from("deck"),
                String::from("nature"),
                String::from("gargoyle")
            ])
            .expect("Hollow Deck cli should parse"),
            MainCli::HollowDeck(vec![String::from("nature"), String::from("gargoyle")])
        );
        assert_eq!(
            parse_main_cli([
                String::from("semantic"),
                String::from("inspect"),
                String::from("witnesses.jsonl"),
            ])
            .expect("semantic cli should parse"),
            MainCli::SemanticTools(vec![
                String::from("semantic"),
                String::from("inspect"),
                String::from("witnesses.jsonl"),
            ])
        );
        assert_eq!(
            parse_main_cli([
                String::from("rampart"),
                String::from("run"),
                String::from("point.hueman.origin"),
                String::from("structural-repair"),
                String::from("Restore the supporting frame."),
            ])
            .expect("rampart cli should parse"),
            MainCli::SemanticTools(vec![
                String::from("rampart"),
                String::from("run"),
                String::from("point.hueman.origin"),
                String::from("structural-repair"),
                String::from("Restore the supporting frame."),
            ])
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
            parse_main_cli([String::from("civic-body"), String::from("witness")])
                .expect("civic-body cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("civic-body"), String::from("witness")])
        );
        assert_eq!(
            parse_main_cli([String::from("civic-crisis"), String::from("witness")])
                .expect("civic-crisis cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("civic-crisis"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("flow-glow"), String::from("witness")])
                .expect("flow-glow cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("flow-glow"), String::from("witness")])
        );
        assert_eq!(
            parse_main_cli([String::from("embodied-action"), String::from("witness")])
                .expect("embodied-action cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("embodied-action"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("current-inheritance"), String::from("witness")])
                .expect("current-inheritance cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("current-inheritance"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("grip"), String::from("witness")])
                .expect("grip cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("grip"), String::from("witness")])
        );
        assert_eq!(
            parse_main_cli([String::from("aura-polarity"), String::from("witness")])
                .expect("aura-polarity cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("aura-polarity"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([
                String::from("foundation-checkpoint"),
                String::from("validate")
            ])
            .expect("foundation-checkpoint cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("foundation-checkpoint"),
                String::from("validate")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("stanislavski"), String::from("witness")])
                .expect("stanislavski cli should parse"),
            MainCli::CurrentSynthesisTui(vec![
                String::from("stanislavski"),
                String::from("witness")
            ])
        );
        assert_eq!(
            parse_main_cli([String::from("falloutman"), String::from("witness")])
                .expect("falloutman cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("falloutman"), String::from("witness")])
        );
        assert_eq!(
            parse_main_cli([String::from("marquise"), String::from("hierarchy")])
                .expect("Marquise cli should parse"),
            MainCli::CurrentSynthesisTui(vec![String::from("marquise"), String::from("hierarchy")])
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
        assert!(usage.contains("app ..."));
        assert!(usage.contains("app launch chroma_cord"));
        assert!(usage.contains("hueman-slice"));
        assert!(usage.contains("deck [args]"));
        assert!(usage.contains("rampart serve"));
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
        assert!(usage.contains("being-object witness"));
        assert!(usage.contains("being-object validate"));
        assert!(usage.contains("move witness"));
        assert!(usage.contains("civic-body witness"));
        assert!(usage.contains("civic-body validate"));
        assert!(usage.contains("civic-crisis witness"));
        assert!(usage.contains("flow-glow witness"));
        assert!(usage.contains("flow-glow validate"));
        assert!(usage.contains("embodied-action witness"));
        assert!(usage.contains("current-inheritance witness"));
        assert!(usage.contains("current-inheritance validate"));
        assert!(usage.contains("grip witness"));
        assert!(usage.contains("aura-polarity witness"));
        assert!(usage.contains("aura-polarity validate"));
        assert!(usage.contains("light-aura witness"));
        assert!(usage.contains("dark-aura witness"));
        assert!(usage.contains("aura-route witness"));
        assert!(usage.contains("foundation-checkpoint witness"));
        assert!(usage.contains("foundation-checkpoint validate"));
        assert!(usage.contains("stanislavski witness"));
        assert!(usage.contains("stanislavski validate"));
        assert!(usage.contains("falloutman witness"));
        assert!(usage.contains("falloutman validate"));
        assert!(usage.contains("falloutman menu witness"));
        assert!(usage.contains("falloutman beat witness"));
        assert!(usage.contains("falloutman hidden-wound witness"));
        assert!(usage.contains("marquise ..."));
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
            binaries_are_fresh(std::slice::from_ref(&bin_path), &repo_root)
                .expect("freshness should check")
        );

        std::thread::sleep(Duration::from_millis(20));
        fs::write(
            src_dir.join("main.rs"),
            "fn main() { println!(\"new\"); }\n",
        )
        .expect("source should rewrite");

        assert!(
            !binaries_are_fresh(std::slice::from_ref(&bin_path), &repo_root)
                .expect("freshness should detect stale binary")
        );

        fs::remove_file(&bin_path).expect("binary should remove");
        fs::remove_file(src_dir.join("main.rs")).expect("source should remove");
        fs::remove_file(repo_root.join("Cargo.toml")).expect("manifest should remove");
        fs::remove_dir_all(repo_root).expect("temp repo should remove");
    }
}
