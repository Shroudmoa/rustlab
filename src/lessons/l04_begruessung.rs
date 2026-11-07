//! L04 - Begruessung
//!
//! Zeigt: mehrzeiliger Raw-String (`r#"..."#`), Name einlesen,
//! `format!` fuer zusammengesetzte Ausgaben.

use std::io;

use crate::registry::LessonResult;

const BANNER: &str = r#"
⣿⣿⣿⠁⠋⡄⢯⠸⣵⠸⢹⡟⣦⢻⢿⣿⣿⣷⣹⣿⣿⣿⣿⣿
⣿⣿⢋⠀⡴⠁⠠⠆⣬⣷⣮⠃⠿⢎⣆⣿⡏⣿⢳⣿⣿⣿⣿⣿
⡿⣱⡏⣼⡏⢸⣛⢾⣿⣿⣿⢓⡆⣀⠙⠎⢣⢁⣿⣿⣿⣿⣿⡟
⣺⡟⣸⣿⢣⣿⣴⣿⣿⣿⣧⣉⣾⣿⢋⣴⣾⣿⣿⣿⣿⣿⣟⣼
⣿⢡⣿⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⣾⣿⣿⣿⣿⣿⣿⢋⣾⣿
⡇⢿⣿⡇⣿⣿⠿⣿⣿⣿⣿⣿⡟⣸⣿⣿⣿⣿⣿⡟⣡⣿⣿⣿
⣇⠘⠟⠇⣜⢿⣶⣿⣿⣿⣿⣿⢣⣿⠫⣿⣿⣿⢏⣼⣿⣿⣿⣿
⣿⣇⣤⡼⠏⠧⣙⠿⢿⣿⣿⣿⠸⢱⣿⢿⠿⢃⣾⡿⣻⣿⣿⣿
"#;

/// Baut die Begruessung; ein leerer Name bekommt einen Ersatztext.
pub fn begruessung(name: &str) -> String {
    let name = name.trim();
    if name.is_empty() {
        "Hallo, Unbekannter!".to_string()
    } else {
        format!("Hallo, {name}!")
    }
}

pub fn run() -> LessonResult {
    println!("{BANNER}");
    println!("Wie heisst du?");

    let mut name = String::new();
    io::stdin().read_line(&mut name)?;

    println!("{}", begruessung(&name));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_wird_eingesetzt() {
        assert_eq!(begruessung("Moa\n"), "Hallo, Moa!");
    }

    #[test]
    fn leerer_name_bekommt_ersatztext() {
        assert_eq!(begruessung("   \n"), "Hallo, Unbekannter!");
    }
}
