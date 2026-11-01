# Proyecto — HUNK Converter

> Documento vivo. Última actualización: 2026-10-31.
> Versión actual: **1.0.0** publicada · Versión en preparación: **1.1.0** · Estado: **aplicación principal Rust + egui, portable y funcional**.

## 1. Visión

HUNK Converter es una aplicación de escritorio local-first para convertir, comprimir, comparar y exportar imágenes. También genera iconos `.ico` multirresolución para Windows.

El producto está diseñado para hacer tres operaciones sin fricción:

1. Convertir una imagen a otro formato.
2. Reducir su peso ajustando la calidad.
3. Comprobar visualmente el resultado antes de guardarlo.

La interacción principal es arrastrar, ajustar y comparar en una sola ventana. No hay cuentas, subidas, telemetría ni pasos de configuración inicial.

La versión actual está construida en **Rust + egui**. El prototipo original de Godot se conserva como material histórico en `../Prototipo`.

El alcance vigente de 1.1 está en [docs/objetivo-1.1-web.md](docs/objetivo-1.1-web.md): núcleo web JPEG/PNG/WebP/AVIF, SVG como entrada, ICO como extra y preprocesado de redimensionado, rotación y paleta.

## 2. Principios de producto

- **Local-first:** todo el procesamiento ocurre en el equipo del usuario. La aplicación no realiza peticiones de red.
- **Portable:** no requiere instalación. Al exportar se elige la ruta con el diálogo nativo; si existe, se propone `export/` junto al ejecutable.
- **Repintado bajo demanda:** la interfaz solo solicita frames cuando hay actividad. Una ventana estática no consume un bucle de render a 60 FPS.
- **Transparencia:** la comparación usa la imagen codificada y decodificada, no una estimación.
- **Ruta elegida por el usuario:** la exportación usa el diálogo nativo y el nombre por defecto conserva el stem original.
- **Código abierto:** licencia MIT, dependencias razonables y pruebas ejecutables sin servicios externos.
- **Interfaz tranquila:** una tarea principal, jerarquía clara y espacio para la imagen.

## 3. Funcionalidad actual

### 3.1 Entrada

- Arrastrar y soltar un archivo en cualquier parte de la ventana.
- Selector nativo de archivos con el botón **Seleccionar una imagen** o `Ctrl+O`.
- `Ctrl+V` pega una imagen del portapapeles y sustituye la sesión actual si ya había una.
- Botón **Cambiar** para reemplazar la imagen conservando los ajustes.
- Formatos: **PNG, JPG/JPEG, WebP, AVIF, BMP, TGA, TIFF y SVG**.
- AVIF se abre con `avif-decode`, conserva alfa y se convierte a RGBA8.
- Orientación EXIF aplicada al cargar JPG/WebP, y orientación TIFF, antes de mostrar, comparar y exportar.
- TIFF de una página en RGB/RGBA/gris de 8 bits, con compresiones habituales (sin comprimir, LZW, Deflate y PackBits); CMYK y 16 bits se rechazan.
- SVG rasterizado con `resvg`, manteniendo proporción y transparencia.
- Detección por contenido con respaldo por extensión.
- Límites: **200 MB**, **40 megapíxeles** y **16 384 px por lado**.
- Los errores se muestran como avisos y no sustituyen la sesión válida anterior.

### 3.2 Formatos de salida

| Formato | Extensión | Compresión | Transparencia | Uso |
| --- | --- | --- | --- | --- |
| WebP | `.webp` | Con pérdida, calidad 1–100 | Sí | Opción recomendada |
| AVIF | `.avif` | Con pérdida, calidad 1–100 | Sí | Compresión moderna |
| JPG | `.jpg` | Con pérdida, calidad 1–100 | No; fondo blanco | Compatibilidad |
| PNG | `.png` | Sin pérdida | Sí | Calidad pixel a pixel |
| ICO | `.ico` | PNG por tamaño | Sí | Iconos de Windows |

