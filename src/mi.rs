// mod chybky;
// pub use chybky::Chyby;

pub use crate::chybky::Chyby;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn jmenne_seznamy(slovo: String, cesta: &Path, soubor: &str) -> Result<String, Chyby> {
    // Načtení slova ze vstupu
    // println!("Zadejte slovo k hledání:");
    // let mut hledane_slovo = String::new();
    // io::stdin().read_line(&mut hledane_slovo)?;
    let hledane_slovo = slovo.trim().to_lowercase(); // Odstranění bílých znaků

    // Cesta k souboru
    //let cesta_k_souboru = Path::new("jmena.csv");
    let cesta_k_souboru = Path::join(cesta, soubor); // "UnicodeData.txt");

    // Otevření souboru
    let soubor = File::open(&cesta_k_souboru)?;
    //  {
    //     Ok(i) => i,
    //     _ => return BADUNIFILE.to_string(),
    // };

    let reader = BufReader::new(soubor);

    // Prohledávání souboru
    for (_index, radek) in reader.lines().enumerate() {
        let mut radek = radek?;
        radek = radek.to_lowercase();
        if radek.contains(&hledane_slovo) {
            // let mut vystup = String::new();
            // println!(
            //     "Slovo '{}' nalezeno na řádku {}: {}",
            //     hledane_slovo,
            //     index + 1,
            //     radek
            let vystup = match radek.split_once(";") {
                Some((i, _)) => i.to_string(),
                _ => "".to_string(),
            };
            return Ok(vystup);
            // break;
            // );
        }
    }

    Err(Chyby::NFWord(slovo))
    // println!("Slovo '{}' nebylo nalezeno.", hledane_slovo);
}
