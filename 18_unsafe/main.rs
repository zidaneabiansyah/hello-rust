#[allow(unused_variables, dead_code)]
fn main() {
    println!("=== 18. Unsafe Rust ===\n");

    // -------------------------------------------------------
    // 1. Raw Pointers
    // -------------------------------------------------------
    println!("--- 1. Raw Pointers ---");

    let mut val = 42;
    // Membuat raw pointer dari reference (aman, tidak dereference)
    let r1 = &val as *const i32;   // const raw pointer
    let r2 = &mut val as *mut i32;  // mut raw pointer

    println!("r1 (alamat): {:?}, r2 (alamat): {:?}", r1, r2);

    // Dereference harus dalam unsafe block
    unsafe {
        println!("r1 dereference: {}", *r1);
        println!("r2 dereference: {}", *r2);
        *r2 = 100;
        println!("setelah *r2 = 100, val = {}", val);
    }

    // Raw pointer bisa null (tidak ada di safe Rust)
    let null_ptr: *const i32 = std::ptr::null();
    println!("null_ptr is_null: {}", null_ptr.is_null()); // true

    // Membuat raw pointer dari alamat mentah (bahaya!)
    let addr = 0x012345usize;
    let _wild_ptr = addr as *const i32;

    // -------------------------------------------------------
    // 2. Unsafe Functions
    // -------------------------------------------------------
    println!("\n--- 2. Unsafe Functions ---");

    let mut vec = vec![1, 2, 3, 4, 5];

    let (left, right) = split_at_mut_unchecked(&mut vec, 2);
    println!("left: {:?}, right: {:?}", left, right);

    unsafe {
        let result = dangerous_add(10, 20);
        println!("dangerous_add(10, 20) = {}", result);
    }

    // -------------------------------------------------------
    // 3. Implementing Safe Abstractions over Unsafe Code
    // -------------------------------------------------------
    println!("\n--- 3. Safe Abstraction ---");

    let mut data = vec![10, 20, 30, 40, 50];
    let first = get_first_element(&data);
    println!("first element: {}", first);

    // Interface tetap aman bagi user
    data.push(60);
    println!("after push: {:?}", data);

    // -------------------------------------------------------
    // 4. Mutable Static Variables
    // -------------------------------------------------------
    println!("\n--- 4. Mutable Static Variables ---");

    static mut COUNTER: u32 = 0;

    unsafe {
        COUNTER += 1;
        COUNTER += 1;
        COUNTER += 1;
        // Menggunakan read_volatile untuk akses yang aman
        println!("COUNTER = {}", std::ptr::read_volatile(&COUNTER));
    }

    // Mutable static berbahaya karena bisa race condition
    // Lebih baik pakai: AtomicU32, Mutex, atau thread-local

    // -------------------------------------------------------
    // 5. Unsafe Traits
    // -------------------------------------------------------
    println!("\n--- 5. Unsafe Traits ---");

    let my_type = MyUnsafeType { value: 42 };
    unsafe {
        MyUnsafeTrait::do_something(&my_type);
    }

    // -------------------------------------------------------
    // 6. Union (only in unsafe)
    // -------------------------------------------------------
    println!("\n--- 6. Union ---");

    #[repr(C)]
    union IntOrFloat {
        i: i32,
        f: f32,
    }

    let val = IntOrFloat { i: 42 };
    unsafe {
        println!("as int: {}", val.i);
        println!("as float: {}", val.f); // mungkin bit yang tidak valid
    }

    // -------------------------------------------------------
    // 7. transmute
    // -------------------------------------------------------
    println!("\n--- 7. transmute ---");

    // Method yang lebih aman dan modern:
    let x: f32 = 3.14;
    let bits: u32 = x.to_bits();
    println!("f32 {} as bits: {:#010x}", x, bits);

    // Reverse
    let back: f32 = f32::from_bits(bits);
    println!("bits {:#010x} as f32: {}", bits, back);

    // transmute tetap bisa untuk konversi layout yang identik
    #[repr(C)]
    struct A { x: i32, y: f64 }
    #[repr(C)]
    struct B { x: i32, y: f64 }

    let a = A { x: 1, y: 2.0 };
    let _b: B = unsafe { std::mem::transmute(a) };
    println!("transmute A -> B: OK (sama layout karena #[repr(C)])");

    // -------------------------------------------------------
    // 8. Calling Unsafe Function Pointers
    // -------------------------------------------------------
    println!("\n--- 8. Function Pointers ---");

    let fn_ptr: unsafe fn(i32) -> i32 = double_value;
    let result = unsafe { fn_ptr(21) };
    println!("double_value(21) = {}", result);

    println!("\n=== Ringkasan Unsafe Rust ===");
    println!("1. Raw pointers (*const, *mut) - buat & dereference");
    println!("2. Unsafe functions - buka blok unsafe");
    println!("3. Mutable static variables - COUNTER global");
    println!("4. Unsafe traits - implementasi yang tidak bisa diverifikasi");
    println!("5. Union - akses field tanpa tag");
    println!("6. transmute - konversi tipe di level bit");
    println!("7. FFI calls - panggil kode bahasa lain");
}

// Unsafe function: caller HARUS dalam unsafe block
unsafe fn dangerous_add(a: i32, b: i32) -> i32 {
    a + b
}

// Safe wrapper around unsafe code
fn split_at_mut_unchecked(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len, "mid out of bounds");

    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Safe abstraction: user tidak perlu tahu ada unsafe di dalam
fn get_first_element(data: &[i32]) -> i32 {
    assert!(!data.is_empty(), "slice kosong");
    unsafe {
        *data.as_ptr() // aman karena sudah assert tidak kosong
    }
}

// Unsafe trait
unsafe trait MyUnsafeTrait {
    unsafe fn do_something(&self);
}

struct MyUnsafeType {
    value: i32,
}

// Implementasi unsafe trait harus pakai unsafe
unsafe impl MyUnsafeTrait for MyUnsafeType {
    unsafe fn do_something(&self) {
        println!("MyUnsafeType doing something with value: {}", self.value);
    }
}

// Unsafe function
unsafe fn double_value(x: i32) -> i32 {
    x * 2
}
