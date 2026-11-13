# Benchmark HUNK vs Squoosh — 2026-10-31

> Benchmark en tres pasadas iniciales: línea base, MozJPEG/OxiPNG (`t-39`) y
> AVIF con libavif/AOM (`t-44`). La cuarta pasada, del 2026-11-13, cierra el
> benchmark final de `t-43` sin cambios de encoder y confirma los resultados.

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
- AVIF usó ravif directo durante esta pasada: speed y alpha quality, 8 bits.
- Los valores por defecto conservaron la paridad anterior de WebP, pero ravif
  no cerraba la brecha de AVIF. Esta decisión se revisó en `t-44`.

## 6. Tercera pasada: AVIF con libavif/AOM (t-44)

Datos completos:
[`benchmark-squoosh-2026-10-31-t44-avif.tsv`](benchmark-squoosh-2026-10-31-t44-avif.tsv).

Configuración medida:

- HUNK `release`, `libavif` 1.0.4 con AOM 3.11 estático.
- Entrada RGBA8 → YUV420, BT.601, rango completo.
- `end-usage=q`, `cq-level`, `sharpness=0`, `denoise=0` y `tune=ssim` para
  cuantizadores ≤ 32.
- Calidad HUNK 20–90, muestreada en 64 puntos; Squoosh `cqLevel` 20–45, 56
  puntos. La comparación se hace interpolando tamaño a igual SSIMULACRA2.

| Métrica | Resultado | Puerta |
| --- | --- | --- |
| Tamaño global a igual calidad | **0,994** mediana (0,704–1,069) | ≤ 1,00 ✅ |
| UI a igual calidad | **0,705–0,960** (mediana 0,894) | Sin caso peor ✅ |
| Peor foto | **+5,8 %** (kodim04, calidad media) | Sin caso UI ✅ |
| Peor caso global | **+6,9 %** (icono, calidad alta) | Aceptado como alfa |
| Velocidad global a igual calidad | **4,58×** mediana (1,77–7,29×) | ≥ 1,2× ✅ |
| Velocidad en UI | **1,82×** mediana (1,77–1,97×) | ≥ 1,2× ✅ |

La captura de UI deja de ser el punto débil: AOM produce entre un 4 % y un 30 %
menos que Squoosh a igual calidad. En fotos la curva queda prácticamente
solapada, con una dispersión de ±2 % en la mayoría de puntos. El icono con alfa
es el peor caso (+6,9 % en calidad alta); no es un caso de UI y sigue dentro de
un margen aceptable.

Consecuencias técnicas:

- `ravif` se retira del camino de salida. La integración usa `libavif-sys` con
  `codec-aom`; no hay dependencia de una DLL externa.
- AOM añade unos 4,2 MB al binario Linux (`21,8 MB → 26,1 MB`).
- La compilación requiere **CMake** además de NASM y un compilador de C.
- El build cruzado Windows GNU se validó localmente; la build MSVC sigue
  pendiente del CI.

## 7. Lectura de producto

- **WebP** ya es una victoria: mismo archivo, más rápido.
- **JPEG** queda en paridad práctica y mantiene una ventaja clara de velocidad.
- **PNG** ya comprime igual o mejor y sigue siendo más rápido.
- **AVIF** queda en paridad práctica, con UI claramente mejor y más velocidad.
- La deuda restante es de producto y revisión incremental, no de compresión.
- Squoosh no es imbatible: su desarrollo está parado desde 2024 y usa versiones
  antiguas de MozJPEG, libwebp, libavif/AOM y OxiPNG.
- La ventaja de HUNK debe ser nativa: velocidad, flujo de escritorio y opciones
  cuidadas; no una copia de la UI.

## 8. Cuarta pasada: benchmark final de cierre (2026-11-13)

Datos completos:
[`benchmark-squoosh-2026-11-13-final.tsv`](benchmark-squoosh-2026-11-13-final.tsv).

Esta pasada repite el benchmark completo después de los cambios de UI y confirma
que no hay regresiones en los encoders.

| Formato | Tamaño HUNK/Squoosh a igual calidad | Velocidad HUNK vs Squoosh |
| --- | --- | --- |
| **JPEG q75** | **1,004** mediana (1,000–1,039) | **3,07×** mediana (2,32–3,34×) |
| **JPEG q90** | **1,003** mediana (0,999–1,023) | **2,76×** mediana (2,39–3,08×) |
| **WebP q75** | **1,000** | **2,84×** mediana (2,68–3,16×) |
| **WebP q90** | **1,000** | **2,62×** mediana (2,24–2,89×) |
| **PNG** | **0,992** mediana (0,955–1,003) | **3,28×** mediana (2,82–3,74×) |
| **AVIF** | **0,995** mediana (0,705–1,069) | **4,64×** mediana (1,82–7,59×) |

AVIF en la captura de UI: **0,894** mediana (0,705–0,960) y **1,91×** de
velocidad mediana. Peor foto: **+5,8 %** (`kodim04`, calidad media). Peor caso
global: **+6,9 %** en el icono con alfa.

Conclusión: WebP mantiene paridad exacta, JPEG y AVIF quedan en paridad
práctica, y PNG comprime igual o mejor. La puerta de 1.1 se mantiene cumplida.

## 9. Próximos pasos

1. Validar los artefactos MSVC/macOS del CI.
2. Hacer push de `main`.
3. Crear el tag `v1.1.0` y publicar la release.
