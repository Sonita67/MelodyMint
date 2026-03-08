use anchor_lang::prelude::*;
use anchor_lang::system_program;

// ID del programa
declare_id!("14B8BKNMwjjUUUtL5HfqAWHUfkBEK4iNmyiBjav8Kr1d");

#[program]
pub mod melody_mint {
    use super::*;

    // INICIALIZACIÓN
    pub fn crear_perfil(ctx: Context<CrearPerfil>, nombre: String) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();
        msg!("🚀 Creando perfil para el artista: {}", nombre);
        msg!("🔑 Wallet del dueño: {}", owner_id);

        //Usamos set_inner para inicializar todos los campos de un jalón, estilo Michoacana
        ctx.accounts.perfil.set_inner(ArtistProfile {
            owner: owner_id,
            artist_name: nombre,
            total_revenue: 0,
            catalog: Vec::new(), //Empezamos con el catálogo vacío
        });

        Ok(())
    }

    //GESTIÓN DEL CATÁLOGO
    pub fn agregar_track(ctx: Context<GestionarCatalogo>, titulo: String, precio: u64, tipo: LicenseType) -> Result<()> {
        // Validación de seguridad: Solo el dueño puede subir música
        require!(
            ctx.accounts.perfil.owner == ctx.accounts.owner.key(),
            MelodyMintError::NotAuthorized
        );

        let nuevo_track = Track {
            title: titulo.clone(),
            price: precio,
            license_type: tipo,
            sold: false,
            play_count: 0,
        };

        ctx.accounts.perfil.catalog.push(nuevo_track);
        msg!("🎵 ¡Track '{}' agregado con éxito al catálogo!", titulo);

        Ok(())
    }

    //NEGOCIO (VENTAS)
    pub fn comprar_licencia(ctx: Context<ComprarLicencia>, indice_track: u8) -> Result<()> {
        let perfil = &mut ctx.accounts.perfil;
        let index = indice_track as usize;

        // 1. Verificaciones de seguridad
        require!(index < perfil.catalog.len(), MelodyMintError::TrackNotFound);
        require!(!perfil.catalog[index].sold, MelodyMintError::LicenseAlreadySold);

        let precio = perfil.catalog[index].price;
        let titulo = &perfil.catalog[index].title;

        // 2. Transferencia de SOL (CPI al System Program)
        //Movemos los lamports del comprador directamente a la wallet del artista
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.comprador.to_account_info(),
                to: ctx.accounts.artista_wallet.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, precio)?;

        // 3. Actualización de estado
        perfil.total_revenue += precio;
        perfil.catalog[index].play_count += 1;

        // Si la licencia es exclusiva, se marca como vendida para que nadie más la compre
        if perfil.catalog[index].license_type == LicenseType::Exclusive {
            perfil.catalog[index].sold = true;
            msg!("🔥 ¡LICENCIA EXCLUSIVA VENDIDA: {}!", titulo);
        }

        msg!("💰 Venta completada: {} lamports por el track '{}'", precio, titulo);
        Ok(())
    }

    //CONSULTAS
    pub fn ver_catalogo(ctx: Context<VerPerfil>) -> Result<()> {
        msg!("📋 CATÁLOGO DE MELODYMINT:");
        msg!("{:#?}", ctx.accounts.perfil.catalog);
        Ok(())
    }
}

//ESTRUCTURAS DE DATOS (ESTADO)

#[account]
#[derive(InitSpace)]
pub struct ArtistProfile {
    pub owner: Pubkey,
    #[max_len(32)] 
    pub artist_name: String,
    pub total_revenue: u64,
    #[max_len(10)] //Límite de 10 canciones por artista para ahorrar espacio
    pub catalog: Vec<Track>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Track {
    #[max_len(50)] 
    pub title: String,
    pub price: u64,
    pub license_type: LicenseType,
    pub sold: bool,
    pub play_count: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub enum LicenseType { Basic, Premium, Exclusive }

//CONTEXTOS (CUENTAS)

#[derive(Accounts)]
pub struct CrearPerfil<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + ArtistProfile::INIT_SPACE,
        seeds = [b"musician_v3", owner.key().as_ref()],
        bump
    )]
    pub perfil: Account<'info, ArtistProfile>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarCatalogo<'info> {
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"musician_v3", owner.key().as_ref()], bump)]
    pub perfil: Account<'info, ArtistProfile>,
}

#[derive(Accounts)]
pub struct ComprarLicencia<'info> {
    #[account(mut)]
    pub comprador: Signer<'info>,
    
    #[account(mut)]
    /// CHECK: Esta es la billetera receptora del artista (validada por constraint)
    pub artista_wallet: AccountInfo<'info>,

    #[account(
        mut, 
        seeds = [b"musician_v3", artista_wallet.key().as_ref()], 
        bump,
        constraint = perfil.owner == artista_wallet.key() @ MelodyMintError::NotAuthorized
    )]
    pub perfil: Account<'info, ArtistProfile>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerPerfil<'info> {
    pub perfil: Account<'info, ArtistProfile>,
}

//ERRORES

#[error_code]
pub enum MelodyMintError {
    #[msg("❌ No tienes permiso para realizar esta acción.")]
    NotAuthorized,
    #[msg("❌ El track solicitado no existe en el catálogo.")]
    TrackNotFound,
    #[msg("❌ Esta licencia exclusiva ya fue adquirida por alguien más.")]
    LicenseAlreadySold,
}
