# Ventana, zoom inicial y línea del comparador

Fecha: 2026-10-29

## Cambios

- **Persistencia de ventana:** `config.ini` guarda `[window]` con `x`, `y`,
  `width` y `height`. Al arrancar se restauran tamaño y posición. Si la ventana
  está maximizada no se sobrescribe la geometría normal.
- **Zoom inicial:** la imagen se abre al 100 % de escala real. `0`/`F` sigue
  ajustando a la ventana.
- **Línea del comparador:** la línea y su sombra llegan ahora a los bordes
  superior e inferior de la imagen visible, sin el margen de 8 px anterior.
- **Alineación del zoom:** el porcentaje se dibuja en una caja de altura fija,
  con un desplazamiento óptico de 7 px para compartir el centro vertical con
  “Vista previa”. Medido sobre la captura final: ambos textos tienen su centro
  en `y=105` (el título incluye el descendente de la “p”).

## Guardado de la ventana

- La geometría se actualiza cuando cambia la ventana.
- Se guarda tras 500 ms sin cambios para no escribir `config.ini` en cada frame.
- `on_exit` fuerza un guardado final.
- En Wayland, si no hay posición de ventana disponible, se guarda solo el tamaño.
- Si el monitor guardado ya no existe, el sistema operativo recoloca la ventana.

## Verificación

- `cargo test`: 31 pruebas, incluidas ida y vuelta de `[window]` y rechazo de
  tamaños inválidos.
- `cargo clippy --all-targets -- -D warnings`: limpio.
- `cargo fmt --all -- --check`: limpio.
- Bajo Xvfb se comprobó que una ventana de 1280×1000 en `+60+0` se guarda en
  `config.ini` y se restaura en la siguiente ejecución.
- Capturas de documentación regeneradas con la imagen al 100 % y la línea
  completa.
