# Formatos de entrada, salida y opciones avanzadas

> Estado: especificación de producto · Fecha: 2026-10-31
> Versión de desarrollo: 1.1.0 · Selector automático validado (`t-35`) · Entrada TIFF implementada (`t-36`) · Alcance 1.1: [objetivo-1.1-web.md](objetivo-1.1-web.md) · Plan histórico: [plan-formatos-y-opciones.md](plan-formatos-y-opciones.md)
>
> En 1.1 el selector solo ofrece JPEG, PNG, WebP, AVIF e ICO. Las filas de JPEG
> XL, TGA y TIFF de salida describen el análisis histórico, no la UI actual.

## 1. Principios

- **Local-first:** todo el procesamiento ocurre en el equipo; no hay subidas ni
  servicios externos.
- **Una imagen estática:** el flujo actual trabaja con un único fotograma. No se
  conservan animaciones, capas ni secuencias.
- **RGBA8:** el pipeline interno usa 8 bits por canal en espacio sRGB asumido.
  No hay gestión de color completa ni salida de 16 bits.
- **Matriz de capacidades, no paridad total:** cada formato expone solo las
  operaciones y opciones que puede ofrecer de forma honesta. SVG e ICO son
  ejemplos claros de formatos de una sola dirección.
- **Calidad y compresión son ejes distintos:** la calidad modifica la fidelidad
  en códecs con pérdida; la compresión empaqueta los mismos píxeles en formatos
  sin pérdida.
- **Sin sobrescrituras:** la exportación mantiene `export/` y la creación
  atómica de nombres únicos.

## 2. Estado actual

### 2.1 Entrada

| Formato | Extensiones | Decodificador | Notas |
| --- | --- | --- | --- |
| PNG | `.png` | `image`/`png` | Conserva transparencia |
| JPEG | `.jpg`, `.jpeg` | `image`/`zune-jpeg` | Aplica orientación EXIF |
| WebP | `.webp` | `image`/`image-webp` | Imagen estática; transparencia |
| AVIF | `.avif` | `avif-decode` | Primera imagen; alfa; se convierte a RGBA8 |
| BMP | `.bmp` | `image` | Se convierte a RGBA8 |
| TGA | `.tga` | `image` | Se detecta por extensión |
| TIFF | `.tif`, `.tiff` | `image`/`tiff` | Primera página; RGB/RGBA/gris de 8 bits |
| SVG | `.svg` | `resvg` | Se rasteriza al tamaño declarado |

### 2.2 Salida

| Formato | Extensión | Política | Opciones actuales |
| --- | --- | --- | --- |
| WebP | `.webp` | Con pérdida | Calidad 1–100 |
| JPG | `.jpg` | Con pérdida | Calidad 1–100; progresivo; submuestreo automático; fondo blanco fijo |
| PNG | `.png` | Sin pérdida | OxiPNG nivel 2 |
| AVIF | `.avif` | Con pérdida | Calidad 1–100; velocidad; calidad alfa; YUV420 fijo |
| JPEG XL | `.jxl` | Sin pérdida | Sin opciones |
| ICO | `.ico` | Sin pérdida | Tamaños 16–256 px |

JPG usa MozJPEG (`mozjpeg-rs`) con progresivo y submuestreo automático: 4:2:0 hasta 79, 4:2:2 entre 80 y 89 y 4:4:4 desde 90. PNG usa OxiPNG nivel 2. WebP usa libwebp avanzado. AVIF usa libavif/AOM con YUV420, BT.601, cuantizador directo, speed y alpha quality. Los controles están expuestos en el bloque `Avanzado` y no se persisten.

La salida se elige en un desplegable con modo `Automático`: conserva el formato de entrada cuando existe salida equivalente y usa WebP como fallback.

### 2.3 Detección de entrada

- PNG, JPEG, WebP, BMP, TIFF, AVIF y otros formatos con firma se detectan por
  **magic bytes**. AVIF se decodifica con `avif-decode` y se convierte a RGBA8.
- TGA no tiene firma fiable y depende de la extensión.
- SVG se reconoce por extensión y se rasteriza con `resvg`.
- El selector de archivos filtra las extensiones admitidas. Arrastrar, pegar o
  abrir desde línea de comandos usa el mismo cargador.
- El portapapeles se trata como un PNG generado en memoria.

### 2.4 Límites de carga

- Tamaño máximo del archivo: **200 MB**.
- Dimensiones máximas: **16 384 px por lado**.
- Máximo: **40 megapíxeles**.
- Los límites se comprueban antes de usar la imagen en la UI. Para formatos
  nuevos hay que evitar que el decodificador asigne memoria desproporcionada
  antes de la validación.

