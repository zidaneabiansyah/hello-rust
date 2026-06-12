// 06_GENERICS — Generic Functions, Struct, Enum, Trait Bounds, Monomorphization

// Generic function
fn ambil_terbesar<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Generic struct dengan multiple type params
struct Pair<A, B> {
    first: A,
    second: B,
}

// Generic impl — method khusus untuk Point<f64>
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic impl — method untuk semua Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Generic impl dengan constraint
impl<T: Clone + std::fmt::Debug> Pair<T, T> {
    fn cetak_keduanya(&self) {
        println!("first: {:?}, second: {:?}", self.first, self.second);
    }
}

// Multiple trait bounds
fn cetak_dan_kali<T>(val: T, faktor: i32)
where
    T: std::fmt::Display + std::ops::Mul<i32, Output = i32> + Copy,
{
    let hasil = val * faktor;
    println!("{} x {} = {}", val, faktor, hasil);
}

// Generic dengan associated type (dari trait)
trait IntoString {
    type Target;
    fn convert(self) -> String;
}

impl IntoString for String {
    type Target = String;
    fn convert(self) -> String {
        self
    }
}

impl IntoString for i32 {
    type Target = i32;
    fn convert(self) -> String {
        format!("Angka: {}", self)
    }
}

fn cetak_via_trait<T: IntoString>(item: T) {
    println!("{}", item.convert());
}

// Monomorphization — Rust bikin kode khusus untuk setiap tipe di compile time
fn identitas<T>(val: T) -> T {
    val
}

fn main() {
    // Generic function
    println!("GENERIC FUNCTION");

    let max_int = ambil_terbesar(10, 20);
    let max_float = ambil_terbesar(3.14, 2.71);
    let max_char = ambil_terbesar('a', 'z');

    println!("Terbesar int: {}", max_int);
    println!("Terbesar float: {}", max_float);
    println!("Terbesar char: {}", max_char);

    // Generic struct
    println!("\nGENERIC STRUCT");

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 3.0, y: 4.0 };
    let _string_point = Point {
        x: String::from("kiri"),
        y: String::from("kanan"),
    };

    println!("int_point: ({}, {})", int_point.x, int_point.y);
    println!("float_point: ({}, {})", float_point.x, float_point.y);

    // Method khusus untuk Point<f64>
    println!("Distance from origin: {:.2}", float_point.distance_from_origin());

    // Generic method
    println!("x dari int_point: {}", int_point.x());

    // Multiple type params
    println!("\nMULTIPLE TYPE PARAMS");

    let pair = Pair {
        first: String::from("Halo"),
        second: 42,
    };
    println!("pair: ({}, {})", pair.first, pair.second);

    let pair2 = Pair {
        first: 1.0,
        second: 2.0,
    };
    pair2.cetak_keduanya();

    // Generic dengan trait bound
    println!("\nTRAIT BOUND LANJUTAN");

    cetak_dan_kali(5, 3);
    // cetak_dan_kali(3.14, 2); // ERROR: f32 ga implement Mul<i32, Output=i32>

    // Associated type
    println!("\nASSOCIATED TYPE");

    let teks = String::from("hello");
    cetak_via_trait(teks);
    cetak_via_trait(42);

    // Monomorphization — semua beres di compile time
    println!("\nMONOMORPHIZATION");

    let a = identitas(100);
    let b = identitas(3.14);
    let c = identitas("Rust");

    println!("{} {} {}", a, b, c);
}
