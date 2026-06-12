// 05_TRAITS — Trait, Implementasi, Trait Bound, Derive

// Trait sederhana
trait Deskripsi {
    fn deskripsi(&self) -> String;
}

// Trait dengan default method
trait Gerak {
    fn kecepatan(&self) -> f64;

    // Method default — bisa di-override
    fn deskripsi_gerak(&self) -> String {
        format!("Bergerak dengan kecepatan {} km/h", self.kecepatan())
    }
}

// Struct untuk implementasi trait
struct Mobil {
    merek: String,
    kecepatan_maks: f64,
}

struct Sepeda {
    merek: String,
    gear: u32,
}

struct Kucing {
    nama: String,
}

// Implementasi trait Deskripsi
impl Deskripsi for Mobil {
    fn deskripsi(&self) -> String {
        format!("Mobil {} dengan kecepatan maks {} km/h", self.merek, self.kecepatan_maks)
    }
}

impl Deskripsi for Sepeda {
    fn deskripsi(&self) -> String {
        format!("Sepeda {} dengan {} gear", self.merek, self.gear)
    }
}

impl Deskripsi for Kucing {
    fn deskripsi(&self) -> String {
        format!("Kucing bernama {}", self.nama)
    }
}

// Implementasi trait Gerak
impl Gerak for Mobil {
    fn kecepatan(&self) -> f64 {
        self.kecepatan_maks
    }
}

impl Gerak for Sepeda {
    fn kecepatan(&self) -> f64 {
        self.gear as f64 * 5.0
    }
}

// Kucing tidak implement Gerak, cuma Deskripsi

// Trait dengan generic (trait bound)
fn cetak_deskripsi(item: &impl Deskripsi) {
    println!("{}", item.deskripsi());
}

// Syntax alternatif: trait bound dengan generics
fn cetak_deskripsi2<T: Deskripsi>(item: &T) {
    println!("{}", item.deskripsi());
}

// Multiple trait bounds
fn cetak_semuanya(item: &(impl Deskripsi + Gerak)) {
    println!("{}", item.deskripsi());
    println!("{}", item.deskripsi_gerak());
}

// where clause
fn cetak_lengkap<T>(item: &T)
where
    T: Deskripsi + Gerak,
{
    println!("LENGKAP:");
    println!("  {}", item.deskripsi());
    println!("  {}", item.deskripsi_gerak());
}

// Trait return
fn buat_kendaraan() -> impl Deskripsi {
    Mobil {
        merek: String::from("Civic"),
        kecepatan_maks: 220.0,
    }
}

// Derive (auto-implement trait)
#[derive(Debug, Clone, PartialEq)]
struct Produk {
    nama: String,
    harga: f64,
}

// Trait dengan associated type
trait Konversi {
    type Output;
    fn konversi(&self) -> Self::Output;
}

struct Celcius(f64);
struct Fahrenheit(f64);

impl Konversi for Celcius {
    type Output = Fahrenheit;
    fn konversi(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

impl Konversi for Fahrenheit {
    type Output = Celcius;
    fn konversi(&self) -> Celcius {
        Celcius((self.0 - 32.0) * 5.0 / 9.0)
    }
}

fn main() {
    // Basic trait
    println!("TRAIT - DESKRIPSI");

    let mobil = Mobil {
        merek: String::from("Toyota"),
        kecepatan_maks: 180.0,
    };
    let sepeda = Sepeda {
        merek: String::from("Polygon"),
        gear: 21,
    };
    let kucing = Kucing {
        nama: String::from("Mimi"),
    };

    println!("{}", mobil.deskripsi());
    println!("{}", sepeda.deskripsi());
    println!("{}", kucing.deskripsi());

    // Default method
    println!("\nDEFAULT METHOD");

    println!("{}", mobil.deskripsi_gerak());
    println!("{}", sepeda.deskripsi_gerak());
    // kucing gak punya gerak

    // Trait bound
    println!("\nTRAIT BOUND");
    cetak_deskripsi(&mobil);
    cetak_deskripsi(&sepeda);
    cetak_deskripsi(&kucing);

    println!("\nMULTIPLE TRAIT BOUND");
    cetak_semuanya(&mobil);
    cetak_semuanya(&sepeda);

    println!("\nWHERE CLAUSE");
    cetak_lengkap(&mobil);
    cetak_lengkap(&sepeda);

    // Trait return
    println!("\nTRAIT RETURN");
    let kendaraan = buat_kendaraan();
    println!("{}", kendaraan.deskripsi());

    // Derive
    println!("\nDERIVE");
    let produk1 = Produk {
        nama: String::from("Laptop"),
        harga: 15000.0,
    };
    let produk2 = Produk {
        nama: String::from("Laptop"),
        harga: 15000.0,
    };

    println!("{:?}", produk1);
    println!("Clone: {:?}", produk1.clone());
    println!("produk1 == produk2: {}", produk1 == produk2);

    // Associated type
    println!("\nASSOCIATED TYPE");

    let c = Celcius(100.0);
    let f = c.konversi();
    println!("100°C = {}°F", f.0);

    let f2 = Fahrenheit(212.0);
    let c2 = f2.konversi();
    println!("212°F = {}°C", c2.0);

    // Trait standar: Iterator
    println!("\nTRAIT ITERATOR (MANUAL)");

    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn baru(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter::baru(5);
    while let Some(val) = counter.next() {
        println!("Counter: {}", val);
    }
}
