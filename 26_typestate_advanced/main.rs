#[allow(dead_code)]
fn main() {
    println!("=== 26. Typestate Pattern - Real-world Builder ===\n");

    // -------------------------------------------------------
    // 1. HTTP Request Builder dengan Typestate
    // -------------------------------------------------------
    println!("--- 1. HTTP Request Builder ---");

    // Pattern: Builder -> URL set -> Method set -> Ready to send

    struct RequestBuilder<State> {
        url: Option<String>,
        method: Option<String>,
        headers: Vec<(String, String)>,
        body: Option<String>,
        _state: std::marker::PhantomData<State>,
    }

    struct NoUrl;
    struct HasUrl;
    struct HasMethod;
    struct ReadyToSend;

    fn new_request() -> RequestBuilder<NoUrl> {
        RequestBuilder {
            url: None,
            method: None,
            headers: Vec::new(),
            body: None,
            _state: std::marker::PhantomData,
        }
    }

    // NoUrl -> HasUrl
    impl RequestBuilder<NoUrl> {
        fn url(self, url: &str) -> RequestBuilder<HasUrl> {
            println!("  [typestate] Set URL: {}", url);
            RequestBuilder {
                url: Some(url.to_string()),
                method: self.method,
                headers: self.headers,
                body: self.body,
                _state: std::marker::PhantomData,
            }
        }
    }

    // HasUrl -> HasMethod
    impl RequestBuilder<HasUrl> {
        fn method(self, method: &str) -> RequestBuilder<HasMethod> {
            println!("  [typestate] Set method: {}", method);
            RequestBuilder {
                url: self.url,
                method: Some(method.to_string()),
                headers: self.headers,
                body: self.body,
                _state: std::marker::PhantomData,
            }
        }
    }

    // HasMethod -> ReadyToSend (bisa juga di chain)
    impl RequestBuilder<HasMethod> {
        fn header(mut self, key: &str, value: &str) -> RequestBuilder<HasMethod> {
            self.headers.push((key.to_string(), value.to_string()));
            self
        }

        fn body(mut self, body: &str) -> RequestBuilder<HasMethod> {
            self.body = Some(body.to_string());
            self
        }

        fn build(self) -> RequestBuilder<ReadyToSend> {
            println!("  [typestate] Build request");
            RequestBuilder {
                url: self.url,
                method: self.method,
                headers: self.headers,
                body: self.body,
                _state: std::marker::PhantomData,
            }
        }
    }

    // ReadyToSend
    impl RequestBuilder<ReadyToSend> {
        fn send(&self) -> String {
            let method = self.method.as_ref().unwrap();
            let url = self.url.as_ref().unwrap();
            let mut request = format!("{} {}", method, url);

            if !self.headers.is_empty() {
                request.push_str("\r\n");
                for (k, v) in &self.headers {
                    request.push_str(&format!("{}: {}\r\n", k, v));
                }
            }

            if let Some(body) = &self.body {
                request.push_str(&format!("\r\n{}", body));
            }

            println!("  [typestate] Sending request:");
            request
        }
    }

    // Flow: new_request() -> url() -> method() -> build() -> send()
    // Tidak bisa skip step!

    let request = new_request()
        .url("https://api.example.com/users")
        .method("POST")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice"}"#)
        .build();

    println!("Request:\n{}\n", request.send());

    // -------------------------------------------------------
    // 2. Database Transaction Builder
    // -------------------------------------------------------
    println!("--- 2. Database Transaction Builder ---");

    struct TransactionBuilder<State> {
        table: Option<String>,
        operations: Vec<String>,
        _state: std::marker::PhantomData<State>,
    }

    struct TxIdle;
    struct TxStarted;
    struct TxCommitted;

    fn start_transaction(table: &str) -> TransactionBuilder<TxStarted> {
        println!("  [tx] BEGIN on table: {}", table);
        TransactionBuilder {
            table: Some(table.to_string()),
            operations: Vec::new(),
            _state: std::marker::PhantomData,
        }
    }

    impl TransactionBuilder<TxStarted> {
        fn insert(mut self, data: &str) -> Self {
            self.operations.push(format!("INSERT INTO {} VALUES ({})", self.table.as_ref().unwrap(), data));
            self
        }

        fn update(mut self, condition: &str, data: &str) -> Self {
            self.operations.push(format!("UPDATE {} SET {} WHERE {}", self.table.as_ref().unwrap(), data, condition));
            self
        }

        fn delete(mut self, condition: &str) -> Self {
            self.operations.push(format!("DELETE FROM {} WHERE {}", self.table.as_ref().unwrap(), condition));
            self
        }

        fn commit(self) -> TransactionBuilder<TxCommitted> {
            println!("  [tx] COMMIT ({} operations)", self.operations.len());
            for op in &self.operations {
                println!("    - {}", op);
            }
            TransactionBuilder {
                table: self.table,
                operations: self.operations,
                _state: std::marker::PhantomData,
            }
        }

        fn rollback(self) -> TransactionBuilder<TxIdle> {
            println!("  [tx] ROLLBACK");
            TransactionBuilder {
                table: self.table,
                operations: self.operations,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl TransactionBuilder<TxCommitted> {
        fn summary(&self) {
            println!("  [tx] Transaction complete on {}", self.table.as_ref().unwrap());
        }
    }

    // Normal flow
    let tx = start_transaction("users")
        .insert("(1, 'Alice')")
        .insert("(2, 'Bob')")
        .update("id = 1", "name = 'Alice Updated'")
        .delete("id = 2")
        .commit();
    tx.summary();

    println!();

    // Rollback flow
    let _tx = start_transaction("orders")
        .insert("(1, 'order1')")
        .rollback();

    // -------------------------------------------------------
    // 3. File Processing Pipeline
    // -------------------------------------------------------
    println!("\n--- 3. File Processing Pipeline ---");

    struct Pipeline<State> {
        input: String,
        steps: Vec<String>,
        _state: std::marker::PhantomData<State>,
    }

    struct PCreated;
    struct PValidated;
    struct PProcessed;
    struct PReady;

    fn create_pipeline(input: &str) -> Pipeline<PCreated> {
        println!("  [pipeline] Created with input: {}", input);
        Pipeline {
            input: input.to_string(),
            steps: Vec::new(),
            _state: std::marker::PhantomData,
        }
    }

    impl Pipeline<PCreated> {
        fn validate(self) -> Pipeline<PValidated> {
            println!("  [pipeline] Validated input: {}", self.input.len());
            Pipeline {
                input: self.input,
                steps: vec!["validate".to_string()],
                _state: std::marker::PhantomData,
            }
        }
    }

    impl Pipeline<PValidated> {
        fn transform(self) -> Pipeline<PProcessed> {
            let transformed = self.input.to_uppercase();
            println!("  [pipeline] Transformed: {} -> {}", self.input, transformed);
            let mut steps = self.steps;
            steps.push("transform".to_string());
            Pipeline {
                input: transformed,
                steps,
                _state: std::marker::PhantomData,
            }
        }

        fn skip_transform(self) -> Pipeline<PProcessed> {
            println!("  [pipeline] Skipping transform");
            Pipeline {
                input: self.input,
                steps: self.steps,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl Pipeline<PProcessed> {
        fn compress(self) -> Pipeline<PReady> {
            println!("  [pipeline] Compressed");
            let mut steps = self.steps;
            steps.push("compress".to_string());
            Pipeline {
                input: self.input,
                steps,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl Pipeline<PReady> {
        fn output(&self) -> String {
            println!("  [pipeline] Steps: {}", self.steps.join(" -> "));
            format!("Result: {} ({} bytes)", self.input, self.input.len())
        }
    }

    let result = create_pipeline("hello world")
        .validate()
        .transform()
        .compress()
        .output();
    println!("  {}\n", result);

    // Pipeline tanpa transform
    let result = create_pipeline("keep as is")
        .validate()
        .skip_transform()
        .compress()
        .output();
    println!("  {}\n", result);

    // -------------------------------------------------------
    // 4. Connection Pool dengan State
    // -------------------------------------------------------
    println!("--- 4. Connection Pool ---");

    struct PoolConfig<State> {
        max_connections: usize,
        timeout: u64,
        retry_count: u32,
        _state: std::marker::PhantomData<State>,
    }

    struct PoolNew;
    struct PoolConfigured;
    struct PoolActive;

    fn new_pool() -> PoolConfig<PoolNew> {
        PoolConfig {
            max_connections: 0,
            timeout: 0,
            retry_count: 0,
            _state: std::marker::PhantomData,
        }
    }

    impl PoolConfig<PoolNew> {
        fn max_connections(self, max: usize) -> PoolConfig<PoolConfigured> {
            println!("  [pool] Max connections: {}", max);
            PoolConfig {
                max_connections: max,
                timeout: 30,
                retry_count: 3,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl PoolConfig<PoolConfigured> {
        fn timeout(self, secs: u64) -> PoolConfig<PoolConfigured> {
            println!("  [pool] Timeout: {}s", secs);
            PoolConfig {
                timeout: secs,
                ..self
            }
        }

        fn retry_count(self, count: u32) -> PoolConfig<PoolConfigured> {
            println!("  [pool] Retry count: {}", count);
            PoolConfig {
                retry_count: count,
                ..self
            }
        }

        fn start(self) -> PoolConfig<PoolActive> {
            println!("  [pool] Pool started with {} connections", self.max_connections);
            PoolConfig {
                max_connections: self.max_connections,
                timeout: self.timeout,
                retry_count: self.retry_count,
                _state: std::marker::PhantomData,
            }
        }
    }

    impl PoolConfig<PoolActive> {
        fn status(&self) {
            println!("  [pool] Active: max={}, timeout={}s, retries={}",
                self.max_connections, self.timeout, self.retry_count);
        }

        fn shutdown(self) {
            println!("  [pool] Shutdown complete");
        }
    }

    let pool = new_pool()
        .max_connections(10)
        .timeout(60)
        .retry_count(5)
        .start();
    pool.status();
    pool.shutdown();

    println!("\n=== Ringkasan Typestate Lanjutan ===");
    println!("1. HTTP Builder - enforce URL/method sebelum send");
    println!("2. Transaction Builder - commit/rollback yang valid");
    println!("3. Pipeline - urutan proses yang benar");
    println!("4. Connection Pool - konfigurasi bertahap");
    println!("5. Semua validasi terjadi di COMPILE TIME!");
    println!("6. Zero-cost abstraction - tidak ada runtime overhead");
    println!("7. Mencegah bugs sebelum code dijalankan");
}