## 3. Matriz de capacidades

`Actual` describe lo que ya funciona. `Planificado` conserva el análisis
histórico de ampliación; para 1.1 rige el alcance de
[`objetivo-1.1-web.md`](objetivo-1.1-web.md), centrado en JPEG, PNG, WebP, AVIF
y SVG.

| Formato | Entrada actual | Salida actual | Entrada planificada | Salida planificada |
| --- | :---: | :---: | :---: | :---: |
| PNG | Sí | Sí | Sí | Sí |
| JPG | Sí | Sí | Sí | Sí |
| WebP | Sí | Sí | Sí | Sí |
| BMP | Sí | No | Sí | No |
| TGA | Sí | No | Sí | Sí |
| SVG | Sí | No | Sí | No |
| TIFF | Sí, primera página | No | Sí, primera página | Sí, una página |
| AVIF | Sí | Sí | Sí | Sí |
| JPEG XL | No | Sí | Sí | Sí |
| ICO | No | Sí | No | Sí |
| GIF | No | No | No | No |
| HEIC/HEIF | No | No | No | No |
| PSD/RAW | No | No | No | No |

### 3.1 Formatos de una sola dirección

- **SVG:** solo entrada. Rasterizar a SVG no tendría sentido en este producto.
- **ICO:** solo salida. La entrada ICO no está en el alcance; si se añade en el
  futuro habría que elegir un tamaño del contenedor.
- **BMP:** solo entrada. La salida BMP no se considera prioritaria: no aporta
  compresión y apenas tiene ventajas frente a PNG/TGA.

## 4. Ampliaciones decididas

### 4.1 Entrada TIFF

- Extensiones: `.tif` y `.tiff`.
- Implementación: feature `tiff` de `image`.
- Alcance actual:
  - primera página;
  - RGB, RGBA, gris y gris con alfa convertidos a RGBA8;
  - compresiones habituales del decodificador `tiff`: sin comprimir, LZW,
    Deflate y PackBits;
  - orientación TIFF aplicada al cargar;
  - límites de dimensiones comprobados antes de decodificar.
- Limitaciones explícitas:
  - no se conservan capas, perfiles ICC, EXIF ni metadatos;
  - CMYK, 16 bits y otros tipos no RGB/RGBA/gris de 8 bits se rechazan con un
    error controlado;
  - las páginas adicionales se ignoran.

TIFF es un contenedor, no un códec único. La UI y la documentación hablan de
“TIFF de una página, RGBA 8 bits”, no de “TIFF” a secas. Como la salida TIFF
queda fuera del alcance web de 1.1, el modo `Automático` usa WebP como fallback
para entradas TIFF.

### 4.2 Entrada AVIF

- Implementada con `avif-decode`, reutilizando el decodificador de la vista
  previa.
- Alcance actual:
  - primera imagen/fotograma;
  - alfa cuando el decodificador la exponga;
  - dimensiones comprobadas antes de decodificar;
  - conversión a RGBA8, incluidos 10/12 bits si el decodificador los entrega.
- Limitaciones:
  - no se conservan animaciones, HDR ni metadatos;
  - la orientación y el perfil de color dependen del decodificador;
  - la decodificación puede ser lenta para imágenes grandes.
- JPEG XL queda fuera del alcance web de 1.1.

### 4.3 Salida TGA

- Formato sin pérdida, RGBA8, con RLE activado por defecto.
- Sin slider de calidad.
- Sin metadatos.
- Límite de 65 535 px por lado del propio formato; el límite de la app es menor.
- Uso previsto: texturas, game dev, herramientas legacy.

### 4.4 Salida TIFF

- TIFF de **una página**, RGBA8.
- Compresión LZW o Deflate; “sin compresión” disponible pero no recomendada por
  tamaño.
- Predictor opcional.
- Sin perfil ICC, sin CMYK, sin 16 bits, sin multipágina.
- No se presentará como “TIFF de archivo” ni como sustituto de un flujo de
  impresión profesional.

## 5. Calidad y compresión

### 5.1 Calidad

Solo aplica a códecs con pérdida:

- WebP lossy.
- JPG.
- AVIF.
- JPEG XL lossy, si se añade ese modo.

Bajar la calidad descarta información de forma irreversible. Puede reducir
mucho el peso, pero introduce artefactos y no es equivalente a comprimir mejor
los mismos píxeles.

### 5.2 Compresión

Aplica a formatos sin pérdida:

