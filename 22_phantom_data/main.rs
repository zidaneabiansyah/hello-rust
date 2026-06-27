#[allow(dead_code)]
fn main() {
    println!("=== 22. PhantomData & Typestate Pattern ===\n");

    // -------------------------------------------------------
    // 1. Apa itu PhantomData?
    // -------------------------------------------------------
    println!("--- 1. PhantomData Basics ---");

    // PhantomData menandakan bahwa type parameter "digunakan"
    // meskipun tidak ada field yang menyimpan tipe tersebut
    // Berguna untuk:
    // - Memberitahu borrow checker bahwa type parameter penting
    // - Memastikan lifetime tidak premature drop
    // - Typestate pattern

    let p = PointPhantom {
        x: 1.0,
        y: 2.0,
        _marker: std::marker::PhantomData,
    };
    println!("PointPhantom({}, {})", p.x, p.y);

    // -------------------------------------------------------
    // 2. PhantomData dengan Lifetime
    // -------------------------------------------------------
    println!("\n--- 2. PhantomData with Lifetime ---");

    // Contoh klasik: container yang hold reference tapi
    // tidak punyai field bertipe lifetime secara langsung

    struct RefHolder<'a, T: 'a> {
        ptr: *const T,
        _marker: std::marker::PhantomData<&'a T>,
    }

    impl<'a, T: 'a> RefHolder<'a, T> {
        fn new(val: &'a T) -> Self {
            RefHolder {
                ptr: val as *const T,
                _marker: std::marker::PhantomData,
            }
        }

        fn get(&self) -> &'a T {
            unsafe { &*self.ptr }
        }
    }

    let value = 42;
    let holder = RefHolder::new(&value);
    println!("RefHolder value: {}", holder.get());

    // -------------------------------------------------------
    // 3. Typestate Pattern: Compile-time State Machine
    // -------------------------------------------------------
    println!("\n--- 3. Typestate Pattern ---");

    // Typestate menggunakan PhantomData untuk memaksa
    // transisi state hanya dalam urutan yang benar
    // DI COMPILE TIME, bukan runtime!

    // Contoh: Builder Pattern dengan Typestate
    //
    // Builder -> HasHost -> HasPort -> Ready
    //                  \               /
    //                   \-> Config <-/

    struct Builder<State> {
        host: Option<String>,
        port: Option<u16>,
        _state: std::marker::PhantomData<State>,
    }

    // State types (unit structs sebagai tag)
    struct NoHost;
    struct HasHost;
    struct HasPort;
    struct Ready;

    // Builder kosong
    fn new_builder() -> Builder<NoHost> {
        Builder {
            host: None,
            port: None,
            _state: std::marker::PhantomData,
        }
    }

    // Transisi: NoHost -> HasHost
    impl Builder<NoHost> {
        fn host(self, h: &str) -> Builder<HasHost> {
            println!("  [typestate] NoHost -> HasHost");
            Builder {
                host: Some(h.to_string()),
                port: None,
                _state: std::marker::PhantomData,
            }
        }
    }

    // Transisi: HasHost -> HasPort
    impl Builder<HasHost> {
        fn port(self, p: u16) -> Builder<HasPort> {
            println!("  [typestate] HasHost -> HasPort");
            Builder {
                host: self.host,
                port: Some(p),
                _state: std::marker::PhantomData,
            }
        }
    }

    // Transisi: HasPort -> Ready
    impl Builder<HasPort> {
        fn build(self) -> Builder<Ready> {
            println!("  [typestate] HasPort -> Ready");
            Builder {
                host: self.host,
                port: self.port,
                _state: std::marker::PhantomData,
            }
        }
    }

    // Method hanya di state Ready
    impl Builder<Ready> {
        fn start(&self) {
            println!(
                "  [typestate] Starting server at {}:{}",
                self.host.as_ref().unwrap(),
                self.port.unwrap()
            );
        }
    }

    // Kita TIDAK bisa skip step! Ini error compile-time:
    // new_builder().build().start();  // ERROR: NoHost tidak punya build()
    // new_builder().port(8080);       // ERROR: NoHost tidak punya port()

    let server = new_builder()
        .host("localhost")
        .port(8080)
        .build();
    server.start();

    // -------------------------------------------------------
    // 4. Typestate: File Read/Write Mode
    // -------------------------------------------------------
    println!("\n--- 4. Typestate: File Modes ---");

    struct FileHandler<Mode> {
        path: String,
        _mode: std::marker::PhantomData<Mode>,
    }

    struct ReadMode;
    struct WriteMode;
    struct AppendMode;

    fn open_read(path: &str) -> FileHandler<ReadMode> {
        println!("  Opening {} for reading", path);
        FileHandler {
            path: path.to_string(),
            _mode: std::marker::PhantomData,
        }
    }

    fn open_write(path: &str) -> FileHandler<WriteMode> {
        println!("  Opening {} for writing", path);
        FileHandler {
            path: path.to_string(),
            _mode: std::marker::PhantomData,
        }
    }

    impl FileHandler<ReadMode> {
        fn read(&self) -> String {
            format!("Content of {}", self.path)
        }

        fn append_mode(self) -> FileHandler<AppendMode> {
            println!("  Converting to append mode");
            FileHandler {
                path: self.path,
                _mode: std::marker::PhantomData,
            }
        }
    }

    impl FileHandler<WriteMode> {
        fn write(&self, data: &str) {
            println!("  Writing to {}: {}", self.path, data);
        }
    }

    impl FileHandler<AppendMode> {
        fn append(&self, data: &str) {
            println!("  Appending to {}: {}", self.path, data);
        }
    }

    // Tidak bisa write ke file yang dibuka untuk read!
    // let file = open_read("test.txt");
    // file.write("data");  // ERROR: ReadMode tidak punya method write()

    let reader = open_read("data.txt");
    println!("  Read result: {}", reader.read());

    let writer = open_write("output.txt");
    writer.write("Hello, Typestate!");

    let appender = reader.append_mode();
    appender.append(" more data");

    // -------------------------------------------------------
    // 5. Typestate: Network Connection
    // -------------------------------------------------------
    println!("\n--- 5. Typestate: Network States ---");

    struct Connection<State> {
        addr: String,
        _state: std::marker::PhantomData<State>,
    }

    struct Disconnected;
    struct Connecting;
    struct Connected;
    struct Authenticated;

    fn connect(addr: &str) -> Connection<Connecting> {
        println!("  Connecting to {}...", addr);
        Connection {
            addr: addr.to_string(),
            _state: std::marker::PhantomData,
        }
    }

    impl Connection<Connecting> {
        fn connected(self) -> Connection<Connected> {
            println!("  Connected to {}", self.addr);
            Connection {
                addr: self.addr,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl Connection<Connected> {
        fn authenticate(self, token: &str) -> Connection<Authenticated> {
            println!("  Authenticated with token: {}", &token[..8.min(token.len())]);
            Connection {
                addr: self.addr,
                _state: std::marker::PhantomData,
            }
        }

        fn disconnect(self) -> Connection<Disconnected> {
            println!("  Disconnected from {}", self.addr);
            Connection {
                addr: self.addr,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl Connection<Authenticated> {
        fn send(&self, data: &str) {
            println!("  Sending to {}: {}", self.addr, data);
        }
    }

    // Flow: Connecting -> Connected -> Authenticated
    // Tidak bisa skip auth!
    let conn = connect("api.example.com")
        .connected()
        .authenticate("secret_token_12345678");
    conn.send("GET /users");

    // -------------------------------------------------------
    // 6. PhantomData untuk Marker Types
    // -------------------------------------------------------
    println!("\n--- 6. PhantomData as Markers ---");

    struct Id<T> {
        value: u64,
        _marker: std::marker::PhantomData<T>,
    }

    struct User;
    struct Product;
    struct Order;

    fn user_id(id: u64) -> Id<User> {
        Id {
            value: id,
            _marker: std::marker::PhantomData,
        }
    }

    fn product_id(id: u64) -> Id<Product> {
        Id {
            value: id,
            _marker: std::marker::PhantomData,
        }
    }

    // Tidak bisa campur aduk ID!
    fn get_user(id: Id<User>) -> String {
        format!("User #{}", id.value)
    }

    fn get_product(id: Id<Product>) -> String {
        format!("Product #{}", id.value)
    }

    let uid = user_id(1);
    let pid = product_id(42);

    println!("  {}", get_user(uid));
    println!("  {}", get_product(pid));

    // Ini error compile-time:
    // get_user(pid);    // ERROR: Id<Product> bukan Id<User>
    // get_product(uid); // ERROR: Id<User> bukan Id<Product>

    // -------------------------------------------------------
    // 7. PhantomData dengan Send/Sync
    // -------------------------------------------------------
    println!("\n--- 7. PhantomData for Auto Traits ---");

    // PhantomData bisa mengontrol Send/Sync

    struct NotThreadSafe {
        data: Vec<u8>,
        _marker: std::marker::PhantomData<*const u8>, // !Send + !Sync
    }

    struct ThreadSafe {
        data: Vec<u8>,
        // Tidak ada PhantomData<*const u8>, jadi Send + Sync by default
    }

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<ThreadSafe>(); // OK
    assert_sync::<ThreadSafe>(); // OK

    // assert_send::<NotThreadSafe>();  // ERROR: not Send
    // assert_sync::<NotThreadSafe>();  // ERROR: not Sync

    println!("  ThreadSafe is Send+Sync: OK");
    println!("  NotThreadSafe is !Send+!Sync: OK");

    println!("\n=== Ringkasan PhantomData & Typestate ===");
    println!("1. PhantomData - menandakan type parameter 'digunakan'");
    println!("2. Lifetime safety - mencegah premature drop");
    println!("3. Typestate - compile-time state machine");
    println!("4. Builder pattern - enforce urutan transisi");
    println!("5. Marker types - Id<User> vs Id<Product> type safety");
    println!("6. Auto traits - kontrol Send/Sync");
    println!("7. Zero-cost abstraction - tidak ada runtime overhead");
}

// Struct dengan PhantomData
struct PointPhantom {
    x: f64,
    y: f64,
    _marker: std::marker::PhantomData<f64>, // menandakan "gunakan" f64
}
