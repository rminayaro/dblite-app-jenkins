use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Categoria {
    pub id: Option<u32>, // Hacer el id opcional
    pub nombre: String,
    pub descripcion: String,
}
