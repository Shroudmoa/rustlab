//! L00 - Sandbox
//!
//! Freie Spielwiese: hier darf alles rein, was du gerade ausprobierst.
//! Wird ueber das Menue mit "0" gestartet.
//!
//! Zeigt: Typen sind in Rust streng - `u8` passt nicht automatisch in `i32`,
//! deshalb bewusst mit `i32::from` umwandeln.

use crate::registry::LessonResult;

/// Addiert zwei Zahlen.
pub fn summe(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run() -> LessonResult {
    println!("Sandbox - hier experimentieren.");

    let summe_wert: i32 = 23;
    let byte_wert: u8 = 32;

    // Kein impliziter Cast: u8 -> i32 muss ausdruecklich passieren.
    let x = summe(summe_wert, i32::from(byte_wert));

    println!("x = {x}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summe_ist_korrekt() {
        assert_eq!(summe(23, 32), 55);
    }
}
