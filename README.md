# 🎵 MelodyMint | Decentralized Music Licensing on Solana

**MelodyMint** es un protocolo de licencias musicales *next-gen* construido sobre **Solana**. Permite a productores y artistas independientes transformar su catálogo musical en un marketplace autónomo, eliminando intermediarios y garantizando que los ingresos lleguen directamente a los creadores de forma instantánea y transparente.

![Solana](https://img.shields.io/badge/Solana-Explorer-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-000000?style=for-the-badge&logo=anchor&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-Crab-blue?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

---

## 🚀 La Visión
La industria musical tradicional es lenta, opaca y burocrática. **MelodyMint** devuelve el control a los creadores utilizando la velocidad y eficiencia de los **Smart Contracts**, asegurando que cada *beat* vendido se traduzca en **SOL líquido** en la wallet del artista de manera inmediata.



---

## ✨ Key Features (Características Clave)

* 🎸 **Artist Profiles (On-Chain Identity):** Espacio dedicado para que cada músico gestione su identidad y reputación directamente en la blockchain.
* 📜 **Tiered Licensing System:** Soporte nativo para licencias *Basic, Premium, y Exclusive* mediante potentes **Enums de Rust**.
* 💸 **Instant Cash-Out:** Liquidación de pagos en tiempo real mediante **CPI (Cross-Program Invocation)** al System Program de Solana.
* 📊 **Live Analytics:** Seguimiento en vivo de estadísticas de ventas, reproducciones y disponibilidad de catálogo.
* 🛡️ **Optimized Storage:** Implementación de `InitSpace` para una gestión ultra-eficiente de la renta (Rent) y escalabilidad.

---

## 🛠️ Tech Stack & Security

* **Smart Contract:** Desarrollado en **Rust** utilizando el **Anchor Framework**.
* **Seguridad de Cuentas:** Validación rigurosa de firmas (`Signer`), semillas (`Seeds/PDA`) y restricciones de acceso para evitar vulnerabilidades.
* **Network:** Desplegado actualmente en **Solana Devnet**.



---

## 📂 Project Structure

```text
melody_mint/
├── programs/
│   └── melody_mint/
│       ├── src/
│       │   ├── lib.rs          <-- Punto de entrada y lógica de negocio
│       │   ├── constants.rs    <-- Semillas y configuraciones fijas
│       │   ├── errors.rs       <-- Códigos de error personalizados
│       │   └── state.rs        <-- Definición de ArtistProfile y Tracks
├── tests/
│   └── anchor.test.ts          <-- Suite de pruebas de integración (Mocha/Chai)
└── client/
    └── client.ts               <-- Script de administración y consulta de catálogo
