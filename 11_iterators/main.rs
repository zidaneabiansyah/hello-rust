/*
ITERATORS & CLOSURES — Deep Dive

Apa itu Iterator?
- Trait di std::iter::Iterator
- Cuma butuh 1 method: .next() -> Option<Self::Item>
- Semua koleksi di Rust bisa di-iterasi (for loop)
- Ada RATUSAN method adapter (map, filter, fold, dll)

Kenapa pake iterator?
- Komposisi: gabungin operasi secara berantai (chain)
- Lazy: gak ngeksekusi sampe di-consumed (collect, for_each, next)
- Zero-cost: performanya SAMA kaya for loop manual (sering lebih cepet)
*/

// Iterator custom — Fibonacci
#[derive(Debug)]
struct Fibonacci {
    a: u64,
    b: u64,
    max: u64,
}

impl Fibonacci {
    fn baru(max: u64) -> Self {
        Fibonacci { a: 0, b: 1, max }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        if current >= self.max {
            return None;
        }
        self.a = self.b;
        self.b = current + self.b;
        Some(current)
    }
}

// IntoIterator custom — bisa di-for-loop
#[derive(Debug)]
struct KoleksiBuah {
    buah: Vec<String>,
}

impl IntoIterator for KoleksiBuah {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.buah.into_iter()
    }
}

// Custom adapter — map versi kita sendiri
#[derive(Debug)]
struct MapIter<I, F> {
    iter: I,
    f: F,
}

impl<I, F, B> Iterator for MapIter<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }
}

