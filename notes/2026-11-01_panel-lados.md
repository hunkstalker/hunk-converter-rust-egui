# Panel de ajustes a la izquierda o derecha

Fecha: 2026-11-01 · Validado: 2026-11-02 · Estado: implementado y validado por el usuario

## Comportamiento

- Nuevo botón compacto de 64×32 en los controles superiores, a la izquierda del
  selector de idioma. Usa un icono de panel con la barra interior en el lado
  actual; el tooltip indica el lado al que se moverá.
- Por defecto el panel está a la derecha.
- La elección se guarda como `panel_side=right` o `panel_side=left` en
  `[general]` de `config.ini`.
- Las configuraciones antiguas sin la clave arrancan a la derecha.
- El panel derecho conserva el borde superior e izquierdo, la esquina superior
  izquierda redondeada y queda pegado por la derecha y por abajo.
- Al pasar a la izquierda, el diseño se refleja: borde superior y derecho,
  esquina superior derecha redondeada, y lados izquierdo e inferior pegados.

## Implementación

- `PanelSide` en `src/settings/mod.rs`, con `Right` por defecto y `toggled()`.
- Parseo y serialización en `src/settings/ini.rs`, con aviso para valores
  desconocidos.
- `HunkApp` conserva el lado y lo incluye al guardar `config.ini`.
- `show_sidebar` elige `Panel::right` o `Panel::left`.
- `sidebar_frame` usa relleno de superficie sin borde completo.
- `paint_sidebar_border` dibuja solo la línea superior, el arco interior y la
  línea lateral interior.
- El botón de cambio de lado está en `show_top_controls`, a la izquierda de
  idioma, con un icono dibujado a mano para no depender de glifos de flecha.

## Verificación

- Panel derecho por defecto con el nuevo contorno.
- Cambio a izquierda desde el botón superior y persistencia de
  `panel_side=left` en `config.ini`.
- Capturas de editor, cuentagotas, ICO, zoom y recorte regeneradas con el nuevo
  contorno, el botón de cambio de lado y el margen inferior de exportación.
- `cargo fmt --check`, clippy y 70 pruebas correctos.
- Builds Linux y Windows GNU regeneradas.
- Validación visual e interactiva del usuario completada el 2026-11-02.

