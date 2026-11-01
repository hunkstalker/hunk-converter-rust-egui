# Retomar HUNK Converter

> Punto de entrada para nuevas sesiones. Última actualización: **2026-11-01**.
> Versión: **1.0.0** publicada · **1.1.0** pendiente de cierre `t-43` · Rama: **main** · Estado: **aplicación principal Rust + egui, portable**.

## 1. Qué es este proyecto

**HUNK Converter** es una aplicación de escritorio local-first para convertir, comprimir, comparar y exportar imágenes. Está hecha en **Rust + egui**, usa el renderizador **glow** y no requiere instalador.

Puede exportar WebP, AVIF, JPG, PNG e ICO. La vista previa compara el original con el archivo codificado real. Incluye un cuentagotas que obtiene colores de la imagen y muestra sus códigos OKLCH/OKLAB/HEX/RGB/RGBA/HSL. Al exportar se abre el explorador nativo para elegir la ruta.

Ruta principal: la raíz de este repositorio.

El prototipo Godot original está archivado en la carpeta hermana `../Prototipo/`. No se debe seguir desarrollando allí.

## 2. Estado actual

- Interfaz en español, catalán e inglés, con detección del idioma del sistema y tema oscuro propio.
- Carga por arrastre, selector nativo, argumento de línea de comandos o `Ctrl+V` desde el portapapeles.
- Entrada: PNG, JPG, WebP, AVIF, BMP, TGA, TIFF y SVG. La orientación EXIF de JPG/WebP y la orientación TIFF se aplican al cargar. TIFF se limita a la primera página y a 8 bits RGB/RGBA/gris.
- Salida de 1.1: WebP, AVIF, JPG, PNG e ICO. JPEG XL sale del selector.
- Selector de salida en cuadrícula de tarjetas con modo Automático implementado y validado.
- Objetivo 1.1: salida web JPEG/PNG/WebP/AVIF + ICO; entrada amplia JPEG/PNG/WebP/AVIF/SVG/BMP/TGA/TIFF; preprocesado de redimensionado, rotación y reducción de paleta. TIFF de entrada forma parte de 1.1. Ver `docs/objetivo-1.1-web.md`.
- Camino único de 1.1.0: **bloque 1** entrada TIFF/AVIF + benchmark; **bloque 2** paridad con Squoosh (MozJPEG, OxiPNG, WebP/AVIF avanzados, preprocesado y UI). Sin release intermedia.
- Benchmark HUNK vs Squoosh: WebP empata, JPEG queda en paridad práctica (2,5× más rápido), PNG comprime igual o mejor (3,2× más rápido) y AVIF queda en paridad práctica (0,994× mediana a igual SSIM, 1,82× más rápido en UI). Ver `docs/benchmark-squoosh-2026-10-31.md`.
- Preprocesado: resize Lanczos3 con proporción enlazada, marco del original en el canvas, rotación 90/180/270 y paleta 2–256 colores con alfa. Los campos aceptan arrastre vertical y expresiones como `1600/4`; el cálculo corre en un worker con debounce.
- Opciones avanzadas implementadas sin persistencia: JPG progresivo/submuestreo, PNG nivel/interlazado, WebP lossless/method/alfa, AVIF speed/alfa y preprocesado. Se reinician en cada sesión. Resize/UI validados en la revisión incremental.
- Calidad 1–100 para WebP/JPG/AVIF; PNG sin pérdida; JPG usa MozJPEG y aplana sobre blanco; PNG usa OxiPNG; AVIF usa libavif/AOM.
- Comparador visual con divisor, damero, zoom continuo (rueda, pinza o atajos), modo 1:1, ajuste a ventana y paneo con clic derecho. La imagen se abre al 100 %, salvo con resize activo, que encuadra el original para mostrar el marco.
- Recorte no destructivo implementado y validado con máscara normalizada, ocho tiradores, presets `Libre`/`Original`/`1:1`/`4:3`/`16:9`, exterior atenuado y comparador limitado al recorte. Se aplica antes de codificar, se quita con `Quitar recorte` y no se persiste. Con recorte aplicado, `0`/`F` encuadra el recorte y `1` lo centra a escala real. En ICO el botón se desactiva; el recorte aplicado se conserva al cambiar de formato.
- Cuentagotas con muestra del píxel original y códigos OKLCH/OKLAB/HEX/RGB/RGBA/HSL. El código se copia al hacer clic; con Ctrl, HEX añade la etiqueta.
- Historial de 21 colores en tres filas: selección con clic izquierdo, bloqueo con clic derecho y limpieza de no fijados. Los bloqueos persisten entre sesiones.
- Arquitectura modular: dominio puro, acciones explícitas, errores tipados y UI dividida por componentes. Ver `docs/arquitectura.md`.
- Idioma, historial, bloqueos, estado maximizado, formato de salida, posición y tamaño de la ventana se guardan en `config.ini`, junto al ejecutable. Las opciones de Preprocesado y Avanzado no se guardan y vuelven a sus valores por defecto al reiniciar.
- La aplicación se abre con la ventana maximizada; los controles nativos permiten maximizar o restaurar.
- Icono de máscara de gas y marca **HUNK Converter / IMAGE TOOLBOX**.
- Pie con enlace directo a `github.com/hunkstalker/hunk-converter-rust-egui`, abierto en el navegador.
- Exportación mediante diálogo nativo, con nombre saneado y confirmación de sobrescritura del sistema.
- Procesamiento en hilo aparte con debounce y descarte de resultados obsoletos.
- Reinicio, atajos y avisos de error.
- **Render bajo demanda:** sin repintado de UI cuando la ventana está quieta.
- **Accesibilidad:** AccessKit activo, foco visible, navegación completa por teclado y contraste WCAG AA.
- Builds portables de Linux, Windows 10/11 y macOS universal; assets y fuente embebidos.
- **70 pruebas Rust** (unitarias e integración), clippy y formato limpios. Ver `docs/pruebas.md`.
- CI de Rust en GitHub Actions: pruebas en `main` y PR, builds solo en tags o ejecución manual, caché de Cargo y release automática al empujar un tag.
- Versión **1.0.0** publicada; **1.1.0** pendiente de cerrar `t-43` (benchmark final, CI y release). No etiquetar todavía.
- Benchmark comparativo completo en `docs/benchmark-2026-10-28.md`.

