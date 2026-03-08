// Importamos las librerías necesarias
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize; // Para convertir structs en JSON


// 1. Definición de rutas simples


// Ruta raíz "/"
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Servidor Rust funcionando :D")
}

// Ruta secundaria "/saludo"
#[get("/saludo")]
async fn saludo() -> impl Responder {
    HttpResponse::Ok().body("Hola wap@, esta es otra ruta")
}


// 2. Definición de datos y ruta JSON


// Estructura de una lección
#[derive(Serialize)]
struct Leccion {
    id: i32,
    titulo: String,
    descripcion: String,
    url_video: String,
}

// Ruta que devuelve una lista de lecciones en JSON
#[get("/lecciones")]
async fn lecciones() -> impl Responder {
    let lista = vec![
        Leccion {
            id: 1,
            titulo: "Abecedario".to_string(),
            descripcion: "Lengua de Señas Mexicana: Abecedario".to_string(),
            url_video: "https://drive.google.com/file/d/1QiDygeeYMRdpjdcaW7RBoI1ONDeFQa".to_string(),
        },
        Leccion {
            id: 2,
            titulo: "Saludos y Despedidas".to_string(),
            descripcion: "Decir hola y adiós".to_string(),
            url_video: "https://drive.google.com/file/d/1Ia-ZsbgGY6XssTOOJSo9MyHyHor5M4O/view".to_string(),
        },
        Leccion {
            id: 3,
            titulo: "Groserías".to_string(),
            descripcion: "Malas palabras".to_string(),
            url_video: "https://youtu.be/A5jLGCtHZRY?si=Q0ZoUYfh7xkFQxL2".to_string(),
        },
    ];

    HttpResponse::Ok().json(lista)
}


// 3. Función principal

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)     // ruta raíz "/"
            .service(saludo)    // ruta "/saludo"
            .service(lecciones) // ruta "/lecciones" con JSON
    })
    .bind(("127.0.0.1", 8080))? // puerto donde escucha el servidor
    .run()
    .await
}