- PNG: Deflate y filtros.
- TGA: RLE.
- TIFF: LZW, Deflate o sin compresión.
- WebP lossless y JPEG XL lossless: esfuerzo del codificador.

La compresión no cambia los píxeles. Un PNG con compresión máxima sigue siendo
idéntico al original; solo tarda más y ocupa menos.

### 5.3 Regla de UI

- Calidad solo se muestra cuando el formato seleccionado es lossy.
- Compresión solo se muestra cuando el formato seleccionado es lossless.
- Si un formato admite ambos modos, primero se elige el modo y después se
  muestran sus opciones.
- Nunca se muestran controles deshabilitados sin explicación.

## 6. Selector de salida y modo automático

### 6.1 Desplegable

La rejilla de tarjetas se sustituye por un desplegable para ahorrar altura y
escalar a más formatos:

```text
FORMATO DE SALIDA
┌──────────────────────────────────────┐
│ Automático · PNG                  ▾  │
└──────────────────────────────────────┘
```

Cada opción del desplegable muestra nombre y una pista corta. El control cerrado
resume el formato resuelto; el modo `Automático` explica su funcionamiento en un
tooltip. El menú se dibuja por encima del contenido y muestra las siete opciones
sin recortes.

### 6.2 Estado del selector

```text
FormatSelection::Auto
FormatSelection::Fixed(OutputFormat)
```

- `Auto`: se resuelve al cargar cada imagen.
- `Fixed`: se conserva aunque se cargue otra imagen.
- El botón de reinicio vuelve a `Auto`.

### 6.3 Resolución automática

| Entrada | Salida automática |
| --- | --- |
| PNG | PNG |
| JPG | JPG |
| WebP | WebP |
| TGA | TGA, cuando exista salida TGA |
| TIFF | TIFF, cuando exista salida TIFF |
| AVIF | AVIF |
| JPEG XL | JPEG XL |
| BMP | WebP |
| SVG | WebP |

Si no hay salida equivalente se usa **WebP** y se explica en la línea de ayuda.

### 6.4 Aviso de recompresión

Autoseleccionar el mismo formato puede implicar recomprimir:

- PNG → PNG es seguro.
- JPG → JPG puede degradar la imagen.
- WebP → WebP puede degradar si el modo es lossy.
- TIFF/TGA sin pérdida son seguros.

Cuando el formato resuelto sea JPG o WebP lossy, la UI mostrará un aviso
discreto junto a la calidad:

> Recompresión con pérdida: ajusta la calidad si quieres conservar más detalle.

No se bloqueará la exportación; solo se hará visible el riesgo.

## 7. Opciones avanzadas

`Compresión` no implica pérdida de calidad. En PNG, TIFF y TGA reduce el tamaño
sin alterar los píxeles; en JPG, WebP y AVIF la pérdida se controla con
`Calidad`.

### 7.1 Principios

- **Contextuales:** solo aparecen las opciones del formato actual.
- **Colapsadas por defecto:** el flujo básico sigue siendo formato + calidad.
- **Con título simple:** `Preprocesado` y `Avanzado` comparten estilo; no se encadenan resúmenes con `·`.
- **Con reset:** restablecer solo el bloque actual.
- **No persistentes:** las opciones por formato y el preprocesado vuelven a sus valores por defecto al reiniciar; `config.ini` no guarda `[advanced]`.
- **Accesibles:** controles con nombre, estado y foco visible.

### 7.2 Controles previstos

JPG y PNG usan MozJPEG/OxiPNG. WebP acepta lossless, method y alpha quality, y
AVIF acepta speed y alpha quality con YUV420 fijo. Los controles están expuestos
en el bloque `Avanzado` y no se persisten en `config.ini`.

Preprocesado actual: resize Lanczos3 con proporción enlazada, marco del tamaño
original en el canvas, rotación en pasos de 90° y paleta de 2–256 colores con
alfa conservado. Los campos numéricos permiten arrastre vertical y expresiones
como `1600/4`; `Enter` aplica y `Esc` cancela. La geometría completa se calcula
en un worker con debounce; la paleta se aplica al resultado codificado.

| Formato | Básico | Avanzado |
| --- | --- | --- |
| PNG | Sin pérdida | Compresión `Rápida / Equilibrada / Máxima`; filtro |
| JPG | Calidad | Fondo para aplanar alfa; submuestreo y progresivo si el encoder lo permite |
| WebP | Calidad | Modo lossless, método/effort y calidad de alfa |
| AVIF | Calidad | Velocidad; alpha quality; YUV420 fijo |
| JPEG XL | Sin pérdida | Esfuerzo; modo lossy cuando se añada |
| TIFF | Sin pérdida | LZW/Deflate/ninguna; predictor |
| TGA | Sin pérdida | RLE on/off |
| ICO | Tamaños | Sin bloque avanzado |
| BMP | Sin pérdida | Sin bloque avanzado |

