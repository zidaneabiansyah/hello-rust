use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::borrow::Cow;

/*
SMART POINTERS — Pointer dengan kemampuan ekstra

Apa bedanya sama reference biasa (&, &mut)?
- Smart pointer OTOMATIS dealokasi memori
- Bisa punya OWNERSHIP (Box, Rc, Arc)
- Bisa punya aturan akses khusus (RefCell, Mutex)
- Implement Deref (bisa pake * dan method) dan Drop (bisa cleanup otomatis)
*/

// 1. BOX<T> — Data di heap, pointer di stack
// Box itu cara paling sederhana naruh data di heap
// Berguna buat: recursive type, trait object, cloning expensive data

// Contoh recursive type — Rust kudu tau ukuran di compile time
// Kalo pake Box, ukuran Box<T> fix (8 byte = pointer), isinya di heap
#[derive(Debug)]
enum List {
    Kosong,
    Node(i32, Box<List>),
}

// Box buat trait object (dynamic dispatch)
trait Hewan {
    fn suara(&self) -> &str;
}

struct Anjing;
struct Kucing;

impl Hewan for Anjing {
    fn suara(&self) -> &str {
        "Guk guk!"
    }
}

impl Hewan for Kucing {
    fn suara(&self) -> &str {
        "Meong!"
    }
}

// 2. RC<T> — Reference Counting (single thread)
// Rc = multiple ownership di single thread
// Setiap kali di-clone, counter +1
// Kalo counter 0, data di-drop
// Rc cuma bisa dipake di single thread (bukan Send)

#[derive(Debug)]
struct Mahasiswa {
    nama: String,
    nilai: HashMap<String, f64>,
}

// 3. REFCELL<T> — Interior Mutability (single thread)
// RefCell ngasih borrow checking di RUNTIME (bukan compile time)
// Biasa: borrow rules dicek pas compile → kalo salah GA BISA COMPILE
// RefCell: borrow rules dicek pas jalan → kalo salah PANIC
// Berguna pas lo tau kodenya bener tapi compiler gak percaya

trait Messenger {
    fn kirim(&self, msg: &str);
}

// Contoh: nge-track pesan yang dikirim
// Kita butuh &self (immutable) tapi pengen ubah internal state
struct Logger {
    pesan_terkirim: RefCell<Vec<String>>,
}

impl Messenger for Logger {
    fn kirim(&self, msg: &str) {
        // &self → immutable reference, tapi RefCell ngizinin kita ubah
        self.pesan_terkirim.borrow_mut().push(String::from(msg));
    }
}

// 4. RC<REFCELL<T>> — Kombinasi multi-ownership + mutable
// Pola paling umum di Rust: Rc<RefCell<T>>
// Contoh: graph / tree dimana banyak node punya reference ke node yg sama

#[derive(Debug)]
struct Node {
    value: i32,
    anak: Vec<Rc<RefCell<Node>>>,
}

// 5. ARC<MUTEX<T>> — Thread-safe shared state
// Arc = Rc tapi buat multi-thread (atomic reference counting)
// Mutex = RefCell tapi buat multi-thread
// Kombinasi paling umum: Arc<Mutex<T>>

// 6. COW<T> — Clone on Write
// Ngasih reference (&T) kalo gak dimodif
// Clone jadi T kalo dimodif
// Hemat alokasi kalo kebanyakan data cuma dibaca

fn analisis_kata(kata: &str) -> Cow<'_, str> {
    if kata.len() < 5 {
        Cow::Borrowed(kata)
    } else {
        Cow::Owned(format!("{} ({} huruf)", kata, kata.len()))
    }
}

// 7. CELL<T> — Interior mutability buat Copy type
// Lebih ringan dari RefCell, cuma bisa dipake buat tipe yg implement Copy (i32, bool, dll)
// Gak pake borrow checking runtime — langsung replace value

