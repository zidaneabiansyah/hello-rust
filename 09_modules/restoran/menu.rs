pub struct Menu {
    pub nama: String,
    pub harga: u32,
}

impl Menu {
    pub fn baru(nama: &str, harga: u32) -> Menu {
        Menu {
            nama: String::from(nama),
            harga,
        }
    }
}
