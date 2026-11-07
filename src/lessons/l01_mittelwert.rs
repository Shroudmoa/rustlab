//! L01 - Mittelwert eines Arrays
//!
//! Zeigt: Slice, `for`-Schleife, Summe, Division mit `as f64`.
//! Erwartet: Summe 127, Mittelwert 21.1666...
//!
//! Falle: `summe / laenge` mit `i32` rechnet ganzzahlig (127 / 6 = 21).
//! Der Compiler schweigt - das ist ein Semantikfehler.

use crate::registry::LessonResult;

/// Summe aller Werte.
pub fn summe(werte: &[i32]) -> i32 {
    werte.iter().sum()
}

/// Arithmetisches Mittel; `None`, wenn das Array leer ist.
pub fn mittelwert(werte: &[i32]) -> Option<f64> {
    if werte.is_empty() {
        return None;
    }

    // Cast VOR der Division, sonst rechnet Rust ganzzahlig.
    Some(summe(werte) as f64 / werte.len() as f64)
}

pub fn run() -> LessonResult {
    let cpuload = [23, 34, 4, 22, 33, 11];

    println!("Werte:      {cpuload:?}");
    println!("Summe:      {}", summe(&cpuload));

    match mittelwert(&cpuload) {
        Some(wert) => println!("Mittelwert: {wert:.4}"),
        None => println!("Mittelwert: keine Werte"),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summe_der_werte() {
        assert_eq!(summe(&[23, 34, 4, 22, 33, 11]), 127);
    }

    #[test]
    fn mittelwert_ist_korrekt() {
        let wert = mittelwert(&[23, 34, 4, 22, 33, 11]).expect("Werte vorhanden");
        assert!((wert - 127.0 / 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn leeres_array_hat_kein_mittel() {
        assert_eq!(mittelwert(&[]), None);
    }

    #[test]
    fn ganzzahlige_division_waere_falsch() {
        // 127 / 6 ist in i32 gleich 21 - deshalb der Cast vor der Division.
        assert_eq!(127 / 6, 21);
        assert!((mittelwert(&[23, 34, 4, 22, 33, 11]).unwrap() - 21.1666).abs() < 0.001);
    }
}
