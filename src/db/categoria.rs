use rusqlite::{Connection, Result};
use crate::models::categoria::Categoria;

pub fn get_categorias(conn: &Connection) -> Result<Vec<Categoria>> {
    let mut stmt = conn.prepare("SELECT CategoriaId, Nombre, Descripcion FROM categorias LIMIT 1000")?;
    let categoria_iter = stmt.query_map([], |row| {
        Ok(Categoria {
            id: Some(row.get(0)?), // Guardar como Some(id)
            nombre: row.get(1)?,
            descripcion: row.get(2)?,
        })
    })?;

    let mut categorias = Vec::new();
    for categoria in categoria_iter {
        categorias.push(categoria?);
    }
    Ok(categorias)
}

pub fn add_categoria(conn: &Connection, categoria: &Categoria) -> Result<usize> {
    conn.execute(
        "INSERT INTO categorias (Nombre, Descripcion) VALUES (?1, ?2)",
        [
            &categoria.nombre,
            &categoria.descripcion,
        ],
    )
}

pub fn update_categoria(conn: &Connection, categoria: &Categoria) -> Result<usize> {
    conn.execute(
        "UPDATE categorias SET Nombre = ?1, Descripcion = ?2 WHERE CategoriaId = ?3",
        [
            &categoria.nombre,
            &categoria.descripcion,
            &categoria.id.expect("ID no encontrado").to_string(), // Manejar el Option
        ],
    )
}

pub fn delete_categoria(conn: &Connection, id: u32) -> Result<usize> {
    conn.execute(
        "DELETE FROM categorias WHERE CategoriaId = ?1",
        [id],
    )
}
