//  IMPORTACIÓN DE LIBRERÍAS
// Actix-web: framework para crear el servidor y definir rutas HTTP
use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};

// Serde: librería que convierte datos entre JSON y estructuras de Rust
use serde::{Serialize, Deserialize};

// SQLx: librería para conectarnos y trabajar con la base de datos SQLite
use sqlx::sqlite::SqlitePoolOptions;


// DEFINICIÓN DE LA ESTRUCTURA

// Esta estructura representa una lección en nuestra base de datos
// Cada campo corresponde a una columna en la tabla "lecciones"
#[derive(Serialize, Deserialize)] // Permite convertir automáticamente entre JSON y Rust
struct Leccion {
    id: i32,             // Identificador único de la lección
    titulo: String,      // Título de la lección
    descripcion: String, // Breve descripción de la lección
    url_video: String,   // Enlace al video asociado
}


// RUTAS CRUD

// GET: obtener todas las lecciones guardadas en la base de datos
#[get("/lecciones")] // Esta ruta se activa cuando alguien hace GET a /lecciones
async fn obtener_lecciones(pool: web::Data<sqlx::SqlitePool>) -> impl Responder {
    // Consulta SQL que trae todas las filas de la tabla "lecciones"
    let lecciones = sqlx::query_as!(
        Leccion, // Los resultados se convierten en nuestra estructura Leccion
        r#"SELECT id, titulo, descripcion, url_video FROM lecciones"#
    )
    .fetch_all(pool.get_ref()) // Ejecutamos la consulta usando la conexión a la base
    .await
    .expect("Error al obtener lecciones"); // Si algo falla, mostramos un error

    // Respondemos con un JSON que contiene todas las lecciones
    HttpResponse::Ok().json(lecciones)
}


// POST: crear una nueva lección
#[post("/lecciones")] // Esta ruta se activa cuando alguien hace POST a /lecciones
async fn crear_leccion(pool: web::Data<sqlx::SqlitePool>, leccion: web::Json<Leccion>) -> impl Responder {
    // Insertamos una nueva fila en la tabla "lecciones"
    let resultado = sqlx::query!(
        r#"INSERT INTO lecciones (titulo, descripcion, url_video) VALUES (?, ?, ?)"#,
        leccion.titulo,      // Título que viene en el JSON
        leccion.descripcion, // Descripción que viene en el JSON
        leccion.url_video    // URL del video que viene en el JSON
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

    // Consulta SQL para actualizar la fila correspondiente
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

    // Consulta SQL para eliminar la fila correspondiente
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


// RUTA RAÍZ
// Esta ruta se activa cuando alguien entra a la raíz "/"
#[get("/")]
async fn raiz() -> impl Responder {
    HttpResponse::Ok().body("servidor rust funcionando :D")
}


// FUNCIÓN PRINCIPAL
// Esta es la función principal que arranca el servidor
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Creamos el "pool" de conexión a la base de datos SQLite
    let pool = SqlitePoolOptions::new()
        .connect("sqlite://proyecto_lsd.db") // Nos conectamos al archivo proyecto_lsd.db
        .await
        .expect("No se pudo conectar a la base de datos");

    // Mensaje de confirmación en la terminal
    println!("Servidor corriendo en http://0.0.0.0:8080");

    // Iniciamos el servidor HTTP en el puerto 8080
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Compartimos la conexión con todas las rutas
         // Registramos la ruta raíz "/"
            .service(raiz)
            .service(obtener_lecciones)// Registramos la ruta GET /lecciones
            .service(crear_leccion)// Registramos la ruta POST /lecciones
            .service(actualizar_leccion)// Registramos la ruta PUT /lecciones/{id}
            .service(borrar_leccion)// Registramos la ruta DELETE /lecciones/{id}
    })
    .bind(("0.0.0.0", 8080))? // El servidor escucha en todas las interfaces
    .run() // Arrancamos el servidor
    .await // Esperamos a que termine
}


