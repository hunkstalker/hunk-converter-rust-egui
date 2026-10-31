# Entrada TIFF de una página

Fecha: 2026-10-31 · Tarea: `t-36`

## Cambios

- `image` activa la feature `tiff`.
- El filtro de archivos acepta `.tif` y `.tiff`.
- TIFF se detecta por magic bytes, con respaldo por extensión.
- Se carga solo la primera página.
- Se admiten RGB, RGBA, gris y gris con alfa de 8 bits.
- Se rechazan CMYK, 16 bits y otros tipos fuera del alcance con un error
  controlado.
- Se aplica la orientación TIFF.
- Los límites de ancho, alto y 40 MP se comprueban antes de decodificar.
- El nombre de formato mostrado en la ficha es `TIFF`.
- Como la salida TIFF llega en `t-38`, `Automático` usa WebP como fallback para
  entradas TIFF.

## Verificación

- `cargo test`: **47 pruebas**.
- `cargo fmt --all -- --check` y `cargo clippy --all-targets -- -D warnings`
  limpios.
- Pruebas añadidas:
  - detección y carga desde archivo;
  - compresiones sin comprimir, LZW, Deflate y PackBits;
  - predictor horizontal en LZW y Deflate;
  - escala de grises a RGBA8;
  - alfa RGBA;
  - orientación 6;
  - primera página de un TIFF multipágina;
  - rechazo de 16 bits.
- Prueba manual bajo Xvfb: un TIFF de 1280×1000 abre como `TIFF`, con vista
  previa y `Automático · WebP`.
- Matriz de integración `tests/format_matrix.rs`: PNG/JPG/WebP/BMP/TGA/TIFF/SVG,
  alfa, límites y archivos corruptos.
- `check_tiff` sobre `dist/pruebas-tiff`: 9 comprobaciones correctas, incluidas
  LZW/Deflate/PackBits, gris, alfa, orientación, multipágina y rechazo de 16 bits.
- Suite total: **47 pruebas**.

## Pendiente

- Validación visual de la build de Windows en el cierre de release.
