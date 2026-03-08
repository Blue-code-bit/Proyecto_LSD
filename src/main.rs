// Importamos las librerías necesarias
use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder}; // Actix-web para servidor y rutas
use serde::{Serialize, Deserialize}; // Serde para convertir datos entre JSON y Rust
use sqlx::sqlite::SqlitePoolOptions; // SQLx para conectarnos a SQLite

// Definimos la estructura de datos que representa una lección
#[derive(Serialize, Deserialize)] // Permite convertir entre JSON y Rust automáticamente
struct Leccion {
    // Identificador único de la lección
    id: i32,
     // Título de la lección
    titulo: String, 
    // Descripción breve
    descripcion: String,
    // Enlace al video
    url_video: String,    
}

// GET: obtener todas las lecciones desde la base de datos
#[get("/lecciones")]
async fn obtener_lecciones(pool: web::Data<sqlx::SqlitePool>) -> impl Responder {
    let lecciones = sqlx::query_as!(
        Leccion,
        r#"SELECT id, titulo, descripcion, url_video FROM lecciones"#
    )
    .fetch_all(pool.get_ref())
    .await
    .expect("Error al obtener lecciones");

    HttpResponse::Ok().json(lecciones)
}

// POST: crear una nueva lección
#[post("/lecciones")]
async fn crear_leccion(pool: web::Data<sqlx::SqlitePool>, leccion: web::Json<Leccion>) -> impl Responder {
    let resultado = sqlx::query!(
        r#"INSERT INTO lecciones (titulo, descripcion, url_video) VALUES (?, ?, ?)"#,
        leccion.titulo,
        leccion.descripcion,
        leccion.url_video
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Created().body("Lección creada"),
        Err(_) => HttpResponse::InternalServerError().body("Error al crear lección"),
    }
}

// PUT: actualizar una lección existente
#[put("/lecciones/{id}")]
async fn actualizar_leccion(
    pool: web::Data<sqlx::SqlitePool>,
    path: web::Path<i32>,
    leccion: web::Json<Leccion>,
) -> impl Responder {
    let id = path.into_inner();

    let resultado = sqlx::query!(
        r#"UPDATE lecciones SET titulo = ?, descripcion = ?, url_video = ? WHERE id = ?"#,
        leccion.titulo,
        leccion.descripcion,
        leccion.url_video,
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body("Lección actualizada"),
        Err(_) => HttpResponse::InternalServerError().body("Error al actualizar lección"),
    }
}

// DELETE: borrar una lección
#[delete("/lecciones/{id}")]
async fn borrar_leccion(pool: web::Data<sqlx::SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    let resultado = sqlx::query!(
        r#"DELETE FROM lecciones WHERE id = ?"#,
        id
    )
    .execute(pool.get_ref())
    .await;

    match resultado {
        Ok(_) => HttpResponse::Ok().body(format!("Lección {} eliminada", id)),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar lección"),
    }
}

// Función principal
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://proyecto_lsd.db")
        .await
        .expect("No se pudo conectar a la base de datos");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(obtener_lecciones)
            .service(crear_leccion)
            .service(actualizar_leccion)
            .service(borrar_leccion)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