fn main() {
    println!("\n1. BOX<T> — HEAP ALLOCATION");

    // Basic Box
    let boxed = Box::new(42);
    println!("Nilai di heap: {}", boxed);
    // Deref: Box otomatis di-deref, jadi bisa pake * kalo perlu
    println!("Deref explicit: {}", *boxed);

    // Box di-drop otomatis pas keluar scope
    {
        let _temp = Box::new(100);
        println!("Box di dalam scope: {}", _temp);
    } // _temp di-drop di sini

    // Recursive type
    println!("\nRecursive type (linked list):");
    let list = List::Node(
        1,
        Box::new(List::Node(
            2,
            Box::new(List::Node(
                3,
                Box::new(List::Kosong),
            )),
        )),
    );
    println!("{:?}", list);

    // Trait object — dynamic dispatch via Box
    println!("\nTrait object (dynamic dispatch):");
    let hewans: Vec<Box<dyn Hewan>> = vec![
        Box::new(Anjing),
        Box::new(Kucing),
        Box::new(Anjing),
    ];

    for hewan in &hewans {
        println!("Suara: {}", hewan.suara());
    }

    println!("\n2. RC<T> — REFERENCE COUNTING");

    // Tanpa Rc — ownership cuma 1
    // let data = vec![1, 2, 3];
    // let a = data; // data di-move ke a
    // let b = data; // ERROR: data udah di-move

    // Pake Rc — multiple ownership
    let nilai_budi = Rc::new(Mahasiswa {
        nama: String::from("Budi"),
        nilai: HashMap::from([
            (String::from("Matematika"), 85.0),
            (String::from("Fisika"), 90.0),
        ]),
    });

    println!("Ref count awal: {}", Rc::strong_count(&nilai_budi));

    let a = Rc::clone(&nilai_budi); // Rc::clone bukan deep copy — cuma increment counter
    println!("Ref count setelah clone 1: {}", Rc::strong_count(&nilai_budi));

    {
        let b = Rc::clone(&nilai_budi);
        println!("Ref count di inner scope: {}", Rc::strong_count(&nilai_budi));
        println!("Nilai Budi via b: {}", b.nilai.get("Matematika").unwrap());
    } // b di-drop, counter turun

    println!("Ref count setelah b di-drop: {}", Rc::strong_count(&nilai_budi));
    println!("Nilai Budi via a: {}", a.nilai.get("Fisika").unwrap());

    println!("\n3. REFCELL<T> — INTERIOR MUTABILITY");

    let logger = Logger {
        pesan_terkirim: RefCell::new(Vec::new()),
    };

    logger.kirim("Hello");
    logger.kirim("Dunia");
    logger.kirim("Rust");

    // borrow() — dapet Ref<T>, immutable borrow di runtime
    {
        let history = logger.pesan_terkirim.borrow();
        println!("Pesan terkirim: {:?}", history);
    } // history di-drop di sini

    // borrow_mut() — dapet RefMut<T>, mutable borrow di runtime
    logger.pesan_terkirim.borrow_mut().push(String::from("Tambahan"));
    println!("Setelah push: {:?}", logger.pesan_terkirim.borrow());

    // RefCell runtime checking — kalo salah bakal PANIC
    // let mut ref1 = logger.pesan_terkirim.borrow_mut();
    // let ref2 = logger.pesan_terkirim.borrow(); // PANIC! udah ada mutable borrow

    println!("\n4. RC<REFCELL<T>> — KOMBINASI");

    // Bikin 3 node yang saling terhubung
    let root = Rc::new(RefCell::new(Node {
        value: 1,
        anak: Vec::new(),
    }));

    let anak1 = Rc::new(RefCell::new(Node {
        value: 2,
        anak: Vec::new(),
    }));

    let anak2 = Rc::new(RefCell::new(Node {
        value: 3,
        anak: Vec::new(),
    }));

    // root punya 2 anak
    root.borrow_mut().anak.push(Rc::clone(&anak1));
    root.borrow_mut().anak.push(Rc::clone(&anak2));

    // anak1 punya anak juga (sharing)
    anak1.borrow_mut().anak.push(Rc::clone(&anak2));

    println!("Root: {:?}", root.borrow());
    println!("Anak1: {:?}", anak1.borrow());
    println!("Anak2: {:?}", anak2.borrow());

    // Ubah value lewat root
    root.borrow_mut().value = 100;
    println!("Setelah diubah:");
    println!("Root: {:?}", root.borrow());

    println!("Ref count root: {}", Rc::strong_count(&root));
    println!("Ref count anak2: {}", Rc::strong_count(&anak2)); // 2: root & anak1

    println!("\n5. ARC<MUTEX<T>> — THREAD-SAFE SHARED STATE");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += i;
            println!("Thread {} nambahin {}, total: {}", i, i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());

    println!("\n6. COW<T> — CLONE ON WRITE");

    let kata_pendek = "aku";
    let kata_panjang = "programming";

    let hasil1 = analisis_kata(kata_pendek);
    let hasil2 = analisis_kata(kata_panjang);

    // Cow::Borrowed — gak alokasi baru
    println!("Pendek (borrowed): {}", hasil1);
    println!("is_borrowed: {}", matches!(hasil1, Cow::Borrowed(_)));

    // Cow::Owned — alokasi string baru
    println!("Panjang (owned): {}", hasil2);
    println!("is_owned: {}", matches!(hasil2, Cow::Owned(_)));

    println!("\n7. CELL<T> — INTERIOR MUTABILITY (COPY TYPE)");

    use std::cell::Cell;

    #[derive(Debug)]
    struct Stats {
        hits: Cell<u32>,
        misses: Cell<u32>,
    }

    let stats = Stats {
        hits: Cell::new(0),
        misses: Cell::new(0),
    };

    // &self (immutable) tapi bisa ubah nilai — beda sama RefCell, pake .get() dan .set()
    stats.hits.set(stats.hits.get() + 1);
    stats.hits.set(stats.hits.get() + 1);
    stats.misses.set(stats.misses.get() + 1);

    println!("Stats: {:?}", stats);
    println!("Hits: {}, Misses: {}", stats.hits.get(), stats.misses.get());

    println!("\nRINGKASAN SMART POINTERS");
    println!("Box<T>      → Heap allocation, single ownership");
    println!("Rc<T>       → Multi ownership (single thread)");
    println!("Arc<T>      → Multi ownership (multi thread, atomic)");
    println!("RefCell<T>  → Interior mutability (runtime borrow check, single thread)");
    println!("Cell<T>     → Interior mutability (Copy type, single thread)");
    println!("Mutex<T>    → Interior mutability (multi thread)");
    println!("Cow<T>      → Clone on write (borrow kalo bisa, clone kalo perlu)");
}
