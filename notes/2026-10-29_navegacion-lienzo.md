# Navegación avanzada del lienzo

Fecha: 2026-10-29
Tarea: `t-16`

## Cambios

- El porcentaje de zoom refleja la escala real de la imagen respecto a sus
  píxeles, no una escala relativa al encuadre. La imagen se abre al 100 %.
- Atajos:
  - `0` o `F`: ajustar a la ventana.
  - `1`: tamaño real 1:1.
  - `+` / `-`: zoom por pasos.
- La rueda y la pinza del trackpad siguen funcionando y hacen zoom anclado al
  puntero.
- El paneo con clic derecho funciona tanto en 1:1 como al ampliar.
- El modo 1:1 se desactiva al cambiar de imagen, reiniciar o ajustar.

## Detalles de implementación

- `actual_size` indica si se dibuja a tamaño real.
- `zoom` sigue siendo relativo al encuadre; `fit_scale` traduce ese valor a
  porcentaje real.
- Al hacer zoom desde 1:1 se convierte la escala real a zoom relativo para no
  perder el punto bajo el cursor.
- `MIN_ZOOM` permite reducir por debajo del encuadre; `0`/`F` recupera el
  estado inicial.
- En la vista ICO no se muestra el zoom.

## Verificación

- `cargo test`: 31 pruebas.
- `cargo clippy --all-targets -- -D warnings`: limpio.
- `cargo fmt --all -- --check`: limpio.
- Comprobado con Xvfb:
  - 1:1 en imagen grande (100 %).
  - 1:1 en imagen pequeña de 100×100.
  - zoom desde 1:1 sin saltar al encuadre.
  - atajos `0`, `1`, `+` y `-`.
  - cabecera sin botones de zoom.
  - vista ICO sin controles.
- Capturas de documentación regeneradas.
