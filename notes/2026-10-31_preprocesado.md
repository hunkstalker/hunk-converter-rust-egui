# Preprocesado: resize, rotación y paleta

Fecha: 2026-10-31 · Tarea: `t-41`

## Cambios

- Nuevo `PreprocessOptions` en dominio:
  - resize activable con ancho/alto y proporción opcional;
  - rotación 0/90/180/270;
  - paleta opcional de 2–256 colores.
- Resize con Lanczos3.
- Paleta con `quantette` (MIT/Apache) y alfa conservado.
- La rotación y el resize actualizan la imagen de trabajo y la vista previa.
- La paleta se aplica al resultado codificado, no a la imagen de comparación.
- Se conserva la imagen original para poder recalcular la geometría.

## UI

Bloque `Preprocesado` colapsado en la barra lateral, con resize, rotación y
paleta. Los controles disparan reencode.

## Verificación

- `cargo test`: **54 pruebas**.
- `cargo fmt --all -- --check` y `cargo clippy --all-targets -- -D warnings`
  limpios.
- Pruebas nuevas:
  - rotación 90 y alfa;
  - resize 16×8 → 8×4 con proporción;
  - paleta de 8 colores con alfa intacto.
- Prueba visual bajo Xvfb: bloque colapsado y expandido; rotación 90 actualiza
  dimensiones y vista previa.

## Pendiente

- Persistencia de las opciones (`t-42`).
- Validación visual de la build de Windows en el cierre de release.
