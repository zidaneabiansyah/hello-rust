fn main() {
    println!("=== 24. Pin & Self-Referential Structs ===\n");

    // -------------------------------------------------------
    // 1. Masalah: Self-Referential Structs
    // -------------------------------------------------------
    println!("--- 1. The Problem ---");

    // Self-referential struct: field yang mereferensikan field lain
    // Dalam safe Rust, ini TIDAK mungkin karena:
    // - Value bisa dipindah (moved) di memori
    // - Reference bisa menjadi dangling setelah move

    // Contoh konsep (tidak bisa di-compile):
    // struct SelfRef {
    //     data: String,
    //     reference: &str, // <- reference ke data di atas
    // }
    // let s = SelfRef { data: "hello".to_string(), reference: &s.data };
    // let moved = s; // reference sekarang dangling!

    println!("Self-referential structs are impossible in safe Rust");
    println!("because values can be moved, invalidating references.\n");

    // -------------------------------------------------------
    // 2. Solusi: Pin<P>
    // -------------------------------------------------------
    println!("--- 2. Pin<P> Solution ---");

    // Pin<P> memastikan value TIDAK BISA dipindah di memori
    // Sehingga reference ke dalamnya tetap valid

    use std::pin::Pin;
    use std::marker::PhantomPinned;

    struct SelfRef {
        data: String,
        pointer: *const String, // raw pointer ke data
        _pin: PhantomPinned,    // menandakan ini tidak bisa Unpin
    }

    impl SelfRef {
        fn new(data: String) -> Pin<Box<Self>> {
            let mut boxed = Box::pin(SelfRef {
                data,
                pointer: std::ptr::null(),
                _pin: PhantomPinned,
            });

            // Safety: setelah pin, kita bisa membuat self-referential pointer
            // Gunakan as_mut() untuk mendapatkan Pin<&mut Self>
            let self_ptr: *const String = &boxed.as_ref().data;
            unsafe {
                let inner = boxed.as_mut().get_unchecked_mut();
                inner.pointer = self_ptr;
            }

            boxed
        }

        fn get_data(&self) -> &str {
            &self.data
        }

        fn get_pointer_value(&self) -> &str {
            unsafe { &*self.pointer }
        }
    }

    let pinned = SelfRef::new("Hello Pin!".to_string());
    println!("data: {}", pinned.get_data());
    println!("pointer -> data: {}", pinned.get_pointer_value());
    println!("same value: {}", pinned.get_data() == pinned.get_pointer_value());

    // Pin::new() - pin a value (membutuhkan Unpin)
    // Pin::new_unchecked() - pin tanpa Unpin check (unsafe)
    // Pin::as_ref() - borrow isi pin
    // Pin::get_mut() - mutable borrow (hanya sekali)

    // -------------------------------------------------------
    // 3. Unpin Trait
    // -------------------------------------------------------
    println!("\n--- 3. Unpin Trait ---");

    // Unpin: menandakan value bisa DI-MOVE meskipun di-pin
    // Sebagian besar tipe Rust implement Unpin secara otomatis

    fn assert_unpin<T: Unpin>() {}
    assert_unpin::<i32>();       // OK: i32 Unpin
    assert_unpin::<String>();    // OK: String Unpin
    assert_unpin::<Vec<u8>>();   // OK: Vec Unpin

    // Yang TIDAK Unpin:
    // - Struct dengan PhantomPinned
    // - async blocks (yang menghasilkan Future)
    // - generator (yield)

    println!("Most standard types implement Unpin");
    println!("Types with PhantomPinned do NOT implement Unpin");
    println!("This is the key to Pin's safety guarantee\n");

    // -------------------------------------------------------
    // 4. Pin in Async Context
    // -------------------------------------------------------
    println!("--- 4. Pin in Async ---");

    // async fn menghasilkan Future yang self-referential!
    // Maka Future harus di-pin sebelum di-poll

    // Conceptual: async block generates something like:
    // enum MyFuture {
    //     State0 { data: String },
    //     State1 { data: String, future: SomeFuture }, // self-referential!
    // }

    // Ini sebabnya:
    // - tokio::spawn membutuhkan future yang Unpin
    // - atau menggunakan Box::pin() untuk pin future

    use std::future::Future;
    use std::task::{Context, Poll};

    struct SimpleFuture {
        data: String,
        polled: bool,
    }

    impl Future for SimpleFuture {
        type Output = String;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.polled {
                println!("  [future] Ready with: {}", self.data);
                Poll::Ready(self.data.clone())
            } else {
                self.polled = true;
                println!("  [future] Pending first time");
                Poll::Pending
            }
        }
    }

    let mut future = SimpleFuture {
        data: "Hello Future!".to_string(),
        polled: false,
    };

    let mut future = Pin::new(&mut future);
    let waker = std::task::Waker::noop();
    let mut cx = Context::from_waker(&waker);

    println!("  polling future...");
    match future.as_mut().poll(&mut cx) {
        Poll::Pending => println!("  [result] Pending"),
        Poll::Ready(v) => println!("  [result] Ready: {}", v),
    }

    println!("  polling again...");
    match future.as_mut().poll(&mut cx) {
        Poll::Pending => println!("  [result] Pending"),
        Poll::Ready(v) => println!("  [result] Ready: {}", v),
    }

    // -------------------------------------------------------
    // 5. Pin Projection (Conceptual)
    // -------------------------------------------------------
    println!("\n--- 5. Pin Projection ---");

    // Pin projection: akses field dalam Pin<&mut Struct>
    // Berguna untuk async state machines

    struct MyStruct {
        value: i32,
        name: String,
    }

    impl MyStruct {
        // Dalam async, kamu akan pakai pin_project crate
        // atau manual unsafe projection

        // Contoh: akses field tanpa projection
        fn increment(&mut self) {
            self.value += 1;
            println!("  [projection] value: {}", self.value);
        }
    }

    let mut s = MyStruct {
        value: 0,
        name: "test".to_string(),
    };

    let mut pinned = Pin::new(&mut s);

    // Untuk field yang Unpin, kita bisa akses langsung:
    println!("  before: value = {}", pinned.value);
    pinned.as_mut().increment();
    println!("  after: value = {}", pinned.value);

    // Untuk field yang TIDAK Unpin, perlu pin_project crate
    // atau unsafe Pin::map_unchecked_mut

    // -------------------------------------------------------
    // 6. Box::pin Pattern
    // -------------------------------------------------------
    println!("\n--- 6. Box::pin Pattern ---");

    // Box::pin: pin value di heap, return Pin<Box<T>>

    let pinned_string = Box::pin("heap-pinned string".to_string());
    println!("pinned on heap: {}", *pinned_string);

    // Berguna untuk:
    // 1. Async futures (tokio::spawn)
    // 2. Self-referential data
    // 3. Recursive types

    // Recursive type example (conceptual):
    // enum List {
    //     Node { value: i32, next: Pin<Box<List>> },
    //     Nil,
    // }

    println!("Box::pin creates heap-allocated pinned values");

    // -------------------------------------------------------
    // 7. unsafe Pin::new_unchecked
    // -------------------------------------------------------
    println!("\n--- 7. unsafe Pin::new_unchecked ---");

    // Kadang kamu perlu pin tanpa Unpin:
    // - Custom future implementation
    // - FFI callbacks
    // - Intrusive linked lists

    struct NotUnpin {
        data: i32,
        _pin: PhantomPinned,
    }

    impl NotUnpin {
        fn new(data: i32) -> Pin<Box<Self>> {
            Box::pin(NotUnpin {
                data,
                _pin: PhantomPinned,
            })
        }
    }

    let pinned_not_unpin = NotUnpin::new(42);
    println!("NotUnpin pinned: data = {}", pinned_not_unpin.data);

    // Pin::new_unchecked hanya boleh digunakan jika:
    // 1. Kamu MEMASTIKAN value tidak akan dipindah
    // 2. Struct implementasi Pin projection yang benar
    // 3. Tidak ada cara lain untuk mencapai ini

    println!("Pin::new_unchecked is unsafe - use only when necessary");

    println!("\n=== Ringkasan Pin & Self-Referential ===");
    println!("1. Pin<P> - mencegah value dipindah di memori");
    println!("2. PhantomPinned - menandakan tipe tidak Unpin");
    println!("3. Unpin - menandakan tipe bisa di-move meskipun di-pin");
    println!("4. Self-referential struct - field mereferensikan field lain");
    println!("5. Async futures - menghasilkan self-referential enum");
    println!("6. Box::pin - pin value di heap");
    println!("7. Pin projection - akses field dalam pinned struct");
}
