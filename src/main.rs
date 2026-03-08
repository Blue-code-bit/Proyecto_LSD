// Importamos las librerías necesarias de actix-web
// - get: para definir rutas HTTP tipo GET
// - App: para construir la aplicación web
// - HttpResponse: para enviar respuestas HTTP
// - HttpServer: para levantar el servidor
// - Responder: para definir qué devuelve una función como respuesta
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// Definimos una ruta GET en la raíz ("/")
// Esta función se ejecuta cuando alguien entra a http://localhost:8080/
#[get("/")]
async fn hello() -> impl Responder {
    // Respondemos con un mensaje respuesta
    HttpResponse::Ok().body("Servidor Rust funcionando")
}

// Función principal del programa
// #[actix_web::main] indica que es el punto de entrada y usa el runtime de Actix
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Creamos el servidor HTTP
    HttpServer::new(|| {
        // Registramos la aplicación y sus rutas
        App::new()
            .service(hello) // añadimos la ruta definida arriba
    })
    // Indicamos en qué dirección y puerto escuchará el servidor
    .bind(("127.0.0.1", 8080))? 
    // Ejecutamos el servidor de manera asíncrona
    .run()
    .await
}

