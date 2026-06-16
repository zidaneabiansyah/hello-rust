use std::sync::{mpsc, Arc, Barrier, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

/*
CONCURRENCY DI RUST

Rust punya pendekatan unik: gak pake garbage collector, gak pake race condition.
Compile-time guarantee: data races DICEK di compile time pake ownership + Send/Sync.

Model concurrency Rust:
1. thread::spawn — bikin OS thread
2. mpsc channel — message passing (seperti Go channel / Erlang mailbox)
3. Arc<Mutex<T>> — shared state (locking)
4. RwLock<T> — multiple readers / single writer
5. Atomics — atomic operations (gak perlu lock)
*/

// Send & Sync trait — penjelasan
// Send: tipe yang aman dipindah antar thread (almost semua tipe)
// Sync: tipe yang aman di-share antar thread (&T aman diakses barengan)
// Semua tipe dasar implement Send + Sync secara otomatis
// Kecuali: Rc, RefCell, Cell (hanya single thread)

fn main() {
    println!("1. THREAD BASIC");

    // thread::spawn — return JoinHandle
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  Thread anak: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("  Thread utama: {}", i);
        thread::sleep(Duration::from_millis(15));
    }

    // join — nunggu thread selesai
    handle.join().unwrap();
    println!("  Thread anak udah selesai");

    println!("\n2. MOVE CLOSURE — ngirim data ke thread");

    let data = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        // move = paksa ownership pindah ke closure
        println!("  Data dari thread: {:?}", data);
    });
    // println!("{:?}", data); // ERROR: data udah di-move

    handle.join().unwrap();

    println!("\n3. MPSC CHANNEL — message passing");

    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let pesan = vec!["Halo", "dari", "thread", "pertama"];
        for msg in pesan {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    let tx2 = tx.clone();
    thread::spawn(move || {
        let pesan = vec!["Halo", "dari", "thread", "kedua"];
        for msg in pesan {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(15));
        }
    });

    // tx original harus di-drop biar receiver tau kalo semua sender udah selesai
    drop(tx);

    // rx — iterator, nunggu semua pesan
    for received in rx {
        println!("  Diterima: {}", received);
    }

    println!("\n4. ARC<MUTEX<T>> — shared state dengan locking");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += i;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("  Total counter: {}", *counter.lock().unwrap());

    println!("\n5. RWLOCK<T> — multiple readers, single writer");

    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut rw_handles = vec![];

    // Reader threads — bisa jalan barengan
    for i in 0..3 {
        let d = Arc::clone(&data);
        rw_handles.push(thread::spawn(move || {
            let read = d.read().unwrap();
            println!("  Reader {}: {:?}", i, *read);
            thread::sleep(Duration::from_millis(10));
        }));
    }

    // Writer thread — butuh akses exclusive
    let d = Arc::clone(&data);
    rw_handles.push(thread::spawn(move || {
        let mut write = d.write().unwrap();
        write.push(4);
        println!("  Writer: nambah 4");
        thread::sleep(Duration::from_millis(5));
    }));

    for h in rw_handles {
        h.join().unwrap();
    }

    println!("\n6. BARRIER — sinkronisasi thread");

    let barrier = Arc::new(Barrier::new(3));
    let mut bar_handles = vec![];

    for i in 0..3 {
        let b = Arc::clone(&barrier);
        bar_handles.push(thread::spawn(move || {
            println!("  Thread {} sampe di barrier", i);
            b.wait(); // nunggu semua thread sampe sini
            println!("  Thread {} lanjut setelah barrier", i);
        }));
    }

    for h in bar_handles {
        h.join().unwrap();
    }

    println!("\n7. CONDVAR — conditional variable");

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Thread worker
    let worker = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        thread::sleep(Duration::from_millis(50));
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
        println!("  Worker: selesai, notify main thread");
    });

    // Main thread nunggu
    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }
    println!("  Main: worker udah selesai");
    worker.join().unwrap();

    println!("\n8. ATOMICS — tanpa lock (lebih ringan)");

    use std::sync::atomic::{AtomicI32, Ordering};

    let atomic_counter = Arc::new(AtomicI32::new(0));
    let mut atom_handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&atomic_counter);
        atom_handles.push(thread::spawn(move || {
            // fetch_add — atomic operation, gak perlu Mutex
            c.fetch_add(1, Ordering::SeqCst);
        }));
    }

    for h in atom_handles {
        h.join().unwrap();
    }

    println!("  Atomic counter: {}", atomic_counter.load(Ordering::SeqCst));

    // Atomic swap
    let old = atomic_counter.swap(0, Ordering::SeqCst);
    println!("  Old value (swap ke 0): {}", old);
    println!("  Setelah swap: {}", atomic_counter.load(Ordering::SeqCst));

    println!("\n9. SCOPED THREADS — thread yang bisa borrow local variable");

    // thread::scope — thread anak bisa pake reference dari scope (gak perlu move + Arc)
    let angka = vec![1, 2, 3, 4, 5, 6];
    let mut hasil = vec![];

    thread::scope(|s| {
        // Scoped thread bisa pake reference dari luar tanpa Arc!
        for chunk in angka.chunks(2) {
            s.spawn(move || {
                println!("  Scoped thread: {:?}", chunk);
            });
        }

        s.spawn(|| {
            hasil.push(100);
        });
    }); // semua thread di scope selesai di sini

    println!("  angka: {:?}", angka);
    println!("  hasil: {:?}", hasil);

    println!("\n10. DEADLOCK DEMO (di-comment biar gak hang)");

    // Deadlock: 2 thread saling nunggu lock
    // let lock1 = Arc::new(Mutex::new(0));
    // let lock2 = Arc::new(Mutex::new(0));
    // let l1 = Arc::clone(&lock1);
    // let l2 = Arc::clone(&lock2);
    //
    // thread::spawn(move || {
    //     let _a = l1.lock().unwrap();
    //     thread::sleep(Duration::from_millis(10));
    //     let _b = l2.lock().unwrap(); // nunggu thread2 release lock2
    // });
    //
    // thread::spawn(move || {
    //     let _b = l2.lock().unwrap();
    //     thread::sleep(Duration::from_millis(10));
    //     let _a = l1.lock().unwrap(); // nunggu thread1 release lock1
    // });
    // // DEADLOCK! ❌

    // Solusi: selalu pake urutan lock yg konsisten

    println!("\n11. THREAD POOL (manual — pake crossbeam kalo butuh real)");

    // Contoh: execute 10 task pake 4 thread worker
    let tasks = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut pool_handles = vec![];

    const NUM_WORKERS: usize = 4;

    for chunk in tasks.chunks((tasks.len() + NUM_WORKERS - 1) / NUM_WORKERS) {
        let chunk = chunk.to_vec();
        let r = Arc::clone(&results);
        pool_handles.push(thread::spawn(move || {
            let processed: Vec<i32> = chunk.iter().map(|x| x * 2).collect();
            let mut res = r.lock().unwrap();
            res.extend(processed);
        }));
    }

    for h in pool_handles {
        h.join().unwrap();
    }

    let mut final_results = results.lock().unwrap();
    final_results.sort();
    println!("  Thread pool results: {:?}", final_results);
}

// Send & Sync — custom type
// struct TidakAman {
//     data: Rc<String>, // Rc gak Send
// }
// // Kalo dipaksa pindah thread: ERROR

// Arc aman karena implement Send + Sync (atomic reference counting)
