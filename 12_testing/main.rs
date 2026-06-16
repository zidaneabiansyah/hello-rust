/*
TESTING DI RUST

Rust punya 3 level test:
1. Unit test — di dalem file sumber, pake #[cfg(test)], bisa akses private fn
2. Doc test — di dalem doc comment (///), ngejamin contoh kode di docs jalan
3. Integration test — di folder tests/, nguji API dari luar (pake library crate)
*/

// Fungsi yg bakal kita test
fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

/// Ngitung luas lingkaran
///
/// # Example
/// ```
/// fn luas_lingkaran(r: f64) -> f64 { 3.14 * r * r }
///
/// let luas = luas_lingkaran(7.0);
/// assert!((luas - 153.86).abs() < 0.01);
/// ```
/// (Note: di library crate, doc test bisa pake `use crate_name::fn;`)
fn luas_lingkaran(r: f64) -> f64 {
    3.14 * r * r
}

struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn baru(max: u32) -> Self {
        Counter { count: 0, max }
    }

    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    println!("Testing di Rust");

    println!("tambah(2, 3) = {}", tambah(2, 3));
    println!("bagi(10, 2) = {:?}", bagi(10, 2));
    println!("luas_lingkaran(7.0) = {:.2}", luas_lingkaran(7.0));

    let mut counter = Counter::baru(3);
    while let Some(val) = counter.next() {
        println!("Counter: {}", val);
    }
}

// UNIT TEST
// #[cfg(test)] — cuma di-compile pas `cargo test`, gak ikut ke binary production
#[cfg(test)]
mod tests {
    // Kalo pake `use super::*;`, semua item di parent bisa diakses (termasuk private fn)
    use super::*;

    // #[test] — Rust server buat fungsi ini sebagai test
    #[test]
    fn test_tambah() {
        assert_eq!(tambah(2, 3), 5);
        assert_eq!(tambah(-1, 1), 0);
        assert_eq!(tambah(0, 0), 0);
    }

    // assert! — ngecek boolean
    #[test]
    fn test_tambah_positif() {
        let hasil = tambah(10, 5);
        assert!(hasil > 0);
    }

    // assert_ne! — ngecek not equal
    #[test]
    fn test_tambah_bukan_negative() {
        assert_ne!(tambah(5, 5), -10);
    }

    // Test Result<T, E>
    #[test]
    fn test_bagi_sukses() {
        let hasil = bagi(10, 2).unwrap();
        assert_eq!(hasil, 5);
    }

    #[test]
    fn test_bagi_error() {
        let hasil = bagi(10, 0);
        assert!(hasil.is_err());
    }

    // #[should_panic] — test harus panic kalo mau lolos
    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_bagi_panic() {
        let _ = bagi(10, 0).unwrap();
        // Kalo ga panic, test FAILED
    }

    // Test struct dengan state
    #[test]
    fn test_counter_mulai_dari_nol() {
        let counter = Counter::baru(5);
        assert_eq!(counter.count, 0);
    }

    #[test]
    fn test_counter_next() {
        let mut counter = Counter::baru(3);
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_counter_habis() {
        let mut counter = Counter::baru(1);
        counter.next();
        assert_eq!(counter.next(), None);
    }

    // Test dengan Result<T, E> — return Ok() kalo sukses, Err() kalo gagal
    // Bisa pake ? operator di dalem test
    #[test]
    fn test_result_ok() -> Result<(), String> {
        let hasil = tambah(2, 2);
        if hasil == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 harusnya 4"))
        }
    }

    #[test]
    fn test_with_question_mark() -> Result<(), String> {
        let hasil = bagi(10, 2)?;
        assert_eq!(hasil, 5);
        Ok(())
    }
}

// DOC TEST
// Di atas fungsi luas_lingkaran ada doc test — jalanin pake `cargo test`
// Rust bakal nge-compile contoh di doc comment dan jalanin sebagai test
