# Entrada AVIF

Fecha: 2026-10-31 · Tarea: `t-37`

## Cambios

- `avif-decode` se reutiliza para cargar AVIF, no solo para la vista previa.
- `avif-parse` comprueba las dimensiones antes de decodificar.
- Se añade `.avif` al filtro de archivos y a la detección por magic bytes.
- Se conserva el alfa cuando el contenedor lo incluye.
- La salida de 10/12 bits se convierte a RGBA8.
- `Automático` resuelve AVIF → AVIF.
- JPEG XL queda fuera del alcance web de 1.1.

## Verificación

- `cargo test`: **47 pruebas**; matriz de entradas ampliada con AVIF y alfa.
- `cargo fmt --all -- --check` y `cargo clippy --all-targets -- -D warnings`
  limpios.
- Prueba manual bajo Xvfb: un AVIF de 256×256 abre como `AVIF`, con alfa,
  `Automático · AVIF` y aviso de recompresión.
- Se añadieron pruebas de archivo AVIF corrupto y carga desde archivo.

## Pendiente

- Validación visual de la build de Windows en el cierre de release.