Las opciones que dependan de un encoder que hoy no las expone se marcarán como
“evaluar” en el plan; no se simularán en la UI.

### 7.2.1 Glosario de opciones avanzadas

| Opción | Qué hace | Pérdida | Compromiso | Cuándo usarla |
| --- | --- | --- | --- | --- |
| **RLE** | Run-Length Encoding: agrupa secuencias de píxeles idénticos. En TGA corresponde al tipo 10. | No | Puede reducir mucho el tamaño en imágenes planas; en fotos ruidosas apenas comprime o incluso crece por la sobrecarga de paquetes. | Sprites, iconos, capturas de UI y zonas de color plano. |
| **LZW** | Compresión por diccionario. Usada por TIFF y GIF. | No | Buena compatibilidad y relación tamaño/velocidad; normalmente menos eficiente que Deflate. | TIFF que debe abrirse en herramientas antiguas. |
| **Deflate** | Compresión LZ77 + Huffman. Es la base de PNG y una opción habitual en TIFF. | No | Suele comprimir mejor que LZW, a costa de algo más de CPU. | TIFF moderno y archivos PNG. |
| **Predictor** | Transforma los valores de color en diferencias antes de comprimir. En TIFF se declara en la etiqueta `Predictor`. | No | Mejora LZW/Deflate; no tiene sentido sin compresión. | TIFF con LZW o Deflate. `Auto` es la opción recomendada. |
| **Filtro PNG** | Preprocesa cada fila (`None`, `Sub`, `Up`, `Average`, `Paeth`) para que Deflate encuentre más repeticiones. | No | Cambia el tamaño y la velocidad, nunca los píxeles. | `Auto` salvo que se esté ajustando un pipeline concreto. |
| **Nivel de compresión** | `Rápida`, `Equilibrada` o `Máxima`: esfuerzo del codificador. | No | Más esfuerzo = archivo más pequeño y más tiempo. | `Equilibrada` por defecto; `Máxima` para el archivo final. |
| **Esfuerzo / method** | Tiempo de búsqueda del codificador (WebP, AVIF, JPEG XL). | Depende del modo | Más esfuerzo = más lento y normalmente más pequeño. No es lo mismo que calidad. | Ajuste fino cuando importa el tamaño final. |
| **Submuestreo de croma** | Reduce la resolución de la información de color: `4:4:4`, `4:2:2`, `4:2:0`. | Sí | Menos color = archivos más pequeños; puede dañar texto, bordes y gráficos. | Fotografía; evitar en capturas de UI o imágenes con texto. |
| **JPG progresivo** | Guarda la imagen en varias pasadas de carga. | Según calidad | No mejora la calidad; puede cambiar el tamaño y la percepción de carga. | Publicación web, no archivo maestro. |
| **Calidad de alfa** | Calidad con la que WebP comprime el canal de transparencia. | Sí | Valores bajos producen bordes sucios en recortes. | Mantener alta en logos, iconos y siluetas. |
| **Fondo / aplanar alfa** | Color sólido sobre el que se aplana la transparencia. | Sí, elimina alfa | El color elegido sustituye a los píxeles transparentes. | JPG, que no admite alfa. |
| **Tamaños ICO** | Genera varias resoluciones dentro del mismo icono. | No | Más tamaños = archivo más grande. | Iconos de Windows. |

#### TGA y RLE

TGA con RLE es un formato **sin pérdida**. El codificador agrupa píxeles
consecutivos iguales y guarda un paquete con la longitud y el valor repetido.
No cuantiza color ni descarta información: al abrirlo se recuperan exactamente
los mismos píxeles.

Es especialmente útil en imágenes con grandes zonas uniformes, como sprites,
iconos, capturas de interfaz o texturas simples. En fotografías con ruido o
degradados suele aportar poco, porque hay pocas secuencias repetidas y la
sobrecarga de paquetes puede aumentar el tamaño.

El plan es activarlo por defecto y permitir desactivarlo desde `Avanzado` para
pipelines antiguos que no acepten TGA tipo 10. La opción se etiquetará como
`RLE (sin pérdida)` para que no se confunda con calidad.

#### Criterio de la UI

- Ninguna opción se mostrará si el encoder no la aplica de verdad.
- Las opciones sin pérdida llevarán la etiqueta `Sin pérdida`.
- Las opciones con pérdida se agruparán bajo `Calidad` o `Color`, nunca bajo
  `Compresión`.
