# UI avanzada y persistencia

Fecha: 2026-10-31 · Tarea: `t-42`

## Cambios

- Bloque `Avanzado` contextual, colapsado, con resumen y reset.
- Opciones expuestas:
  - JPG: progresivo y submuestreo auto/4:2:0/4:2:2/4:4:4.
  - PNG: nivel rápido/equilibrado/máximo e interlazado.
  - WebP: lossless, method y alpha quality.
  - AVIF: speed y alpha quality.
- El selector de salida queda en JPEG/PNG/WebP/AVIF/ICO; JPEG XL sale de
  `OutputFormat::ALL`.
- `[advanced]` en `config.ini` persiste las opciones y el preprocesado.
- Configuraciones antiguas siguen funcionando y `format=jxl` cae a `Auto` con
  advertencia.

## Verificación

- `cargo test`: **55 pruebas**.
- `cargo fmt --all -- --check` y `cargo clippy --all-targets -- -D warnings`
  limpios.
- Prueba de ida y vuelta de `[advanced]` y de configuraciones antiguas.
- Prueba visual bajo Xvfb: bloque `Avanzado · Progresivo · Automático`
  colapsado y expandido.

## Pendiente

- Validación visual de la build de Windows en el cierre de release.
