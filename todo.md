# Tareas pendientes

> Convención: cada tarea usa un identificador `t-N`, un título y una descripción.
> Al completarse se retira de este archivo y se registra en `t-completadas.md`.
> Las tareas se listan en orden ascendente y los identificadores no se reutilizan.
>
> Camino único de 1.1.0:
> 1. **Entrada y preparación:** TIFF (t-36, completada), AVIF (t-37, completada) y benchmark (completado).
> 2. **Paridad con Squoosh:** MozJPEG/OxiPNG (t-39), WebP/AVIF avanzados (t-40),
>    preprocesado (t-41), UI/persistencia (t-42) y cierre (t-43).
>
> t-38 (salida TGA/TIFF), la parte JPEG XL de t-37 y las ampliaciones de JPEG XL,
> QOI y WebP2 quedan fuera del alcance web de 1.1. Ver
> `docs/objetivo-1.1-web.md`.

## t-39 — Paridad JPEG y PNG

**Descripción:** Sustituir el encoder JPEG actual por MozJPEG y el PNG actual por OxiPNG. Exponer calidad, progresivo y submuestreo en JPG; nivel de compresión e interlazado en PNG. Mantener alfa, límites y exportación segura. Repetir el benchmark HUNK vs Squoosh y no cerrar la tarea si JPEG o PNG siguen comprimiendo peor a igual calidad perceptual.

## t-40 — WebP y AVIF avanzados

**Descripción:** Exponer las opciones reales de libwebp (lossless, method/effort, alpha quality) y de AVIF (speed/effort, alpha quality y submuestreo si el encoder lo permite). Medir ravif frente a libavif/AOM y elegir el backend con criterios de tamaño, calidad y velocidad. No mostrar controles que no lleguen al encoder.

## t-41 — Preprocesado: redimensionar, rotar y paleta

**Descripción:** Añadir redimensionado con métodos de calidad y conservación de proporción, rotación en pasos de 90° y reducción de paleta para PNG/WebP. El resultado debe verse en la vista previa antes de exportar y conservar alfa cuando corresponda. Incluye pruebas de dimensiones, orientación y paleta.

## t-42 — UI avanzada y persistencia

**Descripción:** Introducir opciones tipadas por formato, bloque `Avanzado` contextual, colapsado, con resumen y reset. Conectar las opciones de JPG/PNG/WebP/AVIF y el preprocesado con la UI, dejar el selector de salida en JPEG/PNG/WebP/AVIF/ICO retirando JPEG XL, persistir por formato en `config.ini` con migración compatible, añadir tooltips del glosario y mantener accesibilidad por teclado y lector de pantalla.

## t-43 — Cierre 1.1: benchmark, documentación y release

**Descripción:** Repetir el benchmark con los encoders finales y verificar las puertas de tamaño y velocidad. Actualizar `docs/objetivo-1.1-web.md`, `docs/formatos.md`, README, proyecto y retomar; regenerar capturas; revisar contraste y accesibilidad; ejecutar pruebas, clippy y formato; validar builds Linux/Windows y macOS si hay build; preparar la release 1.1 con la matriz web y sus limitaciones.
