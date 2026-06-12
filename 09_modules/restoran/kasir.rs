use super::pelanggan::Pelanggan;

pub fn hitung(pesanan: &[super::menu::Menu]) -> u32 {
    pesanan.iter().map(|m| m.harga).sum()
}

pub fn cetak_struk(pelanggan: &Pelanggan) {
    println!("\nSTRUK PEMBAYARAN");
    println!("Pelanggan: {}", pelanggan.nama);
    println!("----------------");
    for item in &pelanggan.pesanan {
        println!("{} - Rp{}", item.nama, item.harga);
    }
    println!("----------------");
    println!("Total: Rp{}", hitung(&pelanggan.pesanan));
}
