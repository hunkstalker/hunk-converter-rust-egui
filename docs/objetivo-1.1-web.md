# Objetivo 1.1 — Núcleo web de referencia

> HUNK 1.0 ya está publicada. La 1.1 es una iteración de producto, no una
> primera versión. Squoosh se usa como referencia funcional y de compresión,
> no como plantilla de producto.

## 1. Posicionamiento

HUNK debe ser una herramienta de escritorio para **preparar imágenes para web**:
local-first, rápida, sin telemetría y con un flujo de trabajo más cuidado que el
de un laboratorio de códecs en el navegador.

La referencia es Squoosh en lo que aporta valor real:

- mismos formatos web no experimentales;
- calidad de compresión comparable o mejor;
- más velocidad por ser nativo;
- comparación visual fiable;
- opciones avanzadas comprensibles.

La ventaja de HUNK no será “Squoosh en Rust”, sino:

- escritorio y proceso local;
- flujo de exportación directo;
- cuentagotas, historial e ICO como extras;
- menos controles, pero los que haya deben funcionar de verdad;
- sin Analytics ni dependencia del navegador.

## 2. Una sola release

Las fases y los bloques de tareas son **paquetes de trabajo**, no releases
separadas. Todo lo aprobado aquí entra en 1.1:

- entrada TIFF, AVIF y del resto de formatos de importación;
- paridad de códecs JPEG/PNG/WebP/AVIF;
- preprocesado;
- UI avanzada;
- documentación, builds y release.

“Extra” o “fuera del núcleo” significa que no entra en las puertas de compresión
o en el discurso principal, **no** que no se publique en 1.1.

## 3. Alcance de formatos

### Entrada: amplia, incluida en 1.1

| Uso | Formatos |
| --- | --- |
| Entrada principal | JPEG, PNG, WebP, AVIF, SVG |
| Entrada extra | BMP, TGA, TIFF |

La entrada es deliberadamente más amplia que la salida: el usuario puede traer
material legacy o de escritorio y convertirlo a un formato web. TIFF de una
página ya está implementado (`t-36`) y forma parte de 1.1; no tendrá salida TIFF
ni opciones avanzadas.

SVG se mantiene **solo como entrada**. No se rasteriza a SVG porque el producto
no hace vectorización.

### Salida: web + ICO

| Uso | Formatos |
| --- | --- |
| Salida web | JPEG, PNG, WebP, AVIF |
| Extra de escritorio/web | ICO para favicons e iconos de aplicación |

La salida JPEG XL existente se retira del selector en 1.1. No se añaden salidas
TIFF, TGA ni BMP.

### Preprocesado

Además de los códecs, la 1.1 incluye:

- **redimensionar** con métodos de calidad y conservación de proporción;
- **rotar** en pasos de 90°;
- **reducir paleta** para PNG y WebP cuando el usuario quiera un resultado
  indexed/palette;
- **recortar** de forma no destructiva, con presets de proporción y máscara que
  afecta al resultado y a la exportación.

El preprocesado se aplica antes de codificar y debe verse en la vista previa.

### Entrada extra, sin opciones nuevas

Estos formatos de entrada forman parte de 1.1, pero no del núcleo de compresión:

- BMP;
- TGA;
- TIFF de una página.

No se añaden salidas ni opciones avanzadas para ellos. Su función es permitir
convertir material legacy a JPEG/PNG/WebP/AVIF.

### Fuera de alcance

- Salida JPEG XL; se retira del selector en 1.1.
- JPEG XL como formato de entrada.
- QOI y WebP2: experimentales.
- Códecs “Browser JPEG/PNG/GIF” de Squoosh: son fallbacks del navegador, no
  objetivos de producto.
- Salida TIFF, TGA o BMP.
- Animaciones GIF/WebP/AVIF.
- Multipágina, capas, ICC, CMYK, 16 bits y Display P3.
- Procesamiento por lotes.

## 3. Criterios de compresión

La comparación no se hace por número de calidad, porque cada encoder usa una
escala distinta. Se compara **tamaño a igual calidad perceptual**, usando
SSIMULACRA2 o Butteraugli.

Objetivo mínimo:

| Formato | Tamaño frente a Squoosh | Velocidad |
| --- | --- | --- |
| WebP | ≤ 100 % a igual calidad | ≥ 1,5× más rápido |
| JPEG | ≤ 100 % a igual calidad | ≥ 2× más rápido |
| PNG | ≤ 100 % a igual calidad | ≥ 1,5× más rápido |
| AVIF | ≤ 100 % a igual calidad | ≥ 1,2× más rápido |

Objetivo deseable: 5 % menos tamaño en WebP/AVIF y mantener la ventaja de
velocidad. Ningún formato del núcleo puede quedar por debajo de Squoosh en
calidad o velocidad.

Estado tras `t-39`: WebP cumple con paridad exacta, JPEG queda en paridad
práctica (±0,5 % mediana) y PNG comprime igual o mejor.

Estado tras `t-44`: AVIF usa `libavif`/AOM con YUV420, BT.601 y cuantizador
directo. A igual SSIMULACRA2, el tamaño mediano es **0,994×** frente a Squoosh
(0,704–1,069), la UI queda entre **0,705× y 0,960×** y la velocidad es **1,82×**
en UI y **4,58×** de mediana. La puerta de AVIF se cumple.

**1.1.0 sigue sin publicarse** hasta cerrar `t-43`: benchmark final, validación
de los artefactos del CI y release. Las revisiones incrementales de AVIF, UI,
resize y recorte ya están completadas. El bloqueo ya no es AVIF.

## 4. Encoders de referencia

| Formato | Backend objetivo | Motivo |
| --- | --- | --- |
| JPEG | MozJPEG | Progresivo, submuestreo y mejor compresión |
| PNG | OxiPNG | Mejor selección de filtros y compresión sin pérdida |
| WebP | libwebp avanzado | Lossless, method, alpha quality y filtros |
| AVIF | libavif/AOM | Cuantizador directo, YUV420, alfa y velocidad |

No se añadirá una opción si el encoder no la aplica. Las opciones deben ser
contextuales, estar colapsadas por defecto y tener resumen/reset.

## 5. Camino de 1.1.0

Todo pertenece a la misma release. No hay una 1.2 ni un bloque sin numerar.

### Bloque 1 — Entrada y preparación

- Entrada TIFF (`t-36`).
- Entrada AVIF (`t-37`).
- Benchmark reproducible HUNK vs Squoosh, ya iniciado en
  [`benchmark-squoosh-2026-10-31.md`](benchmark-squoosh-2026-10-31.md).

### Bloque 2 — Paridad con Squoosh

- MozJPEG y OxiPNG (`t-39`, completada).
- WebP y AVIF avanzados (`t-40`, completada).
- Preprocesado: redimensionar, rotar y reducir paleta (`t-41`, completada).
- UI, selector web+ICO y persistencia (`t-42`, completada).
- Cerrar la brecha AVIF (`t-44`, completada).
- Correcciones de resize y UI (`t-45`, completada).
- Recorte no destructivo (`t-46`, completada).
- Cierre, benchmark final y release (`t-43`; pendiente el benchmark final, la
  validación del CI y el push/tag).

El bloque 2 no se publica aparte: es la segunda mitad de 1.1.0.

## 6. Puerta de salida de 1.1

La release no se cierra si:

- JPEG o PNG siguen comprimiendo peor que Squoosh a igual calidad;
- WebP o AVIF pierden la ventaja de velocidad;
- resize/rotate/palette no se reflejan en la vista previa;
- hay controles que no llegan al encoder;
- la app deja de ser local-first o introduce telemetría.
