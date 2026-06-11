use std::collections::HashMap;

fn main() {
    // 1. ARRAY — Ukuran tetap
    println!("ARRAY");

    let mut angka: [i32; 5] = [0; 5];
    angka[0] = 10;
    angka[1] = 20;
    angka[2] = 30;
    angka[3] = 40;
    angka[4] = 50;
    println!("Array angka: {:?}", angka);

    // Inisialisasi langsung
    let buah = ["Apel", "Mangga", "Jeruk"];
    println!("Array buah: {:?}", buah);
    println!("Panjang array: {}", buah.len());
    println!("Index 1: {}", buah[1]);

    // Array multi dimensi
    let matrix: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    println!("Matrix 2x2: {:?}", matrix);

    // Iterasi array
    println!("\nIterasi array:");
    for val in buah.iter() {
        println!("Buah: {}", val);
    }

    // 2. VEC — Array dinamis
    println!("\nVEC");

    // vec! macro
    let mut nama = vec!["Alice", "Bob", "Charlie"];
    println!("Vec nama: {:?}", nama);
    println!("Panjang: {}, Kapasitas: {}", nama.len(), nama.capacity());

    // push — menambah elemen
    nama.push("Diana");
    println!("Setelah push: {:?}", nama);

    // pop — hapus elemen terakhir
    let terakhir = nama.pop();
    println!("Pop: {:?}, sisa: {:?}", terakhir, nama);

    // slicing
    let sub = &nama[1..3];
    println!("Slice [1..3]: {:?}", sub);

    // Vec dengan kapasitas awal
    let mut angka2: Vec<i32> = Vec::with_capacity(5);
    angka2.push(100);
    angka2.push(200);
    angka2.push(300);
    println!("Vec dengan capacity: {:?} (len: {}, cap: {})", angka2, angka2.len(), angka2.capacity());

    // iterasi vec
    println!("\nIterasi vec:");
    for (i, val) in nama.iter().enumerate() {
        println!("Index {}: {}", i, val);
    }

    // iterasi sambil ubah
    println!("\nIterasi mutable:");
    let mut numbers = vec![1, 2, 3, 4, 5];
    for n in numbers.iter_mut() {
        *n *= 2;
    }
    println!("Setelah dikali 2: {:?}", numbers);

    // 3. HashMap — Key-value
    println!("\nHASHMAP");

    let mut umur = HashMap::new();
    umur.insert("Alice", 25);
    umur.insert("Bob", 30);
    umur.insert("Charlie", 35);
    println!("HashMap umur: {:?}", umur);
    println!("Umur Alice: {:?}", umur.get("Alice"));

    // insert / update
    umur.insert("Diana", 28);
    println!("Setelah insert Diana: {:?}", umur);

    // entry — cek & insert kalo belom ada
    umur.entry("Eve").or_insert(22);
    umur.entry("Bob").or_insert(99); // Bob udah ada, gak berubah
    println!("Setelah or_insert: {:?}", umur);

    // remove
    umur.remove("Charlie");
    println!("Setelah remove Charlie: {:?}", umur);

    // iterasi HashMap
    println!("\nIterasi HashMap:");
    for (key, val) in &umur {
        println!("{} -> {} tahun", key, val);
    }

    // 4. TUPLE (lanjutan)
    println!("\nTUPLE");

    let data = (1, "hello", 3.14);
    println!("Tuple: {:?}", data);
    println!("Item 0: {}, Item 1: {}, Item 2: {}", data.0, data.1, data.2);

    // 5. CONTOH KOMBINASI
    println!("\nKOMBINASI");

    // Vec of HashMap
    let mut daftar_orang = vec![
        HashMap::from([("nama", "Budi"), ("kota", "Jakarta")]),
        HashMap::from([("nama", "Siti"), ("kota", "Bandung")]),
    ];

    daftar_orang.push(HashMap::from([("nama", "Andi"), ("kota", "Surabaya")]));

    for orang in &daftar_orang {
        println!(
            "Nama: {}, Kota: {}",
            orang.get("nama").unwrap_or(&""),
            orang.get("kota").unwrap_or(&"")
        );
    }
}
