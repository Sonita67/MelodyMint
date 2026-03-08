# Reporte Técnico: MelodyMint Protocol

## Almacenamiento y PDAs
Este contrato utiliza **Program Derived Addresses (PDAs)** para asegurar que el perfil de cada artista y su catálogo musical estén vinculados de forma única y segura a su llave pública.

### Estructura de la PDA
La dirección de la cuenta del perfil del artista se deriva utilizando:
* **La semilla (seed) estática**: `"musician_v3"`
* **La llave pública del dueño** (`owner`)



## Gestión de Espacio
Para optimizar el costo de la renta (Rent) en Solana, se utiliza el trait `InitSpace` de Anchor, permitiendo un cálculo preciso del tamaño de la cuenta.

* **Perfil de Artista**: Incluye espacio para el nombre, ingresos acumulados y un vector de tracks.
* **Catálogo**: El espacio está limitado a un máximo de 10 canciones inicialmente para mantener la eficiencia del almacenamiento.

## Lógica de Licenciamiento
El contrato implementa una lógica de estados mediante **Enums** para gestionar diferentes tipos de derechos sobre las obras:

* **Licencia Básica/Premium**: Permite múltiples ventas sin restricción de stock.
* **Licencia Exclusiva**: Una vez adquirida, el sistema marca el track como `sold: true`, impidiendo nuevas transacciones sobre esa pieza específica.



## Seguridad y Validaciones
* **Verificación de Firmas**: Solo el poseedor de la llave privada del `owner` puede agregar contenido o modificar precios.
* **Integridad de Pagos**: Las transferencias de SOL se realizan de forma atómica mediante **CPI** (Cross-Program Invocation), garantizando que el artista reciba sus fondos en el mismo instante de la compra.

## Métricas de Operación
* **Unidad de medida**: Todos los cálculos internos se realizan en **lamports**.
* **Conversión**: El cliente realiza la conversión a **SOL** (1 SOL = 1,000,000,000 lamports) para una visualización amigable al usuario.

---
**Estatus de Auditoría:** 🟢 Verificado para Devnet  
**Versión de Programa:** 3.0.0 (Estilo: La Michoacana)
