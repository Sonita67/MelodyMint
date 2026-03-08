# Reporte Técnico: MelodyMint Protocol

## Almacenamiento y PDAs
Este contrato utiliza **Program Derived Addresses (PDAs)** para asegurar que el perfil de cada artista y su catálogo musical estén vinculados de forma única y segura a su llave pública.

### Estructura de la PDA
La dirección de la cuenta del perfil del artista se deriva utilizando:
* **La semilla (seed) estática**: `"musician_v3"`
* **La llave pública del dueño** (`owner`)

## Gestión de Espacio
Para optimizar el costo de la renta (Rent) en Solana, se utiliza el trait `InitSpace` de Anchor, permitiendo un cálculo preciso del tamaño de la cuenta según la ocupación real de datos.

* **Perfil de Artista**: Incluye espacio para el nombre, ingresos acumulados y un vector de tracks.
* **Catálogo**: El espacio está optimizado para soportar un listado dinámico de obras musicales manteniendo la eficiencia del almacenamiento.

## Lógica de Licenciamiento
El contrato implementa una lógica de estados mediante **Enums** para gestionar niveles de derechos digitales de forma inmutable:

* **Licencia Básica/Premium**: Diseñadas para distribución masiva sin restricción de stock.
* **Licencia Exclusiva**: Una vez adquirida, el sistema ejecuta un cambio de estado a `sold: true`, bloqueando permanentemente nuevas transacciones para garantizar la propiedad única.

## Seguridad y Validaciones
* **Verificación de Firmas**: El protocolo exige la firma del `owner` para cualquier modificación del catálogo, asegurando la integridad de la propiedad intelectual.
* **Integridad de Pagos**: Las transferencias de SOL se realizan de forma atómica mediante **CPI** (Cross-Program Invocation), garantizando que el artista reciba sus fondos en el mismo slot en que se confirma la compra.

## Métricas de Operación
* **Unidad de medida**: Todos los cálculos internos se procesan en **lamports**.
* **Conversión de Interfaz**: El sistema realiza la conversión automática a **SOL** (1 SOL = 1,000,000,000 lamports) para una gestión financiera clara y profesional.

---
**Estatus de Auditoría:** 🟢 Verificado para Devnet  
**Protocolo:** MelodyMint v3.0.0
