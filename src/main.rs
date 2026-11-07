//! CLI: interaktives Menue oder direkter Lektionsstart.
//!
//! `cargo run`              -> Menue
//! `cargo run -- 1`         -> Lektion 1
//! `cargo run -- --list`    -> alle Lektionen
//! `cargo run -- --help`    -> Hilfe

use std::env;
use std::io::{self, Write};
use std::process::ExitCode;

use rustlab::registry::{self, LESSONS};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return match menue() {
            Ok(()) => ExitCode::SUCCESS,
            Err(fehler) => {
                eprintln!("Fehler: {fehler}");
                ExitCode::FAILURE
            }
        };
    }

    match args[0].as_str() {
        "-h" | "--help" | "help" => {
            hilfe();
            ExitCode::SUCCESS
        }
        "-l" | "--list" | "list" => {
            liste();
            ExitCode::SUCCESS
        }
        id => match registry::find(id) {
            Some(lektion) => match (lektion.run)() {
                Ok(()) => ExitCode::SUCCESS,
                Err(fehler) => {
                    eprintln!("Fehler in Lektion {}: {fehler}", lektion.id);
                    ExitCode::FAILURE
                }
            },
            None => {
                eprintln!("Unbekannte Lektion: {id}");
                eprintln!("Verfuegbar: cargo run -- --list");
                ExitCode::FAILURE
            }
        },
    }
}

fn menue() -> Result<(), Box<dyn std::error::Error>> {
    println!("rustlab {}", env!("CARGO_PKG_VERSION"));

    loop {
        println!();
        liste();
        print!("Auswahl (Enter = Ende): ");
        io::stdout().flush()?;

        let mut eingabe = String::new();
        if io::stdin().read_line(&mut eingabe)? == 0 {
            break; // EOF, z. B. bei umgeleiteter Eingabe
        }

        let eingabe = eingabe.trim();
        if eingabe.is_empty() || eingabe.eq_ignore_ascii_case("q") {
            break;
        }

        match registry::find(eingabe) {
            Some(lektion) => {
                println!("\n--- {} - {} ---", lektion.id, lektion.title);
                if let Err(fehler) = (lektion.run)() {
                    eprintln!("Fehler: {fehler}");
                }
            }
            None => println!("Unbekannte Auswahl: {eingabe}"),
        }
    }

    println!("Ende.");
    Ok(())
}

fn liste() {
    let id_breite = LESSONS.iter().map(|l| l.id.len()).max().unwrap_or(1).max(2);
    let titel_breite = LESSONS
        .iter()
        .map(|l| l.title.chars().count())
        .max()
        .unwrap_or(5)
        .max(5);

    for lektion in LESSONS {
        println!(
            "{:>id_breite$}) {:<titel_breite$}  {}",
            lektion.id, lektion.title, lektion.description
        );
    }
}

fn hilfe() {
    println!("rustlab {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Aufruf:");
    println!("  cargo run                 interaktives Menue");
    println!("  cargo run -- <id>         eine Lektion direkt starten");
    println!("  cargo run -- --list       alle Lektionen anzeigen");
    println!("  cargo run -- --help       diese Hilfe");
    println!();
    println!("Pruefen:");
    println!("  cargo test                Unit- + Integrationstests");
    println!("  cargo clippy --all-targets -- -D warnings");
    println!("  cargo fmt --all -- --check");
}
