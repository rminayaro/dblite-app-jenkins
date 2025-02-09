use warp::Filter;
use rusqlite::{Connection, OpenFlags};
use crate::models::categoria::Categoria;
use crate::db::categoria::{get_categorias, add_categoria, update_categoria, delete_categoria};

pub fn routes(db_path: &str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let db_path_for_get = db_path.to_string();
    let db_path_for_post = db_path.to_string();
    let db_path_for_put = db_path.to_string();
    let db_path_for_delete = db_path.to_string();

    // Ruta GET para categorias
    let get_categorias_route = warp::path("categorias")
        .and(warp::get())
        .and_then(move || {
            let conn = Connection::open_with_flags(
                db_path_for_get.clone(),
                OpenFlags::SQLITE_OPEN_READ_ONLY
            ).expect("Conexión fallida");
            async move {
                let categorias = get_categorias(&conn).unwrap_or_else(|_| vec![]);
                Ok::<_, warp::Rejection>(warp::reply::json(&categorias))
            }
        });

    // Ruta POST para categorias
    let post_categorias_route = warp::path("categorias")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |categoria: Categoria| {
            let conn = Connection::open_with_flags(
                db_path_for_post.clone(),
                OpenFlags::SQLITE_OPEN_READ_WRITE
            ).expect("Conexión fallida");
            async move {
                add_categoria(&conn, &categoria).unwrap();
                Ok::<_, warp::Rejection>(warp::reply::json(&"Categoría agregada exitosamente"))
            }
        });

    // Ruta PUT para categorias
    let put_categorias_route = warp::path("categorias")
        .and(warp::put())
        .and(warp::body::json())
        .and_then(move |categoria: Categoria| {
            let conn = Connection::open_with_flags(
                db_path_for_put.clone(),
                OpenFlags::SQLITE_OPEN_READ_WRITE
            ).expect("Conexión fallida");
            async move {
                update_categoria(&conn, &categoria).unwrap();
                Ok::<_, warp::Rejection>(warp::reply::json(&"Categoría actualizada exitosamente"))
            }
        });

    // Ruta DELETE para categorias
    let delete_categorias_route = warp::path!("categorias" / u32)
        .and(warp::delete())
        .and_then(move |id: u32| {
            let conn = Connection::open_with_flags(
                db_path_for_delete.clone(),
                OpenFlags::SQLITE_OPEN_READ_WRITE
            ).expect("Conexión fallida");
            async move {
                delete_categoria(&conn, id).unwrap();
                Ok::<_, warp::Rejection>(warp::reply::json(&"Categoría eliminada exitosamente"))
            }
        });

    get_categorias_route.or(post_categorias_route).or(put_categorias_route).or(delete_categorias_route)
}
