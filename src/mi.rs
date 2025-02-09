pub use crate::chybky::Chyby;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

static DELIM: char = ';'; // Delimiter
static HLEDANY_SLOUPEC: usize = 1;
static VRACENY_SLOUPEC: usize = 0;

/// funkce přijímá argumenty:
/// slovo = hledaná fráze
/// cesta = cesta k definiční složce
/// soubor = název prohledávaného CSV souboru
/// Pozor funkce jen vrací unicode hodnotu prvního nálezu
/// TODO: je třeba to vylepšit aby se dalo vybrat v jakých
/// sloupcích se má hledaná fráze nacházet
pub fn jmenne_seznamy(slovo: String, cesta: &Path, soubor: &str) -> Result<String, Chyby> {
    // Načtení slova ze vstupu
    let hledane_slovo = slovo.trim().to_lowercase(); // Odstranění bílých znaků

    // Cesta k souboru
    let cesta_k_souboru = Path::join(cesta, soubor); // "UnicodeData.txt");

    // Otevření souboru
    let soubor = File::open(&cesta_k_souboru)?;

    // Načtení souboru.
    let reader = BufReader::new(soubor);

    // Prohledávání souboru
    for (_index, radek) in reader.lines().enumerate() {
        let mut radek = radek?;
        radek = radek.to_lowercase(); // Převede obsah řádku na malé písmena
                                      // jestli se najde hledaný řetězec vrátí hodnotu
        if let Some(vystup) = csv_line_parser(radek, &hledane_slovo) {
            return Ok(vystup);
        };
    }

    Err(Chyby::NFWord(slovo))
    // println!("Slovo '{}' nebylo nalezeno.", hledane_slovo);
}

// funkce parsuje csv radek. Hledá ve druhém sloupci frázi.
// Při úspěchu vrací její unicode hodnotu (první sloupec)
// Jinak vrací none
fn csv_line_parser(line: String, fraze: &String) -> Option<String> {
    let radek: String = line;
    let mezikus: Vec<&str> = radek.split(DELIM).collect();
    if mezikus[HLEDANY_SLOUPEC].contains(fraze) {
        return Some(mezikus[VRACENY_SLOUPEC].to_string());
    } else {
        None
    }
}
