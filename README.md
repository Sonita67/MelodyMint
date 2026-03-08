# 🎵 MelodyMint | Decentralized Music Licensing on Solana
Este proyecto permite a productores y artistas independientes transformar su catálogo musical en un marketplace autónomo, eliminando intermediarios y garantizando que los ingresos lleguen directamente a los creadores de forma instantánea y transparente.

![Solana](https://img.shields.io/badge/Solana-Explorer-9945FF?style=for-the-badge&logo=solana&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Framework-000000?style=for-the-badge&logo=anchor&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-Crab-blue?style=for-the-badge&logo=rust&logoColor=white)

**MelodyMint** es un protocolo de licencias musicales construido sobre la red de **Solana**. Permite a los artistas gestionar sus catálogos y vender derechos de uso directamente a sus fans y creadores de contenido, sin intermediarios y con liquidación inmediata de pagos.

---

## 🚀 Vision
La industria musical tradicional es lenta y burocrática. **MelodyMint** devuelve el control a los creadores utilizando la velocidad y eficiencia de los Smart Contracts en Solana, asegurando que cada "beat" vendido se traduzca en SOL instantáneo en la wallet del artista.

## ✨ Key Features
- 🎸 **Artist Profiles**: Espacio dedicado para que cada músico gestione su identidad on-chain.
- 📜 **Tiered Licensing**: Soporte para licencias *Basic, Premium, y Exclusive* mediante Enums de Rust.
- 💸 **Instant Settlement**: Transferencias de SOL en tiempo real vía CPI (Cross-Program Invocation).
- 📊 **Track Tracking**: Estadísticas de ventas y disponibilidad de catálogo actualizadas en vivo.
- 🛡️ **Built for Scale**: Uso de `InitSpace` para una gestión eficiente del almacenamiento (Rent).

---

## 🛠️ Tech Stack
- **Smart Contract:** Rust + Anchor Framework.
- **Security:** Validaciones de firmas y control de acceso riguroso.
- **Blockchain:** Solana (Devnet/Mainnet).

---

## 📂 Project Structure
```text
melody_mint/
├── programs/
│   └── melody_mint/
│       ├── src/
│       │   ├── lib.rs          <-- Lógica principal (MelodyMint Program)
│       │   ├── errors.rs       <-- Custom Error Codes
│       │   └── state.rs        <-- Estructuras ArtistProfile y Track
├── tests/
