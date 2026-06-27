fn main() {
    println!("=== 21. FFI (Foreign Function Interface) ===\n");

    // -------------------------------------------------------
    // 1. Calling C Standard Library Functions
    // -------------------------------------------------------
    println!("--- 1. Calling C Standard Library ---");

    // FFI memungkinkan Rust memanggil kode C/C++ dan sebaliknya

    // Contoh: panggil fungsi C `strlen`
    extern "C" {
        fn strlen(s: *const std::os::raw::c_char) -> usize;
    }

    let c_string = std::ffi::CString::new("Hello, FFI!").expect("CString::new failed");
    let len = unsafe { strlen(c_string.as_ptr()) };
    println!("strlen of {:?} = {}", c_string, len);

    // Contoh: panggil `abs` dari C
    extern "C" {
        fn abs(n: std::os::raw::c_int) -> std::os::raw::c_int;
    }

    let result = unsafe { abs(-42) };
    println!("abs(-42) = {}", result);

    // -------------------------------------------------------
    // 2. Exposing Rust Functions to C
    // -------------------------------------------------------
    println!("\n--- 2. Exposing Rust to C ---");

    // Fungsi yang bisa dipanggil dari C harus pakai extern "C"
    #[no_mangle]
    extern "C" fn rust_add(a: i32, b: i32) -> i32 {
        println!("  [Rust] rust_add called with {} + {}", a, b);
        a + b
    }

    // Simulasi pemanggilan dari C
    let result = rust_add(10, 20);
    println!("rust_add(10, 20) = {}", result);

    // -------------------------------------------------------
    // 3. C Data Types
    // -------------------------------------------------------
    println!("\n--- 3. C-compatible Types ---");

    use std::os::raw::{c_char, c_double, c_float, c_int, c_long, c_uint};

    // Tipe C yang tersedia di Rust:
    println!("c_int: {} bytes", std::mem::size_of::<c_int>());
    println!("c_uint: {} bytes", std::mem::size_of::<c_uint>());
    println!("c_long: {} bytes", std::mem::size_of::<c_long>());
    println!("c_float: {} bytes", std::mem::size_of::<c_float>());
    println!("c_double: {} bytes", std::mem::size_of::<c_double>());
    println!("c_char: {} bytes", std::mem::size_of::<c_char>());

    // -------------------------------------------------------
    // 4. CString and CStr
    // -------------------------------------------------------
    println!("\n--- 4. CString & CStr ---");

    use std::ffi::CString;

    // CString: Rust-allocated, null-terminated (untuk dikirim ke C)
    let c_str = CString::new("Hello from Rust").expect("CString failed");
    println!("CString: {:?}", c_str);
    println!("as_ptr: {:?} (C-compatible)", c_str.as_ptr());

    // CStr: borrowed slice dari C string (untuk diterima dari C)
    let cstr: &std::ffi::CStr = c_str.as_c_str();
    let rust_str: &str = cstr.to_str().expect("invalid UTF-8");
    println!("back to &str: {}", rust_str);

    // -------------------------------------------------------
    // 5. Structs with C Representation
    // -------------------------------------------------------
    println!("\n--- 5. C-compatible Structs ---");

    #[repr(C)]
    struct PointC {
        x: f64,
        y: f64,
    }

    impl PointC {
        #[no_mangle]
        extern "C" fn distance(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }

    let p = PointC { x: 3.0, y: 4.0 };
    println!("Point({}, {}) distance: {}", p.x, p.y, p.distance());

    // -------------------------------------------------------
    // 6. Calling Shared Libraries (dlopen/dlsym pattern)
    // -------------------------------------------------------
    println!("\n--- 6. Dynamic Library Loading ---");

    // Dalam production, kamu bisa pakai:
    //   let lib = unsafe { libloading::Library::new("libfoo.so") }?;
    //   let func: libloading::Symbol<unsafe extern fn(i32) -> i32> =
    //       unsafe { lib.get(b"my_function") }?;
    //   let result = unsafe { func(42) };

    // Untuk demo ini, kita tunjukkan konsepnya:
    println!("Dynamic loading pattern (requires libloading crate):");
    println!("  1. Library::new(\"libfoo.so\")");
    println!("  2. lib.get(b\"function_name\")");
    println!("  3. Call the function pointer");

    // -------------------------------------------------------
    // 7. Callback Functions (Function Pointers)
    // -------------------------------------------------------
    println!("\n--- 7. Callback Pattern ---");

    // FFI sering menggunakan callback function
    extern "C" fn callback(data: i32) {
        println!("  Callback called with: {}", data);
    }

    // Menggunakan fn pointer dengan calling convention yang sesuai
    fn call_with_callback(callback: extern "C" fn(i32), value: i32) {
        println!("  Calling with value: {}", value);
        callback(value);
    }

    call_with_callback(callback, 42);

    // -------------------------------------------------------
    // 8. Binding to C Libraries (Conceptual)
    // -------------------------------------------------------
    println!("\n--- 8. C Binding Pattern ---");

    // Dalam proyek nyata, kamu akan punya:
    //
    // // build.rs (build script)
    // fn main() {
    //     println!("cargo:rustc-link-lib=ssl");  // link libssl
    //     println!("cargo:rustc-link-search=/usr/lib");
    // }
    //
    // // src/lib.rs
    // extern "C" {
    //     fn SSL_CTX_new(method: *const SSL_METHOD) -> *mut SSL_CTX;
    //     fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    //     // ...
    // }

    // Contoh pattern binding:
    #[repr(C)]
    struct OpenSSLContext {
        _private: [u8; 0], // opaque type
    }

    extern "C" {
        // Simulasi: di dunia nyata ini akan link ke libssl
        // fn SSL_CTX_new(method: *const c_void) -> *mut OpenSSLContext;
    }

    println!("OpenSSL binding pattern demonstrated");
    println!("  - Opaque types with #[repr(C)]");
    println!("  - extern \"C\" blocks for function declarations");
    println!("  - build.rs for linking configuration");

    // -------------------------------------------------------
    // 9. Safety Considerations
    // -------------------------------------------------------
    println!("\n--- 9. Safety in FFI ---");

    println!("FFI guidelines:");
    println!("  1. Selalu wrap FFI calls dalam safe Rust abstraction");
    println!("  2. Validasi input sebelum dikirim ke C");
    println!("  3. Handle null pointers dengan hati-hati");
    println!("  4. Pastikan memory management konsisten");
    println!("  5. Gunakan bindgen/cbindgen untuk auto-generate bindings");
    println!("  6. Hindari panic cross FFI boundary!");
    println!("  7. Gunakan #[repr(C)] untuk struct yang di-share");

    // Contoh safe wrapper:
    fn safe_strlen(s: &str) -> usize {
        use std::ffi::CString;
        // Validasi input, lalu panggil FFI dengan aman
        let c_str = CString::new(s).expect("string contains null byte");
        unsafe { strlen(c_str.as_ptr()) }
    }

    println!("  safe_strlen(\"Rust\") = {}", safe_strlen("Rust"));

    // -------------------------------------------------------
    // 10. Common FFI Crates
    // -------------------------------------------------------
    println!("\n--- 10. Common FFI Crates ---");

    println!("  - bindgen: auto-generate Rust bindings from C headers");
    println!("  - cbindgen: auto-generate C headers from Rust");
    println!("  - libloading: dynamic library loading");
    println!("  - windows-sys: Windows API bindings");
    println!("  - libc: C standard library bindings (already in std)");
    println!("  - cc: compile C/C++ in build.rs");
    println!("  - cmake: cmake integration for C++ projects");

    println!("\n=== Ringkasan FFI ===");
    println!("1. extern \"C\" - declare foreign functions");
    println!("2. #[no_mangle] - preserve function name for linking");
    println!("3. CString/CStr - bridge string types");
    println!("4. #[repr(C)] - C-compatible memory layout");
    println!("5. build.rs - configure linking");
    println!("6. Safety: wrap unsafe FFI in safe abstractions");
}
