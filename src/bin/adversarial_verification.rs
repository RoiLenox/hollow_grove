use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use hollow_grove::verification::{
    DEFAULT_VERIFICATION_SEED, SoakConfig, VerificationProfile, render_exhaustive_matrix,
    report_to_json, report_to_markdown, run_profile_with_overrides, run_soak_with_checkpoint,
    semantic_hash_only, soak_report_to_json, soak_report_to_markdown,
};

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(usage());
    };

    match command {
        "hash-corpus" => run_hash_corpus(&args[1..]),
        "report" => run_report(&args[1..]),
        "matrix" => run_matrix(&args[1..]),
        "soak" => run_soak_command(&args[1..]),
        "help" | "--help" | "-h" => Err(usage()),
        _ => Err(usage()),
    }
}

fn run_hash_corpus(args: &[String]) -> Result<(), String> {
    let seed = parse_u64(flag_value(args, "--seed").unwrap_or("0xC0FFEE42"))?;
    let cases = parse_u64(flag_value(args, "--cases").unwrap_or("65536"))?;
    println!("{}", semantic_hash_only(cases, seed));
    Ok(())
}

fn run_report(args: &[String]) -> Result<(), String> {
    let profile = VerificationProfile::from_str(flag_value(args, "--profile").unwrap_or("fast"))
        .map_err(|_| usage())?;
    let seed = parse_u64(flag_value(args, "--seed").unwrap_or("0xC0FFEE42"))?;
    let property_cases = optional_u64_flag(args, "--property-cases")?;
    let differential_cases = optional_u64_flag(args, "--differential-cases")?;
    let hash_cases = optional_u64_flag(args, "--hash-cases")?;
    let compare_label = flag_value(args, "--compare-other-label").map(ToOwned::to_owned);
    let compare_hash = flag_value(args, "--compare-other-hash").map(ToOwned::to_owned);
    let report = run_profile_with_overrides(
        profile,
        seed,
        compare_label,
        compare_hash,
        property_cases,
        differential_cases,
        hash_cases,
    );

    if let Some(path) = flag_value(args, "--json") {
        write_output(path.into(), &report_to_json(&report))?;
    }
    if let Some(path) = flag_value(args, "--md") {
        write_output(path.into(), &report_to_markdown(&report))?;
    }
    if let Some(path) = flag_value(args, "--matrix") {
        write_output(
            path.into(),
            &render_exhaustive_matrix(&report.exhaustive.rows),
        )?;
    }

    println!("{}", report_to_markdown(&report));
    Ok(())
}

fn run_matrix(args: &[String]) -> Result<(), String> {
    let profile = VerificationProfile::from_str(flag_value(args, "--profile").unwrap_or("fast"))
        .map_err(|_| usage())?;
    let seed = parse_u64(flag_value(args, "--seed").unwrap_or("0xC0FFEE42"))?;
    let property_cases = optional_u64_flag(args, "--property-cases")?;
    let differential_cases = optional_u64_flag(args, "--differential-cases")?;
    let hash_cases = optional_u64_flag(args, "--hash-cases")?;
    let report = run_profile_with_overrides(
        profile,
        seed,
        None,
        None,
        property_cases,
        differential_cases,
        hash_cases,
    );
    let matrix = render_exhaustive_matrix(&report.exhaustive.rows);
    if let Some(path) = flag_value(args, "--output") {
        write_output(path.into(), &matrix)?;
    }
    print!("{matrix}");
    Ok(())
}

fn run_soak_command(args: &[String]) -> Result<(), String> {
    let duration_seconds = parse_u64(flag_value(args, "--duration-seconds").unwrap_or("300"))?;
    let seed = parse_u64(flag_value(args, "--seed").unwrap_or("0xC0FFEE42"))?;
    let report_interval_seconds =
        parse_u64(flag_value(args, "--report-interval-seconds").unwrap_or("60"))?;
    let final_json = flag_value(args, "--json").map(PathBuf::from);
    let final_md = flag_value(args, "--md").map(PathBuf::from);
    let checkpoint_json = flag_value(args, "--checkpoint-json").map(PathBuf::from);
    let checkpoint_md = flag_value(args, "--checkpoint-md").map(PathBuf::from);

    let mut checkpoint = |report: &hollow_grove::verification::SoakReport| {
        if let Some(path) = checkpoint_json.as_ref() {
            let _ = write_output(path.clone(), &soak_report_to_json(report));
        }
        if let Some(path) = checkpoint_md.as_ref() {
            let _ = write_output(path.clone(), &soak_report_to_markdown(report));
        }
    };

    let report = run_soak_with_checkpoint(
        SoakConfig {
            duration_seconds,
            seed,
            report_interval_seconds,
        },
        &mut checkpoint,
    );

    if let Some(path) = final_json {
        write_output(path, &soak_report_to_json(&report))?;
    }
    if let Some(path) = final_md {
        write_output(path, &soak_report_to_markdown(&report))?;
    }

    println!("{}", soak_report_to_markdown(&report));
    if report.succeeded() {
        Ok(())
    } else {
        Err(report
            .failure_reasons
            .first()
            .cloned()
            .unwrap_or_else(|| "soak failed".to_string()))
    }
}

fn write_output(path: PathBuf, contents: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }
    fs::write(&path, contents)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|window| window[0] == flag)
        .map(|window| window[1].as_str())
}

fn parse_u64(value: &str) -> Result<u64, String> {
    if let Some(hex) = value.strip_prefix("0x") {
        u64::from_str_radix(hex, 16).map_err(|error| format!("invalid integer `{value}`: {error}"))
    } else {
        value
            .parse::<u64>()
            .map_err(|error| format!("invalid integer `{value}`: {error}"))
    }
}

fn optional_u64_flag(args: &[String], flag: &str) -> Result<Option<u64>, String> {
    match flag_value(args, flag) {
        Some(value) => parse_u64(value).map(Some),
        None => Ok(None),
    }
}

fn usage() -> String {
    format!(
        "usage:\n  cargo run --bin adversarial_verification -- report --profile fast --seed 0xC0FFEE --json artifacts/adversarial_verification_report.json --md artifacts/adversarial_verification_report.md --matrix artifacts/verification/exhaustive_matrix.tsv\n  cargo run --bin adversarial_verification -- hash-corpus --cases 65536 --seed 0xC0FFEE\n  cargo run --bin adversarial_verification -- matrix --profile fast --output artifacts/verification/exhaustive_matrix.tsv\n  cargo run --bin adversarial_verification -- soak --duration-seconds 300 --seed {:#x} --report-interval-seconds 60 --json artifacts/verification/soak_report.json --md artifacts/verification/soak_report.md --checkpoint-json artifacts/verification/soak_checkpoint.json --checkpoint-md artifacts/verification/soak_checkpoint.md",
        DEFAULT_VERIFICATION_SEED
    )
}
