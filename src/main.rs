// test chování výpisu znaků
//
/********************************************************

TODO:

přidat Vstupní hodnoty:

Name,  // jmenná hodnota extrahovaná z autority:
       // https://www.unicode.org/reports/tr44/#UnicodeData.txt
       // https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt
Alias,  // staré jmenné tvary
Nick,  // vlastní jmenné tvary = zkratky

Poslední tři výčty bude asi lepší sloučit do jednoho vstupu
Seznamy se pak budou prohledávat v pořadí Nick – Name – Alias


**********************************************************/

// pracovní definice aby se nezbláznil analyzer
// #![allow(dead_code, unused_macros, unused_imports)]
// #![allow(unused_variables, unused_mut)]

// mod ecka;
// mod knihovny;
// use knihovny::{self as k, Chyby};
mod chybky;
use crate::chybky::Chyby;

mod mi;
// use crate::mi::*;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
// use std::fs::File;
//use std::path::Path;
use std::{
    char, env, fs,
    io::{self, BufRead},
    path::Path,
    process::exit,
    str::{Chars, FromStr},
    u32, u8,
};
use toml;

/* ––––––––––––––––––––––––––––––––––––––––––––––––––
 * makra
––––––––––––––––––––––––––––––––––––––––––––––––––– */
// Miluju dokumentační komentáře. Ve spojení s LSP jeden neobjevuje
// co už objevil :)

/// Ošetření chyby návratu Result
///
/// # Errors
/// Ukončení programu s návratovou hodnotou 1
///
/// # Examples
/// ```
/// let vysledek = ma_res!(expr, err: &str);
/// ```
/// ` `
macro_rules! ma_res {
    ($co: expr, $hlaska: expr) => {
        match $co {
            Ok(i) => i,
            _ => {
                eprintln!("{}", HYHY); // k::Chyby::BadString($hlaska.to_string()));
                exit(1);
            }
        }
    };
}

/// Ošetření chyby návratu Option
///
/// # Errors
/// Ukončení programu s návratovou hodnotou 1
///
/// # Examples
/// ```
/// let vysledek = ma_opt!(expr, err: &str);
/// ```
/// ` `
macro_rules! ma_opt {
    ($co: expr, $hlaska: expr) => {
        match $co {
            Some(i) => i,
            None => {
                eprintln!("{}", HYHY); // k::Chyby::BadString($hlaska.to_string()));
                exit(1);
            }
        }
    };
}

/****************************************************************
 * konec maker
*****************************************************************/
// chybové hlášky
// static BADCHAR: &str = "Chybný znak";
static BADSTRING: &str = "Chybka";
// static BADSOME: &str = "Oops";
// static HYHY: Chyby = k::Chyby::BadNeco;
static HYHY: Chyby = mi::Chyby::BadNeco;
// static NOTFOUND: &str = "Nenalezeno";
// static BADUNIFILE: &str = "Chybí soubor ~/.config/znaky/UnicodeData.txt";

/// hexa literal to nibble
///
/// # Examples
/// ```
/// let nibble: u8 = char_to_nibble('F');
///
/// assert_eq!(15, nibble);
/// ```
/// # Errors
///
/// exit with 1
/// ` `
fn char_to_nibble(qq: char) -> u8 {
    let t = ma_opt!(qq.to_digit(16), HYHY); // převod na hex číslo
    let r = ma_res!(u8::try_from(t), HYHY); // převod na Byte
    r // návratová hodnota
}

/// fce vrátí **následující** znak z řetězce
///
/// ` `
fn vrat_znak(mm: &mut Chars) -> char {
    let k = ma_opt!(mm.next(), HYHY);
    k // návratová hodnota
}

// někdy se jeden v typech ztratí a není jasné co deklarovat.
// tak tahle funkce napráská wo co gou :D
/*
fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}
*/

