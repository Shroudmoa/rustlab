//! Integrationstests fuer das fertige Binary.
//!
//! Sie pruefen, was Unit-Tests nicht sehen: Startet die CLI, findet sie alle
//! Lektionen, laufen alle Lektionen fehlerfrei durch?

use std::io::Write;
use std::process::{Command, Stdio};

fn binary() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rustlab"));
    cmd.stdin(Stdio::null());
    cmd
}

#[test]
fn listet_alle_lektionen() {
    let ausgabe = binary().arg("--list").output().expect("Binary startet");
    assert!(ausgabe.status.success());

    let stdout = String::from_utf8_lossy(&ausgabe.stdout);
    for lektion in rustlab::registry::LESSONS {
        assert!(
            stdout.contains(lektion.title),
            "Titel fehlt in --list: {}",
            lektion.title
        );
    }
}

#[test]
fn jede_lektion_laeuft_ohne_fehler() {
    for lektion in rustlab::registry::LESSONS {
        let ausgabe = binary().arg(lektion.id).output().expect("Binary startet");

        assert!(
            ausgabe.status.success(),
            "Lektion {} ({}) ist fehlgeschlagen: {}",
            lektion.id,
            lektion.title,
            String::from_utf8_lossy(&ausgabe.stderr)
        );
    }
}

#[test]
fn unbekannte_lektion_ist_ein_fehler() {
    let ausgabe = binary()
        .arg("gibt-es-nicht")
        .output()
        .expect("Binary startet");

    assert!(!ausgabe.status.success());
    assert!(String::from_utf8_lossy(&ausgabe.stderr).contains("Unbekannte Lektion"));
}

#[test]
fn menue_beendet_mit_enter() {
    let mut kind = Command::new(env!("CARGO_BIN_EXE_rustlab"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Binary startet");

    kind.stdin
        .as_mut()
        .expect("stdin offen")
        .write_all(b"\n")
        .expect("Eingabe schreiben");

    let ausgabe = kind.wait_with_output().expect("Binary endet");
    assert!(ausgabe.status.success());
    assert!(String::from_utf8_lossy(&ausgabe.stdout).contains("Ende."));
}

#[test]
fn menue_startet_eine_lektion() {
    let mut kind = Command::new(env!("CARGO_BIN_EXE_rustlab"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Binary startet");

    // Lektion 1 starten, danach Enter zum Beenden.
    kind.stdin
        .as_mut()
        .expect("stdin offen")
        .write_all(b"1\n\n")
        .expect("Eingabe schreiben");

    let ausgabe = kind.wait_with_output().expect("Binary endet");
    assert!(ausgabe.status.success());
    assert!(String::from_utf8_lossy(&ausgabe.stdout).contains("Mittelwert:"));
}
