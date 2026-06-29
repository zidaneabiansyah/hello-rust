#[allow(non_camel_case_types, dead_code)]
fn main() {
    println!("=== 25. const Generics & const fn ===\n");

    // -------------------------------------------------------
    // 1. const Generics Dasar
    // -------------------------------------------------------
    println!("--- 1. Basic const Generics ---");

    // const generics: parameter泛型 yang berupa nilai konstan
    // Berguna untuk array ukuran tetap, matrix, etc.

    let arr3 = ArrayWrapper { data: [1, 2, 3] };
    let arr5 = ArrayWrapper { data: [10, 20, 30, 40, 50] };
    let arr2 = ArrayWrapper { data: [100, 200] };

    println!("arr3: {:?}", arr3.data);
    println!("arr3 sum: {}", arr3.sum());
    println!("arr5: {:?}", arr5.data);
    println!("arr5 sum: {}", arr5.sum());
    println!("arr2: {:?}", arr2.data);
    println!("arr2 sum: {}", arr2.sum());

    // Tipe berbeda untuk N berbeda!
    // ArrayWrapper<[i32; 3]> != ArrayWrapper<[i32; 5]>

    // -------------------------------------------------------
    // 2. const Generics dengan Trait Bounds
    // -------------------------------------------------------
    println!("\n--- 2. const Generics with Trait Bounds ---");

    let m3x3 = Matrix::<f64, 3, 3>::identity();
    println!("3x3 Identity Matrix:");
    m3x3.print();

    let m2x3 = Matrix::<f64, 2, 3>::new([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ]);
    println!("\n2x3 Matrix:");
    m2x3.print();

    // -------------------------------------------------------
    // 3. const fn
    // -------------------------------------------------------
    println!("\n--- 3. const fn ---");

    // const fn: fungsi yang bisa dijalankan saat compile time
    // Berguna untuk: array init, const expressions, etc.

    const fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    const fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // Dijalankan saat compile time!
    const RESULT: i32 = add(10, 20);
    const PRODUCT: i32 = multiply(6, 7);

    println!("const add(10, 20) = {}", RESULT);
    println!("const multiply(6, 7) = {}", PRODUCT);

    // const fn juga bisa dijalankan saat runtime
    let runtime_result = add(100, 200);
    println!("runtime add(100, 200) = {}", runtime_result);

    // -------------------------------------------------------
    // 4. const fn dengan Conditional Logic
    // -------------------------------------------------------
    println!("\n--- 4. const fn with Conditions ---");

    const fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }

    const fn abs_val(x: i32) -> i32 {
        if x < 0 {
            -x
        } else {
            x
        }
    }

    const MAX_VAL: i32 = max(10, 20);
    const ABS_NEG: i32 = abs_val(-42);

    println!("max(10, 20) = {}", MAX_VAL);
    println!("abs(-42) = {}", ABS_NEG);

    // -------------------------------------------------------
    // 5. const fn dengan Loops
    // -------------------------------------------------------
    println!("\n--- 5. const fn with Loops ---");

    const fn factorial(n: u64) -> u64 {
        let mut result = 1u64;
        let mut i = 1u64;
        while i <= n {
            result *= i;
            i += 1;
        }
        result
    }

    const fn fibonacci(n: u64) -> u64 {
        if n <= 1 {
            return n;
        }
        let mut a = 0u64;
        let mut b = 1u64;
        let mut i = 2;
        while i <= n {
            let temp = a + b;
            a = b;
            b = temp;
            i += 1;
        }
        b
    }

    const F5: u64 = factorial(5);
    const F10: u64 = factorial(10);
    const FIB8: u64 = fibonacci(8);

    println!("factorial(5) = {}", F5);
    println!("factorial(10) = {}", F10);
    println!("fibonacci(8) = {}", FIB8);

    // -------------------------------------------------------
    // 6. Array Initialization dengan const fn
    // -------------------------------------------------------
    println!("\n--- 6. Array Init with const fn ---");

    const fn make_squares<const N: usize>() -> [i32; N] {
        let mut arr = [0i32; N];
        let mut i = 0;
        while i < N {
            arr[i] = (i * i) as i32;
            i += 1;
        }
        arr
    }

    const SQUARES_5: [i32; 5] = make_squares::<5>();
    const SQUARES_10: [i32; 10] = make_squares::<10>();

    println!("squares[5]: {:?}", SQUARES_5);
    println!("squares[10]: {:?}", SQUARES_10);

    // -------------------------------------------------------
    // 7. const Generics untuk Type-Level Programming
    // -------------------------------------------------------
    println!("\n--- 7. Type-Level Programming ---");

    // Menggunakan const generics untuk encode informasi di type level

    struct BitArray<const N: usize> {
        data: [u8; N],
    }

    impl<const N: usize> BitArray<N> {
        fn new() -> Self {
            BitArray { data: [0u8; N] }
        }

        fn set(&mut self, index: usize, value: bool) {
            if index < N * 8 {
                let byte_idx = index / 8;
                let bit_idx = index % 8;
                if value {
                    self.data[byte_idx] |= 1 << bit_idx;
                } else {
                    self.data[byte_idx] &= !(1 << bit_idx);
                }
            }
        }

        fn get(&self, index: usize) -> bool {
            if index < N * 8 {
                let byte_idx = index / 8;
                let bit_idx = index % 8;
                (self.data[byte_idx] >> bit_idx) & 1 == 1
            } else {
                false
            }
        }

        fn count_ones(&self) -> u32 {
            self.data.iter().map(|b| b.count_ones()).sum()
        }
    }

    let mut bits = BitArray::<4>::new(); // 32 bits
    bits.set(0, true);
    bits.set(5, true);
    bits.set(31, true);

    println!("bit 0: {}", bits.get(0));
    println!("bit 5: {}", bits.get(5));
    println!("bit 31: {}", bits.get(31));
    println!("bit 1: {}", bits.get(1));
    println!("ones count: {}", bits.count_ones());

    // -------------------------------------------------------
    // 8. const Generics untuk Matrix Math
    // -------------------------------------------------------
    println!("\n--- 8. Matrix Math ---");

    struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        fn new(data: [[T; COLS]; ROWS]) -> Self {
            Matrix { data }
        }
    }

    // Workaround: implement manual untuk ukuran spesifik
    impl<const R: usize, const C: usize> Matrix<f64, R, C> {
        fn zero() -> Self {
            Matrix {
                data: [[0.0; C]; R],
            }
        }
    }

    impl<const N: usize> Matrix<f64, N, N> {
        fn identity() -> Self {
            let mut m = [[0.0f64; N]; N];
            let mut i = 0;
            while i < N {
                m[i][i] = 1.0;
                i += 1;
            }
            Matrix { data: m }
        }
    }

    impl<const R: usize, const C: usize> Matrix<f64, R, C> {
        fn print(&self) {
            for row in &self.data {
                print!("  [");
                for (j, val) in row.iter().enumerate() {
                    if j > 0 {
                        print!(", ");
                    }
                    print!("{:6.2}", val);
                }
                println!("]");
            }
        }
    }

    // -------------------------------------------------------
    // 9. const Generics untuk Buffer Sizes
    // -------------------------------------------------------
    println!("\n--- 9. Buffer Sizes ---");

    struct Buffer<const SIZE: usize> {
        data: [u8; SIZE],
        len: usize,
    }

    impl<const SIZE: usize> Buffer<SIZE> {
        fn new() -> Self {
            Buffer {
                data: [0u8; SIZE],
                len: 0,
            }
        }

        fn capacity(&self) -> usize {
            SIZE
        }

        fn remaining(&self) -> usize {
            SIZE - self.len
        }

        fn push(&mut self, byte: u8) -> bool {
            if self.len < SIZE {
                self.data[self.len] = byte;
                self.len += 1;
                true
            } else {
                false // buffer full
            }
        }
    }

    let mut buf = Buffer::<16>::new();
    for i in 0..20 {
        if buf.push(i) {
            print!("{:2} ", i);
        } else {
            print!("X "); // buffer full
        }
    }
    println!();
    println!("capacity: {}, remaining: {}", buf.capacity(), buf.remaining());

    println!("\n=== Ringkasan const Generics & const fn ===");
    println!("1. const generics - parameter泛型 bernilai konstan");
    println!("2. Array<[T; N]> - tipe berbeda untuk N berbeda");
    println!("3. const fn - fungsi yang bisa dijalankan saat compile time");
    println!("4. Const expressions - nilai dihitung saat compile");
    println!("5. Array initialization - make_squares::<N>()");
    println!("6. Type-level programming - encode informasi di tipe");
    println!("7. Buffer sizes - Buffer<16> vs Buffer<256>");
    println!("8. Matrix dimensions - Matrix<f64, 3, 3>");
}

// ============================================================
// Array Wrapper dengan const generics
// ============================================================

struct ArrayWrapper<const N: usize> {
    data: [i32; N],
}

impl<const N: usize> ArrayWrapper<N> {
    fn sum(&self) -> i32 {
        self.data.iter().sum()
    }
}
