# Panel de color flotante y herramientas recolocables

Fecha: 2026-11-01 · Estado: implementado y validado por el usuario

## Comportamiento

- La tarjeta de historial/cuentagotas se mueve con un handle de seis puntos en
  la cabecera. Solo el handle arrastra; los swatches y códigos conservan sus
  clics.
- La posición se limita al área de la ventana y dura la sesión. No se escribe
  en `config.ini`.
- Las herramientas rápidas (borrar, recortar, cuentagotas, invertir, girar)
  siguen al panel. Por defecto quedan a su derecha; si no hay espacio a la
  derecha, pasan a la izquierda.
- El desplegable de recorte se abre hacia el lado con más espacio. Con las
  herramientas a la izquierda, los presets se despliegan a la izquierda del
  botón de recorte.
- La tarjeta puede quedar por encima del panel derecho de formatos; el área usa
  `Order::Foreground`. Mientras la cubre, los controles de debajo no reciben
  clics en la zona solapada.

## Implementación

- `color_panel_pos` y `color_panel_size` en `HunkApp`.
- `Area::fixed_pos` + `constrain_to` para la tarjeta.
- El handle actualiza la posición con el delta del puntero y la limita con el
  tamaño conocido del panel.
- `quick_tools_position` calcula el lado y la posición de la columna.
- `crop_presets_position` elige el lado del flyout.
- El panel conserva su posición inicial abajo a la izquierda cuando todavía no
  se ha arrastrado.

## Verificación

- Captura inicial con la tarjeta abajo a la izquierda y herramientas a la
  derecha.
- Captura con la tarjeta sobre el panel derecho: herramientas a la izquierda.
- Captura con edición de recorte y tarjeta a la derecha: presets desplegados a
  la izquierda.
- `cargo fmt --check`, clippy y 70 pruebas correctos.
- Builds Linux y Windows GNU regeneradas.
- Validación visual e interactiva del usuario completada.
