# Exportación segura a `export/`

Fecha: 2026-10-29

## Cambio

- La exportación mantiene `export/` junto al ejecutable como destino fijo. No
  hay selector de carpeta.
- El archivo se crea con `OpenOptions::create_new(true)`. La comprobación de
  existencia y la creación son una sola operación atómica.
- Si el nombre ya existe, se prueban `nombre-2.ext`, `nombre-3.ext`, etc.
- Si la escritura falla después de crear el archivo, se elimina el archivo
  parcial para no dejar restos.
- Los errores de creación o escritura siguen mostrándose como aviso.

## Verificación

- `cargo test`: 31 pruebas. La prueba de exportación comprueba que tres
  exportaciones consecutivas generan `foto.webp`, `foto-2.webp` y `foto-3.webp`
  y conservan el contenido de cada archivo.
- Prueba de integración bajo Xvfb: con `export/demo.webp` preexistente, `Ctrl+E`
  creó `demo-2.webp` y dejó intacto `demo.webp`.
- `cargo fmt --all -- --check` y clippy limpios.

**Validación:** usuario, 2026-10-29.
