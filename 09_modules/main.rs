mod restoran;

use restoran::{dapur, kasir, menu};
use restoran::pelanggan::Pelanggan;

fn main() {
    println!("RESTORAN RUST");

    let menu_makanan = menu::Menu::baru("Nasi Goreng", 25000);
    println!("Menu: {} - Rp{}", menu_makanan.nama, menu_makanan.harga);

    match dapur::masak(&menu_makanan) {
        Ok(hidangan) => println!("Hidangan siap: {}", hidangan),
        Err(e) => println!("Gagal masak: {}", e),
    }

    let mut pelanggan = Pelanggan::baru("Budi");
    pelanggan.pesan(menu_makanan);

    let total = kasir::hitung(&pelanggan.pesanan);
    println!("Total pesanan: Rp{}", total);

    kasir::cetak_struk(&pelanggan);
}
