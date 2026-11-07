# Rustlab

Kleine, testbare Rust-Lektionen - analog zum C#-Projekt, aber mit Tests,
einer zentralen Registry und einer CLI, die auch ohne Menue funktioniert.

## Struktur

```
src/main.rs              CLI + interaktives Menue (nur Ein-/Ausgabe)
src/lib.rs               Library-Wurzel (damit Tests den Code importieren koennen)
src/registry.rs          Lesson-Struct + LESSONS-Tabelle
src/lessons/mod.rs       Liste aller Lektionsmodule
src/lessons/lXX_*.rs     eine Datei pro Lektion
tests/cli.rs             Integrationstests fuer das fertige Binary
README.md                Notizen / Theorie
```

Jede Lektion hat zwei Teile:

- **reine Funktionen** (z. B. `mittelwert(&[i32]) -> Option<f64>`) - testbar, ohne I/O
- **`run()`** - nur Eingabe/Ausgabe, ruft die reinen Funktionen auf

Dadurch kann `cargo test` die Logik pruefen, ohne dass jemand Tippen muss.

## Starten

```bash
cargo run              # interaktives Menue
cargo run -- 1         # Lektion 1 direkt starten
cargo run -- --list    # alle Lektionen anzeigen
cargo run -- --help    # Hilfe
```

Im Menue beendet **Enter** (oder `q`) das Programm.

## Pruefen

```bash
cargo test                          # Unit- + Integrationstests
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

`cargo build` prueft nur Syntax und Typen. `cargo test` prueft zusaetzlich das
**Verhalten** - also genau die Semantikfehler, die der Compiler nicht sieht.

## Neue Lektion anlegen

1. Datei `src/lessons/l05_thema.rs` erstellen:

   ```rust
   //! L05 - Thema
   //!
   //! Zeigt: ...
   //! Erwartet: ...

   use crate::registry::LessonResult;

   pub fn berechne(werte: &[i32]) -> i32 {
       werte.iter().sum()
   }

   pub fn run() -> LessonResult {
       println!("{}", berechne(&[1, 2, 3]));
       Ok(())
   }

   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn summe_ist_korrekt() {
           assert_eq!(berechne(&[1, 2, 3]), 6);
       }
   }
   ```

2. In `src/lessons/mod.rs` ergaenzen: `pub mod l05_thema;`
3. In `src/registry.rs` einen Eintrag in `LESSONS` ergaenzen.
4. `cargo test`, `cargo run -- 5`, Ergebnis mit "Erwartet" vergleichen.
5. Commit: `L05: Thema`.

Die Registry ist die einzige Stelle, an der eine Lektion "angemeldet" wird -
Menue, `--list` und die Integrationstests ziehen automatisch mit.

## IHK

- **Syntaxfehler:** Verstoss gegen die Grammatik; der Compiler meldet den Fehler.
  Beispiel: `let x: i32 = "Hallo";` (E0308) oder fehlendes Semikolon.
- **Semantikfehler:** Code kompiliert, rechnet aber falsch. Beispiel:
  `summe / laenge` mit `i32` statt `summe as f64 / laenge as f64` -
  ganzzahlige Division, der Compiler schweigt.

## Gemerkte Fehler

| Fehler | Ursache | Loesung |
|---|---|---|
| `E0425` cannot find value | Variable nie deklariert oder Tippfehler | deklarieren / Schreibweise pruefen |
| `E0308` mismatched types | z. B. `String` statt `&str`, `i32` statt `f64` | Typ anpassen, `&`, `as`, `parse()` |
| `E0382` use of moved value | Wert wurde verschoben (z. B. `String` an Funktion gegeben) | `&` borgen oder `.clone()` |
| `E0502` cannot borrow as mutable | gleichzeitig `&` und `&mut` auf denselben Wert | Borrow beenden, Reihenfolge aendern |
| `E0599` no method named ... | Methode existiert fuer den Typ nicht | Typ pruefen, passende Methode nutzen |
| Panic bei `unwrap()` | `Option`/`Result` war `None`/`Err` | mit `match`, `?` oder `unwrap_or` behandeln |
| Semantik: Schleife laeuft nie | `i > werte.len()` statt `i < werte.len()` | Vergleichsrichtung pruefen |
| Semantik: Off-by-one | `..=werte.len()` statt `..werte.len()` | obere Grenze exklusiv |
| Semantik: falsche Laenge | `.len()` (Bytes) statt `.chars().count()` | Zeichen zaehlen, wenn Umlaute vorkommen |
