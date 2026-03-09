// IMPORTACIÓN DE LIBRERÍAS DE SOLANA
use anchor_lang::prelude::*;

// ID del programa: debe coincidir con el que muestra Playground al hacer "Deploy".
declare_id!("9jgxxoABZciCdDN4f9Stwv9sTaJkQmyT682NtR4bryME"); 

#[program]
pub mod proyecto_lsd {
    use super::*;

    /// MÉTODO: CREATE (Crear Lección)
    pub fn crear_leccion(
        ctx: Context<CrearLeccion>, 
        id_leccion: u64, 
        titulo: String, 
        descripcion: String, 
        url_video: String
    ) -> Result<()> {
        let leccion = &mut ctx.accounts.leccion_pda;
        
        leccion.id = id_leccion;
        leccion.titulo = titulo;
        leccion.descripcion = descripcion;
        leccion.url_video = url_video;
        leccion.autor = *ctx.accounts.autor.key;
        
        msg!("Lección creada exitosamente con ID: {}", id_leccion);
        Ok(())
    }

    /// MÉTODO: UPDATE (Actualizar Lección)
    pub fn actualizar_leccion(
        ctx: Context<ActualizarLeccion>, 
        _id_leccion: u64, 
        nuevo_titulo: String, 
        nueva_desc: String, 
        nueva_url: String
    ) -> Result<()> {
        let leccion = &mut ctx.accounts.leccion_pda;
        leccion.titulo = nuevo_titulo;
        leccion.descripcion = nueva_desc;
        leccion.url_video = nueva_url;
        
        msg!("Lección actualizada correctamente.");
        Ok(())
    }

    /// MÉTODO: DELETE (Borrar Lección)
    pub fn borrar_leccion(_ctx: Context<BorrarLeccion>, id_leccion: u64) -> Result<()> {
        msg!("Cuenta de la lección ID: {} cerrada. Fondos devueltos al autor.", id_leccion);
        Ok(())
    }
}

// ESTRUCTURA DE LA BASE DE DATOS
#[account]
pub struct Leccion {
    pub id: u64,          // Identificador numérico
    pub autor: Pubkey,    // Wallet del creador
    pub titulo: String,   
    pub descripcion: String, 
    pub url_video: String,   
}

// CONTEXTOS DE VALIDACIÓN

#[derive(Accounts)]
#[instruction(id_leccion: u64)]
pub struct CrearLeccion<'info> {
    #[account(
        init, 
        payer = autor, 
        space = 8 + 8 + 32 + 100 + 200 + 200, 
        seeds = [b"leccion", autor.key().as_ref(), id_leccion.to_le_bytes().as_ref()], 
        bump
    )]
    pub leccion_pda: Account<'info, Leccion>,

    #[account(mut)]
    pub autor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id_leccion: u64)]
pub struct ActualizarLeccion<'info> {
    #[account(
        mut,
        seeds = [b"leccion", autor.key().as_ref(), id_leccion.to_le_bytes().as_ref()], 
        bump,
        has_one = autor
    )]
    pub leccion_pda: Account<'info, Leccion>,
    pub autor: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(id_leccion: u64)]
pub struct BorrarLeccion<'info> {
    #[account(
        mut,
        close = autor,
        seeds = [b"leccion", autor.key().as_ref(), id_leccion.to_le_bytes().as_ref()], 
        bump,
        has_one = autor
    )]
    pub leccion_pda: Account<'info, Leccion>,
    #[account(mut)]
    pub autor: Signer<'info>,
}