- Salida objetivo de 1.1: JPEG, PNG, WebP, AVIF e ICO. JPEG XL se retira del selector. La entrada es más amplia: JPEG, PNG, WebP, AVIF, SVG, BMP, TGA y TIFF.
- Selector de salida en cuadrícula de tarjetas con modo **Automático** (mismo formato que la entrada cuando existe salida equivalente; WebP como fallback).
- Calidad inicial: **82 %**.
- WebP, JPG y AVIF muestran deslizador de calidad; PNG es sin pérdida y muestra el deslizador deshabilitado; ICO usa selección de tamaños.
- AVIF se añade como formato de salida. JPEG XL queda fuera del selector de 1.1.
- JPG aplana la transparencia sobre blanco de forma predecible.
- ICO incluye 16, 32, 48, 64, 128 y 256 px, ajustando la imagen centrada en un lienzo transparente sin recortar.
- Se exige al menos un tamaño ICO seleccionado.

### 3.3 Comparador y vista previa

- Vista dividida entre **Original** y **Resultado**.
- Divisor arrastrable con ratón; el cursor de redimensionado solo aparece sobre la línea o el círculo.
- Respuesta a `←`/`→`, `Inicio` y `Fin`.
- La imagen resultante se obtiene decodificando el archivo codificado real.
- Damero para transparencia.
- Modo ICO: se oculta el divisor y se muestra el icono generado, con recuento de tamaños.
- La imagen se abre al 100 %; con resize activo se encuadra el original para mostrar su marco. `0`/`F` ajusta a la ventana conservando la proporción.
- **Zoom continuo** con rueda o pinza, anclado al cursor, y atajos de teclado.
- **Modo 1:1** con `1` para ver la imagen a escala real; `0`/`F` vuelve al encuadre completo.
- **Paneo con clic derecho** cuando la imagen está ampliada, con límites para no perderla de vista.
- El porcentaje de zoom refleja la escala real. No hay botones de zoom en la vista previa: se controla con teclado, rueda o pinza.
- El divisor, su círculo y el cuentagotas se adaptan al zoom y al desplazamiento.
- La vista previa se limita a 2048 px de lado; la exportación conserva la resolución completa.

### 3.3.1 Preprocesado y resize

- Ancho y alto se sincronizan en el mismo frame cuando `Mantener proporción` está activo.
- La referencia del tamaño original es un marco tenue en el canvas; no añade texto a la barra lateral.
- Los campos numéricos se ajustan con arrastre vertical y aceptan expresiones `1600x2`, `1600/4`, `1600*2` o `1600-200`; `Enter` aplica y `Esc` cancela.
- El resize completo se calcula en un worker con debounce, sin bloquear la UI.
- `Preprocesado` y `Avanzado` comparten el mismo componente de sección y reset.

### 3.4 Cuentagotas

- Icono de **cuentagotas** en la tarjeta flotante inferior izquierda.
- Un clic activa o desactiva el modo; después basta con hacer clic sobre la imagen.
- También se puede pulsar el icono y arrastrar hasta el píxel: se muestrea mientras se arrastra y se conserva el último color al soltar.
- `Esc` o clic derecho sobre la imagen desactivan el modo sin alterar la muestra.
- El color muestreado se refleja en los códigos OKLCH/OKLAB/HEX/RGB/RGBA/HSL.
- La marca visual de muestreo solo se dibuja mientras el cuentagotas está activo.
- Códigos simultáneos: **OKLCH, OKLAB, HEX, RGB, RGBA y HSL**. Al hacer clic en el propio código se copia el valor; con `Ctrl`, HEX copia `HEX#...` y el resto conserva su función CSS completa.
- El muestreo no mueve el divisor del comparador.
- En modo ICO se desactiva, porque la vista no representa la imagen original.
- La muestra y el zoom se reinician al cargar otra imagen o reiniciar la sesión.
- **Historial:** 21 colores en tres filas. Clic izquierdo selecciona; clic derecho fija o libera. El color no fijado más antiguo se reemplaza cuando está lleno. El botón de papelera limpia los no fijados.
- El historial y los bloqueos se guardan en `config.ini` y persisten entre sesiones.

### 3.5 Idiomas y configuración

- Selector **ES / CA / EN** junto al botón de reinicio.
- En el primer arranque se detecta el idioma del sistema con `sys-locale`.
- La preferencia, el historial, los bloqueos, el estado maximizado, el formato de salida y la posición/tamaño de la ventana se guardan en `config.ini`, junto al ejecutable.
- Si el archivo no es escribible, la app sigue funcionando sin persistencia.

