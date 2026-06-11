fn sapa() {
    println!("Halo, selamat belajar Rust!");
}

fn sapa_orang(nama: &str) {
    println!("Halo {}, selamat belajar Rust!", nama);
}

fn tambah(a: i32, b: i32) -> i32 {
    a + b // tanpa titik koma = return
}

fn kurang(a: i32, b: i32) -> i32 {
    return a - b; // pake return jg boleh
}

fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("tidak bisa membagi dengan nol"));
    }
    Ok(a / b)
}

fn hitung_lingkaran(r: f64) -> (f64, f64) {
    let luas = 3.14 * r * r;
    let keliling = 2.0 * 3.14 * r;
    (luas, keliling)
}

fn jumlah_angka(angka: &[i32]) -> i32 {
    let mut total = 0;
    for val in angka {
        total += val;
    }
    total
}

// Function pointer sebagai parameter
fn operasi_matematika(a: i32, b: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}

// Closure sebagai parameter (generik)
fn apply<F>(a: i32, b: i32, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(a, b)
}

fn buat_counter() -> impl FnMut() -> i32 {
    let mut counter = 0;
    move || {
        counter += 1;
        counter
    }
}

// Ownership — fungsi yang take ownership
fn ambil_ownership(s: String) {
    println!("String diterima: {}", s);
} // s di-drop di sini

// Borrowing — fungsi yang pinjam (&)
fn pinjam_reference(s: &str) {
    println!("Reference: {}", s);
}

// Mutable reference
fn ubah_nilai(x: &mut i32) {
    *x += 10;
}

struct Orang {
    nama: String,
    umur: i32,
}

impl Orang {
    // Method dengan &self (read-only)
    fn perkenalan(&self) {
        println!("Halo, nama saya {}, umur {} tahun", self.nama, self.umur);
    }

    // Method dengan &mut self (bisa ubah)
    fn ulang_tahun(&mut self) {
        self.umur += 1;
    }

    // Associated function (tanpa self) — kayak static method
    fn baru(nama: &str, umur: i32) -> Orang {
        Orang {
            nama: String::from(nama),
            umur,
        }
    }
}

fn main() {
    println!("FUNGSI SEDERHANA");
    sapa();
    sapa_orang("Budi");

    let hasil_tambah = tambah(10, 5);
    println!("10 + 5 = {}", hasil_tambah);

    let hasil_kurang = kurang(10, 5);
    println!("10 - 5 = {}", hasil_kurang);

    println!("\nRESULT");
    match bagi(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match bagi(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    println!("\nMULTIPLE RETURN VALUE");
    let (luas, keliling) = hitung_lingkaran(7.0);
    println!("Lingkaran r=7: Luas={:.2}, Keliling={:.2}", luas, keliling);

    println!("\nSLICE SEBAGAI PARAMETER");
    println!("Jumlah: {}", jumlah_angka(&[1, 2, 3, 4, 5]));

    println!("\nFUNCTION SEBAGAI VALUE");
    let kali = |a: i32, b: i32| a * b;
    let hasil_kali = operasi_matematika(6, 7, kali);
    println!("6 * 7 = {}", hasil_kali);

    // Closure langsung
    let hasil = apply(10, 5, |a, b| a / b);
    println!("10 / 5 = {}", hasil);

    println!("\nCLOSURE");
    let mut counter1 = buat_counter();
    println!("Counter 1: {}", counter1());
    println!("Counter 1: {}", counter1());
    let mut counter2 = buat_counter();
    println!("Counter 2: {}", counter2());
    println!("Counter 1 lagi: {}", counter1());

    println!("\nOWNERSHIP");
    let s1 = String::from("Halo Rust");
    ambil_ownership(s1);
    // println!("{}", s1); // ERROR: s1 udah di-move

    println!("\nBORROWING");
    let s2 = String::from("Pinjam dong");
    pinjam_reference(&s2);
    println!("s2 masih bisa dipake: {}", s2);

    println!("\nMUTABLE REFERENCE");
    let mut num = 5;
    println!("Sebelum: {}", num);
    ubah_nilai(&mut num);
    println!("Sesudah: {}", num);

    println!("\nMETHOD");
    let mut budi = Orang::baru("Budi", 25);
    budi.perkenalan();
    budi.ulang_tahun();
    budi.perkenalan();

    println!("\nCLOSURE DENGAN CAPTURE");
    let faktor = 3;
    let kali_faktor = |x: i32| x * faktor;
    println!("5 * 3 = {}", kali_faktor(5));
}
