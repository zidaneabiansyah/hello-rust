use super::menu::Menu;

pub struct Pelanggan {
    pub nama: String,
    pub pesanan: Vec<Menu>,
}

impl Pelanggan {
    pub fn baru(nama: &str) -> Pelanggan {
        Pelanggan {
            nama: String::from(nama),
            pesanan: Vec::new(),
        }
    }

    pub fn pesan(&mut self, menu: Menu) {
        self.pesanan.push(menu);
    }
}
