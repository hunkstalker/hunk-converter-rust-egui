# Estado vacío integrado en el canvas

Fecha: 2026-11-12 · Estado: validado por el usuario

## Cambio

La pantalla inicial ya no es una tarjeta independiente. La aplicación abre
directamente el canvas y muestra el estado vacío dentro de él:

- fondo de lienzo con borde sutil y el mismo margen que la vista de imagen;
- icono de marca, título, descripción y acción principal centrados;
- mensaje explícito para arrastrar una imagen o pegarla con `Ctrl+V`;
- formatos de entrada y nota de privacidad;
- panel lateral visible con estado «Sin imagen», formato y calidad configurables;
- panel de color visible con historial y códigos vacíos;
- controles superiores de tema e idioma visibles desde el primer frame.

La lógica de carga no cambia: siguen funcionando arrastrar y soltar, `Ctrl+O`,
`Ctrl+V` y el selector nativo. Hasta que hay imagen, el cuentagotas, el recorte,
la inversión y el giro permanecen ocultos. Al cargar una imagen se entra en la
misma vista de siempre, con comparador, tarjetas y herramientas.

## Archivos

- `src/app/mod.rs`: el editor y el canvas vacío comparten `show_preview` y el
  panel lateral se dibuja siempre.
- `src/app/ui/preview.rs`: el canvas se asigna siempre, decide entre
  `paint_canvas` y el estado vacío, y reserva un hueco fijo para el panel de color
  según el lado por defecto. Así el mensaje no se mueve cuando el usuario arrastra
  el panel de color.
- `src/app/ui/empty.rs`: fondo y contenido del estado vacío dentro del canvas.
- `src/app/ui/sidebar.rs`: tarjeta «Sin imagen», controles de formato y calidad
  activos, acciones dependientes de píxeles desactivadas.
- `src/app/ui/color_card.rs`: el panel de color se muestra sin muestra activa.
- `src/app/encoding.rs`: `request_encode` no hace nada si no hay imagen.
- `src/i18n.rs`: cadenas `EmptyHint` y `NoImage`.
- Limpieza asociada: se eliminaron helpers del tema y 18 cadenas i18n sin
  uso, se simplificaron las condiciones redundantes de `preview.rs` y se añadió
  una prueba para la reserva estable del contenido vacío.

## Verificación

- `cargo fmt --all -- --check`: correcto.
- `cargo clippy --all-targets -- -D warnings`: correcto.
- `cargo test`: **73 pruebas** correctas.
- Capturas revisadas bajo Xvfb: canvas vacío en oscuro y claro, panel lateral a
  izquierda y derecha, y editor con imagen después del cambio.
- Comprobado que el formato se puede cambiar sin imagen y se persiste en
  `config.ini`; no se encola ninguna codificación.
- Comprobado que arrastrar el panel de color no desplaza el contenido del canvas.
- Builds portables regeneradas:
  - Linux x86_64: **26.196.072 B**.
  - Windows x86_64 GNU: **23.942.144 B**.
- Validación visual del usuario completada.