/// fce chars_count přijme řetězec. Spočítá kolik má znaků.
/// Pokud má nulové modulo (počet je sudý), vrátí polovinu
/// počtu znaků type usize.
/// V ostatních případech vyvolá výjimku
/// ` `
fn chars_count(ret: String) -> Result<usize, &'static str> {
    let pocet = ret.chars().count();
    let u = pocet % 2;
    match u {
        0 => Ok(pocet / 2),
        _ => Err(&BADSTRING),
    }
}

// Definice struktury Data
// obsahuje strukturu Prefixy
#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
struct Data {
    prefixy: Prefixy,
}

// Definice struktury Prefixy
#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
struct Prefixy {
    decim: String,
    unicode: String,
    utf: String,
}

// Implementace Default::default() vracející
// základní hodnoty struktury Prefixy
impl Default for Prefixy {
    fn default() -> Self {
        return Prefixy {
            decim: "dx".to_owned(),
            unicode: "ux".to_owned(),
            utf: "0x".to_owned(),
        };
    }
}

// Implementace Default::default() vracející
// základní hodnoty struktury Data - což je struktura Prefixy
impl Default for Data {
    fn default() -> Self {
        return Data {
            prefixy: Prefixy::default(),
        };
    }
}

/// `parsetoml` načte základní prefixy. Pokud existuje configurační soubor,
/// načte prefixy z něho.
/// ` `
fn parsetoml(config_path: &Path) -> Data {
    // vytvoření proměnné `config_file` s adresou configuračního souboru
    // let mut config_file = env::var("HOME").unwrap_or("none".to_string());
    // config_file.push_str("/.config/unitochar/config.toml");
    let config_file = Path::join(config_path, "config.toml");
    // Načtení souboru do proměnné `contents`
    let contents = match fs::read_to_string(config_file) {
        // Při úspěchu obsahuje `contents` obsah souboru
        Ok(c) => c.to_lowercase(),
        // Pokud se vyskytne chyba, vrátí se prázdný řetězec
        _ => " ".to_string(),
    };

    // Parsování obsahu do objektu ze struktury `Data`
    // Pokud se vyskytne chyba, načte se pro danou položku defaultní hodnota
    let mapka: Data = toml::from_str(&contents).unwrap_or_else(|_| Data {
        prefixy: Default::default(),
    });

    // println!("decim {}", mapka.prefixy.decim);
    // println!("utf {}", mapka.prefixy.utf);
    mapka
}

/* ****************************************************************************
 *
 * Hlavní funkce
 *
 */

