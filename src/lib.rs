//! rustlab - kleine, testbare Rust-Lektionen.
//!
//! Jede Lektion liegt in [`lessons`] und ist in [`registry::LESSONS`]
//! eingetragen. Die Logik ist als reine Funktion umgesetzt und wird mit
//! `cargo test` geprueft; die `run`-Funktion macht nur die Ein-/Ausgabe.

pub mod lessons;
pub mod registry;
