// 07_LIFETIME — Lifetime Annotation, Elision, Struct with Lifetime, 'static

/*
LIFETIME itu apa?
- Lifetime = masa hidup / seberapa lama suatu reference valid
- Rust pake lifetime buat mastiin reference ga dangling (ga指向 memori yg udah dibersihin)
- Biasanya Rust bisa nebak sendiri (lifetime elision), tapi kadang perlu dikasih tau manual
- Lifetimes ditandain pake 'a, 'b, etc
*/

// Fungsi dengan lifetime explicit
// Bilang: "return value punya lifetime yg sama dengan parameter x dan y"
fn mana_lebih_pendek<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() { x } else { y }
}

// Tanpa lifetime annotation — GA BIKIN COMPILE
// fn mana_lebih_pendek(x: &str, y: &str) -> &str {
//     if x.len() < y.len() { x } else { y }
// }
// Rust ga tau return value ngereference yg mana, makanya perlu 'a

// Multiple lifetimes — bedain mana yg dipake return
fn _pilih<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

// Struct dengan reference — WAJIB pake lifetime
struct Potongan<'a> {
    bagian: &'a str, // Potongan ga bisa hidup lebih lama dari bagian yg di-reference
}

impl<'a> Potongan<'a> {
    fn panjang(&self) -> usize {
        self.bagian.len()
    }

    // Lifetime elision — Rust bisa nebak kalo return &str pake lifetime &self
    fn ambil(&self) -> &str {
        self.bagian
    }
}

// Lifetime elision rules (Rust otomatis nebak):
// 1. Setiap parameter reference dapet lifetime sendiri
// 2. Kalo cuma 1 parameter input, return pake lifetime itu
// 3. Kalo ada &self atau &mut self, return pake lifetime self

// Contoh elision:
fn first_word(s: &str) -> &str {
    // Rust otomatis ngasih lifetime — setara sama fn first_word<'a>(s: &'a str) -> &'a str
    for (i, &ch) in s.as_bytes().iter().enumerate() {
        if ch == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Lifetime 'static — reference yg hidup sepanjang program
// &'static str itu string literal, karena disimpan di binary
fn ambil_konstanta() -> &'static str {
    "Rust static"
}

// Contoh error lifetime (di-comment biar ga error):
// fn dangling() -> &String {
//     let s = String::from("halo");
//     &s // ERROR: s di-drop pas fungsi selesai, return jadi dangling
// }

fn main() {
    // Fungsi dengan lifetime
    println!("LIFETIME BASIC");

    let string1 = String::from("panjang");
    let string2 = String::from("pendek");

    let hasil = mana_lebih_pendek(&string1, &string2);
    println!("Yg lebih pendek: {}", hasil);

    // Struct dengan lifetime
    println!("\nSTRUCT DENGAN LIFETIME");

    let kalimat = String::from("Belajar Rust itu seru");
    let potong = Potongan { bagian: &kalimat[..2] };
    println!("Potongan: {}", potong.bagian);
    println!("Panjang: {}", potong.panjang());
    println!("Ambil: {}", potong.ambil());

    // 'static
    println!("\nSTATIC LIFETIME");

    let s: &'static str = "Halo dunia";
    println!("Static: {}", s);
    println!("Konstanta: {}", ambil_konstanta());

    // Lifetime elision
    println!("\nLIFETIME ELISION");

    let kata = String::from("Rust is cool");
    let pertama = first_word(&kata);
    println!("First word: {}", pertama);

    // Contoh lifetime error (di-uncomment buat liat error):
    // let hasil2;
    // {
    //     let x = String::from("sementara");
    //     hasil2 = mana_lebih_pendek(&string1, &x);
    //     // ^^^ x udah di-drop di sini
    // }
    // println!("{}", hasil2);
    // ERROR: hasil2 ngarah ke x yg udah gak ada
}