### 3.6 Exportación

- Botón principal **Exportar .ext** y atajo `Ctrl+S`.
- Al pulsar se abre el **explorador nativo** para elegir carpeta y nombre.
- Si existe, se propone la carpeta `export/` junto al ejecutable como destino inicial.
- Si no se escribe extensión, se añade la del formato actual.
- El nombre base se conserva y se sanea para Windows.
- Los errores de permisos o escritura se comunican mediante aviso.

### 3.7 Reinicio y atajos

| Atajo | Acción |
| --- | --- |
| `Ctrl+O` | Abrir imagen |
| `Ctrl+V` | Pegar una imagen del portapapeles |
| `Ctrl+S` | Exportar |
| `Ctrl+R` | Reiniciar la sesión |
| `0` / `F` | Ajustar a la ventana |
| `1` | Tamaño real 1:1 |
| `+` / `-` | Zoom por pasos |
| `←` / `→` | Mover el divisor |
| `Inicio` / `Fin` | Extremos del divisor |
| `Tab` / `Shift+Tab` | Mover el foco |
| `Enter` / `Espacio` | Activar el control enfocado |
| `↑` / `↓` | Paneo vertical con la imagen ampliada |
| `Shift+←` / `Shift+→` | Paneo horizontal con la imagen ampliada |
| `L` | Bloquear el color enfocado del historial |
| `Esc` | Cancelar el cuentagotas |

`Ctrl+E` sigue funcionando como alias de exportación.

### Accesibilidad

- AccessKit activo para lectores de pantalla en Windows, macOS y Linux.
- Roles, nombres, estados y valores en controles estándar y personalizados; avisos como alertas.
- Foco visible en todos los controles y orden de tabulación completo.
- Formatos como grupo de radios; historial con tab stop único y navegación por flechas.
- Cuentagotas y comparador operables con teclado; el divisor se expone como slider.
- Texto informativo con contraste WCAG AA y prueba automática de contraste.
- Validación manual con teclado completada; AccessKit expone la interfaz a los lectores compatibles.

- **Reiniciar** elimina la imagen, el resultado, el muestreo y el estado de exportación.
- Restablece formato WebP, calidad 82 y todos los tamaños ICO.
- No borra archivos ya exportados.

## 4. Experiencia de uso

### Estados

1. **Vacío:** tarjeta central con ilustración, acción principal y formatos admitidos.
2. **Editando:** panel lateral de ajustes + comparador + metadatos.
3. **Procesando:** se mantiene la última vista válida, aparece un indicador y se bloquea la exportación.
4. **Listo:** comparador, peso y ahorro actualizados; exportación disponible.
5. **Exportado:** confirmación con la ruta relativa.
6. **Error:** aviso temporal; la sesión existente se conserva cuando es posible.

### Ventana y distribución

- La aplicación se abre con la ventana maximizada. Los controles nativos permiten maximizar o restaurar; el estado, la posición y el tamaño se guardan en `config.ini`.
- Tamaño de ventana por defecto 1280 × 800; mínimo 1024 × 800.
- Vista previa a pantalla completa con el selector de idioma y el botón de reinicio arriba a la derecha, sobre el lienzo. Los controles de ventana son los nativos.
- Panel de ajustes a la derecha, con scroll vertical cuando no cabe. Incluye archivo, formato en cuadrícula, calidad o tamaños ICO, preprocesado, avanzado, pesos y exportación.
- Historial y cuentagotas en una tarjeta flotante abajo a la izquierda del lienzo, con botones rápidos de invertir colores y girar 90°.
- Pie global con estado del proyecto, versión y enlace al repositorio.

### Render bajo demanda

- La interfaz no mantiene una animación de UI en reposo.
- egui solo pide repintado cuando hay eventos o mientras se está procesando una imagen.
- En reposo, el proceso no consume CPU de render.
- Es una diferencia deliberada frente al prototipo Godot, que mantenía el bucle de render continuo.

## 5. Arquitectura

### Stack

