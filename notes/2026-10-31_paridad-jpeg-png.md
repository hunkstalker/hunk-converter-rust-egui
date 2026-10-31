# Paridad JPEG y PNG

Fecha: 2026-10-31 · Tarea: `t-39`

## Cambios

- JPG pasa de `image`/baseline a MozJPEG-rs con:
  - progresivo;
  - submuestreo automático: 4:2:0 hasta 79, 4:2:2 entre 80 y 89,
    4:4:4 desde 90;
  - trellis, Huffman optimizado y deringing del preset balanceado.
- PNG pasa de `png`/Balanced a OxiPNG nivel 2 sobre la imagen RGBA.
- Se mantienen alfa, aplanado JPG sobre blanco, límites y exportación segura.

## Benchmark

Segunda pasada frente a Squoosh, 7 imágenes opacas y 3 repeticiones:

| Formato | Tamaño HUNK/Squoosh | Velocidad |
| --- | --- | --- |
| JPEG q75 | 1,004 mediana (0,999–1,039) | 2,48× |
| JPEG q90 | 1,003 mediana (0,999–1,023) | 2,50× |
| PNG | 0,992 mediana (0,955–1,003) | 3,22× |

El peor caso es la captura de UI a JPEG q75: +3,9 %. En fotos la diferencia es
prácticamente nula. PNG queda igual o mejor en todas las imágenes opacas.

## Verificación

- `cargo test`: **49 pruebas**.
- `cargo fmt --all -- --check` y `cargo clippy --all-targets -- -D warnings`
  limpios.
- Nuevas pruebas:
  - JPG progresivo;
  - submuestreo 4:2:0/4:2:2/4:4:4 según calidad;
  - PNG conserva todos los píxeles, no solo el alfa.
- Datos: `docs/benchmark-squoosh-2026-10-31-t39.tsv`.

## Pendiente

- Exponer progresivo/submuestreo y nivel PNG en la UI (`t-42`).
- Validación visual de la build de Windows en el cierre de release.
