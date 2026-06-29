#[allow(unused_variables)]
fn main() {
    println!("=== 23. Drop & RAII ===\n");

    // RAII = Resource Acquisition Is Initialization
    // Resource dialokasikan saat dibuat, dilepas saat drop

    // -------------------------------------------------------
    // 1. Dasar Drop Trait
    // -------------------------------------------------------
    println!("--- 1. Basic Drop ---");

    {
        let _a = MyResource::new("Resource A");
        let _b = MyResource::new("Resource B");
        println!("  inside scope, using resources...");
    }
    // _b di-drop duluan (LIFO order), lalu _a
    println!("  scope ended\n");

    // -------------------------------------------------------
    // 2. Drop Order
    // -------------------------------------------------------
    println!("--- 2. Drop Order ---");

    {
        let first = MyResource::new("first");
        let second = MyResource::new("second");
        let third = MyResource::new("third");
        println!("  before drop, resources: first, second, third");
        drop(third); // drop manual sebelum scope end
        println!("  after drop(third)");
    }
    // sisa: second, first (LIFO)
    println!();

    // -------------------------------------------------------
    // 3. Drop dengan Cleanup (File, Socket, etc.)
    // -------------------------------------------------------
    println!("--- 3. Resource Cleanup ---");

    {
        let mut file = TempFile::new("data.txt");
        file.write("hello world");
        file.write(" second line");
        println!("  file active, writing data...");
    }
    // File di-flush dan ditutup otomatis
    println!("  file cleaned up\n");

    // -------------------------------------------------------
    // 4. Drop & Memori (Vec, String, etc.)
    // -------------------------------------------------------
    println!("--- 4. Collection Drop ---");

    {
        let mut items = vec![
            MyResource::new("item 1"),
            MyResource::new("item 2"),
            MyResource::new("item 3"),
        ];
        items.push(MyResource::new("item 4"));
        println!("  vec has {} items", items.len());
    }
    // Semua item di-drop, lalu vec itself
    println!();

    // -------------------------------------------------------
    // 5. Drop dengan Custom Logic
    // -------------------------------------------------------
    println!("--- 5. Custom Drop Logic ---");

    {
        let conn = DatabaseConn::new("postgres://localhost/mydb");
        conn.query("SELECT * FROM users");
        conn.query("INSERT INTO logs VALUES (now(), 'action')");
    }
    println!();

    // -------------------------------------------------------
    // 6. Drop Interaksi dengan Ownership
    // -------------------------------------------------------
    println!("--- 6. Drop & Ownership ---");

    let resource = MyResource::new("owned");
    let r2 = resource; // ownership moved
    // resource sudah tidak bisa diakses
    println!("  resource moved to r2");
    drop(r2); // drop manual
    println!("  r2 explicitly dropped");
    // println!("{}", resource.name); // ERROR: value moved
    println!();

    // -------------------------------------------------------
    // 7. Drop & Smart Pointers
    // -------------------------------------------------------
    println!("--- 7. Drop & Smart Pointers ---");

    use std::rc::Rc;

    {
        let shared = Rc::new(MyResource::new("shared via Rc"));
        let clone1 = Rc::clone(&shared);
        let clone2 = Rc::clone(&shared);
        println!("  strong_count: {}", Rc::strong_count(&shared));
        println!("  all clones refer to same resource");
    }
    // last Rc di-drop, referensi count -> 0, resource di-drop
    println!("  all Rc dropped, resource freed\n");

    // -------------------------------------------------------
    // 8. Drop Panics
    // -------------------------------------------------------
    println!("--- 8. Drop & Panics ---");

    // Drop tidak boleh panic! Jika panic saat drop, program akan abort
    // Best practice: catch_unwind atau hindari panic di drop

    {
        let safe_drop = SafeDrop::new("no panic here");
        println!("  {}", safe_drop);
    }
    println!("  safe_drop completed without panic\n");

    // -------------------------------------------------------
    // 9. Manually Calling Drop
    // -------------------------------------------------------
    println!("--- 9. Manual Drop ---");

    let r = MyResource::new("manual");
    println!("  about to manually drop");
    drop(r); // std::mem::drop(r)
    println!("  manually dropped");
    // r.name sudah tidak bisa diakses
    println!();

    // -------------------------------------------------------
    // 10. Drop Order dalam Struct
    // -------------------------------------------------------
    println!("--- 10. Struct Field Drop Order ---");

    {
        let pair = Pair {
            first: MyResource::new("field A"),
            second: MyResource::new("field B"),
        };
        println!("  Pair created with two fields");
        println!("  Fields di-drop sesuai urutan deklarasi: first, lalu second");
    }
    println!();

    println!("=== Ringkasan Drop & RAII ===");
    println!("1. Drop trait - custom destructor saat value dihapus");
    println!("2. Drop order - LIFO (reverse creation order)");
    println!("3. Manual drop - std::mem::drop(value)");
    println!("4. RAII - resource management tanpa garbage collector");
    println!("5. Drop & ownership - only owner can trigger drop");
    println!("6. Drop & smart pointers - reference counting based drop");
    println!("7. Jangan panic di drop handler!");
}