## 3. Comandos

```bash
# Entorno local o contenedor (ver docs/desarrollo.md)
./scripts/setup_dev.sh --install
docker compose run --rm dev cargo test

cargo run --release -- /ruta/a/imagen.jpg
cargo run --release

cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check

# Demo y capturas
cargo run --release --bin make_demo -- muestra.png 4000 3000
cargo run --release --bin bench -- muestra.png
HUNK_DEMO=1 HUNK_SCREENSHOT=docs/hunk-converter-editor.png \
  cargo run --release
```

Builds portables:

```bash
./scripts/build_dist.sh linux
./scripts/build_dist.sh windows
./scripts/build_dist.sh macos
./scripts/build_dist.sh all
```

- Linux usa las bibliotecas gráficas del sistema (X11/OpenGL).
- Windows se compila con MSVC en CI y por defecto con `x86_64-pc-windows-gnu` en local; requiere toolchain/linker de Windows.
- macOS produce un binario universal `arm64 + x86_64`; no está firmado ni notarizado.
- La build Windows de la release está validada en Windows 10/11, con icono correcto en Explorer y barra de tareas.
- AVIF compila AOM de forma estática: requiere CMake, NASM y compilador de C; no añade DLLs de runtime.

## 4. Decisiones que no se deben romper

- **Portapapeles:** `Ctrl+V` intenta leer una imagen del portapapeles y sustituye la imagen actual; si no hay imagen, no hace nada.
- **Idioma:** `config.ini` guarda el idioma (es/ca/en), el historial, el estado maximizado y la geometría de la ventana; en el primer arranque se detecta el idioma del sistema.
- **Historial:** 21 colores máximo; al llenarse se sustituye el no fijado más antiguo. Los fijados nunca se reemplazan. Si todos están fijados, el color nuevo no se guarda y se avisa. El bloqueo se persiste en `config.ini` con `colorN=#RRGGBBAA|1`.
- **Arquitectura:** `domain` no conoce egui ni ficheros; `ui` devuelve `AppAction` para los efectos de aplicación; `app` es el único que toca disco, workers y configuración. No introducir Clean Architecture completa ni un trait por códec.
- **Errores:** usar enums tipados y `i18n::present_error`; no volver a `Result<_, String>` ni a comparar mensajes en español.
- **Parámetros:** máximo 3 además de `self`; agrupar en DTOs cuando haga falta.
- **Cuentagotas:** el clic derecho sobre la imagen lo desactiva. Los códigos de color se copian haciendo clic sobre ellos y muestran un aviso temporal.
- **Marca:** el nombre visible es **HUNK Converter**; el nombre técnico del binario sigue siendo `hunk-converter`.
- **Salida portable:** diálogo nativo; si existe, se propone `<carpeta del ejecutable>/export/`.
- **Sobrescritura:** la confirma el diálogo nativo del sistema.
- **Vista previa fiel:** decodificar el buffer final, no estimar.
- **Cuentagotas:** icono de pipeta; un clic activa o desactiva el modo y también se puede arrastrar desde el icono hasta la imagen; muestrea el píxel original; no mueve el divisor; la marca solo se ve mientras está activo; se cancela con `Esc`; se desactiva en ICO; limpia la muestra al cambiar de imagen o reiniciar.
- **Comparador:** el cursor de redimensionado solo aparece sobre la línea o el círculo del divisor. La rueda hace zoom (100–800 %) y el clic derecho desplaza la imagen cuando está ampliada.
- **Vista:** el zoom se reinicia al cargar otra imagen o reiniciar; el encuadre se conserva al cambiar de formato.
- **JPG:** transparencia compuesta sobre blanco.
- **ICO:** frames PNG, contenedor manual, ajuste centrado sin recortar la imagen (independiente del recorte no destructivo), al menos un tamaño.
- **Calidad inicial:** 82 %.
- **PNG:** sin pérdida y con el deslizador de calidad deshabilitado.
- **AVIF:** salida con pérdida mediante libavif/AOM, cuantizador directo y control de calidad; vista previa decodificada con `avif-decode`.
- **Badges del comparador:** el formato de salida va a la izquierda y `ORIGINAL` a la derecha, igual que las mitades visibles.
- **Reinicio:** restablece formato, calidad, tamaños y muestra de color.
- **Privacidad:** sin red, cuentas ni telemetría.
- **Render:** no añadir animaciones permanentes ni bucles de repintado innecesarios. Cualquier animación debe justificar sus frames.
- **Tema:** todo el estilo vive en `src/theme.rs`; mantener la paleta y jerarquía.

