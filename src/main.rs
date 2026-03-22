// Importamos la librería principal de Anchor.
// Sin esto, no podemos usar nada de Solana ni Anchor.
use anchor_lang::prelude::*;

// Le decimos a Solana cuál es el ID único de este programa.
// Este ID lo genera Playground automáticamente — nunca lo cambiamos a mano.
declare_id!("9jgxxoABZciCdDN4f9Stwv9sTaJkQmyT682NtR4bryME"); 

// Este atributo le dice a Anchor que aquí empieza el programa principal.
// Todo lo que esté dentro de este bloque son las acciones que puede hacer el usuario.
#[program]
pub mod proyecto_lsd {
    // Esto permite usar todo lo que importamos arriba dentro de este módulo.
    use super::*;

    
    //PASO 1: Crear una lección
    // El administrador usa esto para agregar una nueva lección de señas.
    
    pub fn crear_leccion(
        ctx: Context<CrearLeccion>, // Las cuentas que necesita esta acción
        id_leccion: u64, // Número que identifica la lección 
        titulo: String,  // Nombre de la lección, 
        descripcion: String,// Texto que explica cómo hacer la seña O de lo que se trata
        url_video: String  // Link al video demostrativo
    ) -> Result<()> {   // Retorna "Ok" si todo salió bien, o un error si algo falló

        // Obtenemos la cuenta donde se guardará la lección.
        // "&mut" significa que vamos a modificar su contenido.
        let leccion = &mut ctx.accounts.leccion_pda;
        
        // Llenamos cada campo de la lección con los datos que recibimos.
        leccion.id = id_leccion;
        leccion.titulo = titulo;
        leccion.descripcion = descripcion;
        leccion.url_video = url_video;

        // Guardamos la wallet de quien creó la lección.
        // El "*" convierte la referencia en un valor que podemos copiar.
        leccion.autor = *ctx.accounts.autor.key;
        
        // Imprimimos un mensaje en los registros de la transacción.
        // Útil para verificar que la acción se ejecutó correctamente.
        msg!("Lección creada exitosamente con ID: {}", id_leccion);

        // Todo salió bien, retornamos éxito.
        Ok(())
    }

    
    //PASO 2: Actualizar una lección
    // Permite cambiar el contenido de una lección que ya existe.
    // Solo el autor original puede actualizarla.
    pub fn actualizar_leccion(
        ctx: Context<ActualizarLeccion>, 
        _id_leccion: u64,  // El "_" significa que lo recibimos solo para encontrar la cuenta, no lo usamos directamente
        nuevo_titulo: String, 
        nueva_desc: String, 
        nueva_url: String
    ) -> Result<()> {

        // Obtenemos la lección que ya existe en blockchain.
        let leccion = &mut ctx.accounts.leccion_pda;

        // Sobreescribimos los campos con los nuevos valores.
        leccion.titulo = nuevo_titulo;
        leccion.descripcion = nueva_desc;
        leccion.url_video = nueva_url;
        
        msg!("Lección actualizada correctamente.");
        Ok(())
    }

    
    //PASO 3: Borrar una lección
    // Elimina una lección de la blockchain.
    // El SOL que se usó para crearla se devuelve al autor.
    // Solo el autor original puede borrarla.

    pub fn borrar_leccion(
        _ctx: Context<BorrarLeccion>, // El "_" significa que no usamos el contexto directamente
        id_leccion: u64 // Necesitamos el ID para encontrar la lección a borrar
    ) -> Result<()> {

        // No necesitamos escribir nada aquí.
        // Anchor se encarga de cerrar la cuenta automáticamente
        // gracias al atributo "close = autor" que definiremos abajo.
        msg!("Cuenta de la lección ID: {} cerrada. Fondos devueltos al autor.", id_leccion);
        Ok(())
    }

    
    //PASO 4: Registrar un usuario
    // Se ejecuta la primera vez que alguien entra a la plataforma.
    // Crea su perfil en blockchain con 0 puntos y sin certificado.
    
    pub fn registrar_usuario(
        ctx: Context<RegistrarUsuario>,
        nombre: String // Nombre o apodo que el estudiante elige
    ) -> Result<()> {

        // Obtenemos la cuenta del nuevo usuario.
        let usuario = &mut ctx.accounts.usuario_pda;

        // Llenamos su perfil con los datos iniciales.
        usuario.nombre = nombre;

        // Guardamos su wallet igual que hacemos con el autor en las lecciones.
        usuario.wallet = *ctx.accounts.autor.key;

        // Todos los usuarios empiezan desde cero.
        usuario.puntos = 0;
        usuario.lecciones_completadas = 0;

        // Todavía no tiene certificado porque apenas empieza.
        usuario.certificado = false;

        msg!("Usuario registrado exitosamente.");
        Ok(())
    }


