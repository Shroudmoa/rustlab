//! L02 - Maximum und Minimum in einem Array
//!
//! Zeigt: Startwert = erstes Element, Schleife ueber den Rest,
//! Vergleich mit `max`/`min`.
//!
//! Falle: Bedingung `i > laenge` statt `i < laenge` -> Schleife laeuft nie,
//! das Ergebnis bleibt der Startwert (Semantikfehler, Compiler schweigt).
//! Erwartet: Max 553, Min 2.

use crate::registry::LessonResult;

/// Liefert `(max, min)`; `None`, wenn das Array leer ist.
pub fn max_min(werte: &[i32]) -> Option<(i32, i32)> {
    let mut rest = werte.iter().copied();
    let erstes = rest.next()?;

    let mut max = erstes;
    let mut min = erstes;

    for wert in rest {
        if wert > max {
            max = wert;
        }
        if wert < min {
            min = wert;
        }
    }

    Some((max, min))
}

pub fn run() -> LessonResult {
    let werte = [4, 6, 42, 553, 342, 2];

    println!("Werte: {werte:?}");

    match max_min(&werte) {
        Some((max, min)) => {
            println!("Max: {max}");
            println!("Min: {min}");
        }
        None => println!("Keine Werte vorhanden."),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_und_min_sind_korrekt() {
        assert_eq!(max_min(&[4, 6, 42, 553, 342, 2]), Some((553, 2)));
    }

    #[test]
    fn ein_element_ist_max_und_min() {
        assert_eq!(max_min(&[7]), Some((7, 7)));
    }

    #[test]
    fn leeres_array_hat_kein_ergebnis() {
        assert_eq!(max_min(&[]), None);
    }

    #[test]
    fn negative_werte_funktionieren() {
        assert_eq!(max_min(&[-5, -1, -9]), Some((-1, -9)));
    }
}
