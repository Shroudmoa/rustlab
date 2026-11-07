//! Zentrale Registrierung aller Lektionen.
//!
//! Neue Lektion anlegen:
//! 1. Modul in `src/lessons/` erstellen
//! 2. `pub mod ...;` in `src/lessons/mod.rs` ergaenzen
//! 3. Einen Eintrag in [`LESSONS`] ergaenzen
//!
//! Menue, `--list` und die Integrationstests ziehen automatisch mit.

use crate::lessons;

/// Ergebnis einer Lektion. `Box<dyn Error>` erlaubt `?` auf alle Fehlertypen.
pub type LessonResult = Result<(), Box<dyn std::error::Error>>;

/// Eine Lektion im Menue.
pub struct Lesson {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub run: fn() -> LessonResult,
}

/// Alle Lektionen in Menue-Reihenfolge.
pub const LESSONS: &[Lesson] = &[
    Lesson {
        id: "0",
        title: "Sandbox",
        description: "Freie Spielwiese zum Ausprobieren",
        run: lessons::l00_sandbox::run,
    },
    Lesson {
        id: "1",
        title: "Mittelwert berechnen",
        description: "Array, Schleife, Summe, Division mit as f64",
        run: lessons::l01_mittelwert::run,
    },
    Lesson {
        id: "2",
        title: "Max und Min im Array",
        description: "Startwert = erstes Element, Schleife ab Index 1",
        run: lessons::l02_max_min::run,
    },
    Lesson {
        id: "3",
        title: "Passwort-Check",
        description: "String einlesen, trimmen, Zeichen zaehlen",
        run: lessons::l03_passwort::run,
    },
    Lesson {
        id: "4",
        title: "Begruessung",
        description: "ASCII-Banner, Name einlesen, format!",
        run: lessons::l04_begruessung::run,
    },
];

/// Sucht eine Lektion anhand ihrer ID.
pub fn find(id: &str) -> Option<&'static Lesson> {
    LESSONS.iter().find(|lesson| lesson.id == id)
}
