# Plan de desarrollo: formatos y opciones avanzadas

> **Histórico:** este plan queda sustituido para 1.1 por
> [`objetivo-1.1-web.md`](objetivo-1.1-web.md). Se conserva como referencia de
> las decisiones anteriores. TGA/TIFF de salida, JPEG XL, QOI y WebP2 no forman
> parte del núcleo web.
>
> Estado: histórico · Fecha: 2026-10-30
> Especificación funcional: [formatos.md](formatos.md)

## 1. Objetivo

Ampliar la compatibilidad de formatos y añadir opciones avanzadas sin romper el
flujo principal:

1. Abrir la imagen.
2. Elegir formato de salida.
3. Ajustar calidad o compresión.
4. Comparar y exportar.

La ampliación no busca “paridad total” entre formatos. Busca una matriz clara:
cada formato abre, exporta y ofrece las opciones que realmente puede cumplir.

## 2. Entregables

- Desplegable de formato de salida con modo `Automático`.
- Entrada TIFF, AVIF y JPEG XL.
- Salida TGA y TIFF de una página.
- Bloque `Avanzado` contextual y colapsable.
- Compresión configurable para formatos sin pérdida.
- Opciones lossy y de color dentro del alcance definido.
- Persistencia en `config.ini` y migración compatible.
- Pruebas, capturas, documentación y validación de release.

## 3. Fases

### Fase 0 — Especificación

- Documento de formatos y matriz de capacidades.
- Plan de desarrollo y tareas.
- Sin cambios de código.

### Fase 1 — Selector de salida

Sustituye las tarjetas por un desplegable y añade `Automático`. Es la base para
que los formatos nuevos no saturen la barra lateral.

### Fase 2 — Nuevas entradas

TIFF, AVIF y JPEG XL. Pueden desarrollarse en paralelo una vez definido el
catálogo de formatos del selector.

### Fase 3 — Nuevas salidas

TGA y TIFF. Añaden variantes al catálogo y preparan el terreno para las opciones
de compresión.

### Fase 4 — Opciones sin pérdida

Modelo de opciones por formato y bloque avanzado con compresión para PNG, TGA,
TIFF, WebP lossless y JPEG XL.

### Fase 5 — Opciones con pérdida y color

Calidad, esfuerzo, submuestreo y fondo/alfa donde el encoder lo permita.

### Fase 6 — Cierre

Pruebas completas, capturas, documentación, accesibilidad, validación de builds
y release.

## 4. Tareas

### t-35 — Selector de salida con modo automático

**Objetivo:** sustituir la rejilla de formatos por un desplegable compacto y
añadir la resolución automática por formato de entrada.

**Alcance:**

- `FormatSelection::Auto | Fixed(OutputFormat)`.
- Desplegable con nombre y pista de cada formato.
- Resumen del formato resuelto bajo el control.
- Reglas de `Auto` y fallback WebP.
- Aviso de recompresión para JPG/WebP.
- Persistencia de la selección.
- Accesibilidad: teclado, foco, estado seleccionado y texto alternativo.

**No incluye:** formatos nuevos, opciones avanzadas.

**Dependencias:** ninguna.

**Estado:** completada y validada el 2026-10-31.

**Aceptación:**

- `Auto` resuelve correctamente PNG/JPG/WebP y usa WebP para BMP/SVG.
- Una selección `Fixed` no cambia al cargar otra imagen.
- El reinicio vuelve a `Auto`.
- El desplegable funciona con teclado y lector de pantalla.
- Pruebas de la tabla de resolución y persistencia.

### t-36 — Entrada TIFF

**Objetivo:** abrir TIFF de una página y convertirlo al pipeline RGBA8.

**Alcance:**

- Feature `tiff` de `image`.
- Filtro `.tif`/`.tiff`.
- Primera página, RGB/RGBA/gris, 8 bits.
- Compresiones habituales soportadas por `tiff`.
- Orientación TIFF.
- Límites de tamaño y dimensiones.
- Errores específicos y pruebas con LZW, Deflate, PackBits y sin compresión.

**No incluye:** multipágina, CMYK fiable, 16 bits, ICC, capas.

