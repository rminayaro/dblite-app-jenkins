use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Usuario {
    pub id: Option<u32>,            // Asegúrate de que esta propiedad esté presente
    pub nombre: String,
    pub email: String,
    pub telefono: String,
    pub direccion: String,
    pub clave: String,
}
