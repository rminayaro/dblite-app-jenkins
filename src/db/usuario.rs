use rusqlite::{Connection, Result};
use crate::models::usuario::Usuario;

pub fn get_usuarios(conn: &Connection) -> Result<Vec<Usuario>> {
    let mut stmt = conn.prepare("SELECT UsuarioId, Nombre, Email, Telefono, Direccion, Clave FROM usuarios LIMIT 1000")?;
    let usuario_iter = stmt.query_map([], |row| {
        Ok(Usuario {
            id: Some(row.get(0)?),
            nombre: row.get(1)?,
            email: row.get(2)?,
            telefono: row.get(3)?,
            direccion: row.get(4)?,
            clave: row.get(5)?,
        })
    })?;

    let mut usuarios = Vec::new();
    for usuario in usuario_iter {
        usuarios.push(usuario?);
    }
    Ok(usuarios)
}

pub fn add_usuario(conn: &Connection, usuario: &Usuario) -> Result<usize> {
    conn.execute(
        "INSERT INTO usuarios (Nombre, Email, Telefono, Direccion, Clave) VALUES (?1, ?2, ?3, ?4, ?5)",
        [
            &usuario.nombre,
            &usuario.email,
            &usuario.telefono,
            &usuario.direccion,
            &usuario.clave,
        ],
    )
}

pub fn update_usuario(conn: &Connection, usuario: &Usuario) -> Result<usize> {
    conn.execute(
        "UPDATE usuarios SET Nombre = ?1, Email = ?2, Telefono = ?3, Direccion = ?4, Clave = ?5 WHERE UsuarioId = ?6",
        [
            &usuario.nombre,
            &usuario.email,
            &usuario.telefono,
            &usuario.direccion,
            &usuario.clave,
            &usuario.id.expect("ID no encontrado").to_string(),
        ],
    )
}

pub fn delete_usuario(conn: &Connection, id: u32) -> Result<usize> {
    conn.execute(
        "DELETE FROM usuarios WHERE UsuarioId = ?1",
        [id],
    )
}


pub fn verificar_usuario(conn: &Connection, nombre: &str, clave: &str) -> Result<Option<Usuario>> {
    let mut stmt = conn.prepare("SELECT UsuarioId, Nombre, Email, Telefono, Direccion, Clave FROM usuarios WHERE Nombre = ?1 AND Clave = ?2 LIMIT 1")?;
    let mut usuario_iter = stmt.query_map([nombre, clave], |row| {
        Ok(Usuario {
            id: Some(row.get(0)?),
            nombre: row.get(1)?,
            email: row.get(2)?,
            telefono: row.get(3)?,
            direccion: row.get(4)?,
            clave: row.get(5)?,
        })
    })?;

    if let Some(usuario) = usuario_iter.next() {
        usuario.map(Some)
    } else {
        Ok(None)
    }
}