**Estado:** completada y validada por pruebas automatizadas el 2026-10-31.

**Dependencias:** ninguna; se integra en el catálogo de t-35.

**Aceptación:**

- TIFF RGB, RGBA y gris se abren correctamente.
- Un TIFF de varias páginas carga solo la primera.
- TIFF corrupto o no soportado muestra error controlado.
- Se respetan los límites antes de un consumo desproporcionado.
- Tests de orientación, alfa y conversión a RGBA8.

### t-37 — Entrada AVIF y JPEG XL

**Objetivo:** eliminar la asimetría actual y permitir abrir los formatos que ya
se exportan.

**Alcance:**

- Reutilizar `avif-decode` y `jxl-oxide`.
- Detección por cabecera y filtros `.avif`, `.jxl`.
- Primera imagen/fotograma.
- Alfa cuando el decodificador la exponga.
- Límites compartidos.
- Carga sin bloquear la UI de forma perceptible; si la decodificación es lenta,
  moverla a un worker y mostrar estado.

**No incluye:** animaciones, metadatos, gestión ICC ni multipágina.

**Dependencias:** t-35 para el catálogo y las pruebas de `Auto`.

**Aceptación:**

- AVIF y JXL con y sin alfa se abren y se pueden reexportar.
- Archivos corruptos muestran error sin cerrar la app.
- La UI no se congela durante la carga.
- Tests de detección, dimensiones y roundtrip.

### t-38 — Salida TGA y TIFF

**Objetivo:** añadir dos salidas sin pérdida con alcance acotado.

**Alcance:**

- TGA RGBA8 con RLE configurable (por defecto activo).
- TIFF de una página RGBA8 con LZW, Deflate o sin compresión.
- Sin slider de calidad.
- Hints de formato y límites.
- Tests de roundtrip, alfa y compresión.

**No incluye:** TIFF multipágina, 16 bits, CMYK, ICC, capas ni TGA con metadatos.

**Dependencias:** t-35 para el selector; puede ejecutarse en paralelo con t-36 y
t-37.

**Aceptación:**

- TGA y TIFF se exportan y se vuelven a abrir.
- El alfa se conserva.
- TIFF LZW/Deflate produce archivos válidos.
- Los nombres siguen sin sobrescribirse.
- Tests de estructura y contenido.

### t-39 — Opciones avanzadas sin pérdida

**Objetivo:** introducir un modelo de opciones por formato y el bloque
`Avanzado` para compresión lossless.

**Alcance:**

- Refactor de `EncodeOptions` a opciones tipadas por formato.
- Bloque colapsable con resumen y reset.
- PNG: `Rápida / Equilibrada / Máxima` y filtro.
- TGA: RLE on/off.
- TIFF: compresión y predictor.
- WebP lossless y JPEG XL: esfuerzo si el encoder lo permite.
- Persistencia en `config.ini` y migración.
- Tooltips con la terminología del glosario de `docs/formatos.md`.
- Accesibilidad y pruebas de que cada opción llega al encoder correcto.

**No incluye:** opciones lossy, color de fondo ni submuestreo.

**Dependencias:** t-35 y t-38.

**Aceptación:**

- Cambiar compresión reencoda y actualiza peso/preview.
- Las opciones de un formato no afectan a otro.
- Reset vuelve a los valores seguros.
- Configuraciones antiguas siguen funcionando.
- Tests de mapeo de opciones y roundtrip.

### t-40 — Opciones con pérdida y color

**Objetivo:** completar el modo avanzado con los controles que sí puede ofrecer
el stack de códecs.

**Alcance:**

- JPG: calidad, fondo para aplanar alfa y submuestreo/progresivo si se incorpora
  un encoder que los soporte.
- WebP: calidad, método/effort, calidad de alfa y modo lossless.
- AVIF: calidad y velocidad/effort; submuestreo si el encoder lo permite.
- JPEG XL: esfuerzo y modo lossy cuando se añada.
- Aviso de recompresión y controles de alfa.
- Tests de mapeo, tamaños y calidad visual mínima.

**No incluye:** ICC, CMYK, 16 bits, Display P3 ni perfiles embebidos.

