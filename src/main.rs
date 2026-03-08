// Importamos las librerías necesarias

use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder}; // Actix-web para servidor y rutas
use serde::{Serialize, Deserialize}; // Serde para convertir datos entre JSON y Rust
use sqlx::sqlite::SqlitePoolOptions; // SQLx para conectarnos a SQLite

// Definimos la estructura de datos que representa una lección
#[derive(Serialize, Deserialize)] // Permite convertir entre JSON y Rust automáticamente
struct Leccion {
    id: i32,  // Identificador único de la lección
    titulo: String,  // Título de la lección
    descripcion: String,// Descripción breve
    url_video: String,// Enlace al video
}

// Rutas CRUD


// GET: obtener todas las lecciones desde la base de datos
#[get("/lecciones")] // Define la ruta GET /lecciones
async fn obtener_lecciones(pool: web::Data<sqlx::SqlitePool>) -> impl Responder {
    // Ejecutamos una consulta SQL que trae todas las lecciones
    let lecciones = sqlx::query_as!(
        Leccion, // Mapea los resultados a la estructura Leccion
        r#"SELECT id, titulo, descripcion, url_video FROM lecciones"# // Consulta SQL
    )
    .fetch_all(pool.get_ref()) // Ejecuta la consulta usando el pool de conexión
    .await
    .expect("Error al obtener lecciones"); // Si falla, muestra error

    HttpResponse::Ok().json(lecciones) // Devuelve las lecciones en formato JSON
    curl http://127.0.0.1:8080/lecciones

}

// POST: crear una nueva lección
#[post("/lecciones")] // Define la ruta POST /lecciones
async fn crear_leccion(pool: web::Data<sqlx::SqlitePool>, leccion: web::Json<Leccion>) -> impl Responder {
    // Insertamos la nueva lección en la base de datos
    let resultado = sqlx::query!(
        r#"INSERT INTO lecciones (titulo, descripcion, url_video) VALUES (?, ?, ?)"#, // Consulta SQL
        leccion.titulo,       // Valor del título
        leccion.descripcion,  // Valor de la descripción
        leccion.url_video     // Valor del enlace
    )
    .execute(pool.get_ref()) // Ejecuta la consulta
    .await;

    // Verificamos si se insertó correctamente
    match resultado {
        Ok(_) => HttpResponse::Created().body("Lección creada"), // Éxito
        Err(_) => HttpResponse::InternalServerError().body("Error al crear lección"), // Error
    }
    curl -X POST http://127.0.0.1:8080/lecciones \
-H "Content-Type: application/json" \
-d '{"id":4,"titulo":"Colores","descripcion":"Aprender colores en LSM","url_video":"https://ejemplo.com/video"}'

}

// PUT: actualizar una lección existente
#[put("/lecciones/{id}")] // Define la ruta PUT /lecciones/{id}
async fn actualizar_leccion(
    pool: web::Data<sqlx::SqlitePool>,// Conexión a la base
    path: web::Path<i32>,// ID de la lección en la URL
    leccion: web::Json<Leccion>,// Datos nuevos en formato JSON
) -> impl Responder {
    let id = path.into_inner(); // Extraemos el ID de la URL

    // Ejecutamos la consulta SQL para actualizar la lección
    let resultado = sqlx::query!(
        r#"UPDATE lecciones SET titulo = ?, descripcion = ?, url_video = ? WHERE id = ?"#,
        leccion.titulo,// Nuevo título
        leccion.descripcion,// Nueva descripción
        leccion.url_video,// Nuevo enlace
        id // ID de la lección a actualizar
    )
    .execute(pool.get_ref())
    .await;

    // Verificamos si se actualizó correctamente
    match resultado {
        Ok(_) => HttpResponse::Ok().body("Lección actualizada"), // Éxito
        Err(_) => HttpResponse::InternalServerError().body("Error al actualizar lección"), // Error
    }
    curl -X PUT http://127.0.0.1:8080/lecciones/2 \
-H "Content-Type: application/json" \
-d '{"id":2,"titulo":"Saludos y Despedidas","descripcion":"Decir hola, adiós y buenas noches","url_video":"https://ejemplo.com/saludos"}'

}

// DELETE: borrar una lección
#[delete("/lecciones/{id}")] // Define la ruta DELETE /lecciones/{id}
async fn borrar_leccion(pool: web::Data<sqlx::SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner(); // Extraemos el ID de la URL

    // Ejecutamos la consulta SQL para eliminar la lección
    let resultado = sqlx::query!(
        r#"DELETE FROM lecciones WHERE id = ?"#,
        id // ID de la lección a borrar
    )
    .execute(pool.get_ref())
    .await;

    // Verificamos si se eliminó correctamente
    match resultado {
        Ok(_) => HttpResponse::Ok().body(format!("Lección {} eliminada", id)), // Éxito
        Err(_) => HttpResponse::InternalServerError().body("Error al eliminar lección"), // Error
    }
    curl -X DELETE http://127.0.0.1:8080/lecciones/3

}


// Función principal

#[actix_web::main] // Macro que indica que es el punto de entrada del servidor Actix
async fn main() -> std::io::Result<()> {
    // Creamos el pool de conexión a SQLite
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://proyecto_lsd.db") // Conectamos al archivo de base de datos
        .await
        .expect("No se pudo conectar a la base de datos"); // Si falla, muestra error

    // Iniciamos el servidor HTTP en el puerto 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Compartimos la conexión con todas las rutas
            .service(obtener_lecciones) // Registramos la ruta GET
            .service(crear_leccion) // Registramos la ruta POST
            .service(actualizar_leccion)// Registramos la ruta PUT
            .service(borrar_leccion)// Registramos la ruta DELETE
    })
    .bind(("127.0.0.1", 8080))? // El servidor escucha en localhost:8080
    .run()// Ejecuta el servidor
    .await// Espera a que termine
}

}
