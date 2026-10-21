//! VegieBreakDAW application entry point.
//!
//! Phase 0 is intentionally headless. It provides diagnostics, project creation,
//! and project inspection so the model and toolchain can be verified before the
//! UI and audio device layers land.

use std::error::Error;
use std::path::PathBuf;

use vb_audio::{AudioBackend, AudioConfig, NullBackend};
use vb_core::{PROJECT_SCHEMA_VERSION, Project};
use vb_host::{HostCapabilities, PluginFormat};
use vb_stems::{DEFAULT_MODEL, DemucsOnnxSeparator};

const APP_NAME: &str = "VegieBreakDAW";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    if let Err(error) = run(std::env::args().skip(1).collect()) {
        eprintln!("{APP_NAME}: {error}");
        std::process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    match args.first().map(String::as_str) {
        None | Some("--help" | "-h" | "help") => {
            print_help();
            Ok(())
        }
        Some("--version" | "-V" | "version") => {
            println!("{APP_NAME} {VERSION}");
            Ok(())
        }
        Some("--doctor") => doctor(),
        Some("--new") => {
            let path = args.get(1).ok_or("missing project path for --new")?;
            create_project(PathBuf::from(path))
        }
        Some("--inspect") => {
            let path = args.get(1).ok_or("missing project path for --inspect")?;
            inspect_project(PathBuf::from(path))
        }
        Some(command) => Err(format!("unknown command: {command}").into()),
    }
}

fn print_help() {
    println!(
        "{APP_NAME} {VERSION}
A clean-room, open-source, FL-inspired DAW written in Rust.

USAGE:
    vegiebreak [COMMAND]

COMMANDS:
    --doctor              Print environment and subsystem readiness
    --new <path>          Create and save an empty .vbproj
    --inspect <path>      Load and summarize a .vbproj
    --version             Print the version
    --help                Print this help

No account, telemetry, or network access is used by these commands."
    );
}

fn doctor() -> Result<(), Box<dyn Error>> {
    let config = AudioConfig::DEFAULT;
    config.validate()?;
    let backend = NullBackend::new(config);
    let capabilities = HostCapabilities::default();
    let stems = DemucsOnnxSeparator::new(None);

    println!("{APP_NAME} {VERSION} diagnostics");
    println!("  os:                 {}", std::env::consts::OS);
    println!("  arch:               {}", std::env::consts::ARCH);
    println!("  project schema:     {PROJECT_SCHEMA_VERSION}");
    println!(
        "  audio config:       {} Hz, {} ch, {} frames",
        config.sample_rate, config.channels, config.block_size
    );
    println!("  audio backend:      {} (lifecycle ready)", backend.name());
    println!(
        "  block duration:     {:.3} ms",
        config.block_duration_seconds() * 1000.0
    );
    println!(
        "  plugin formats:     {}",
        capabilities
            .formats
            .iter()
            .map(|format| format.label())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "  plugin isolation:   {}",
        if capabilities.out_of_process {
            "out-of-process"
        } else {
            "not wired yet"
        }
    );
    println!(
        "  AAX:                {}",
        if PluginFormat::Aax.is_distributable() {
            "built in"
        } else {
            "optional; Avid SDK required"
        }
    );
    println!(
        "  stem model:         {} {} ({})",
        DEFAULT_MODEL.name, DEFAULT_MODEL.version, DEFAULT_MODEL.license
    );
    println!(
        "  stem backend:       {}",
        if stems.model_path().is_some() {
            "model configured"
        } else {
            "interface only; model not configured"
        }
    );
    println!("  account required:   no");
    println!("  telemetry:          none");
    Ok(())
}

fn create_project(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let name = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .unwrap_or("Untitled");
    let project = Project::new(name);
    project.save_to_path(&path)?;
    println!("created {} ({})", path.display(), project.name);
    Ok(())
}

fn inspect_project(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let project = Project::load_from_path(&path)?;
    println!("project:      {}", project.name);
    println!("path:         {}", path.display());
    println!("schema:       {}", project.schema_version);
    println!("sample rate:  {} Hz", project.sample_rate);
    println!("tempo:        {:.2} BPM", project.tempo_bpm);
    println!(
        "time sig:     {}/{}",
        project.time_signature.numerator, project.time_signature.denominator
    );
    println!("tracks:       {}", project.tracks.len());
    println!("patterns:     {}", project.patterns.len());
    println!("mixer chans:  {}", project.mixer.len());
    println!(
        "length:       {:.2} beats",
        project.arrangement_length_beats()
    );
    Ok(())
}