**Dependencias:** t-39.

**Aceptación:**

- Los controles solo aparecen cuando son válidos.
- JPG conserva el fondo elegido.
- WebP lossless conserva píxeles y alfa.
- AVIF/JXL respetan sus parámetros de esfuerzo.
- No se muestran opciones simuladas o sin efecto.
- Tests y validación visual.

### t-41 — Cierre, documentación y release

**Objetivo:** validar la ampliación completa y preparar la release.

**Alcance:**

- Actualizar `docs/formatos.md`, `README`, `proyecto.md` y `retomar.md`.
- Regenerar capturas.
- Revisar contraste y accesibilidad de los controles nuevos.
- Pruebas completas y clippy/formato.
- Validación Linux y Windows; macOS si hay build.
- Nota de release con matriz de formatos y limitaciones.

**Dependencias:** t-35 a t-40.

**Aceptación:**

- La matriz documentada coincide con la UI.
- `Auto` y las opciones avanzadas están cubiertos por pruebas.
- Las builds abren y exportan todos los formatos del alcance.
- No hay regresiones en exportación sin sobrescritura.
- Release publicada o preparada con notas claras.

## 5. Orden recomendado

```text
t-35 Selector + Auto
  ├── t-36 Entrada TIFF
  ├── t-37 Entrada AVIF/JXL
  └── t-38 Salida TGA/TIFF
        └── t-39 Opciones sin pérdida
              └── t-40 Opciones con pérdida y color
                    └── t-41 Cierre y release
```

t-36, t-37 y t-38 pueden solaparse si se trabaja en ramas distintas, pero el
catálogo de formatos de t-35 debe existir antes de integrarlas en la UI.

## 6. Riesgos y mitigaciones

| Riesgo | Mitigación |
| --- | --- |
| TIFF tiene demasiadas variantes | Documentar subconjunto y fallar con error claro |
| Decodificar AVIF/JXL bloquea la UI | Medir; mover a worker si supera el umbral perceptible |
| El encoder actual de JPG no permite submuestreo/progresivo | Evaluar `jpeg-encoder`/mozjpeg antes de prometerlo |
| AVIF no expone submuestreo en el encoder actual | Mostrar solo lo que se pueda aplicar o evaluar ravif |
| La barra lateral se queda sin espacio | Desplegable + bloque colapsado + scroll existente |
| Configuración avanzada rompe configs antiguas | Secciones opcionales y valores por defecto |
| Se promete “TIFF completo” | Etiquetar siempre “RGBA8, una página” |
| El modo automático recomprime JPG/WebP | Aviso visible y opción de formato fijo |

## 7. Estrategia de pruebas

- **Unitarias:** resolución de `Auto`, persistencia, mapeo de opciones,
  validación de límites.
- **Roundtrip:** abrir → exportar → reabrir para cada formato nuevo.
- **Golden/size:** que la compresión cambie el peso sin cambiar píxeles en
  formatos sin pérdida.
- **UI:** visibilidad condicional de controles, resumen del bloque avanzado,
  reset y navegación por teclado.
- **Accesibilidad:** foco, nombres, estados y contraste.
- **Manual:** selector de archivos, arrastre, línea de comandos, escalado 125 %
  y 150 %, lectura con NVDA/Narrator/VoiceOver.

## 8. Fuera de alcance de la release

- Animaciones.
- TIFF multipágina, capas o pirámides.
- HEIC/HEIF, PSD, RAW.
- CMYK, ICC, 16 bits, Display P3.
- Salida BMP o SVG; entrada ICO.
- Procesamiento por lotes y redimensionado.

## 9. Decisiones cerradas

- El selector de salida será un desplegable.
- `Automático` resolverá al mismo formato cuando exista salida y usará WebP como
  fallback.
- La selección fija del usuario no se cambiará al cargar otra imagen.
- El bloque avanzado será contextual y colapsado.
- Calidad y compresión no se mezclarán en la misma etiqueta.
- TIFF se limitará a una página RGBA8.
- BMP no tendrá salida en esta fase.
- No se simularán opciones que el encoder no pueda aplicar.