fn main() {
    // let blabla = k::Chyby::BadChar(String::from("ch znak"));
    let mut config_path = env::var("HOME").unwrap_or("none".to_string());
    config_path.push_str("/.config/znaky/");
    let cesta = Path::new(config_path.as_str());

    // vytvoření instance prefixů
    let datika = parsetoml(cesta).prefixy;

    let mut predpony = vec![
        datika.unicode.as_str(),
        datika.utf.as_str(),
        datika.decim.as_str(),
    ];
    predpony.push(""); // prázdný řetězec na konci je důležitý kvůli defaultu bez předpony

    let codepoint: u32;
    let znak: char;

    let argumenty: Vec<String> = env::args().collect(); // načtení argumentů
    let mut argument = String::new(); // deklarace proměnné vstupu
                                      // let soustava: u32; // deklarace proměnné

    if argumenty.len() > 2 {
        // test kolik argumentů je předáno programu
        exit(1); // pokud více než jeden, program je ukončen s nenulovou hodnotou
    } else if argumenty.len() == 2 {
        // jeden argument, načte se
        argument = argumenty[1].to_lowercase(); // převod argumentu na malá písmena
    } else {
        let _ = io::stdin().lock().read_line(&mut argument); // není argument, čte se ze standardního
                                                             // vstupu ( klávesnice nebo roura)
        argument = argument.trim_end().to_lowercase(); // odřízne řídící znak nového řádku a převede vstup
                                                       // na malá písmena
    };

    // podle předpony (prefixu) vyberem jak zpracovat vstup
    if let Some(neco) = predpony.iter().find(|neco| argument.starts_with(*neco)) {
        match neco {
            val if val == &datika.unicode => {
                // codepoint vstup
                argument = ma_opt!(argument.strip_prefix(val), HYHY).to_string();
                codepoint = ma_res!(u32::from_str_radix(argument.as_str(), 16), blabla);
                znak = ma_opt!(char::from_u32(codepoint), HYHY);
                println!("{znak}");
            }
            val if val == &datika.utf => {
                // utf8 vstup
                argument = ma_opt!(argument.strip_prefix(val), HYHY).to_string();
                znak = char::from_str(u_literal(argument).as_str()).unwrap();
                println!("{znak}");
            }
            val if val == &datika.decim => {
                // dekadický vstup
                argument = ma_opt!(argument.strip_prefix(val), HYHY).to_string();
                codepoint = ma_res!(u32::from_str_radix(argument.as_str(), 10), blabla);
                znak = ma_opt!(char::from_u32(codepoint), HYHY);
                println!("{znak}");
            }
            _ => {
                // argument = ma_res!(
                //     jmenne_seznamy(argument, cesta, "UnicodeData.txt"),
                //     "cosi jineho"
                // );
                let zac: String = String::from(argument);
                // let zac = &argument;

                argument = mi::jmenne_seznamy(zac.clone(), cesta, "mujslovnik.txt")
                    .or_else(|_| mi::jmenne_seznamy(zac.clone(), cesta, "UnicodeData.txt"))
                    .unwrap_or_else(|_| "".to_string());

                codepoint = ma_res!(u32::from_str_radix(argument.as_str(), 16), blabla);
                znak = ma_opt!(char::from_u32(codepoint), HYHY);
                println!("{znak}");

                // pokud není prefix, pracuj jako s dekadickým vstupem
                // codepoint = ma_res!(u32::from_str_radix(argument.as_str(), 10), BADCHAR);
                // znak = ma_opt!(char::from_u32(codepoint), BADSOME);
                // println!("{znak}");
            }
        }

        // neco.get_mut(0..2) vrátí první dva znaky

        //let retezec: String = "e299az".to_string();  // chybný znak 'z' v testovacím žetězci
        // let retezec: String = "c990".to_string(); // testovací řetězec s utf8 hodnotou
        // println!("{}", u_literal(retezec));
        exit(0)
    }
}

/* *****************************************************************************
 *
 * Konec hlavní fce
 *
 */

/// Převod řetězce obsahující utf8 znaky
///
fn u_literal(retezec: String) -> String {
    let mut vys: Vec<u8> = vec![]; // vytvoření prázdného vektoru do kterého se budou ukládat
                                   // hodnoty jednotlivých byte
    let mut znaky: Chars = retezec.chars(); // řetězec "rozbijem" na znaky
                                            //let pocetznaku = retezec.chars().count();
    let pocetznaku = ma_res!(chars_count(retezec.clone()), HYHY);

    for _ in 0..pocetznaku {
        let firs_char = vrat_znak(&mut znaky); // v ll je znak reprezentující první nibble
        let second_char = vrat_znak(&mut znaky); // v kk je znak reprezentující druhý nibble
        let mut first_nibble = char_to_nibble(firs_char); // převeď znak na jeho hexa hodnotu
        let second_nibble = char_to_nibble(second_char);
        first_nibble = first_nibble << 4; // předpokládám, že posun registru o čtyři bity by měl být
                                          // přímější a rychlejší než aritmetický součin
        vys.push(first_nibble | second_nibble); // to samé platí o binárním součtu než aritmetickém
                                                // println!("{} {} = {}", first_nibble, second_nibble, i); // testovací mezivýpis
    }
    let h = Bytes::from(vys);
    ma_res!(std::str::from_utf8(&h), HYHY).to_string()
}
