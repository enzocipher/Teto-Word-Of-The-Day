use getrandom;
use random_word::Lang;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose, Engine as _};

fn get_random_u128() -> Result<u128, getrandom::Error> {
    let mut buf = [0u8; 16];
    getrandom::fill(&mut buf)?;
    Ok(u128::from_ne_bytes(buf))
}

// No se prq me complique tanto si solo tenia que retornar un String, KISS
fn teto_word_of_the_day() -> String {
    random_word::get(Lang::Es).to_string()
}

// SHA-256 con el numero y la palabra
fn hasher_cool(number: [u8; 16], teto_word: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(number);
    hasher.update(teto_word.as_bytes());

    let result = hasher.finalize();
    general_purpose::URL_SAFE_NO_PAD.encode(result) //No me gusto el = al final y para jwt como que no va
}

fn main() {
    let number = get_random_u128().unwrap().to_le_bytes();
    let teto_word = teto_word_of_the_day();
    let secret = hasher_cool(number, &teto_word);

    println!("{}", secret);
}