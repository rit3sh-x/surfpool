//! AUTO-GENERATED — DO NOT EDIT BY HAND.
//!
//! This file is overwritten by `crates/types/src/bin/update_mainnet_features.rs`,
//! scheduled weekly via `.github/workflows/update-mainnet-features.yml`.
//! To regenerate manually, run:
//!
//! ```text
//! cargo run -p surfpool-types --bin update_mainnet_features \
//!     --features update-features-tool
//! ```
//!
//! Last updated: 2026-05-12 (queried from mainnet RPC)

use solana_pubkey::Pubkey;

/// Feature gates that are NOT yet active on Solana mainnet-beta and must
/// therefore be disabled in the local SVM to match mainnet behavior.
pub const MAINNET_DISABLED_FEATURES: &[Pubkey] = &[
    // Deprecate legacy vote instructions
    Pubkey::from_str_const("depVvnQ2UysGrhwdiwU42tCadZL8GcBb1i2GYhMopQv"),
    // Enable ExtendProgramChecked instruction
    Pubkey::from_str_const("2oMRZEDWT2tqtYMofhmmfQ8SsjqUFzT6sYXppQDavxwz"),
    // Enable zk token proof program to read proof from accounts instead of instruction data #34750
    Pubkey::from_str_const("zkiTNuzBKxrCLMKehzuQeKZyLtX2yvFcEKMML8nExU8"),
    // Raise minimum stake delegation to 1.0 SOL #24357
    Pubkey::from_str_const("9onWzzvCzNC2jfhxxeqRgs5q7nFAAKpCUvkj6T6GJK9i"),
    // Re-enables execution of SBPFv0 programs
    Pubkey::from_str_const("TestFeature21111111111111111111111111111111"),
    // Re-enables zk-elgamal-proof program
    Pubkey::from_str_const("zkesAyFB19sTkX8i9ReoKaMNDA4YNTPYJpZKPDt7FMW"),
    // SIMD-0161: Disables execution of SBPFv0 programs
    Pubkey::from_str_const("TestFeature11111111111111111111111111111111"),
    // SIMD-0167: Enable Loader-v4
    Pubkey::from_str_const("2aQJYqER2aKyb3cZw22v4SL2xMX7vwXBRWfvS4pTrtED"),
    // SIMD-0178, SIMD-0179 and SIMD-0189: Enable deployment and execution of SBPFv3 programs
    Pubkey::from_str_const("BUwGLeF3Lxyfv1J1wY8biFHBB2hrk2QhbNftQf3VV3cC"),
    // SIMD-0204: Slashable event verification
    Pubkey::from_str_const("sProgVaNWkYdP2eTRAy1CPrgb3b9p8yXCASrPEqo6VJ"),
    // SIMD-0219: Stricter ABI and Runtime Constraints
    Pubkey::from_str_const("Eoh7e1sDqtyPtuiWAhBNSJinvtJWTTDgeUMRi3RF8zWS"),
    // SIMD-0266: Efficient Token program
    Pubkey::from_str_const("ptokFjwyJtrwCa9Kgo9xoDS59V4QccBGEaRFnRPnSdP"),
    // SIMD-0268: Raise CPI nesting limit from 4 to 8
    Pubkey::from_str_const("6TkHkRmP7JZy1fdM6fg5uXn76wChQBWGokHBJzrLB3mj"),
    // SIMD-0286: Raise block limit to 100M
    Pubkey::from_str_const("P1BCUMpAC7V2GRBRiJCNUgpMyWZhoqt3LKo712ePqsz"),
    // SIMD-0326: Alpenglow: new consensus algorithm
    Pubkey::from_str_const("mustRekeyVm2QHYB3JPefBiU4BY3Z6JkW2k3Scw5GWP"),
    // SIMD-0337: Markers for Alpenglow Fast Leader Handover, DATA_COMPLETE_SHRED placement rules
    Pubkey::from_str_const("8MhfKhoZEoiySpVe248bDkisyEcBA7JQLyUS94xoTSqN"),
    // Verify retransmitter signature #1840
    Pubkey::from_str_const("51VCKU5eV6mcTc9q9ArfWELU2CqDoi13hdAjr6fHMdtv"),
    // add big_mod_exp syscall #28503
    Pubkey::from_str_const("EBq48m8irRKuE7ZnMTLvLg2UuGSqhe8s8oMqnmja1fJw"),
    // blake3 syscall
    Pubkey::from_str_const("HTW2pSyErTj4BV6KBM9NZ9VBUJVxt7sacNWcf76wtzb3"),
    // enable Zk Token proof program and syscalls
    Pubkey::from_str_const("zk1snxsc6Fh3wsGNbbHAJNHiJoYgF29mMnTSusGx5EJ"),
    // enable Zk Token proof program transfer with fee
    Pubkey::from_str_const("zkNLP7EQALfC1TYeB3biDU7akDckj8iPkvh9y2Mt2K3"),
    // enable account data direct mapping
    Pubkey::from_str_const("6f2qai82RU7Dutj1WJfRzLJKYA36QWvTa89CR1imgj7N"),
    // enable the remaining_compute_units syscall
    Pubkey::from_str_const("5TuppMutoyzhUSfuYdhgzD47F92GL1g89KpCZQKqedxP"),
    // enable turbine extended fanout experiments #
    Pubkey::from_str_const("turbRpTzBzDU6PJmWvRTbcJXXGxUs19CvQamUrRD9bN"),
    // fail libsecp256k1_verify if count appears wrong
    Pubkey::from_str_const("8aXvSuopd1PUj7UhehfXJRg6619RHp8ZvwTyyJHdUYsj"),
    // full inflation on devnet and testnet
    Pubkey::from_str_const("DT4n6ABDqs6w4bnfwrXT9rsprcPf6cdDga1egctaPkLC"),
    // generate duplicate proofs for chained merkle root conflicts
    Pubkey::from_str_const("chaie9S2zVfuxJKNRGkyTDokLwWxx6kD2ZLsqQHaDD8"),
    // include transaction loaded accounts data size in base fee calculation #30657
    Pubkey::from_str_const("EaQpmC6GtRssaZ3PCUM5YksGqUdMLeZ46BQXYtHYakDS"),
    // increase tx account lock limit to 128 #27241
    Pubkey::from_str_const("9LZdXeKGeBV6hRLdxS1rHbHoEUsKqesCC2ZAPTPKJAbK"),
    // stakes must be at least the minimum delegation to earn rewards
    Pubkey::from_str_const("G6ANXD6ptCSyNd9znZm7j4dEczAJCfx7Cy43oBx3rKHJ"),
    // vote only on retransmitter signed fec sets
    Pubkey::from_str_const("RfEcA95xnhuwooVAhUUksEJLZBF7xKCLuqrJoqk4Zph"),
];
