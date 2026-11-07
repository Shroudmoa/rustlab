//! L03 - Passwort-Check
//!
//! Zeigt: `String` einlesen, `trim()` (entfernt auch das Enter-Zeichen),
//! Zeichen zaehlen mit `chars().count()`.
//!
//! Falle: `.len()` zaehlt Bytes, nicht Zeichen. "aeoeue" mit Umlauten ist
//! laenger als es aussieht - fuer Menschen zaehlen Zeichen.

use std::io;

use crate::registry::LessonResult;

/// Mindestlaenge in Zeichen. Ein Passwort muss echt laenger sein.
pub const MINDESTLAENGE: usize = 10;

/// Anzahl der Zeichen ohne umgebende Leerzeichen/Zeilenumbrueche.
pub fn laenge(passwort: &str) -> usize {
    passwort.trim().chars().count()
}

/// `true`, wenn das Passwort lang genug ist.
pub fn ist_sicher(passwort: &str) -> bool {
    laenge(passwort) > MINDESTLAENGE
}

pub fn run() -> LessonResult {
    println!("Passwort eingeben (mehr als {MINDESTLAENGE} Zeichen):");

    let mut passwort = String::new();
    io::stdin().read_line(&mut passwort)?;

    let anzahl = laenge(&passwort);
    if ist_sicher(&passwort) {
        println!("OK - {anzahl} Zeichen.");
    } else {
        println!("Zu kurz - {anzahl} Zeichen.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genau_mindestlaenge_ist_zu_kurz() {
        assert!(!ist_sicher(&"a".repeat(MINDESTLAENGE)));
    }

    #[test]
    fn ein_zeichen_mehr_ist_ok() {
        assert!(ist_sicher(&"a".repeat(MINDESTLAENGE + 1)));
    }

    #[test]
    fn whitespace_zaehlt_nicht() {
        assert_eq!(laenge("  geheim\n"), 6);
    }

    #[test]
    fn umlaute_zaehlen_als_ein_zeichen() {
        // 3 Zeichen, aber 6 Bytes in UTF-8.
        assert_eq!(laenge("äöü"), 3);
        assert_eq!("äöü".len(), 6);
    }
}