- **Rust 1.99.1**, fijado en `rust-toolchain.toml`.
- **eframe/egui 0.37** con renderizador **glow** (OpenGL).
- `image` 0.25 para decodificación PNG/JPG/WebP/BMP/TGA/TIFF.
- `mozjpeg-rs` para JPG progresivo con submuestreo automático.
- `oxipng` para compresión PNG sin pérdida.
- `libavif-sys` con AOM para AVIF: YUV420, cuantizador directo, speed y alpha quality.
- `webp`/libwebp para codificación WebP.
- `png` 0.18 para la codificación PNG.
- `avif-decode`/rav1d para decodificar AVIF y `avif-parse` para comprobar dimensiones antes de decodificar.
- `zune-jpegxl` para codificación JPEG XL y `jxl-oxide` para su vista previa.
- `resvg` 0.49 para SVG.
- `sys-locale` para detectar el idioma del sistema.
- Icono de máscara de gas en `assets/branding/hunk-converter.png`, con fuente editable `.af`; `make_icons` genera `icon_256.png`, `icon.ico` e `icon.icns`.
- `rfd` para diálogo de archivos nativo.
- Procesamiento en un hilo con canal `std::sync::mpsc`.
- Sin runtime de Python, Node, Godot ni ImageMagick.

### Mapa de archivos

| Ruta | Responsabilidad |
| --- | --- |
| `src/main.rs` | Arranque de eframe, ventana, icono e inicialización |
| `src/app/mod.rs` | Shell: composición, estado y `eframe::App` |
| `src/app/actions.rs` | `AppAction`: efectos explícitos devueltos por la UI |
| `src/app/encoding.rs` | Worker de codificación, revisiones y previews |
| `src/app/export.rs` | Exportación portable y nombres sin sobrescritura |
| `src/app/session.rs` | Carga, pegado, reinicio, demo y formato |
| `src/app/state.rs` | `SourceImage`, `RenderedResult`, `Toast` y mensajes del worker |
| `src/app/ui/` | Componentes egui: panel derecho, color flotante, canvas, avisos |
| `src/domain/format.rs` | Política de formatos, extensiones y badges |
| `docs/formatos.md` | Matriz de entrada/salida y opciones avanzadas |
| `docs/plan-formatos-y-opciones.md` | Plan de desarrollo de formatos y opciones |
| `src/domain/color.rs` | Códigos de color, HSL y aplanado RGBA |
| `src/domain/history.rs` | Historial, límite, bloqueos y limpieza |
| `src/domain/export_name.rs` | Saneado y numeración de nombres de salida |
| `src/domain/error.rs` | Errores tipados de carga, codificación, exportación y configuración |
| `src/codec/mod.rs` | Carga, validación y dispatcher de codificación |
| `src/codec/encoders.rs` | WebP, JPG, PNG y JPEG XL |
| `src/codec/avif.rs` | AVIF: carga con `avif-decode`/`avif-parse` y salida con AOM |
| `src/codec/ico.rs` | Contenedor ICO y frames PNG |
| `src/codec/svg.rs` | Rasterización SVG |
| `src/settings/mod.rs` | `Settings`, ruta y guardado de `config.ini` |
| `src/settings/ini.rs` | Parser/serializador INI, compatibilidad y avisos |
| `src/i18n.rs` | Traducciones es/ca/en y presentación de errores tipados |
| `src/theme.rs` | Paleta, tipografía, estilos y frames |
| `src/demo.rs` | Imagen sintética para demos y capturas |
| `src/bin/bench.rs` | Benchmark de códecs y memoria |
| `src/bin/make_demo.rs` | Generador de imágenes de prueba |
| `src/bin/make_icons.rs` | Genera `icon_256.png`, `icon.ico` e `icon.icns` desde el PNG fuente |
| `scripts/build_dist.sh` | Empaquetado portable Linux/Windows/macOS |
| `docs/benchmark-2026-10-28.md` | Comparación de rendimiento y recursos |
| `docs/arquitectura.md` | Decisiones de arquitectura y reglas de código |
| `tests` integrados en los módulos | Pruebas unitarias Rust |

### Principios de arquitectura

La aplicación es un **monolito modular por features con núcleo funcional y
shell imperativo**. No se usa Clean Architecture completa ni un contenedor de
dependencias: la app está terminada y no se espera que escale. Las decisiones
completas están en [`docs/arquitectura.md`](docs/arquitectura.md).

