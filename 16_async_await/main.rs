use std::time::Instant;
use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. ASYNC BASIC");

    let hasil = async { format!("Halo {}, dari async!", "Budi") }.await;
    println!("  {}", hasil);

    let async_add = |x: i32, y: i32| async move { x + y };
    let jumlah = async_add(10, 20).await;
    println!("  async add: {}", jumlah);

    println!("\n2. TOKIO::SPAWN — concurrent tasks");

    async fn task(id: u32, detik: u64) -> String {
        sleep(Duration::from_secs(detik)).await;
        format!("  Task {} selesai dalam {} detik", id, detik)
    }

    let h1 = tokio::spawn(task(1, 2));
    let h2 = tokio::spawn(task(2, 1));

    let start = Instant::now();
    let (r1, r2) = (h1.await?, h2.await?);
    println!("{}", r1);
    println!("{}", r2);
    println!("  total: {}ms (concurrent, bukan sequential)", start.elapsed().as_millis());

    println!("\n3. JOIN — jalan beberapa future bersama");

    async fn lambat(id: u32, detik: u64) -> String {
        sleep(Duration::from_secs(detik)).await;
        format!("  join task {} ({} detik)", id, detik)
    }

    let start = Instant::now();
    let (a, b) = tokio::join!(lambat(1, 2), lambat(2, 1));
    println!("{}", a);
    println!("{}", b);
    println!("  total: {}ms", start.elapsed().as_millis());

    println!("\n4. TRY_JOIN — dengan error handling");

    async fn bagi(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err(String::from("cannot divide by zero"));
        }
        sleep(Duration::from_millis(10)).await;
        Ok(a / b)
    }

    match tokio::try_join!(bagi(10, 2), bagi(20, 4)) {
        Ok((x, y)) => println!("  hasil: {}, {}", x, y),
        Err(e) => println!("  error: {}", e),
    }

    match tokio::try_join!(bagi(10, 2), bagi(10, 0)) {
        Ok((x, y)) => println!("  hasil: {}, {}", x, y),
        Err(e) => println!("  error (expected): {}", e),
    }

    println!("\n5. SELECT — race antar future");

    let start = Instant::now();
    tokio::pin! {
        let slow = lambat(10, 3);
        let fast = lambat(20, 1);
    }

    let result = tokio::select! {
        r = &mut slow => r,
        r = &mut fast => r,
    };
    println!("{}", result);
    println!("  select selesai dalam: {}ms", start.elapsed().as_millis());

    println!("\n6. TIMEOUT");

    let result = timeout(Duration::from_secs(1), lambat(99, 2)).await;
    match result {
        Ok(msg) => println!("  {}", msg),
        Err(_) => println!("  Timeout! task 2 detik gagal dalam 1 detik"),
    }

    let result = timeout(Duration::from_secs(3), lambat(99, 1)).await;
    match result {
        Ok(msg) => println!("  {}", msg),
        Err(_) => println!("  Timeout!"),
    }

    println!("\n7. ASYNC CHANNEL (mpsc)");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(32);

    let sender = tokio::spawn(async move {
        for i in 0..5 {
            tx.send(format!("Pesan {}", i)).await.unwrap();
            sleep(Duration::from_millis(5)).await;
        }
    });

    let receiver = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("  received: {}", msg);
        }
    });

    let _ = tokio::join!(sender, receiver);

    println!("\n8. ASYNC MUTEX");

    let counter = std::sync::Arc::new(tokio::sync::Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let c = std::sync::Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            let mut num = c.lock().await;
            *num += i;
        }));
    }

    for h in handles {
        h.await?;
    }
    println!("  async counter: {}", *counter.lock().await);

    println!("\n9. PERFORMANCE: SYNC vs ASYNC");

    let start = Instant::now();
    for _ in 0..5 {
        sleep(Duration::from_millis(10)).await;
    }
    println!("  async sequential 5x10ms: {}ms", start.elapsed().as_millis());

    let start = Instant::now();
    let mut handles = vec![];
    for _ in 0..5 {
        handles.push(tokio::spawn(async {
            sleep(Duration::from_millis(10)).await;
        }));
    }
    for h in handles {
        h.await?;
    }
    println!("  async concurrent 5x10ms: {}ms", start.elapsed().as_millis());

    println!("\n10. RINGKASAN");
    println!("  async fn     -> returns impl Future");
    println!("  .await       -> tunggu future selesai (yield kalo blm ready)");
    println!("  tokio::spawn -> jalanin future di task terpisah");
    println!("  tokio::join! -> jalan beberapa future concurrent");
    println!("  tokio::try_join! -> join + error propagation");
    println!("  tokio::select!  -> race (ambil yg pertama selesai)");
    println!("  timeout()    -> batasin waktu eksekusi");
    println!("  mpsc channel -> async message passing");
    println!("  async Mutex  -> locking di async context");

    Ok(())
}
