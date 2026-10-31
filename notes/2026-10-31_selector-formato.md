# Selector de formato de salida con modo automático

Fecha: 2026-10-31 · Tarea: `t-35`

## Cambios

- Se sustituye la rejilla de tarjetas por un desplegable de formato.
- Se añade `FormatSelection::{Auto, Fixed}`.
- `Auto` resuelve el formato de salida desde el formato de entrada:
  - PNG/JPG/WebP → mismo formato;
  - BMP/SVG → WebP;
  - los formatos nuevos ampliarán la tabla en `t-36`–`t-38`.
- Una selección fija no cambia al cargar otra imagen.
- El control cerrado muestra el formato resuelto en una sola línea; las pistas
  están en las opciones del menú y en el tooltip de `Automático`.
- Se avisa de la recompresión cuando la entrada coincide con una salida con
  pérdida.
- La elección se persiste en `[output] format=auto|<formato>` de `config.ini`.
- El reinicio vuelve a `Auto`.
- El selector permanece dentro del área con scroll, en el mismo lugar que
  ocupaban las tarjetas. El menú se dibuja por encima del contenido y muestra
  las siete opciones completas.
- La superficie lateral ocupa el ancho original y llega hasta el footer, sin
  margen inferior.
- Accesibilidad: ComboBox con nombre accesible, foco visible y opciones con
  texto seleccionado en color oscuro sobre el fondo de acento.

## Verificación

- `cargo test`: **36 pruebas**.
- `cargo fmt --all -- --check` y `cargo clippy --all-targets -- -D warnings`
  limpios.
- Pruebas añadidas para resolución `Auto`, fallback WebP y parser de
  `[output]`.
- Bajo Xvfb:
  - el desplegable muestra las siete opciones actuales completas;
  - seleccionar JPG actualiza la UI y guarda `format=jpg`;
  - una entrada JPG muestra el aviso de recompresión;
  - la captura de editor muestra `Automático · PNG` para una entrada PNG;
  - el marco lateral recupera x=30..389 y llega hasta el footer;
  - las capturas de documentación se regeneraron y ocultan los anillos de foco
    mediante `HUNK_SCREENSHOT`;
  - validación visual del usuario: el diseño vuelve a parecerse a `HEAD`.
- Builds actualizadas:
  - Linux x86_64;
  - Windows GNU x86_64. En este entorno no se pudo incrustar el icono con
    `windres`; la build MSVC del CI sí lo incrusta.

## Pendiente

- Validación visual de la build de Windows durante el cierre de release.
