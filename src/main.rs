// Importamos las librerías necesarias para nuestro servidor
use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder}; 
// actix-web: framework para crear el servidor y definir rutas HTTP

use serde::{Serialize, Deserialize}; 
// serde: librería que nos ayuda a convertir datos entre JSON y estructuras de Rust

use sqlx::sqlite::SqlitePoolOptions; 
// sqlx: librería para conectarnos y trabajar con la base de datos SQLite

// Definimos la estructura que representa una lección en nuestra base de datos
#[derive(Serialize, Deserialize)] // Esto permite que se convierta automáticamente a JSON y desde JSON
struct Leccion {
    id: i32,              // Identificador único de la lección
    titulo: String,       // Título de la lección
    descripcion: String,  // Breve descripción de la lección
    url_video: String,    // Enlace al video asociado
}

// RUTAS CRUD

// GET: obtener todas las lecciones guardadas en la base de datos
#[get("/lecciones")] // Esta ruta se activa cuando alguien hace GET a /lecciones
async fn obtener_lecciones(pool: web::Data<sqlx::SqlitePool>) -> impl Responder {
    // Ejecutamos una consulta SQL que trae todas las filas de la tabla "lecciones"
    let lecciones = sqlx::query_as!(
        Leccion, // Los resultados se convierten en nuestra estructura Leccion
        r#"SELECT id, titulo, descripcion, url_video FROM lecciones"#
    )
    .fetch_all(pool.get_ref()) // Ejecutamos la consulta usando la conexión a la base
    .await
    .expect("Error al obtener lecciones"); // Si algo falla, mostramos un error

    HttpResponse::Ok().json(lecciones) // Respondemos con un JSON que contiene todas las lecciones
}

// POST: crear una nueva lección
#[post("/lecciones")] // Esta ruta se activa cuando alguien hace POST a /lecciones
async fn crear_leccion(pool: web::Data<sqlx::SqlitePool>, leccion: web::Json<Leccion>) -> impl Responder {
    // Insertamos una nueva fila en la tabla "lecciones"
    let resultado = sqlx::query!(
        r#"INSERT INTO lecciones (titulo, descripcion, url_video) VALUES (?, ?, ?)"#,
        // Título que viene en el JSON
        leccion.titulo,
        // Descripción que viene en el JSON
        leccion.descripcion,  
        // URL del video que viene en el JSON
        leccion.url_video     
    )
    .execute(pool.get_ref()) // Ejecutamos la consulta
    .await;

    // Verificamos si se insertó correctamente
    match resultado {
        Ok(_) => HttpResponse::Created().body("Lección creada"), // Si todo salió bien
        Err(_) => HttpResponse::InternalServerError().body("Error al crear lección"), // Si hubo error
    }
}

// PUT: actualizar una lección existente
#[put("/lecciones/{id}")] // Esta ruta se activa cuando alguien hace PUT a /lecciones/{id}
async fn actualizar_leccion(
    pool: web::Data<sqlx::SqlitePool>, // Conexión a la base
    path: web::Path<i32>,              // ID de la lección que viene en la URL
    leccion: web::Json<Leccion>,       // Datos nuevos que vienen en el JSON
) -> impl Responder {
    let id = path.into_inner(); // Extraemos el ID de la URL

    // Ejecutamos la consulta SQL para actualizar la fila correspondiente
    let resultado = sqlx::query!(
        r#"UPDATE lecciones SET titulo = ?, descripcion = ?, url_video = ? WHERE id = ?"#,
        leccion.titulo,
        leccion.descripcion,
        leccion.url_video,
        id
    )
    .execute(pool.get_ref())
    .await;

    // Verificamos si se actualizó correctamente
    match resultado {
        Ok(_) => HttpResponse::Ok().body("Lección actualizada"),
        Err(_) => HttpResponse::InternalServerError().body("Error al actualizar lección"),
    }
}

// DELETE: borrar una lección
#[delete("/lecciones/{id}")] // Esta ruta se activa cuando alguien hace DELETE a /lecciones/{id}
async fn borrar_leccion(pool: web::Data<sqlx::SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner(); // Extraemos el ID de la URL

    // Ejecutamos la consulta SQL para eliminar la fila correspondiente
    let resultado = sqlx::query!(
        r#"DELETE FROM lecciones WHERE id = ?"#,
        id
    )
    .execute(pool.get_ref())
    .await;

    // Verificamos si se eliminó correctamente
    match resultado {
        Ok(_) => HttpResponse::Ok().body(format!("Lección {} eliminada", id)),
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar lección"),
    }
}


//FUNCIÓN PRINCIPAL 


// Esta es la función principal que arranca el servidor
#[actix_web::main] // Macro que indica que es el punto de entrada del servidor Actix
async fn main() -> std::io::Result<()> {
    // Creamos el "pool" de conexión a la base de datos SQLite
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://proyecto_lsd.db") // Nos conectamos al archivo proyecto_lsd.db
        .await
        .expect("No se pudo conectar a la base de datos");

    // Iniciamos el servidor HTTP en el puerto 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Compartimos la conexión con todas las rutas
             // Registramos la ruta GET
        .service(obtener_lecciones)
        // Registramos la ruta POST
            .service(crear_leccion) 
        // Registramos la ruta PUT
            .service(actualizar_leccion) 
         // Registramos la ruta DELETE
            .service(borrar_leccion)               
    })
    .bind(("127.0.0.1", 8080))? // El servidor escucha en localhost:8080
    .run()  // Arrancamos el servidor
    .await   // Esperamos a que termine
}
