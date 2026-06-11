fn main() {
    // 1. VARIABEL
    // let = immutable (default)
    // let mut = mutable

    let nama: &str = "Budi";
    let mut umur: i32 = 25;
    let tinggi: f64 = 170.5;
    let menikah: bool = false;
    let inisial: char = 'B';

    println!("VARIABEL DAN TIPE DATA");
    println!("Nama: {} | Umur: {} | Tinggi: {} | Menikah: {} | Inisial: {}", nama, umur, tinggi, menikah, inisial);

    // type inference — Rust bisa nebak tipe
    let pekerjaan = "Programmer";
    println!("Pekerjaan: {} (inferred)", pekerjaan);

    // mutable variable
    umur = 26;
    println!("Umur setelah diubah: {}", umur);

    // const — harus dikasih tipe
    const TAHUN_LAHIR: i32 = 2000;
    println!("Tahun lahir: {}", TAHUN_LAHIR);

    // shadowing — variable bisa dideklar ulang
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("Shadowing x: {}", x);

    // String vs &str
    let teks_str: &str = "Hello";
    let teks_string: String = String::from("Hello juga");
    println!("&str: {}, String: {}", teks_str, teks_string);

    // tipe numerik
    let desimal: f32 = 3.14;
    let biner = 0b1010; // 10
    let heks = 0xFF;    // 255
    println!("f32: {}, biner: {}, heks: {}", desimal, biner, heks);

    // tuple — kumpulan tipe beda
    println!("\nTUPLE");
    let orang: (&str, i32, f64) = ("Budi", 25, 170.5);
    println!("Nama: {}, Umur: {}, Tinggi: {}", orang.0, orang.1, orang.2);
    let (nama2, umur2, tinggi2) = orang;
    println!("Destructure: {} {} {}", nama2, umur2, tinggi2);

    // 2. PERCABANGAN (if-else)
    println!("\nPERCABANGAN");

    let nilai = 85;

    if nilai >= 90 {
        println!("Grade: A");
    } else if nilai >= 75 {
        println!("Grade: B");
    } else if nilai >= 60 {
        println!("Grade: C");
    } else {
        println!("Grade: D");
    }

    // if expression — mirip ternary
    let status = if umur >= 17 { "Dewasa" } else { "Anak-anak" };
    println!("Status: {}", status);

    // match — switch versi Rust
    println!("\nMatch:");
    match nilai {
        90..=100 => println!("A"),
        75..=89 => println!("B"),
        60..=74 => println!("C"),
        _ => println!("D"),
    }

    // 3. PERULANGAN (loop, while, for)

    // loop — infinite, pake break
    println!("\nPERULANGAN");

    println!("Loop with counter:");
    let mut counter = 0;
    loop {
        if counter >= 3 {
            break;
        }
        println!("Loop ke-{}", counter);
        counter += 1;
    }

    // while
    println!("\nWhile:");
    let mut hitungan = 0;
    while hitungan < 3 {
        println!("Hitungan: {}", hitungan);
        hitungan += 1;
    }

    // for — range
    println!("\nFor range:");
    for i in 0..5 {
        println!("Iterasi ke-{}", i);
    }

    // for dengan include (..=)
    println!("\nFor range inclusive:");
    for i in 1..=3 {
        println!("{}", i);
    }

    // for iterasi array
    println!("\nFor iterasi array:");
    let buah = ["Apel", "Mangga", "Jeruk"];
    for item in buah.iter() {
        println!("Buah: {}", item);
    }

    // enumerate
    println!("\nFor dengan index:");
    for (index, value) in buah.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }

    // break & continue
    println!("\nBreak & Continue:");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        if i > 7 {
            break;
        }
        println!("Bilangan ganjil: {}", i);
    }
}
