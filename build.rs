//! Auto-sync `assets/profile.toml` from a sibling `../resume` checkout.
//!
//! Layout on the author's machine (and the documented setup) is:
//!   ~/Projects/resume/                  <- source of truth (resume.toml)
//!   ~/Projects/GitanElyon.github.io/    <- this repo
//! When that sibling checkout exists, a fresh `resume.toml` is copied over
//! the vendored `assets/profile.toml` before compilation, so updating the
//! resume repo updates this site on the next build with no manual step.
//! In CI (or anywhere without the sibling) the checked-in vendored copy is
//! used as-is. This script never fails the build.

use std::path::Path;

fn mtime(path: &Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
    let manifest = Path::new(&manifest_dir);
    let vendored = manifest.join("assets/profile.toml");
    let sibling = manifest.join("../resume/resume.toml");

    println!("cargo:rerun-if-changed=assets/profile.toml");
    println!("cargo:rerun-if-changed=../resume/resume.toml");
    println!("cargo:rerun-if-changed=build.rs");

    if let Ok(canonical) = sibling.canonicalize() {
        let source_newer = match (mtime(&canonical), mtime(&vendored)) {
            (Some(s), Some(v)) => s > v,
            (Some(_), None) => true,
            _ => false,
        };
        // Also copy when contents differ even if mtimes tie (e.g. git checkout).
        let contents_differ = match (std::fs::read(&canonical), std::fs::read(&vendored)) {
            (Ok(a), Ok(b)) => a != b,
            (Ok(_), Err(_)) => true,
            _ => false,
        };
        if source_newer || contents_differ {
            match std::fs::read(&canonical) {
                Ok(bytes) => {
                    if let Err(e) = std::fs::write(&vendored, &bytes) {
                        println!(
                            "cargo:warning=profile sync: cannot write {}: {e}",
                            vendored.display()
                        );
                    } else {
                        println!(
                            "cargo:warning=profile sync: updated assets/profile.toml from {}",
                            canonical.display()
                        );
                    }
                }
                Err(e) => {
                    println!(
                        "cargo:warning=profile sync: cannot read {}: {e}",
                        canonical.display()
                    );
                }
            }
        }
    }
}
