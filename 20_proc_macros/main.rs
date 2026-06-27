// Procedural Macros biasanya dibuat dalam crate terpisah (#[proc-macro])
// Tapi untuk demo ini, kita simulasi konsepnya dengan declarative macros
// dan contoh pattern yang biasa dibuat dengan procedural macros.
//
// Dalam proyek nyata, kamu akan punya:
//   my_macros/           <- crate proc-macro
//     Cargo.toml          <- [lib] proc-macro = true
//     src/lib.rs          <- #[proc_macro_derive], #[proc_macro_attribute]
//   app/                  <- binary crate
//     Cargo.toml          <- my_macros = { path = "../my_macros" }
//     src/main.rs

#[allow(unused_macros)]
macro_rules! vec_of_strings {
    ($($x:expr),* $(,)?) => {
        vec![$($x.to_string()),*]
    };
}

fn main() {
    println!("=== 20. Procedural Macros ===\n");

    // -------------------------------------------------------
    // 1. Recap: Declarative Macros (macro_rules!)
    // -------------------------------------------------------
    println!("--- 1. Recap Declarative Macros ---");

    // macro_rules! sudah dipelajari di modul 17
    // Procedural macros BERBEDA: berbasis kode Rust yang dieksekusi saat compile time

    vec_of_strings!["hello", "world", "foo", "bar"].iter().for_each(|s| print!("{} ", s));
    println!();

    // -------------------------------------------------------
    // 2. Derive Macros (conceptual)
    // -------------------------------------------------------
    println!("\n--- 2. Derive Macros (conceptual) ---");

    // #[derive(Debug, Clone, PartialEq)] adalah procedural macro
    // Contoh:
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let p1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let p2 = p1.clone();
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p1 == p2: {}", p1 == p2);

    // Derive macro menghasilkan kode seperti:
    // impl std::fmt::Debug for Person { ... }
    // impl Clone for Person { ... }
    // impl PartialEq for Person { ... }

    // -------------------------------------------------------
    // 3. Attribute Macros (conceptual)
    // -------------------------------------------------------
    println!("\n--- 3. Attribute Macros (conceptual) ---");

    // Contoh attribute macro yang populer:
    // #[tokio::main]  -> ubah async fn jadi tokio runtime
    // #[test]         -> register fungsi sebagai test
    // #[route(GET, "/")] -> web framework routing

    // Pattern yang bisa diimplementasi dengan attribute macro:
    trait RouteHandler {
        fn handle(&self) -> String;
    }

    struct GetHandler;
    impl RouteHandler for GetHandler {
        fn handle(&self) -> String {
            "GET response".to_string()
        }
    }

    let handler = GetHandler;
    println!("handler result: {}", handler.handle());

    // -------------------------------------------------------
    // 4. Function-like Macros (conceptual)
    // -------------------------------------------------------
    println!("\n--- 4. Function-like Macros (conceptual) ---");

    // Contoh: sql!(...) -> embed SQL query saat compile time
    // sql!(SELECT * FROM users WHERE id = 1)
    // Macro ini parse string dan generate query builder code

    // Simulasi dengan declarative macro:
    macro_rules! sql {
        ($query:expr) => {
            format!("SQL: {}", $query)
        };
    }

    let query = sql!("SELECT * FROM users WHERE id = 1");
    println!("{}", query);

    // -------------------------------------------------------
    // 5. Simulasi Custom Derive Macro
    // -------------------------------------------------------
    println!("\n--- 5. Simulasi Custom Derive ---");

    // Pattern: Builder pattern via macro_rules!
    // Dalam proc macro, ini bisa lebih elegan

    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        debug: bool,
    }

    // Simulasi: generate Builder dari struct
    macro_rules! generate_builder {
        ($struct_name:ident {
            $( $field:ident : $field_type:ty ),* $(,)?
        }) => {
            paste::paste! {
                #[derive(Debug, Default)]
                struct [<$struct_name Builder>] {
                    $( $field: Option<$field_type>, )*
                }

                impl [<$struct_name Builder>] {
                    fn new() -> Self {
                        Self::default()
                    }

                    $(
                        fn $field(mut self, value: $field_type) -> Self {
                            self.$field = Some(value);
                            self
                        }
                    )*

                    fn build(self) -> Result<$struct_name, String> {
                        Ok($struct_name {
                            $(
                                $field: self.$field.ok_or(
                                    concat!("field '", stringify!($field), "' is required")
                                )?,
                            )*
                        })
                    }
                }
            }
        };
    }

    // Kita tidak punya paste crate, jadi kita tulis manual untuk demo ini
    // generate_builder! akan menghasilkan ConfigBuilder

    // Builder pattern manual (apa yang proc macro hasilkan):
    #[derive(Debug, Default)]
    struct ConfigBuilder {
        host: Option<String>,
        port: Option<u16>,
        debug: Option<bool>,
    }

    impl ConfigBuilder {
        fn new() -> Self {
            Self::default()
        }

        fn host(mut self, value: String) -> Self {
            self.host = Some(value);
            self
        }

        fn port(mut self, value: u16) -> Self {
            self.port = Some(value);
            self
        }

        fn debug(mut self, value: bool) -> Self {
            self.debug = Some(value);
            self
        }

        fn build(self) -> Result<Config, String> {
            Ok(Config {
                host: self.host.ok_or("field 'host' is required")?,
                port: self.port.ok_or("field 'port' is required")?,
                debug: self.debug.unwrap_or(false),
            })
        }
    }

    let config = ConfigBuilder::new()
        .host("localhost".to_string())
        .port(8080)
        .debug(true)
        .build()
        .unwrap();

    println!("Config: {:#?}", config);

    // -------------------------------------------------------
    // 6. Serde: Contoh Proc Macro yang Populer
    // -------------------------------------------------------
    println!("\n--- 6. Serde Pattern ---");

    #[derive(Debug)]
    struct Serializable {
        name: String,
        value: i32,
    }

    // Serde derive macro menghasilkan:
    // - impl Serialize (menulis ke JSON/TOML/dll)
    // - impl Deserialize (membaca dari format apapun)

    // Tanpa serde, kita bisa tulis manual:
    impl Serializable {
        fn to_json_manual(&self) -> String {
            format!(
                r#"{{"name":"{}","value":{}}}"#,
                self.name, self.value
            )
        }
    }

    let s = Serializable {
        name: "test".to_string(),
        value: 42,
    };
    println!("manual JSON: {}", s.to_json_manual());

    // -------------------------------------------------------
    // 7. Compile-time Code Generation
    // -------------------------------------------------------
    println!("\n--- 7. Compile-time Code Generation ---");

    // Proc macro bisa melakukan validasi saat compile time!

    // Contoh: validate field names, cek type, generate impl

    macro_rules! assert_valid_ident {
        ($name:ident) => {
            // Dalam proc macro, kita bisa validate di compile time
            // Contoh sederhana: pastikan identifier bukan keyword
            paste::paste! {
                // Ini hanya demonstrasi konsep
            }
        };
    }

    // Contoh: auto-generate Display impl untuk enum
    macro_rules! auto_display {
        ($enum_name:ident {
            $( $variant:ident => $display:expr ),* $(,)?
        }) => {
            impl std::fmt::Display for $enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $( $enum_name::$variant => write!(f, "{}", $display), )*
                    }
                }
            }
        };
    }

    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    auto_display!(Color {
        Red => "🔴 Red",
        Green => "🟢 Green",
        Blue => "🔵 Blue",
    });

    for color in [Color::Red, Color::Green, Color::Blue] {
        println!("Color: {} (debug: {:?})", color, color);
    }

    println!("\n=== Ringkasan Procedural Macros ===");
    println!("1. Derive macros (#[derive(MyTrait)]) - generate impl otomatis");
    println!("2. Attribute macros (#[my_attr]) - transform item");
    println!("3. Function-like macros (my_macro!(...)) - macro seperti fungsi");
    println!("4. Crate terpisah dengan [lib] proc-macro = true");
    println!("5. Menggunakan proc_macro::TokenStream");
    println!("6. Contoh populer: serde, tokio, thiserror, clap");
}