    // PASO 5: Completar una lección
    // Esta es la acción más importante del proyecto.
    // Se llama desde el frontend cuando la cámara detecta
    // que el usuario hizo correctamente una seña con la mano.
    // Le suma 10 puntos a su perfil en blockchain.
    
    pub fn completar_leccion(
        ctx: Context<CompletarLeccion>,
        _id_leccion: u64 // Recibimos el ID de la lección completada
    ) -> Result<()> {

        // Obtenemos el perfil del usuario.
        let usuario = &mut ctx.accounts.usuario_pda;

        // Le sumamos 10 puntos por haber completado la lección.
        usuario.puntos += 10;

        // También contamos cuántas lecciones ha terminado en total.
        usuario.lecciones_completadas += 1;

        msg!("Lección completada. Puntos acumulados: {}", usuario.puntos);
        Ok(())
    }

    
    //PASO 6: Reclamar certificado
    // Cuando el usuario acumula 100 puntos puede reclamar
    // su certificado de aprendizaje guardado en blockchain.
    // Solo se puede reclamar una vez.

    pub fn reclamar_certificado(ctx: Context<ReclamarCertificado>) -> Result<()> {

        // Obtenemos el perfil del usuario.
        let usuario = &mut ctx.accounts.usuario_pda;

        // Verificamos que tenga al menos 100 puntos.
        // Si no los tiene, el programa se detiene y manda un error al frontend.
        require!(usuario.puntos >= 100, ErrorCode::PuntosInsuficientes);

        // Verificamos que no haya reclamado el certificado antes.
        // El "!" significa "que NO sea verdadero", o sea, que certificado sea false.
        require!(!usuario.certificado, ErrorCode::CertificadoYaReclamado);

        // Marcamos que ya tiene certificado para que no pueda reclamarlo de nuevo.
        usuario.certificado = true;

        msg!("¡Certificado emitido para {}!", usuario.nombre);
        Ok(())
    }
}


// ESTRUCTURA DE DATOS: Lección
// Es como una tabla en una base de datos normal,
// pero en lugar de vivir en un servidor, vive en blockchain.
// Cada lección ocupa su propio espacio en la red de Solana.
#[account] // Este atributo le dice a Anchor que esto es una cuenta de blockchain
pub struct Leccion {
    pub id: u64,// Número identificador de la lección (1, 2, 3...)
    pub autor: Pubkey,// Wallet de quien creó esta lección
    pub titulo: String,// Nombre de la lección
    pub descripcion: String, // Descripción de la seña
    pub url_video: String,// Link al video demostrativo
}


// ESTRUCTURA DE DATOS: Usuario
// Guarda el progreso de cada estudiante en blockchain.
// Se crea una sola vez por wallet cuando el usuario se registra.

#[account]
pub struct Usuario {
    pub wallet: Pubkey, // Wallet del estudiante 
    pub nombre: String,// Nombre o apodo que eligió al registrarse
    pub puntos: u64, // Puntos acumulados (gana 10 por cada lección)
    pub lecciones_completadas: u64,// Cuántas lecciones ha terminado en total
    pub certificado: bool,// false = sin certificado, true = ya lo tiene
}


// CONTEXTOS DE VALIDACIÓN
// Los contextos le dicen a Anchor exactamente qué cuentas
// necesita cada acción y cómo verificarlas antes de ejecutarse.
// Son los equisitos que debe cumplir cada acción.

// Requisitos para CREAR una lección
#[derive(Accounts)]
#[instruction(id_leccion: u64)] // Necesitamos el ID para calcular la dirección de la cuenta
pub struct CrearLeccion<'info> {

    #[account(
        init, // Crear esta cuenta nueva en blockchain
        payer = autor,// El autor paga el costo de guardar datos en SOL
        space = 8 + 8 + 32 + 100 + 200 + 200,
        // Aquí calculamos cuántos bytes necesita esta cuenta:
        // 8   = discriminador que Anchor agrega a todas las cuentas (siempre va)
        // 8   = id (tipo u64 ocupa 8 bytes)
        // 32  = autor (una Pubkey siempre ocupa 32 bytes)
        // 100 = titulo (reservamos 100 bytes para el texto)
        // 200 = descripcion (reservamos 200 bytes)
        // 200 = url_video (reservamos 200 bytes)
        seeds = [b"leccion", autor.key().as_ref(), id_leccion.to_le_bytes().as_ref()],
        // Las seeds son la "receta" para calcular la dirección única de esta cuenta.
        // Usamos la palabra "leccion" + la wallet del autor + el ID de la lección.
        // Esto garantiza que cada lección tenga una dirección diferente en blockchain.
        bump // Anchor calcula automáticamente el número extra que necesita la dirección
    )]
    pub leccion_pda: Account<'info, Leccion>, // La cuenta de la lección que se va a crear

    #[account(mut)] // "mut" porque su balance de SOL va a disminuir al pagar el storage
    pub autor: Signer<'info>, // Quien firma y paga la transacción

    pub system_program: Program<'info, System>, // Programa nativo de Solana para crear cuentas
}

