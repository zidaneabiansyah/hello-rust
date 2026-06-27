#[allow(dead_code)]
fn main() {
    println!("=== 19. Trait Objects & Dynamic Dispatch ===\n");

    // -------------------------------------------------------
    // 1. Dasar Trait Objects (dyn Trait)
    // -------------------------------------------------------
    println!("--- 1. Basic Trait Objects ---");

    let cat = Cat { name: "Kitty".to_string() };
    let dog = Dog { name: "Buddy".to_string() };

    // Vec berisi berbagai tipe yang implement Animal
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(cat),
        Box::new(dog),
        Box::new(Cat { name: "Mimi".to_string() }),
    ];

    for animal in &animals {
        animal.speak();
        animal.info();
    }

    // -------------------------------------------------------
    // 2. Static Dispatch vs Dynamic Dispatch
    // -------------------------------------------------------
    println!("\n--- 2. Static vs Dynamic Dispatch ---");

    // Static dispatch: compiler tahu tipe saat compile time
    fn static_dispatch(animal: &impl Animal) {
        print!("[static] ");
        animal.speak();
    }

    // Dynamic dispatch: tipe ditentukan saat runtime
    fn dynamic_dispatch(animal: &dyn Animal) {
        print!("[dynamic] ");
        animal.speak();
    }

    let cat = Cat { name: "StaticCat".to_string() };
    static_dispatch(&cat);
    dynamic_dispatch(&cat);

    // -------------------------------------------------------
    // 3. Object Safety Rules
    // -------------------------------------------------------
    println!("\n--- 3. Object Safety ---");

    // Trait yang object-safe: tidak puny Self di posisi tertentu
    // Tidak object-safe:
    //   fn clone(&self) -> Self;           // return Self
    //   fn new() -> Self where Self: Sized; // Sized bound
    //   fn generic_method<T>(&self, t: T);  // generic

    // Contoh: Defaultable TIDAK object-safe (punya Self)
    // let obj: &dyn Defaultable = ...; // ERROR

    // Contoh: Printable object-safe
    let obj: &dyn Printable = &MyPrintable { data: "hello".to_string() };
    obj.print(); // OK

    // -------------------------------------------------------
    // 4. Trait Objects with Multiple Traits (Object Safe)
    // -------------------------------------------------------
    println!("\n--- 4. Multiple Trait Objects ---");

    // Menggunakan supertraits untuk multiple trait
    // Animal sudah punya Display sebagai supertrait, jadi bisa langsung pakai
    fn describe(animal: &dyn Animal) {
        print!("[display+animal] ");
        println!("Display: {}", animal);
        animal.speak();
    }

    let cat = Cat { name: "DisplayCat".to_string() };
    describe(&cat);

    // -------------------------------------------------------
    // 5. Box<dyn Trait> as Return Type
    // -------------------------------------------------------
    println!("\n--- 5. Returning Trait Objects ---");

    let animal = create_animal("cat", "Whiskers");
    animal.speak();

    let animal = create_animal("dog", "Rex");
    animal.speak();

    // -------------------------------------------------------
    // 6. dyn Trait in Enums (Alternative Pattern)
    // -------------------------------------------------------
    println!("\n--- 6. Enum Dispatch Alternative ---");

    enum AnimalEnum {
        Cat(Cat),
        Dog(Dog),
    }

    impl AnimalEnum {
        fn speak(&self) {
            match self {
                AnimalEnum::Cat(c) => c.speak(),
                AnimalEnum::Dog(d) => d.speak(),
            }
        }
    }

    let animals_enum = vec![
        AnimalEnum::Cat(Cat { name: "EnumCat".to_string() }),
        AnimalEnum::Dog(Dog { name: "EnumDog".to_string() }),
    ];

    for a in &animals_enum {
        a.speak();
    }

    // -------------------------------------------------------
    // 7. VTable & Memory Layout
    // -------------------------------------------------------
    println!("\n--- 7. VTable Info ---");

    // Trait object = fat pointer (data ptr + vtable ptr)
    // VTable berisi: size, align, drop function, dan function pointers

    let cat = Cat { name: "VTableCat".to_string() };
    let animal: &dyn Animal = &cat;

    // Memastikan trait object bekerja
    println!("Cat name via Animal: {}", animal.name());
    println!("Cat speak via Animal: {}", {
        animal.speak();
        "ok"
    });

    // -------------------------------------------------------
    // 8. Weak<dyn Trait> dan Rc<dyn Trait>
    // -------------------------------------------------------
    println!("\n--- 8. Smart Pointers with Trait Objects ---");

    use std::rc::Rc;

    let shared: Rc<dyn Animal> = Rc::new(Cat { name: "SharedCat".to_string() });
    let shared2 = Rc::clone(&shared);
    println!("strong_count: {}", Rc::strong_count(&shared));
    shared.speak();
    shared2.speak();

    println!("\n=== Ringkasan Trait Objects ===");
    println!("1. dyn Trait - dynamic dispatch saat runtime");
    println!("2. Static dispatch (impl Trait) - zero-cost, monomorphized");
    println!("3. Object safety - aturan yang menentukan bisa dibuat trait object");
    println!("4. Box<dyn Trait> - heap-allocated trait object");
    println!("5. &dyn Trait - borrowed trait object");
    println!("6. Enum dispatch - alternatif tanpa dynamic dispatch");
    println!("7. VTable - fat pointer dengan metadata fungsi virtual");
}

// ============================================================
// Traits
// ============================================================

trait Animal: std::fmt::Display {
    fn speak(&self);
    fn info(&self) {
        println!("  (no info available)");
    }
    fn name(&self) -> &str;
}

// Object safety: tidak ada Self di return/posisi generic
trait Printable {
    fn print(&self);
}

// TIDAK object-safe: punya generic method
trait Cloneable: Sized {
    fn clone_self(&self) -> Self;
}

// TIDAK object-safe: return Self
trait Factory {
    fn create() -> Self where Self: Sized;
}

// ============================================================
// Structs
// ============================================================

#[derive(Debug, Clone)]
struct Cat {
    name: String,
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cat({})", self.name)
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("  {} says: Meow!", self.name);
    }

    fn info(&self) {
        println!("  {} is a cat, type: feline", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Printable for Cat {
    fn print(&self) {
        println!("  Printing Cat: {}", self.name);
    }
}

#[derive(Debug, Clone)]
struct Dog {
    name: String,
}

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dog({})", self.name)
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("  {} says: Woof!", self.name);
    }

    fn info(&self) {
        println!("  {} is a dog, type: canine", self.name);
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct MyPrintable {
    data: String,
}

impl Printable for MyPrintable {
    fn print(&self) {
        println!("  MyPrintable: {}", self.data);
    }
}

// ============================================================
// Functions
// ============================================================

fn create_animal(kind: &str, name: &str) -> Box<dyn Animal> {
    match kind {
        "cat" => Box::new(Cat { name: name.to_string() }),
        "dog" => Box::new(Dog { name: name.to_string() }),
        _ => panic!("unknown animal kind: {}", kind),
    }
}
