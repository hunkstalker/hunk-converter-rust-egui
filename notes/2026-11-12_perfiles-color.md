# Perfiles de color y conmutador oscuro/claro

Fecha: 2026-11-12 · Estado: validado por el usuario

## Objetivo

Dejar de tener los colores hardcodeados y centralizarlos en perfiles. La UI no
debe conocer si el tema es oscuro o claro: solo pide `fondo`, `superficie`,
`texto`, `acento`, etc. Así se podrán añadir más perfiles en el futuro sin
tocar los componentes.

## Implementación

- `src/theme.rs` define `Palette` y `ThemeProfile`, con un registro `PROFILES`
  que contiene `dark` y `light`.
- El perfil activo se guarda en un `AtomicUsize`; `T::apply(ctx, id)` cambia la
  paleta y los `Visuals` de egui sin reiniciar.
- Los accesores dinámicos (`T::bg()`, `T::surface()`, `T::text()`, `T::accent()`,
  etc.) sustituyen a las antiguas constantes en toda la UI.
- Se migraron también los colores que estaban sueltos: toast de error, damero,
  sombra y línea del comparador, marco de referencia, muestra del cuentagotas,
  atenuado y cuadrícula del recorte, badge `ORIGINAL` y candado del historial.
- `[general] theme=dark|light` se lee y se guarda en `config.ini`. Sin la clave
  se usa `dark`; un perfil desconocido genera aviso y vuelve a `dark`.
- El botón sol/luna (64×32) está a la izquierda del botón de panel, con la misma
  altura y separación que el resto de controles superiores.
- El icono muestra el modo activo: luna en oscuro, sol en claro.
- La barra nativa y los bordes de la ventana los gestiona el SO:
  `sync_window_theme` queda desactivado para que el perfil de HUNK solo cambie
  el contenido. En Windows/macOS la decoración sigue la apariencia del sistema;
  en Linux X11 depende del gestor de ventanas y en Wayland del compositor.

## Paleta clara

Fondos cálidos y superficies claras. Tras la revisión visual se oscurecieron los
textos para que no se diluyeran sobre el blanco:

- texto principal `#0C100B`, con contraste mínimo **14,66:1**;
- texto secundario `#262E26`, con contraste mínimo **10,69:1**;
- texto tenue `#343D34`, con contraste mínimo **8,61:1**;
- texto deshabilitado `#606A60`, con contraste mínimo **4,31:1**;
- acento oliva `#425816`, con contraste mínimo **6,08:1** como texto y **7,43:1**
  para la tinta crema sobre el acento.

La tipografía Manrope es fina y de tamaño pequeño en muchas etiquetas, así que
no basta con cumplir el mínimo AA de 4,5:1: los textos secundarios y tenues
quedan por encima de 8:1 para compensar el grosor de trazo. También se sube
`weak_text_alpha` a 0,95 en el tema claro. Las pruebas cubren todos los fondos de
ambos perfiles, el texto deshabilitado (3:1), el acento (4,5:1), la tinta sobre
el acento y el toast de error.

## Verificación

- `cargo fmt --all -- --check`: correcto.
- `cargo clippy --all-targets -- -D warnings`: correcto.
- `cargo test`: **72 pruebas** correctas (67 unitarias + 5 de integración).
- Capturas revisadas bajo Xvfb: editor en oscuro, editor en claro, pantalla
  inicial en claro y persistencia al pulsar el conmutador.
- Builds portables regeneradas:
  - Linux x86_64: **26.199.016 B**.
  - Windows x86_64 GNU: **23.945.728 B**.
- Validación visual del usuario completada, incluida la legibilidad del tema
  claro y la barra nativa gestionada por el SO.
