// client.ts - Versión "La Michoacana" de MelodyMint
const runClient = async () => {
  const program = pg.program;
  const wallet = pg.wallet;

  // 1. Derivar la dirección de la cuenta (PDA) usando la semilla musician_v3
  const [artistPda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("musician_v3"), wallet.publicKey.toBuffer()],
    program.programId
  );

  console.log("=".repeat(50));
  console.log("Wallet (Dueño):", wallet.publicKey.toString());
  console.log("Perfil PDA:", artistPda.toString());
  console.log("=".repeat(50));

  try {
    // 2. Intentar obtener la cuenta del Artista
    const account = await program.account.artistProfile.fetch(artistPda);
    
    console.log("\n🎸 ESTADO DEL ARTISTA:");
    console.log("   Nombre:", account.artistName);
    console.log("   Tracks en Catálogo:", account.catalog.length);
    // Convertimos lamports (10^-9) a SOL para que se entienda
    console.log("   Ingresos Totales:", (Number(account.totalRevenue) / 1e9).toFixed(4), "SOL");
    
    console.log("\n💿 CATÁLOGO DE MÚSICA:");
    if (account.catalog.length === 0) {
      console.log("   (El catálogo está vacío actualmente)");
    } else {
      account.catalog.forEach((track, i) => {
        // En Rust usamos un Enum, aquí en JS se lee como un objeto
        const tipoLicencia = Object.keys(track.licenseType)[0]; 
        
        console.log(`   ${i + 1}. ${track.title} [${tipoLicencia.toUpperCase()}]`);
        console.log(`      Precio: ${(Number(track.price) / 1e9).toFixed(4)} SOL`);
        console.log(`      Ventas/Reproducciones: ${track.playCount}`);
        console.log(`      Estado: ${track.sold ? "❌ AGOTADO (Exclusivo)" : "✅ DISPONIBLE"}`);
        console.log("      " + "-".repeat(25));
      });
    }

  } catch (e) {
    console.log("\n❌ ERROR O PERFIL NO ENCONTRADO");
    console.log("ℹ️  Asegúrate de haber hecho 'Deploy' y haber corrido el Test para crear el perfil.");
    console.log("   Detalle:", e.message);
  }
  
  console.log("\n" + "=".repeat(50));
};

runClient();
