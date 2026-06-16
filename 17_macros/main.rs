/*
MACROS (macro_rules!) — Metaprogramming di Rust

Macro itu code yang nulis code (code generation).
Bedanya sama function:
- Macro jalan di COMPILE TIME
- Macro bisa pake syntax Rust yang gak valid di function
- Macro flexible: bisa terima variadic args, beda tipe, dll

macro_rules! adalah "declarative macro" — pola declarative yang di-match.
*/

// Macro paling sederhana — kayak function tapi tanpa argumen
macro_rules! halo {
    () => {
        println!("Halo dari macro!");
    };
}

// Macro dengan parameter: $nama:tipe
// $x:expr — expression
macro_rules! sapa {
    ($nama:expr) => {
        println!("Halo {}, selamat datang!", $nama);
    };
}

// Macro dengan multiple args
macro_rules! jumlah {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// Macro dengan block expression
macro_rules! ulang {
    ($n:expr, $body:block) => {
        for _ in 0..$n {
            $body;
        }
    };
}

// Repetition — $(...),*
// Kayak kita pake vec![] atau println!()
macro_rules! buat_map {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($k, $v);)*
        map
    }};
}

// Macro untuk generate getter/setter sederhana
macro_rules! struct_with_getter {
    ($name:ident { $($field:ident : $type:ty),* $(,)? }) => {
        struct $name {
            $($field: $type),*
        }

        impl $name {
            $(
                fn $field(&self) -> &$type {
                    &self.$field
                }
            )*
        }
    };
}

// Macro debug — ngeprint nama variable + nilainya
macro_rules! debug_print {
    ($($x:expr),*) => {
        $(
            println!("  {} = {:?}", stringify!($x), $x);
        )*
    };
}

// Macro generate function
macro_rules! buat_faktorial {
    () => {
        fn faktorial(n: u64) -> u64 {
            if n <= 1 { 1 } else { n * faktorial(n - 1) }
        }
    };
}

buat_faktorial!();

// Macro dengan pattern matching
macro_rules! cek_tipe {
    ($val:expr) => {
        match $val {
            x if std::any::TypeId::of::<String>() == std::any::TypeId::of::<String>() => {
                // Gak bisa dynamic type checking di Rust — ini cuma contoh pattern
                println!("  nilainya: {}", x);
            }
            _ => println!("  unknown type"),
        }
    };
}

// vec! macro buatan sendiri
macro_rules! my_vec {
    ($($x:expr),*) => {{
        let mut v = Vec::new();
        $(v.push($x);)*
        v
    }};
    ($($x:expr),+; $n:expr) => {{
        let mut v = Vec::new();
        for _ in 0..$n {
            $(v.push($x.clone());)*
        }
        v
    }};
}

// Macro untuk bikin enum dengan method
macro_rules! enum_with_display {
    ($name:ident { $($var:ident => $desc:expr),* $(,)? }) => {
        enum $name {
            $($var),*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$var => write!(f, $desc)),*
                }
            }
        }
    };
}

enum_with_display! {
    Warna {
        Merah => "Warna Merah",
        Hijau => "Warna Hijau",
        Biru => "Warna Biru",
    }
}

// Macro yang bisa detect tipe argumen
macro_rules! assert_type {
    (string: $val:expr) => {
        assert!(std::any::TypeId::of::<String>() == std::any::TypeId::of::<$val>());
    };
}

fn main() {
    println!("1. MACRO BASIC");

    halo!();
    sapa!("Budi");

    let hasil = jumlah!(5, 3);
    println!("  jumlah: {}", hasil);

    ulang!(3, {
        println!("  looping...");
    });

    println!("\n2. VARIADIC MACRO");

    let map = buat_map! {
        "apel" => 1,
        "mangga" => 2,
        "jeruk" => 3,
    };
    println!("  map: {:?}", map);

    println!("\n3. MACRO GENERATE STRUCT");

    struct_with_getter! {
        Mahasiswa {
            nama: String,
            umur: i32,
            jurusan: String,
        }
    }

    let mhs = Mahasiswa {
        nama: String::from("Budi"),
        umur: 20,
        jurusan: String::from("Informatika"),
    };
    println!("  nama: {}, umur: {}, jurusan: {}", mhs.nama(), mhs.umur(), mhs.jurusan());

    println!("\n4. DEBUG PRINT MACRO");

    let x = 42;
    let y = "hello";
    let z = vec![1, 2, 3];
    debug_print!(x, y, z);

    println!("\n5. MACRO GENERATE FUNCTION");

    println!("  faktorial 5 = {}", faktorial(5));

    println!("\n6. CUSTOM VEC!");

    let v = my_vec![1, 2, 3, 4, 5];
    println!("  my_vec: {:?}", v);

    let repeated = my_vec![0; 5];
    println!("  my_vec repeat: {:?}", repeated);

    println!("\n7. ENUM MACRO");

    let warna = Warna::Merah;
    println!("  {}", warna);

    let warna2 = Warna::Biru;
    println!("  {}", warna2);

    println!("\n8. DESIGNATORS DI RUST");

    println!("  Designators (tipe parameter macro):");
    println!("    expr   -> expression");
    println!("    ident  -> identifier (nama variable/fungsi/type)");
    println!("    ty     -> type");
    println!("    block  -> block ({{ ... }})");
    println!("    stmt   -> statement");
    println!("    pat    -> pattern");
    println!("    tt     -> token tree (apa aja)");
    println!("    literal -> literal value");
    println!("    lifetime -> lifetime ('a)");
    println!("    vis    -> visibility (pub, pub(crate))");

    println!("\n9. REPETITION PATTERNS");

    // $()* — zero or more
    // $()+ — one or more
    // $(,)? — optional comma
    // $()? — optional

    macro_rules! repeat_example {
        ($($x:expr),*) => {
            format!("total {} items", count!($($x),*))
        };
    }

    // Macro helper
    macro_rules! count {
        () => (0);
        ($x:expr $(, $rest:expr)*) => (1 + count!($($rest),*));
    }

    println!("  count: {}", count!(1, 2, 3, 4, 5));

    println!("\n10. PRACTICAL USE — konfigurasi");

    macro_rules! config {
        ($($key:ident: $value:expr),* $(,)?) => {{
            let mut cfg = std::collections::BTreeMap::new();
            $(cfg.insert(stringify!($key).to_string(), format!("{}", $value));)*
            cfg
        }};
    }

    let cfg = config! {
        host: "localhost",
        port: 8080,
        debug: true,
    };
    println!("  config: {:?}", cfg);
}