- Cada control tendrá una explicación corta en tooltip, reutilizando este
  glosario.

### 7.3 Color

Alcance de esta fase:

- Conservar o aplanar alfa.
- Elegir el color de fondo para JPG.
- Calidad de alfa en WebP, si el encoder la expone.
- Submuestreo de croma en JPG; AVIF usa YUV420 fijo.

Fuera de alcance:

- Gestión ICC completa.
- CMYK.
- 16 bits por canal.
- Display P3.
- Perfiles de color embebidos.

La app asume sRGB y RGBA8. No se mostrarán controles de color que no puedan
aplicarse de forma fiable.

### 7.4 Persistencia

Propuesta de `config.ini`:

```ini
[output]
format=auto

[advanced.png]
compression=balanced
filter=auto

[advanced.jpg]
background=#FFFFFF
subsampling=420
progressive=0
```

- Configuraciones antiguas siguen funcionando: si faltan secciones se usan los
  valores por defecto.
- `format=auto` es el valor inicial.
- Los valores inválidos generan advertencia y no rompen el arranque.

## 8. UI

```text
┌─ Panel lateral 390 px ─────────────────────┐
│ IMAGEN                                      │
│ demo.png · PNG · 1600 × 1000 px             │
│                                             │
│ FORMATO DE SALIDA                           │
│ ┌───────────────────────────────────────┐   │
│ │ Automático · PNG                  ▾   │   │
│ └───────────────────────────────────────┘   │
│ PNG · Sin pérdida · conserva transparencia  │
│                                             │
│ CALIDAD                              82 %   │
│ [──────────●──────────────]                 │
│                                             │
│ ▸ Preprocesado                              │
│ ▸ Avanzado                                  │
│                                             │
│ HISTORIAL                                   │
│ CUENTAGOTAS                                 │
│ [ Exportar .png ]                           │
└─────────────────────────────────────────────┘
```

- El bloque avanzado se expande dentro de la barra lateral, no en un modal.
- La barra lateral mantiene el scroll y el botón de exportar fijo abajo.
- El desplegable y los controles deben funcionar a 1024×800 y con escalado
  125–200 %.
- El desplegable, el bloque avanzado y los grupos de opciones deben ser
  navegables con teclado y anunciar estado expandido/seleccionado.

## 9. Errores y mensajes

- Formato no reconocido: aviso claro, sin cerrar la imagen actual.
- TIFF/AVIF/JXL no soportado o corrupto: error de decodificación localizado.
- Límite superado: tamaño, lado o megapíxeles.
- Memoria insuficiente o decodificación demasiado grande: error controlado, no
  cierre de la app.
- Exportación: errores de carpeta/escritura ya existentes.
- Nunca se sobrescribe un archivo; se mantiene la numeración atómica.

## 10. Pruebas y criterios de aceptación

### 10.1 Automáticas

- Resolución de `Auto` para cada formato de entrada y fallback WebP.
- Persistencia y migración de `config.ini`.
- Roundtrip de AVIF con alfa y sin alfa, con AOM y cuantizador directo.
- Entrada AVIF desde archivo, con detección por cabecera.
- TIFF de varias páginas: solo se carga la primera.
- TIFF RGB, RGBA, gris, gris con alfa y orientación.
- TIFF CMYK y 16 bits rechazados con error controlado.
- TGA RLE y no RLE.
- TIFF LZW, Deflate, PackBits y sin compresión.
- Opciones avanzadas aplicadas al encoder correcto.
- Límites de tamaño y dimensiones antes de asignar memoria desproporcionada.
- Contraste y accesibilidad de los controles nuevos.

### 10.2 Manuales

- Abrir cada formato con el selector, arrastre y línea de comandos cuando
  aplique.
- Exportar cada formato y reabrirlo.
- Verificar que `Auto` no cambia una selección fija.
- Verificar el aviso de recompresión JPG/WebP.
- Revisar 100 %, 125 % y 150 % de escalado.
- Validación visual en Linux y Windows; macOS si hay build disponible.
- Comprobación con lector de pantalla de los controles nuevos.

## 11. Fuera de alcance

- Animaciones GIF/WebP/AVIF.
- TIFF multipágina, capas o pirámides.
- HEIC/HEIF, PSD, RAW y formatos de cámara.
- CMYK, ICC, 16 bits y Display P3.
- Salida SVG, BMP o ICO desde entrada ICO.
- Procesamiento por lotes.

Estas decisiones se pueden revisar más adelante, pero no forman parte de la
primera ampliación de formatos.