- `domain` es puro: no conoce egui, ficheros ni hilos.
- `ui` solo lee estado y devuelve `AppAction` para los efectos de aplicación.
- `app` aplica las acciones y es el único que toca disco, workers o configuración.
- `codec` y `settings` contienen implementaciones concretas.
- Los errores son enums tipados (`LoadError`, `EncodeError`, `ExportError`,
  `ClipboardError`, `ConfigError`) y se traducen con `i18n::present_error`.
- Máximo 3 parámetros además de `self`; los grupos se pasan como DTO
  (`FormatCard`, `HistorySwatch`, `Badge`, `SourceInput`, `CanvasContext`).
- Las funciones de UI no guardan configuración, no copian al portapapeles y no
  lanzan hilos de forma oculta.

### Flujo de datos

```text
Archivo local
  → lectura y decodificación (image / resvg)
  → RGBA8
  → parámetros de salida (formato, calidad, tamaños ICO)
  → hilo de trabajo
      → codificación real
      → decodificación para preview
      → PNG de preview + buffer final
  → hilo principal
      → texturas de egui
      → pesos, ahorro, comparador, cuentagotas
  → ruta elegida por el usuario
```

### Procesamiento y concurrencia

- Solo hay una codificación activa a la vez.
- Los cambios de calidad se agrupan con un debounce de 160 ms.
- Si cambian los ajustes mientras se procesa, se guarda la última petición.
- Un número de generación descarta resultados obsoletos.
- Las texturas se crean en el hilo de UI; las imágenes se codifican en un worker.
- El cierre espera a que termine la tarea activa.

### Cuentagotas: implementación

- La vista previa guarda la geometría del rectángulo ajustado de la imagen.
- La posición del puntero se convierte a coordenadas del píxel original completo.
- El color se lee de la imagen RGBA original, no del resultado recomprimido.
- Se conservan los cuatro canales, de modo que RGBA muestra la transparencia real.
- El icono alterna el modo al hacer clic; si se arrastra desde él, se activa y muestrea bajo el puntero.
- El modo tiene prioridad sobre el arrastre del divisor.
- La marca de muestreo se pinta con contraste claro/oscuro.
- `Esc` cancela el modo.
- En modo ICO el icono queda deshabilitado para no muestrear una representación engañosa.

### ICO

- Cabecera estándar ICO: `reserved=0`, `type=1`, `count=N`.
- Entradas de 16 bytes con ancho/alto (0 para 256), planos, bits, tamaño y offset.
- Frames PNG RGBA, centrados en un lienzo cuadrado transparente.
- Tamaños 16/32/48/64/128/256; preview del frame mayor.

### Tema visual

- Paleta carbón con acento lima.
- Manrope incluida con licencia OFL.
- Tema egui personalizado: tipografía, botones, paneles, sliders, scrollbars y tooltips.
- Icono de la aplicación generado desde `assets/branding/hunk-converter.png` e integrado en la ventana, el `.exe` de Windows (`icon.ico`), el `.app` de macOS (`icon.icns`) y el lanzador `.desktop` de Linux.
- Las capturas de documentación se generan con el modo de demo y el capturador interno.

## 6. Distribución portable

### Salida

- La ruta se elige en el diálogo nativo; si existe, se propone `<carpeta del ejecutable>/export/`.
- `config.ini` se crea junto al ejecutable y guarda idioma, historial, estado maximizado, formato de salida, posición y tamaño de la ventana.
- El diálogo nativo gestiona la confirmación si el archivo ya existe.

### Estructura de la build

```text
hunk-converter-1.0.0-windows-x86_64/
├── hunk-converter.exe
├── config.ini          (se crea al primer arranque)
├── export/
├── LEEME.txt
├── LICENSE.txt
└── LICENCIAS/
    └── Manrope-OFL.txt
```

