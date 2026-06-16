use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("1. STRING METHODS");
    let s = "  Hello, Rust World!  ";

    println!("trim: '{}'", s.trim());
    println!("to_lowercase: {}", s.to_lowercase());
    println!("to_uppercase: {}", s.to_uppercase());
    println!("contains 'Rust': {}", s.contains("Rust"));
    println!("replace: {}", s.replace("Rust", "Rust Lang"));

    let words: Vec<&str> = s.trim().split_whitespace().collect();
    println!("split whitespace: {:?}", words);

    let csv = "apel,mangga,jeruk";
    let items: Vec<&str> = csv.split(',').collect();
    println!("split comma: {:?}", items);

    let s2 = "Rust";
    println!("chars: {:?}", s2.chars().collect::<Vec<char>>());
    println!("starts_with 'R': {}", s2.starts_with('R'));
    println!("ends_with 'st': {}", s2.ends_with("st"));

    let multiline = "baris 1\nbaris 2\nbaris 3";
    for (i, line) in multiline.lines().enumerate() {
        println!("  line {}: {}", i, line);
    }

    println!("\n2. STRING SLICING & COLLECT");
    let s = String::from("Hello Rust");
    let slice = &s[0..5];
    println!("slice [0..5]: {}", slice);

    let collected: String = vec!["Hello", " ", "World"].iter().copied().collect();
    println!("collect from vec: {}", collected);

    let joined = ["a", "b", "c"].join(", ");
    println!("join: {}", joined);

    println!("\n3. FORMATTING");
    let nama = "Budi";
    let umur = 25;
    let msg = format!("Nama: {}, Umur: {}", nama, umur);
    println!("format!: {}", msg);
    println!("{:?}", vec![1, 2, 3]);
    println!("{:#?}", vec![1, 2, 3]);
    println!("{:.2}", 3.14159);
    println!("{:>10} {:010}", "kanan", 42);
    println!("{:<10} kiri", "xxx");

    println!("\n4. PATH & PATHBUF");
    let path = Path::new("/home/user/documents/file.txt");
    println!("file_name: {:?}", path.file_name());
    println!("parent: {:?}", path.parent());
    println!("extension: {:?}", path.extension());
    println!("stem: {:?}", path.file_stem());
    println!("is_absolute: {}", path.is_absolute());

    let mut pb = PathBuf::from("/home/user");
    pb.push("documents");
    pb.push("file.txt");
    println!("PathBuf push: {:?}", pb);
    pb.pop();
    println!("after pop: {:?}", pb);

    let p = Path::new("data");
    fs::create_dir_all(p.join("sub"))?;
    println!("created dir: {:?}", p);

    println!("\n5. WRITE FILE");
    let mut file = File::create("data/contoh.txt")?;
    file.write_all(b"Halo Rust!\n")?;
    file.write_all(b"Ini baris kedua")?;
    println!("  file written");

    println!("\n6. READ FILE (read_to_string)");
    let content = fs::read_to_string("data/contoh.txt")?;
    println!("  content:\n{}", content);

    println!("\n7. READ FILE (BufReader — line by line)");
    let file = File::open("data/contoh.txt")?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("  line {}: {}", i, line);
    }

    println!("\n8. BINARY READ/WRITE");
    let mut file = File::create("data/binary.bin")?;
    let bytes = vec![0u8, 1, 2, 3, 255, 128];
    file.write_all(&bytes)?;

    let mut file = File::open("data/binary.bin")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    println!("  read bytes: {:?}", buf);

    println!("\n9. OPENOPTIONS — append, read_write, create_if_not_exists");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("data/contoh.txt")?;
    file.write_all(b"\nbaris baru (append)")?;
    println!("  appended");

    let content = fs::read_to_string("data/contoh.txt")?;
    println!("  after append:\n{}", content);

    println!("\n10. SEEK — baca dari posisi tertentu");
    let mut file = File::open("data/contoh.txt")?;
    let mut buf = [0u8; 5];
    file.seek(SeekFrom::Start(5))?;
    file.read_exact(&mut buf)?;
    println!("  bytes at offset 5: {:?}", String::from_utf8_lossy(&buf));

    println!("\n11. BUFWRITER — buffered write (lebih cepat)");
    let file = File::create("data/buffered.txt")?;
    let mut writer = BufWriter::new(file);
    for i in 0..100 {
        writeln!(writer, "Baris ke-{}", i)?;
    }
    writer.flush()?;
    println!("  100 lines written");

    println!("\n12. DIRECTORY OPERATIONS");
    let entries = fs::read_dir("data")?;
    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;
        let kind = if meta.is_dir() { "DIR" } else { "FILE" };
        println!("  [{}] {:?} ({} bytes)", kind, entry.file_name(), meta.len());
    }

    println!("\n13. FILE INFO");
    let meta = fs::metadata("data/contoh.txt")?;
    println!("  len: {} bytes", meta.len());
    println!("  readonly: {}", meta.permissions().readonly());
    println!("  modified: {:?}", meta.modified());

    println!("\n14. CREATE & REMOVE");
    fs::write("data/sementara.txt", "temporary")?;
    println!("  created sementara.txt");
    fs::remove_file("data/sementara.txt")?;
    println!("  removed sementara.txt");

    fs::remove_dir_all("data/sub")?;
    println!("  removed data/sub");

    // cleanup
    Ok(())
}
