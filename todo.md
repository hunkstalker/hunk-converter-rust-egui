# Tareas pendientes

> Convención: cada tarea usa un identificador `t-N`, un título y una descripción.
> Al completarse se retira de este archivo y se registra en `t-completadas.md`.
> Las tareas se listan en orden ascendente y los identificadores no se reutilizan.

## t-36 — Entrada TIFF

**Descripción:** Abrir archivos `.tif` y `.tiff` con la feature `tiff` de `image`. Soportar la primera página, RGB/RGBA/gris y las compresiones habituales del decodificador; convertir a RGBA8, aplicar la orientación y respetar los límites actuales de tamaño y dimensiones. Documentar que no se soportan multipágina, CMYK fiable, 16 bits, ICC ni capas.

## t-37 — Entrada AVIF y JPEG XL

**Descripción:** Reutilizar `avif-decode` y `jxl-oxide` para abrir AVIF y JPEG XL, que hoy solo se pueden exportar. Añadir filtros y detección por cabecera, cargar la primera imagen/fotograma, conservar alfa cuando el decodificador lo exponga y respetar los límites. Si la decodificación bloquea la UI de forma perceptible, moverla a un worker con estado visible.

## t-38 — Salida TGA y TIFF

**Descripción:** Añadir salida TGA RGBA8 con RLE configurable y TIFF de una página RGBA8 con compresión LZW, Deflate o sin compresión. Ambos serán formatos sin pérdida, sin slider de calidad, con pistas claras en la UI. TIFF no incluirá multipágina, 16 bits, CMYK, ICC ni capas. Incluye roundtrip, alfa y pruebas de estructura.

## t-39 — Opciones avanzadas sin pérdida

**Descripción:** Introducir un modelo de opciones tipadas por formato y un bloque `Avanzado` contextual, colapsable y con resumen/reset. Añadir compresión para PNG (`Rápida`/`Equilibrada`/`Máxima` y filtro), TGA (RLE), TIFF (LZW/Deflate/ninguna y predictor) y esfuerzo para WebP lossless/JPEG XL si el encoder lo permite. Persistir las opciones por formato en `config.ini` con migración compatible y cubrir el mapeo con pruebas.

## t-40 — Opciones con pérdida y color

**Descripción:** Completar el bloque avanzado con las opciones que el stack de códecs pueda aplicar de verdad: JPG (fondo para aplanar alfa y submuestreo/progresivo si se incorpora un encoder compatible), WebP (método/effort, calidad de alfa y modo lossless), AVIF (velocidad/effort y submuestreo si el encoder lo permite) y JPEG XL (esfuerzo y modo lossy cuando se añada). No se mostrarán controles simulados ni opciones de ICC, CMYK, 16 bits o Display P3.

## t-41 — Cierre, documentación y release

**Descripción:** Validar la ampliación completa: actualizar `docs/formatos.md`, README, proyecto y retomar; regenerar capturas; revisar contraste y accesibilidad; ejecutar pruebas, clippy y formato; validar builds Linux/Windows y macOS si hay build disponible; y preparar la release con la matriz de formatos y sus limitaciones.
