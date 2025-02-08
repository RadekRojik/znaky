#![allow(dead_code)]
use std::fmt;

/// Výčet vlastních chyb (čeština je krásná :D )
#[derive(Debug)]
pub enum Chyby {
    BadChar(String),        // špatný znak
    BadString(String),      // vstupní údaj nic moc
    BadNeco,                // Všeobecná chybka
    NFFile(std::io::Error), // Nenalezení = neotevření souboru
    NFWord(String),         // Nenalezení hledaného výrazu
}

// implementace traitu Display na formátování výstupu Chyby
impl fmt::Display for Chyby {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chyby::BadChar(_err) => write!(f, "Chybný znak"),
            Chyby::BadString(_err) => write!(f, "Chybný vstup"),
            Chyby::BadNeco => write!(f, "Etwa schief gelaufen"),
            Chyby::NFFile(_err) => write!(f, "Soubor nenalezen"),
            Chyby::NFWord(err) => write!(f, "{err} nenalezeno"),
        }
    }
}

// implementace traitu Error na Chyby
// tím se enum Chyby stává custom Error
impl std::error::Error for Chyby {}

// implementace From = konverze chybových typů
impl From<std::io::Error> for Chyby {
    fn from(err: std::io::Error) -> Self {
        Chyby::NFFile(err)
    }
}
