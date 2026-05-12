//! Weekly auto-updater for the mainnet feature disable list.
//!
//! Queries mainnet-beta to find which agave feature gates are inactive there,
//! then overwrites `crates/types/src/mainnet_disabled_features.rs` with the
//! refreshed list. Intended to be invoked by the scheduled GitHub Actions
//! workflow `.github/workflows/update-mainnet-features.yml`.
//!
//! Run locally with:
//!
//! ```text
//! cargo run -p surfpool-types --bin update_mainnet_features \
//!     --features update-features-tool
//! ```
//!
//! Exits 0 in all success cases; the workflow uses `git diff --quiet` on the
//! generated file to decide whether to open a PR.

use std::{fs, path::PathBuf, time::Duration};

use agave_feature_set::FEATURE_NAMES;
use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;

const MAINNET_RPC: &str = "https://api.mainnet-beta.solana.com";
const GENERATED_FILE: &str = "src/mainnet_disabled_features.rs";
const BATCH_SIZE: usize = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GENERATED_FILE);
    println!("Target file: {}", target.display());
    println!("Querying mainnet RPC: {MAINNET_RPC}");

    let client = RpcClient::new_with_timeout(MAINNET_RPC.to_string(), Duration::from_secs(60));

    // Stable ordering: sort by the human-readable description so reruns
    // produce the same byte-for-byte output.
    let mut entries: Vec<(String, Pubkey)> = FEATURE_NAMES
        .iter()
        .map(|(pk, name)| ((*name).to_string(), *pk))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut inactive: Vec<(String, Pubkey)> = Vec::new();
    for chunk in entries.chunks(BATCH_SIZE) {
        let keys: Vec<Pubkey> = chunk.iter().map(|(_, pk)| *pk).collect();
        let accounts = client.get_multiple_accounts(&keys)?;
        for ((name, pk), account) in chunk.iter().zip(accounts.into_iter()) {
            if !is_activated(account.as_ref()) {
                inactive.push((name.clone(), *pk));
            }
        }
    }

    println!(
        "Mainnet has {} active features and {} inactive (will be disabled).",
        entries.len() - inactive.len(),
        inactive.len()
    );

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let rendered = render_file(&inactive, &today);

    let original = fs::read_to_string(&target).unwrap_or_default();
    if rendered == original {
        println!("No drift — {} already up to date.", target.display());
    } else {
        fs::write(&target, &rendered)?;
        println!(
            "Wrote {} ({} inactive features)",
            target.display(),
            inactive.len()
        );
    }
    Ok(())
}

/// A feature account on mainnet is bincode-serialized `Option<Slot>`:
/// `data[0]` is the variant tag — `1` means `Some(activated_slot)`,
/// `0` (or empty data) means inactive.
fn is_activated(account: Option<&solana_account::Account>) -> bool {
    match account {
        Some(acc) => !acc.data.is_empty() && acc.data[0] == 1,
        None => false,
    }
}

/// Renders the entire contents of `mainnet_disabled_features.rs`. The binary
/// owns 100% of this file — there are no hand-edited regions to preserve.
fn render_file(features: &[(String, Pubkey)], today_iso: &str) -> String {
    let mut out = String::new();
    out.push_str("//! AUTO-GENERATED — DO NOT EDIT BY HAND.\n");
    out.push_str("//!\n");
    out.push_str("//! This file is overwritten by `crates/types/src/bin/update_mainnet_features.rs`,\n");
    out.push_str("//! scheduled weekly via `.github/workflows/update-mainnet-features.yml`.\n");
    out.push_str("//! To regenerate manually, run:\n");
    out.push_str("//!\n");
    out.push_str("//! ```text\n");
    out.push_str("//! cargo run -p surfpool-types --bin update_mainnet_features \\\n");
    out.push_str("//!     --features update-features-tool\n");
    out.push_str("//! ```\n");
    out.push_str("//!\n");
    out.push_str(&format!(
        "//! Last updated: {today_iso} (queried from mainnet RPC)\n"
    ));
    out.push('\n');
    out.push_str("use solana_pubkey::Pubkey;\n");
    out.push('\n');
    out.push_str("/// Feature gates that are NOT yet active on Solana mainnet-beta and must\n");
    out.push_str("/// therefore be disabled in the local SVM to match mainnet behavior.\n");
    out.push_str("pub const MAINNET_DISABLED_FEATURES: &[Pubkey] = &[\n");
    for (name, pk) in features {
        out.push_str(&format!("    // {name}\n"));
        out.push_str(&format!("    Pubkey::from_str_const(\"{pk}\"),\n"));
    }
    out.push_str("];\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_file_emits_header_and_entries() {
        let pk = Pubkey::new_from_array([1u8; 32]);
        let out = render_file(&[("Test feature".to_string(), pk)], "2026-05-11");

        assert!(out.starts_with("//! AUTO-GENERATED"));
        assert!(out.contains("Last updated: 2026-05-11"));
        assert!(out.contains("use solana_pubkey::Pubkey;"));
        assert!(out.contains("pub const MAINNET_DISABLED_FEATURES: &[Pubkey] = &["));
        assert!(out.contains("    // Test feature\n"));
        assert!(out.contains(&format!("    Pubkey::from_str_const(\"{pk}\"),\n")));
        assert!(out.trim_end().ends_with("];"));
    }

    #[test]
    fn render_file_handles_empty_list() {
        let out = render_file(&[], "2026-05-11");
        assert!(out.contains("pub const MAINNET_DISABLED_FEATURES: &[Pubkey] = &[\n];\n"));
        assert!(!out.contains("Pubkey::from_str_const"));
    }

    #[test]
    fn render_file_is_idempotent() {
        let pk = Pubkey::new_from_array([2u8; 32]);
        let features = vec![("A".to_string(), pk), ("B".to_string(), pk)];
        let once = render_file(&features, "2026-05-11");
        let twice = render_file(&features, "2026-05-11");
        assert_eq!(once, twice);
    }
}