// ============================================================
// Basic Drop Implementation
// ============================================================

struct MyResource {
    name: String,
}

impl MyResource {
    fn new(name: &str) -> Self {
        println!("  [+] Creating {}", name);
        MyResource {
            name: name.to_string(),
        }
    }
}

impl Drop for MyResource {
    fn drop(&mut self) {
        println!("  [-] Dropping {}", self.name);
    }
}

// ============================================================
// File-like Resource with Cleanup
// ============================================================

struct TempFile {
    filename: String,
    buffer: String,
}

impl TempFile {
    fn new(filename: &str) -> Self {
        println!("  [+] Opening file: {}", filename);
        TempFile {
            filename: filename.to_string(),
            buffer: String::new(),
        }
    }

    fn write(&mut self, data: &str) {
        if !self.buffer.is_empty() {
            self.buffer.push('\n');
        }
        self.buffer.push_str(data);
        println!("  [write] '{}' to {}", data, self.filename);
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        println!("  [+] Flushing {} ({} bytes)", self.filename, self.buffer.len());
        println!("  [-] Closing file: {}", self.filename);
    }
}

// ============================================================
// Database Connection Pattern
// ============================================================

struct DatabaseConn {
    url: String,
    connected: bool,
}

impl DatabaseConn {
    fn new(url: &str) -> Self {
        println!("  [+] Connecting to: {}", url);
        DatabaseConn {
            url: url.to_string(),
            connected: true,
        }
    }

    fn query(&self, sql: &str) {
        if self.connected {
            println!("  [query] {}", sql);
        }
    }
}

impl Drop for DatabaseConn {
    fn drop(&mut self) {
        if self.connected {
            println!("  [+] Committing transactions");
            println!("  [-] Disconnecting from: {}", self.url);
            self.connected = false;
        }
    }
}

// ============================================================
// Safe Drop (no panics)
// ============================================================

struct SafeDrop {
    name: String,
}

impl SafeDrop {
    fn new(name: &str) -> Self {
        SafeDrop {
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for SafeDrop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SafeDrop({})", self.name)
    }
}

impl Drop for SafeDrop {
    fn drop(&mut self) {
        // Jangan panic di sini! Gunakan catch_unwind jika perlu
        println!("  [-] SafeDrop '{}' dropped safely", self.name);
    }
}

// ============================================================
// Struct with Multiple Fields
// ============================================================

struct Pair {
    first: MyResource,
    second: MyResource,
}

impl Drop for Pair {
    fn drop(&mut self) {
        println!("  [-] Dropping Pair");
        // field pertama (first) drop duluan, lalu second
    }
}