fn main() {
    println!("1. ITERATOR BASIC — next() langsung");

    let angka = vec![1, 2, 3, 4, 5];
    let mut iter = angka.iter();
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next (habis): {:?}", iter.next());

    // for loop sebenernya pake into_iter()
    println!("\n2. FOR LOOP (into_iter)");
    for val in &angka {
        println!("{}", val);
    }

    // 3 jenis iter
    println!("\n3. iter() vs iter_mut() vs into_iter()");
    let data = vec!["a", "b", "c"];

    // iter() — borrow &T
    for x in data.iter() {
        // x: &&str
        println!("iter: {}", x);
    }
    println!("Masih bisa pake data: {:?}", data);

    // iter_mut() — mutable borrow &mut T
    let mut data2 = vec![1, 2, 3];
    for x in data2.iter_mut() {
        *x *= 10;
    }
    println!("iter_mut: {:?}", data2);

    // into_iter() — consume, dapet T (ownership)
    let data3 = vec!["x", "y", "z"];
    for x in data3.into_iter() {
        println!("into_iter: {}", x);
    }
    // println!("{:?}", data3); // ERROR: data3 udah di-move

        println!("\n4. MAP — transform setiap elemen");
    // map = transformasi 1-to-1

    let angka = vec![1, 2, 3, 4, 5];

    // map return iterator LAZY — belum dijalanin
    let dikali: Vec<i32> = angka.iter().map(|x| x * 2).collect();
    println!("map x2: {:?}", dikali);

    // map ke tipe lain
    let jadi_string: Vec<String> = angka.iter().map(|x| format!("Angka {}", x)).collect();
    println!("map ke string: {:?}", jadi_string);

    // map pake method reference
    let nama = vec!["budi", "siti", "andi"];
    let kapital: Vec<String> = nama.iter().map(|s| s.to_uppercase()).collect();
    println!("map uppercase: {:?}", kapital);

    println!("\n5. FILTER — seleksi pake predicate");

    let angka = 1..=20;

    let genap: Vec<i32> = angka.filter(|x| *x % 2 == 0).collect();
    println!("Genap: {:?}", genap);

    // filter bisa digabung sama map
    let result: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Genap dikuadratkan: {:?}", result);

    println!("\n6. FILTER_MAP — filter + map dalam 1 langkah");
    // filter_map = filter (Option) + map
    // Return Some(x) = keep, None = skip

    let data = vec!["1", "dua", "3", "empat", "5"];
    let angka_aja: Vec<i32> = data
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("parse angka: {:?}", angka_aja);

    // filter_map dengan Result
    let entries = vec![
        ("Alice", Some(85)),
        ("Bob", None),
        ("Charlie", Some(90)),
    ];
    let nilai_aja: Vec<&str> = entries
        .iter()
        .filter_map(|(nama, nilai)| nilai.map(|_| *nama))
        .collect();
    println!("Yang punya nilai: {:?}", nilai_aja);

    println!("\n7. FOLD — akumulasi nilai (paling fleksibel)");
    // fold(initial, |acc, item| new_acc)

    let angka = vec![1, 2, 3, 4, 5];

    // sum manual pake fold
    let sum = angka.iter().fold(0, |acc, x| acc + x);
    println!("Sum pake fold: {}", sum);

    // Product
    let product = angka.iter().fold(1, |acc, x| acc * x);
    println!("Product pake fold: {}", product);

    // Count
    let count = angka.iter().fold(0, |acc, _| acc + 1);
    println!("Count pake fold: {}", count);

    // String concatenation
    let kata = vec!["Rust", " ", "is", " ", "cool"];
    let kalimat = kata.iter().fold(String::new(), |acc, s| acc + s);
    println!("String pake fold: {}", kalimat);

    println!("\n8. REDUCE — fold tanpa initial value (return Option)");

    let angka = vec![3, 1, 4, 1, 5, 9];

    let max = angka.iter().copied().reduce(|acc, x| if acc > x { acc } else { x });
    println!("Max pake reduce: {:?}", max);

    let sum2 = angka.iter().copied().reduce(|acc, x| acc + x);
    println!("Sum pake reduce: {:?}", sum2);

    // reduce di empty = None
    let kosong: Vec<i32> = vec![];
    println!("Reduce empty: {:?}", kosong.iter().copied().reduce(|a, b| a + b));

    println!("\n9. FLAT_MAP & FLATTEN");
    // flat_map = map + flatten — setiap item bisa jadi 0..n item

    // flat_map dengan nested vec
    let data = vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![6, 7, 8, 9],
    ];
    let flat: Vec<i32> = data.iter().flat_map(|v| v.iter()).copied().collect();
    println!("flat_map nested vec: {:?}", flat);

    // flat_map + expand
    let angka = vec![1, 2, 3];
    let expanded: Vec<i32> = angka.iter().flat_map(|x| std::iter::repeat(*x).take(*x as usize)).collect();
    println!("flat_map expand: {:?}", expanded);

    // flatten — sederhanain nested
    let nested = vec![
        Some(1),
        None,
        Some(2),
        Some(3),
        None,
    ];
    let flattened: Vec<i32> = nested.iter().flatten().copied().collect();
    // flatten di Option = skip None
    println!("flatten Option: {:?}", flattened);

    println!("\n10. CHAIN & ZIP");

    // chain — gabung 2 iterator
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<i32> = a.iter().chain(b.iter()).copied().collect();
    println!("chain: {:?}", chained);

    // zip — gabung 2 iterator jadi tuple
    let nama = vec!["Budi", "Siti", "Andi"];
    let umur = vec![25, 30, 22];
    let zipped: Vec<(&str, i32)> = nama.iter().zip(umur.iter()).map(|(n, u)| (*n, *u)).collect();
    println!("zip: {:?}", zipped);

    // zip dengan range
    let buah = vec!["Apel", "Mangga", "Jeruk"];
    let indexed: Vec<(usize, &str)> = (0..).zip(buah.iter()).map(|(i, b)| (i, *b)).collect();
    println!("zip dengan range: {:?}", indexed);

    println!("\n11. SKIP, TAKE, SKIP_WHILE, TAKE_WHILE");

    let angka = 1..=20;

    let first_5: Vec<i32> = angka.clone().take(5).collect();
    println!("take 5: {:?}", first_5);

    let after_10: Vec<i32> = (1..=20).skip(10).collect();
    println!("skip 10: {:?}", after_10);

    // take_while — ambil selama kondisi true
    let ambil_genap: Vec<i32> = (1..=20).take_while(|x| x % 2 != 0).collect();
    println!("take_while ganjil: {:?}", ambil_genap);

    // skip_while — skip selama kondisi true
    let angka2 = vec![0, 0, 0, 1, 2, 0, 3];
    let setelah_nol: Vec<i32> = angka2.iter().skip_while(|x| **x == 0).copied().collect();
    println!("skip_while 0: {:?}", setelah_nol);

    println!("\n12. COLLECT — ke berbagai koleksi");

    // collect ke Vec (default)
    let angka: Vec<i32> = (1..=5).collect();
    println!("Vec: {:?}", angka);

    // collect ke HashSet (unique)
    use std::collections::HashSet;
    let unik: HashSet<i32> = vec![1, 2, 2, 3, 3, 3].into_iter().collect();
    println!("HashSet: {:?}", unik);

    // collect ke HashMap
    use std::collections::HashMap;
    let map: HashMap<&str, i32> = vec![("a", 1), ("b", 2)].into_iter().collect();
    println!("HashMap: {:?}", map);

    // collect ke String
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let s: String = chars.iter().collect();
    println!("String: {}", s);

    // collect ke Result<Vec<T>, E> — berhenti kalo ada error
    let strings = vec!["1", "2", "3"];
    let parsed: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("Result<Vec>: {:?}", parsed);

    // collect ke Option<Vec<T>> — None kalo ada None
    let items = vec![Some(1), Some(2), Some(3)];
    let semua: Option<Vec<i32>> = items.into_iter().collect();
    println!("Option<Vec>: {:?}", semua);

    println!("\n13. PARTITION, UNZIP");

    let angka = vec![1, 2, 3, 4, 5, 6];
    let (genap, ganjil): (Vec<i32>, Vec<i32>) = angka.iter().partition(|x| *x % 2 == 0);
    println!("partition genap: {:?}", genap);
    println!("partition ganjil: {:?}", ganjil);

    // unzip — pecahin Vec<(A, B)> jadi (Vec<A>, Vec<B>)
    let pasangan = vec![("Budi", 25), ("Siti", 30), ("Andi", 22)];
    let (nama, umur): (Vec<&str>, Vec<i32>) = pasangan.into_iter().unzip();
    println!("unzip nama: {:?}", nama);
    println!("unzip umur: {:?}", umur);

    println!("\n14. FIND, POSITION, ANY, ALL");

    let angka = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // find — cari first match
    let pertama_genap = angka.iter().find(|x| *x % 2 == 0);
    println!("find genap: {:?}", pertama_genap);

    // position — cari index first match
    let pos = angka.iter().position(|x| *x > 5);
    println!("position >5: {:?}", pos);

    // any — apa ada yg match?
    let ada_besar = angka.iter().any(|x| *x > 8);
    println!("any >8: {}", ada_besar);

    // all — semua match?
    let semua_kecil = angka.iter().all(|x| *x < 20);
    println!("all <20: {}", semua_kecil);

    println!("\n15. ENUMERATE, REV, CYCLE");

    // enumerate — dapet index
    let buah = vec!["Apel", "Mangga", "Jeruk"];
    for (i, item) in buah.iter().enumerate() {
        println!("{}: {}", i, item);
    }

    // rev — balik urutan
    let reversed: Vec<i32> = (1..=5).rev().collect();
    println!("rev: {:?}", reversed);

    // cycle — muter terus
    let cyclic: Vec<i32> = vec![1, 2, 3].iter().cycle().take(10).copied().collect();
    println!("cycle: {:?}", cyclic);

    println!("\n16. CLOSURES DEEP DIVE");

    // Fn — bisa dipanggil berkali-kali, immutable borrow
    // FnMut — bisa dipanggil berkali-kali, mutable borrow
    // FnOnce — cuma bisa dipanggil 1x, bisa take ownership

    // Fn: closure yang gak ngubah captured variable
    let faktor = 3;
    let kali = |x: i32| x * faktor;
    println!("Fn closure: {}", kali(5));
    println!("Fn closure lagi: {}", kali(10));

    // FnMut: closure yang ngubah captured variable
    let mut total = 0;
    let mut akumulator = |x: i32| {
        total += x;
        total
    };
    println!("FnMut: {}", akumulator(5));
    println!("FnMut: {}", akumulator(10));
    println!("FnMut: {}", akumulator(15));

    // FnOnce: closure yang consume captured value
    let s = String::from("halo");
    let consume = || {
        drop(s); // s di-move ke closure
    };
    consume();
    // consume(); // ERROR: udah di-call 1x, s udah di-drop

    // move keyword — paksa closure take ownership
    let msg = String::from("dunia");
    let print_msg = move || {
        println!("{}", msg);
        // msg di-drop setelah closure di-call
    };
    print_msg();
    // println!("{}", msg); // ERROR: msg udah di-move

    println!("\n17. COMPOSING COMBINATORS");

    // Chaining itu kekuatan Rust iterator
    let result: Vec<String> = (1..=20)
        .filter(|x| x % 2 == 1)           // ganjil
        .map(|x| x * 3)                   // kali 3
        .filter(|x| *x > 10)              // > 10
        .map(|x| format!("Angka {}", x)) // jadi string
        .collect();

    println!("Composed: {:?}", result);

    // Contoh real: cari kata terpanjang
    let kalimat = "Rust is a systems programming language";
    let kata_terpanjang = kalimat
        .split_whitespace()
        .map(|s| (s.len(), s))
        .reduce(|a, b| if a.0 >= b.0 { a } else { b })
        .map(|(_, kata)| kata);

    println!("Kata terpanjang: {:?}", kata_terpanjang);

    println!("\n18. CUSTOM ITERATOR");

    let fib = Fibonacci::baru(50);
    println!("Fibonacci < 50:");
    for (i, val) in fib.enumerate() {
        println!("  fib[{}] = {}", i, val);
    }

    println!("\n19. INTO_ITERATOR CUSTOM");

    let koleksi = KoleksiBuah {
        buah: vec![
            String::from("Apel"),
            String::from("Mangga"),
            String::from("Jeruk"),
        ],
    };

    for buah in koleksi {
        println!("Buah: {}", buah);
    }

    println!("\n20. LAZY EVALUATION");

    // Iterator itu LAZY — gak ngejalanin apa2 sampe di-consumed
    let angka = vec![1, 2, 3, 4, 5];

    let lazy = angka.iter().map(|x| {
        println!("  map dipanggil buat {}", x);
        x * 2
    });
    println!("Belum ada yg dipanggil...");

    // Baru di sini dieksekusi
    let result: Vec<i32> = lazy.collect();
    println!("Hasil: {:?}", result);

    // for_each — kalo gak butuh return value, lebih cepat dari for loop
    println!("\nfor_each:");
    (1..=5).for_each(|x| print!("{} ", x));
    println!();
}
