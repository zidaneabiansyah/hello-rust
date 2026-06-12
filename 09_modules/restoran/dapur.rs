use super::menu::Menu;

#[derive(Debug)]
pub enum BahanError {
    Habis(String),
}

impl From<BahanError> for String {
    fn from(e: BahanError) -> String {
        match e {
            BahanError::Habis(msg) => msg,
        }
    }
}

pub fn cek_bahan(menu: &Menu) -> Result<(), BahanError> {
    if menu.nama == "Nasi Goreng" {
        Ok(())
    } else {
        Err(BahanError::Habis(format!("Bahan untuk {} habis", menu.nama)))
    }
}

pub fn masak(menu: &Menu) -> Result<String, String> {
    cek_bahan(menu)?;
    Ok(format!("{} siap disajikan!", menu.nama))
}
