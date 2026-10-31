# Benchmark HUNK vs Squoosh — 2026-10-31

> Benchmark en dos pasadas: línea base y estado tras MozJPEG/OxiPNG (`t-39`).
> Los resultados son indicativos y deben repetirse tras cada cambio de encoder.

## 1. Objetivo

Medir si HUNK puede ofrecer, como mínimo, la misma compresión y velocidad que
Squoosh en los formatos web no experimentales:

- JPEG (MozJPEG en Squoosh);
- PNG (OxiPNG en Squoosh);
- WebP;
- AVIF.

La comparación de tamaño se hace **a igual calidad perceptual**, no a igual
número de calidad.

## 2. Metodología

- HUNK: build `release`, `bench_squoosh`, 3 repeticiones y mediana.
- Squoosh: `@squoosh/cli 0.7.3` / `@squoosh/lib 0.4.0`, Node 16, 3 repeticiones
  y mediana.
- Tiempo HUNK: `codec::encode`, que incluye la decodificación de la vista previa.
- Tiempo Squoosh: codificación + decodificación del resultado.
- Calidad: SSIMULACRA2 con `ssimulacra2_rs`; 100 es perfecto.
- Corpus:
  - 6 fotos Kodak (kodim01, 04, 08, 13, 19, 23);
  - captura de UI de HUNK;
  - icono de 256 px con alfa.
- Métricas agregadas sobre las 7 imágenes opacas; el icono se usa para tamaño,
  alfa y velocidad, no para la media de calidad.
- Squoosh AVIF usa `cqLevel` en esta versión de la librería; sus puntos se
  comparan por curva de calidad, no por el número de calidad.
- Hardware: 28 núcleos, 31 GiB de RAM, Linux x86_64.

Los datos completos están en
[`benchmark-squoosh-2026-10-31.tsv`](benchmark-squoosh-2026-10-31.tsv).

## 3. Línea base (antes de t-39)

| Formato | Tamaño HUNK/Squoosh a igual calidad | Velocidad HUNK vs Squoosh | Lectura |
| --- | --- | --- | --- |
| **WebP** | **1,000** en las 7 imágenes | **2,35–2,80×** más rápido | Paridad exacta; ventaja nativa clara |
| **JPEG** | **1,096** mediana (0,991–1,175) | **6,1–10,0×** más rápido | Squoosh comprime mejor; hace falta MozJPEG |
| **PNG** | **1,138** mediana (1,128–1,201); icono 1,411 | **4,4×** mediana (3,9–9,7×) | OxiPNG gana en tamaño; HUNK gana en velocidad |
| **AVIF** | **1,065** mediana (0,985–1,381) | **1,4–3,4×** más rápido | Cerca, pero UI y calidad alta necesitan ajuste |

### WebP

HUNK y Squoosh produjeron **exactamente los mismos bytes y la misma puntuación
SSIMULACRA2** en las 7 imágenes, a calidad 75 y 90. Es el mismo libwebp; la
diferencia es que HUNK es nativo y evita el coste de WASM.

Conclusión: paridad de compresión y ventaja de velocidad. Falta exponer
lossless, method y alpha quality para superar funcionalmente a Squoosh.

### JPEG

Squoosh/MozJPEG gana sobre todo en calidad media y baja. A calidad alta la
diferencia se reduce y HUNK puede empatar o ganar ligeramente en algunas fotos.

La imagen de UI es el caso más sensible: HUNK necesita un 14,8 % más de tamaño
a igual calidad. Es el patrón típico de un encoder baseline frente a MozJPEG.

Conclusión: HUNK es mucho más rápido, pero **no cumple el objetivo de tamaño**.
El siguiente paso es integrar MozJPEG. Sin ese cambio no se puede afirmar
paridad con Squoosh.

### PNG

OxiPNG produce entre un 12,8 % y un 20,1 % menos en las imágenes opacas; en el
icono con alfa, un 41,1 % menos. HUNK es entre 3,9 y 9,7 veces más rápido.