// Requisitos para ACTUALIZAR una lección
#[derive(Accounts)]
#[instruction(id_leccion: u64)]
pub struct ActualizarLeccion<'info> {
    #[account(
        mut,     // "mut" porque vamos a modificar el contenido de la lección
        seeds = [b"leccion", autor.key().as_ref(), id_leccion.to_le_bytes().as_ref()],
        // Las mismas seeds que usamos al crear — así encontramos la cuenta correcta
        bump,
        has_one = autor
        // Verifica que el "autor" de la cuenta sea el mismo que está firmando ahora.
        // Esto impide que otra persona modifique una lección que no es suya.
    )]
    pub leccion_pda: Account<'info, Leccion>,
    pub autor: Signer<'info>, // Debe ser el mismo autor que creó la lección
}

// Requisitos para BORRAR una lección
#[derive(Accounts)]
#[instruction(id_leccion: u64)]
pub struct BorrarLeccion<'info> {
    #[account(
        mut,
        close = autor,
        // "close = autor" le dice a Anchor que cierre esta cuenta al terminar
        // y que devuelva el SOL al autor automáticamente.
        seeds = [b"leccion", autor.key().as_ref(), id_leccion.to_le_bytes().as_ref()],
        bump,
        has_one = autor // Solo el autor original puede borrar su lección
    )]
    pub leccion_pda: Account<'info, Leccion>,

    #[account(mut)] // "mut" porque va a recibir el SOL de vuelta
    pub autor: Signer<'info>,
}

// Requisitos para REGISTRAR un usuario nuevo
#[derive(Accounts)]
pub struct RegistrarUsuario<'info> {
    #[account(
        init,          // Crear el perfil del usuario en blockchain
        payer = autor, // El usuario paga el costo de su propio perfil
        space = 8 + 32 + 50 + 8 + 8 + 1,
        // Calculamos el espacio necesario para el perfil:
        // 8  = discriminador de Anchor (siempre va)
        // 32 = wallet (Pubkey)
        // 50 = nombre (reservamos 50 bytes, suficiente para un nombre corto)
        // 8  = puntos (u64)
        // 8  = lecciones_completadas (u64)
        // 1  = certificado (bool ocupa solo 1 byte)
        seeds = [b"usuario", autor.key().as_ref()],
        // La dirección del perfil se calcula con "usuario" + la wallet.
        // Esto garantiza que cada wallet tenga exactamente un perfil único.
        bump
    )]
    pub usuario_pda: Account<'info, Usuario>, // La cuenta del perfil a crear

    #[account(mut)] // "mut" porque paga el storage en SOL
    pub autor: Signer<'info>,

    pub system_program: Program<'info, System>, // Necesario para crear la cuenta nueva
}

// Requisitos para COMPLETAR una lección (sumar puntos al usuario)
#[derive(Accounts)]
#[instruction(_id_leccion: u64)] // Recibimos el ID aunque no lo usemos en las seeds
pub struct CompletarLeccion<'info> {
    #[account(
        mut, // "mut" porque vamos a sumar puntos al perfil
        seeds = [b"usuario", autor.key().as_ref()],
        // Encontramos el perfil del usuario con las mismas seeds de cuando se registró
        bump
    )]
    pub usuario_pda: Account<'info, Usuario>,

    pub autor: Signer<'info>, // El usuario que completó la lección
}

// Requisitos para RECLAMAR el certificado
#[derive(Accounts)]
pub struct ReclamarCertificado<'info> {
    #[account(
        mut, // "mut" porque vamos a cambiar "certificado" de false a true
        seeds = [b"usuario", autor.key().as_ref()],
        bump
    )]
    pub usuario_pda: Account<'info, Usuario>,

    pub autor: Signer<'info>,
}


// ERRORES PERSONALIZADOS
// En lugar de mostrar errores técnicos confusos,
// definimos mensajes claros que el frontend puede
// mostrar directamente al usuario.
#[error_code]
pub enum ErrorCode {
    // Este error se lanza cuando el usuario intenta reclamar
    // su certificado sin tener suficientes puntos.
    #[msg("Necesitas al menos 100 puntos para reclamar el certificado.")]
    PuntosInsuficientes,

    // Este error se lanza cuando el usuario ya reclamó su certificado
    // e intenta reclamarlo de nuevo.
    #[msg("Ya reclamaste tu certificado anteriormente.")]
    CertificadoYaReclamado,
}
