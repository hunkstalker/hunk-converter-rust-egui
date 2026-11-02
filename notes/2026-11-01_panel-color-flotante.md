# Panel de color flotante y herramientas recolocables

Fecha: 2026-11-01 · Validado: 2026-11-02 · Estado: implementado y validado por el usuario

## Comportamiento

- La tarjeta de historial/cuentagotas se mueve con un handle de seis puntos en
  la cabecera. Solo el handle arrastra; los swatches y códigos conservan sus
  clics. Un doble clic en el handle devuelve la tarjeta a su posición inicial.
- La posición se limita al área de la ventana y se guarda en `config.ini` como
  `color_panel_x` y `color_panel_y` dentro de `[window]`. El doble clic de reset
  elimina ambas claves y vuelve al anclaje inicial.
- Las herramientas rápidas (borrar, recortar, cuentagotas, invertir, girar)
  siguen al panel. Por defecto quedan a su derecha; cuando la tarjeta cruza el
  centro horizontal de la ventana, pasan a la izquierda.
- El desplegable de recorte se abre hacia el lado con más espacio. Con las
  herramientas a la izquierda, los presets se despliegan a la izquierda del
  botón de recorte.
- La tarjeta puede quedar por encima del panel derecho de formatos; el área usa
  `Order::Foreground`. Mientras la cubre, los controles de debajo no reciben
  clics en la zona solapada.

## Implementación

- `color_panel_pos` y `color_panel_size` en `HunkApp`. El doble clic pone la
  posición a `None` para que vuelva al anclaje inicial. La posición se persiste
  al terminar el arrastre y al restablecerla.
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
- Doble clic en el handle comprobado: la tarjeta vuelve al anclaje inicial y se
  eliminan `color_panel_x`/`color_panel_y` de `config.ini`.
- `cargo fmt --check`, clippy y 70 pruebas correctos.
- Builds Linux y Windows GNU regeneradas.
- Arrastre, recolocación, doble clic de reset y margen inferior de exportación
  validados por el usuario el 2026-11-02.
