use warp::Filter;
use rusqlite::Connection;
use crate::models::usuario::Usuario;
use crate::db::usuario::{get_usuarios, add_usuario, update_usuario, delete_usuario, verificar_usuario};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    nombre: String,
    clave: String,
}

async fn handle_login(info: LoginInfo, db: Arc<Mutex<Connection>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    match verificar_usuario(&db, &info.nombre, &info.clave) {
        Ok(Some(_usuario)) => Ok(warp::reply::json(&"Login exitoso")),
        Ok(None) => Ok(warp::reply::json(&"Credenciales incorrectas")),
        Err(_) => Ok(warp::reply::json(&"Error interno del servidor")),
    }
}

async fn handle_get_usuarios(db: Arc<Mutex<Connection>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    let usuarios = get_usuarios(&db).unwrap_or_else(|_| vec![]);
    Ok(warp::reply::json(&usuarios))
}

async fn handle_add_usuario(usuario: Usuario, db: Arc<Mutex<Connection>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    add_usuario(&db, &usuario).unwrap();
    Ok(warp::reply::json(&"Usuario agregado exitosamente"))
}

async fn handle_update_usuario(usuario: Usuario, db: Arc<Mutex<Connection>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    update_usuario(&db, &usuario).unwrap();
    Ok(warp::reply::json(&"Usuario actualizado exitosamente"))
}

async fn handle_delete_usuario(id: u32, db: Arc<Mutex<Connection>>) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().await;
    delete_usuario(&db, id).unwrap();
    Ok(warp::reply::json(&"Usuario eliminado exitosamente"))
}

pub fn routes(db_path: &str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db = Arc::new(Mutex::new(Connection::open(db_path).unwrap()));

    // Ruta GET para usuarios
    let get_usuarios_route = warp::path("usuarios")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handle_get_usuarios);

    // Ruta POST para usuarios
    let post_usuarios_route = warp::path("usuarios")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handle_add_usuario);

    // Ruta PUT para usuarios
    let put_usuarios_route = warp::path("usuarios")
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handle_update_usuario);

    // Ruta DELETE para usuarios
    let delete_usuarios_route = warp::path!("usuarios" / u32)
        .and(warp::delete())
        .and(with_db(db.clone()))
        .and_then(handle_delete_usuario);

    // Ruta POST para login
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handle_login);

    // Combina todas las rutas
    get_usuarios_route
        .or(post_usuarios_route)
        .or(put_usuarios_route)
        .or(delete_usuarios_route)
        .or(login_route)
}

fn with_db(db: Arc<Mutex<Connection>>) -> impl Filter<Extract = (Arc<Mutex<Connection>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
