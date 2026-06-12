use std::fs::File;
use std::io::{self, Read};

fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(format!("Tidak bisa membagi {} dengan 0", a));
    }
    Ok(a / b)
}

fn baca_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut isi = String::new();
    file.read_to_string(&mut isi)?;
    Ok(isi)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("RESULT & MATCH");

    match bagi(10, 2) {
        Ok(hasil) => println!("10 / 2 = {}", hasil),
        Err(e) => println!("Error: {}", e),
    }

    match bagi(10, 0) {
        Ok(hasil) => println!("10 / 0 = {}", hasil),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nOPERATOR ?");

    match baca_file("Cargo.toml") {
        Ok(isi) => println!("Teks di Cargo.toml:\n{}", isi),
        Err(e) => println!("Gagal baca file: {}", e),
    }

    match baca_file("ga_ada.txt") {
        Ok(isi) => println!("{}", isi),
        Err(e) => println!("Gagal baca file: {}", e),
    }

    println!("\nCOMBINE RESULT");

    let hasil = bagi(10, 2)?;
    println!("Dengan ?: {}", hasil);

    let hasil2 = bagi(10, 0).unwrap_or(0);
    println!("unwrap_or: {}", hasil2);

    let hasil3 = bagi(10, 2).unwrap_or(0);
    println!("unwrap_or: {}", hasil3);

    let hasil4 = bagi(10, 0).unwrap_or_else(|e| {
        println!("Error: {}, pake default", e);
        0
    });
    println!("unwrap_or_else: {}", hasil4);

    println!("\nMAP & AND_THEN");

    let kudua = bagi(10, 2).map(|x| x * 2);
    println!("map: {:?}", kudua);

    let dibagi_lagi = bagi(10, 2).and_then(|x| bagi(x, 2));
    println!("and_then: {:?}", dibagi_lagi);

    let dibagi_lagi2 = bagi(10, 0).and_then(|x| bagi(x, 2));
    println!("and_then (error): {:?}", dibagi_lagi2);

    Ok(())
}
