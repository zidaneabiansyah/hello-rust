// 04_ENUM — Enum, Option, Result, Match, if let

// Enum dasar
enum Warna {
    Merah,
    Hijau,
    Biru,
}

// Enum dengan data
#[derive(Debug)]
enum Bentuk {
    Lingkaran(f64),          // radius
    Persegi(f64),            // sisi
    PersegiPanjang(f64, f64), // lebar, tinggi
}

impl Bentuk {
    fn luas(&self) -> f64 {
        match self {
            Bentuk::Lingkaran(r) => 3.14 * r * r,
            Bentuk::Persegi(s) => s * s,
            Bentuk::PersegiPanjang(l, t) => l * t,
        }
    }
}

// Enum dengan struct-like
enum Pesan {
    Teks(String),
    Koordinat { x: i32, y: i32 },
    Diam,
}

// Option<T>
fn cari_nama(daftar: &[&str], target: &str) -> Option<usize> {
    for (i, &nama) in daftar.iter().enumerate() {
        if nama == target {
            return Some(i);
        }
    }
    None
}

// Result<T, E>
fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Tidak bisa membagi dengan nol"));
    }
    Ok(a / b)
}

fn main() {
    // Match enum sederhana
    println!("ENUM SEDERHANA");

    let warna = Warna::Hijau;

    match warna {
        Warna::Merah => println!("Warna merah"),
        Warna::Hijau => println!("Warna hijau"),
        Warna::Biru => println!("Warna biru"),
    }

    // Enum dengan data
    println!("\nENUM DENGAN DATA");

    let bentuk1 = Bentuk::Lingkaran(7.0);
    let bentuk2 = Bentuk::Persegi(4.0);
    let bentuk3 = Bentuk::PersegiPanjang(5.0, 3.0);

    let bentuk_vec = vec![bentuk1, bentuk2, bentuk3];

    for bentuk in &bentuk_vec {
        println!("Luas: {:.2}", bentuk.luas());
    }

    // Match dengan binding
    println!("\nMATCH DENGAN BINDING");

    let pesan = Pesan::Koordinat { x: 10, y: 20 };

    match pesan {
        Pesan::Teks(t) => println!("Pesan teks: {}", t),
        Pesan::Koordinat { x, y } => {
            println!("Koordinat: x={}, y={}", x, y);
        }
        Pesan::Diam => println!("Tidak ada pesan"),
    }

    // Option<T>
    println!("\nOPTION<T>");

    let orang = vec!["Alice", "Bob", "Charlie"];

    let hasil1 = cari_nama(&orang, "Bob");
    let hasil2 = cari_nama(&orang, "David");

    match hasil1 {
        Some(index) => println!("Bob ditemukan di index {}", index),
        None => println!("Bob tidak ditemukan"),
    }

    match hasil2 {
        Some(index) => println!("David ditemukan di index {}", index),
        None => println!("David tidak ditemukan"),
    }

    // if let
    println!("\nIF LET");

    let nilai_option: Option<i32> = Some(42);

    if let Some(nilai) = nilai_option {
        println!("Nilainya adalah {}", nilai);
    }

    // if let dengan else
    let nilai_option2: Option<i32> = None;

    if let Some(nilai) = nilai_option2 {
        println!("Nilainya adalah {}", nilai);
    } else {
        println!("Tidak ada nilai");
    }

    // Result<T, E>
    println!("\nRESULT<T, E>");

    match bagi(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match bagi(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Operator ?
    // (harus di fungsi yg return Result, kita demo pake match aja)

    // unwrap / expect
    println!("\nUNWRAP & EXPECT");

    let ok_value: Result<i32, &str> = Ok(100);
    println!("unwrap: {}", ok_value.unwrap());

    // Kalau Err, bakal panic:
    // let err_value: Result<i32, &str> = Err("gagal");
    // println!("{}", err_value.unwrap()); // PANIC!

    // expect dengan custom message
    let ok_value2: Option<i32> = Some(50);
    println!("expect: {}", ok_value2.expect("Seharusnya ada nilai"));

    // Kombinasi: Vec<Option<T>>
    println!("\nKOMBINASI");

    let data = vec![Some(1), None, Some(3), Some(4), None];

    // Filter hanya nilai Some
    let hasil: Vec<i32> = data
        .iter()
        .filter_map(|&x| x)
        .collect();

    println!("Data setelah filter: {:?}", hasil);

    // Enum dengan match exhaustive
    println!("\nMATCH EXHAUSTIVE");

    let angka = 3;

    match angka {
        1 => println!("satu"),
        2 => println!("dua"),
        3 => println!("tiga"),
        _ => println!("lainnya"), // _ = catch-all
    }

    // match dengan range
    match angka {
        1..=3 => println!("1-3"),
        4..=6 => println!("4-6"),
        _ => println!(">6"),
    }
}