- El modo Linux genera una carpeta y un `.tar.gz`.
- El modo Windows genera una carpeta y un `.zip` (el CI lo crea con PowerShell).
- El modo macOS genera una carpeta y un `.tar.gz` con binario universal `arm64 + x86_64`.
- La build de Linux depende de las bibliotecas gráficas del sistema (X11/OpenGL), como cualquier app nativa.
- Compilar requiere un compilador de C y CMake para libwebp y libavif/AOM; en x86/x86_64 también NASM para `rav1d` y AOM.
- AVIF enlaza AOM de forma estática; no añade DLLs de runtime.
- La build Windows importa únicamente DLLs del sistema; no necesita DLLs de MinGW.
- La release [v1.0.0](https://github.com/hunkstalker/hunk-converter-rust-egui/releases/tag/v1.0.0) está publicada y validada en Windows 10/11, con icono en Explorer y barra de tareas.
- El CI de GitHub ejecuta pruebas en `main` y PR, y compila Linux Debian x86_64, Windows 10/11 x86_64 y macOS universal solo al empujar un tag `*.*.*` o `v*.*.*`, o al lanzarlo manualmente. Los jobs usan `rust-cache`; `sccache` se evaluó y se descartó por ahora.

## 7. Rendimiento y recursos

La comparación completa entre el prototipo Godot y la aplicación Rust está en [`docs/benchmark-2026-10-28.md`](docs/benchmark-2026-10-28.md).

Resumen del contexto de referencia:

- La aplicación Rust es drásticamente más pequeña y usa menos RAM.
- El arranque a ventana es más rápido.
- La interfaz no consume CPU de render en reposo: repinta bajo demanda.
- WebP, JPG, PNG e ICO son más rápidos en la versión Rust; AVIF usa AOM nativo y queda en paridad práctica con Squoosh.
- Las cifras absolutas de CPU en reposo de Godot están infladas por usar llvmpipe sin VSync; no representan vatios reales en una GPU.

## 8. Límites conocidos

- No se conservan metadatos, pero la orientación EXIF se aplica a los píxeles al cargar.
- Sin recorte ni procesamiento por lotes: fuera del alcance del producto.
- HEIC no está soportado; queda fuera del alcance. AVIF está disponible como salida y JXL se retira del selector.
- No se conservan perfiles ICC.
- La build macOS universal se genera en CI, pero no está firmada ni notarizada; la firma se descartó por falta de presupuesto.
- El resultado ICO usa PNG dentro del contenedor, compatible con Windows moderno.

## 9. Pruebas y verificación

- **62 pruebas Rust**: codificación WebP/JPG/PNG/AVIF con AOM/JPEG XL, JPG progresivo y submuestreo automático, PNG sin pérdida con OxiPNG, WebP lossless/method/alfa, AVIF YUV420/cuantizador/alfa, preprocesado y opciones avanzadas sin persistencia, orientación EXIF y TIFF, compresiones TIFF, entrada TIFF de una página, matriz de entradas PNG/JPG/WebP/AVIF/BMP/TGA/TIFF/SVG, límites y errores, calidad, transparencia, aplanado JPG, ICO, códigos de color, carga SVG, historial, bloqueos, configuración, nombres de exportación, rutas portables, geometría de ventana, selección de formato, contraste, navegación por teclado y zona de agarre del comparador.
- `cargo fmt --all -- --check` limpio.
- `cargo clippy --all-targets -- -D warnings` limpio.
- CI de Godot histórica en el repositorio archivado; CI de Rust con builds Linux, Windows y macOS en este repositorio.
- Exportación Linux verificada bajo Xvfb mediante el diálogo nativo.
- Prueba de arranque, memoria, CPU y escalado de códecs documentada en el informe de benchmark.

Comandos:

```bash
cargo run --release -- /ruta/a/imagen.jpg
cargo test
cargo clippy --all-targets -- -D warnings
./scripts/build_dist.sh all
```

## 10. Tareas y documentación

- `todo.md`: tareas pendientes con identificador `t-N`, título y descripción.
- `t-completadas.md`: tareas cerradas.
- `retomar.md`: contexto para nuevas sesiones.
- `notes/AAAA-MM-DD_asunto.md`: evidencia fechada e inmutable.
- `docs/benchmark-2026-10-28.md`: resultados de rendimiento y metodología.

Al preguntar por las tareas, se debe devolver toda la información de cada tarea pendiente de `todo.md`, sin omitir descripciones.

## 11. Licencia y créditos

- Código: MIT (ver `LICENSE`).
- Tipografía Manrope: SIL Open Font License 1.1 (ver `assets/fonts/OFL.txt`).
- Prototipo Godot histórico: `../Prototipo`.
- Motores y bibliotecas: Godot (histórico), eframe/egui, image, webp, png, avif-decode, zune-jpegxl, jxl-oxide y resvg.
