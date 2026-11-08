//! Verify the registry in a cybercore-packs checkout: every manifest parses
//! and validates, matches its registry entry, and every resource file matches
//! its SHA-256.
//!
//!     cargo run --example verify_registry [-- <repo-root>]

use cybercore_packs::{resolve_manifest_path, PackManifest, Registry};
use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let root = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."));
    let registry = match Registry::from_path(root.join("registry/index.json")) {
        Ok(registry) => registry,
        Err(error) => {
            eprintln!("registry/index.json: {error}");
            return ExitCode::FAILURE;
        }
    };

    let mut failures = 0;
    let mut resources = 0;
    for entry in &registry.packs {
        let result =
            PackManifest::from_path(resolve_manifest_path(&root, entry)).and_then(|manifest| {
                if manifest.id != entry.id
                    || manifest.version != entry.version
                    || manifest.kind != entry.kind
                {
                    return Err(cybercore_packs::PackError::Invalid(format!(
                        "{} manifest ({} {}) does not match its registry entry ({} {})",
                        entry.id, manifest.id, manifest.version, entry.id, entry.version
                    )));
                }
                manifest.verify_resources(&root)?;
                Ok(manifest.resources.len())
            });
        match result {
            Ok(count) => {
                resources += count;
                println!("ok  {} {} ({count} resources)", entry.id, entry.version);
            }
            Err(error) => {
                failures += 1;
                eprintln!("FAIL {}: {error}", entry.id);
            }
        }
    }

    if failures > 0 {
        eprintln!("{failures} pack(s) failed verification");
        return ExitCode::FAILURE;
    }
    println!(
        "{} packs, {resources} resources verified",
        registry.packs.len()
    );
    ExitCode::SUCCESS
}
