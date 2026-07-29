use std::process::Command;

use hollow_grove::runtime_federation::{
    HOLLOW_GROVE_OLLAMA_MODEL_SCHEMA_VERSION, render_hollow_grove_ollama_modelfile,
    validate_ollama_model_reference,
};

#[test]
fn modelfile_is_deterministic_and_uses_the_selected_base_model() {
    let first = render_hollow_grove_ollama_modelfile("gemma3:4b").unwrap();
    let second = render_hollow_grove_ollama_modelfile("gemma3:4b").unwrap();

    assert_eq!(first, second);
    assert!(first.starts_with(&format!("# {HOLLOW_GROVE_OLLAMA_MODEL_SCHEMA_VERSION}\n")));
    assert!(first.contains("FROM gemma3:4b\n"));
    assert!(first.contains("PARAMETER temperature 0\n"));
    assert!(first.ends_with("\"\"\"\n"));
}

#[test]
fn modelfile_preserves_the_proposal_only_constitution() {
    let modelfile = render_hollow_grove_ollama_modelfile("llama3.2").unwrap();

    for required in [
        "Stonebend / Hollow Grove:",
        "[0, 0, 1000]",
        "Glaüshouse / Grove Hollow:",
        "[0, 0, -1000]",
        "The Y axis is reserved.",
        "\"proposed_position\"",
        "\"confidence_millis\"",
        "untrusted Aura proposal",
        "Only Rust Federation and Festival may validate",
    ] {
        assert!(
            modelfile.contains(required),
            "missing Modelfile law: {required}"
        );
    }
}

#[test]
fn model_references_are_strict_and_cannot_inject_modelfile_lines() {
    for valid in [
        "llama3.2",
        "gemma3:4b",
        "registry.example/team/model:latest",
        "hollow-grove",
    ] {
        validate_ollama_model_reference(valid).unwrap();
    }

    for invalid in [
        "",
        "-flag",
        "model name",
        "model\nSYSTEM unsafe",
        "../model",
        "team//model",
        "/absolute",
    ] {
        assert!(validate_ollama_model_reference(invalid).is_err());
        assert!(render_hollow_grove_ollama_modelfile(invalid).is_err());
    }
}

#[test]
fn semantic_cli_prints_a_ready_to_create_modelfile() {
    let output = Command::new(env!("CARGO_BIN_EXE_hollow_grove_semantic"))
        .args(["model", "show-modelfile", "gemma3:4b"])
        .output()
        .unwrap();

    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("FROM gemma3:4b"));
    assert!(stdout.contains("Status") || stdout.contains("proposal"));
    assert!(output.stderr.is_empty());
}
