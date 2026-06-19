use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub nama: String,
    pub deskripsi: String,
    pub selesai: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub nama: String,
    pub deskripsi: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItem {
    pub nama: Option<String>,
    pub deskripsi: Option<String>,
    pub selesai: Option<bool>,
}

impl Item {
    pub fn baru(nama: String, deskripsi: String) -> Self {
        Item {
            id: Uuid::new_v4().to_string(),
            nama,
            deskripsi,
            selesai: false,
        }
    }

    pub fn update(&mut self, data: UpdateItem) {
        if let Some(nama) = data.nama {
            self.nama = nama;
        }
        if let Some(deskripsi) = data.deskripsi {
            self.deskripsi = deskripsi;
        }
        if let Some(selesai) = data.selesai {
            self.selesai = selesai;
        }
    }
}