## 5. Estructura

```text
src/main.rs                  eframe, ventana e icono
src/app/                     Shell, acciones, estado y componentes egui
src/app/ui/                  Panel derecho, color flotante, canvas, avisos
src/domain/                  Formatos, color, historial, exportación y errores
src/codec/                   WebP, JPG, PNG, AVIF, JPEG XL, ICO, SVG y TIFF
src/settings/                config.ini, idioma, historial, formato y maximizado
src/i18n.rs                  Traducciones es/ca/en y errores tipados
src/demo.rs                  Imagen sintética para demos/capturas
src/theme.rs                 Tema egui
src/bin/bench.rs             Benchmark de códecs
src/bin/make_demo.rs         Generador de muestra
src/bin/make_icons.rs        Genera PNG, ICO e ICNS desde el icono fuente
scripts/build_dist.sh        Empaquetado portable Linux/Windows/macOS
assets/                      Manrope, iconos y licencias
docs/                        Capturas, benchmark, arquitectura, formatos y plan
notes/                       Evidencia fechada
proyecto.md                 Documento de producto
todo.md                     Tareas pendientes
t-completadas.md            Tareas cerradas
```

## 6. Tareas

- Pendientes: `t-43` (benchmark final, validación del CI y release).
- Cerradas: `t-completadas.md` (t-1 a t-10, t-13, t-14, t-16, t-18, t-19, t-20, t-21, t-22, t-23, t-25, t-26, t-27, t-28, t-29, t-30, t-31, t-32, t-33, t-34, t-35, t-36, t-37, t-39, t-40, t-41, t-42, t-44, t-45 y t-46).
- Evidencia: todas las notas de `notes/`, incluida la migración.
- **Regla:** si el usuario pregunta qué tareas hay, devolver toda la información de cada tarea pendiente de `todo.md`: identificador `t-N`, título y descripción completa. Al cerrar una tarea, moverla a `t-completadas.md` y actualizar este archivo.

Siguiente paso: **cerrar `t-43`**: repetir el benchmark final, validar los artefactos MSVC/macOS del CI y preparar el push/tag. No etiquetar v1.1.0 hasta completar ese cierre.

## 7. Benchmarks y límites

Los informes completos están en `docs/benchmark-2026-10-28.md` y
`docs/benchmark-squoosh-2026-10-31.md`.

- La versión Rust es mucho más pequeña que el prototipo Godot, arranca antes, usa menos RAM y es más rápida codificando.
- Frente a Squoosh, WebP empata en tamaño y es 2,3–2,8× más rápido; JPEG queda en paridad práctica y 2,5× más rápido; PNG comprime igual o mejor y 3,2× más rápido; AVIF queda en paridad práctica (0,994× mediana a igual SSIM) y es 1,82× más rápido en UI.
- La CPU en reposo del prototipo Godot medida bajo llvmpipe no representa un equipo con GPU, pero sí evidencia el repintado continuo.
- El benchmark frente a Squoosh usa SSIMULACRA2 y mide WebP, JPEG, PNG y AVIF; el icono se reserva para tamaño, alfa y velocidad.

Pendientes conocidos:

- Firma y notarización de macOS descartada; la build universal se distribuye sin firmar.

## 8. Flujo de trabajo para la próxima sesión

1. Leer `retomar.md`, `proyecto.md` y `todo.md`.
2. Revisar `git status`, `notes/` y `docs/benchmark-2026-10-28.md`.
3. Ejecutar `cargo test` antes de tocar código.
4. Implementar una tarea pendiente.
5. Añadir pruebas y actualizar capturas si cambia la interfaz.
6. Mover la tarea a `t-completadas.md` y actualizar proyecto/retomar.
7. Crear una nota fechada en `notes/` con decisiones y verificación.
8. Hacer commit con un mensaje claro.