Conclusión: el tamaño no cumple el objetivo. OxiPNG nativo debería reducir la
brecha y conservar una velocidad competitiva. Hay que medir OxiPNG nativo a
nivel 2 frente al `Balanced` actual.

### AVIF

La comparación por curva de calidad muestra que HUNK está cerca en fotos
(mediana 6,5 % más grande), pero pierde claramente en la captura de UI
(26–38 % más grande). En varias fotos empata o gana ligeramente.

HUNK es más rápido, pero el tiempo de Squoosh incluye su decodificación y el de
HUNK también. La ventaja es real, aunque menor que en WebP/JPEG/PNG.

Conclusión: AVIF no está mal, pero hay que decidir entre seguir con ravif y
exponer speed/alfa o evaluar libavif/AOM. No se debe cambiar a ciegas.

## 4. Segunda pasada: MozJPEG y OxiPNG (t-39)

Datos completos:
[`benchmark-squoosh-2026-10-31-t39.tsv`](benchmark-squoosh-2026-10-31-t39.tsv).

| Formato | Tamaño HUNK/Squoosh a igual calidad | Velocidad HUNK vs Squoosh | Lectura |
| --- | --- | --- | --- |
| **JPEG q75** | **1,004** mediana (0,999–1,039) | **2,48×** mediana | Paridad práctica; UI +3,9 % |
| **JPEG q90** | **1,003** mediana (0,999–1,023) | **2,50×** mediana | Paridad práctica |
| **PNG** | **0,992** mediana (0,955–1,003) | **3,22×** mediana | HUNK comprime mejor |
| **WebP** | 1,000 | 2,35–2,80× | Sin cambios |
| **AVIF** | 1,065 | 1,4–3,4× | Pendiente de ajuste |

MozJPEG-rs reproduce el comportamiento de Squoosh con una diferencia mediana
inferior al 0,5 % en JPEG. La captura de UI a calidad 75 sigue siendo el peor
caso (+3,9 %), pero la velocidad se mantiene entre 2 y 2,7 veces por encima.

OxiPNG nativo iguala o supera a Squoosh en las siete imágenes opacas y es unas
3,2 veces más rápido. El icono con alfa también queda ligeramente por debajo
del tamaño de Squoosh.

## 5. Opciones avanzadas (t-40)

- WebP usa `WebPConfig` avanzado: lossless, method y alpha quality.
- AVIF usa ravif directo: speed y alpha quality, manteniendo 8 bits.
- Los valores por defecto conservan la paridad anterior: WebP sigue produciendo
  los mismos bytes y AVIF mantiene la curva medida.
- No se integró libavif/AOM: ravif es nativo, mantiene velocidad competitiva y
  evita otra dependencia C. La brecha de AVIF en UI sigue pendiente de ajuste.

## 6. Lectura de producto

- **WebP** ya es una victoria: mismo archivo, más rápido.
- **JPEG** queda en paridad práctica y mantiene una ventaja clara de velocidad.
- **PNG** ya comprime igual o mejor y sigue siendo más rápido.
- **AVIF** es el único formato que aún necesita una segunda ronda de ajuste.
- Squoosh no es imbatible: su desarrollo está parado desde 2024 y usa versiones
  antiguas de MozJPEG, libwebp, libavif/AOM y OxiPNG.
- La ventaja de HUNK debe ser nativa: velocidad, flujo de escritorio y opciones
  cuidadas; no una copia de la UI.

## 7. Próximos pasos

1. Exponer las opciones en la UI (`t-42`) sin romper los valores por defecto.
2. Hacer una segunda curva AVIF específica de UI y decidir si el ajuste de
   calidad/velocidad basta o si hace falta evaluar AOM.
3. Repetir el benchmark con resize/rotate/palette cuando existan.
4. Solo entonces cerrar la puerta de compresión de 1.1.
