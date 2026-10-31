# Tareas completadas

> Registro histórico. Las tareas cerradas se retiran de `todo.md` y se conservan aquí.
> Los identificadores no se reutilizan.

## t-1 — Fundaciones del producto y marca

**Descripción:** Crear la estructura base, la identidad visual de hunk-converter, el tema oscuro con acento lima, la tipografía incluida, el icono de aplicación y los estados vacío/editor.

**Completada:** 2026-10-28
**Resultado:** Identidad y base del producto, inicialmente materializadas en Godot 4.7.2 con renderizador GL Compatibility, tema generado en código, pantalla de bienvenida ilustrada y estructura de escenas/scripts.

## t-2 — Carga local de imágenes

**Descripción:** Implementar arrastrar y soltar, selector de archivos, validación de formatos y límites, decodificación de PNG/JPG/WebP/BMP/TGA/SVG y preparación de la vista previa.

**Completada:** 2026-10-28
**Resultado:** Carga desde cualquier parte de la ventana, selector nativo y validaciones de archivo, tamaño y dimensiones. La versión Rust conserva esta funcionalidad y añade rasterización SVG con `resvg`.

## t-3 — Motor de conversión y compresión

**Descripción:** Codificar WebP, JPG y PNG con control de calidad para los formatos con pérdida, conservar transparencia cuando el formato lo permite y aplanar JPG sobre blanco de forma predecible.

**Completada:** 2026-10-28
**Resultado:** Motor de codificación separado de la interfaz, con calidad 1–100, WebP/JPG con pérdida, PNG sin pérdida y aplanado JPG sobre blanco. Reimplementado en Rust en la migración.

## t-4 — Comparador visual antes/después

**Descripción:** Construir una superficie de comparación con divisor arrastrable, etiquetas, damero para transparencia y visualización fiel del resultado codificado. Añadir un modo específico para previsualizar iconos ICO.

**Completada:** 2026-10-28
**Resultado:** Comparador con divisor, damero y modo ICO sin divisor. La vista previa se obtiene decodificando la salida real. La versión Rust añade muestreo de color desde el propio lienzo.

## t-5 — Exportación ICO multirresolución

**Descripción:** Implementar el contenedor ICO con frames PNG, selección de tamaños 16/32/48/64/128/256, ajuste centrado sin recorte y validación de que al menos haya un tamaño.

**Completada:** 2026-10-28
**Resultado:** Exportador ICO con cabecera/directorio estándar, frames RGBA validados y prueba de estructura completa, reimplementado en Rust durante la migración.

## t-6 — Panel de ajustes y metadatos

**Descripción:** Diseñar el panel de archivo, formato de salida, calidad/tamaños ICO, pesos finales y porcentaje de ahorro, con exportación fija al pie.

**Completada:** 2026-10-28
**Resultado:** Panel lateral con formatos, calidad, tamaños ICO, metadatos, comparación de pesos y exportación siempre accesible. La versión Rust lo adapta a 390 px.

## t-7 — Flujo portable y carpeta export/

**Descripción:** Guardar los resultados en una carpeta `export/` junto al ejecutable, evitar sobrescrituras, sanear nombres, abrir la carpeta desde la interfaz y generar builds portables.

**Completada:** 2026-10-28
**Resultado:** Guardado directo en `export/` junto al ejecutable, nombres únicos, saneado de nombres, apertura de carpeta y builds Linux/Windows. Validado en el prototipo y de nuevo en la aplicación Rust.

## t-8 — Robustez, accesibilidad y reinicio

**Descripción:** Añadir procesamiento en segundo plano con descarte de resultados obsoletos, manejo de errores, avisos temporales, reinicio completo y atajos de teclado.

**Completada:** 2026-10-28
**Resultado:** Procesamiento con hilos, agrupación de cambios, descarte de resultados obsoletos, avisos y reinicio. La accesibilidad completa se añadió en t-22 para la versión Rust.

## t-9 — Pruebas, documentación y builds iniciales

**Descripción:** Construir una suite headless, documentar el proyecto, generar capturas, preparar integración continua y producir builds portables de Linux y Windows para el prototipo.

**Completada:** 2026-10-28
**Resultado:** Suite Godot de 71 comprobaciones, capturas, documentación viva, CI, y builds portables Linux/Windows. Esta base sirvió para comparar y decidir la migración.

## t-19 — Cuentagotas de color con códigos múltiples

**Descripción:** Añadir un cuentagotas que permita seleccionar píxeles de la imagen cargada desde la vista previa y mostrar sus códigos HEX, RGB, RGBA y HSL, con copia al portapapeles. El modo debe poder activarse y desactivarse, indicar las coordenadas del píxel, dibujar una marca de muestreo y desactivarse cuando no tenga sentido (vista ICO).

**Completada:** 2026-10-28
**Resultado:** Cuentagotas implementado en el panel inferior de la versión Rust con un icono dedicado. Se activa con un clic o arrastrando desde el icono hasta la imagen, muestrea el píxel original, conserva la posición del divisor, muestra coordenadas y los cuatro códigos con copia, dibuja la marca solo mientras está activo y se desactiva en la vista ICO. También se cancela con `Esc`. Verificado con interacción real bajo Xvfb.

## t-20 — Prueba de concepto Rust + egui

**Descripción:** Crear una PoC separada con Rust y egui que reproduzca el flujo principal (carga, formatos, calidad, comparador, ICO y exportación) y permita comparar consumo y rendimiento con el prototipo Godot.

**Completada:** 2026-10-28
**Resultado:** PoC funcional con 7 pruebas, capturas y benchmark. El ejecutable Linux midió 7,5 MiB frente a 70,7 MiB de Godot, abrió la ventana alrededor de un 28 % antes y redujo la RAM en reposo a la mitad. La comparación motivó la migración definitiva.

## t-21 — Migración a Rust + egui como aplicación principal

**Descripción:** Convertir el piloto en la aplicación principal, completar la paridad imprescindible del producto, reemplazar el selector de color de fondo por el cuentagotas, añadir soporte SVG, renombrar la aplicación a hunk-converter 0.2.0, empaquetar Linux y Windows, actualizar documentación y archivar el prototipo Godot.

**Completada:** 2026-10-28
**Resultado:** Aplicación principal en Rust con egui; UI event-driven sin repintado continuo, comparador, ICO, exportación portable, cuentagotas y SVG. Suite de 13 pruebas Rust, clippy y formato limpios. Builds portables Linux y Windows generadas. El prototipo Godot queda archivado como referencia. La build Windows se validó posteriormente en Windows 10/11 en t-23.

## t-25 — Zoom con rueda y paneo del lienzo

**Descripción:** Añadir zoom con la rueda del ratón sobre la imagen, anclado al punto bajo el cursor, y desplazamiento con clic derecho cuando la imagen está ampliada. El divisor, su zona de agarre y el cuentagotas deben seguir funcionando correctamente, y el porcentaje de zoom debe poder restablecerse.

**Completada:** 2026-10-29
**Resultado:** Zoom de 100 % a 800 % con la rueda, anclado al cursor y limitado para no perder la imagen. Paneo con clic derecho y cursor de agarre. El porcentaje aparece en la cabecera y se puede pulsar para volver al ajuste inicial. El divisor, el cuentagotas y la marca de muestreo se adaptan al zoom y al desplazamiento. Verificado con rueda y arrastre reales bajo Xvfb.

## t-26 — Salida AVIF y JPEG XL

**Descripción:** Añadir AVIF y JPEG XL como formatos de salida, con vista previa fiel del archivo codificado, control de calidad cuando corresponda, exportación portable y soporte en las builds Linux y Windows.

**Completada:** 2026-10-29
**Resultado:** AVIF con calidad 1–100 y JPEG XL sin pérdida. PNG y JPEG XL no muestran deslizador porque no modifican la calidad. Las vistas previas se decodifican con `avif-decode` y `jxl-oxide`. Se reorganizó la rejilla de formatos y se corrigieron los badges del comparador. Verificado con pruebas y exportación real AVIF/JPEG XL.

## t-14 — Interfaz en inglés

**Descripción:** Internacionalizar los textos de interfaz, avisos y tooltips, con español como idioma predeterminado e inglés como alternativa. Incluir un selector de idioma y recordar la preferencia.

**Completada:** 2026-10-29
**Resultado:** Interfaz traducida a español, catalán e inglés. Selector en la cabecera, detección del idioma del sistema en el primer arranque y persistencia en `config.ini`.

## t-27 — Historial de colores del cuentagotas

**Descripción:** Añadir un historial de 21 colores integrado en la tarjeta del cuentagotas, con selección por clic izquierdo, bloqueo por clic derecho, limpieza de colores no fijados y persistencia en `config.ini`. Al llenarse, sustituir el color no fijado más antiguo y avisar si todos están fijados.

**Completada:** 2026-10-29
**Resultado:** Historial de 21 colores en tres filas, con candado visible, anillo de selección, papelera para limpiar no fijados y avisos de historial lleno. Verificado con muestreo real, bloqueo, selección y persistencia en config.ini.

## t-28 — Marca HUNK Converter e icono de máscara de gas

**Descripción:** Cambiar el nombre visible a HUNK Converter, el subtítulo a IMAGE TOOLBOX, actualizar el pie de versión y crear un icono original de máscara de gas.

**Completada:** 2026-10-29
**Resultado:** Cabecera «HUNK Converter / IMAGE TOOLBOX», pie «Rust + egui · Open Source · v0.2.0», icono SVG original de máscara de gas rasterizado a PNG y usado en la ventana y la cabecera.

## t-29 — Ventana maximizada persistente

**Descripción:** Abrir la aplicación con la ventana maximizada por defecto, permitir maximizar o restaurar con un botón y guardar el estado en `config.ini`.

**Completada:** 2026-10-29
**Resultado:** La app arranca con la ventana maximizada salvo que `config.ini` indique lo contrario. Los controles nativos de la ventana actualizan el estado guardado. En desarrollo, `HUNK_WINDOWED=1` fuerza una ventana normal.

## t-30 — Pulido de cabecera, tarjeta de color y pie

**Descripción:** Quitar el botón propio de maximizar/restaurar y dejar los controles nativos de la ventana, reservar el espacio de coordenadas y códigos en la tarjeta del cuentagotas para que no cambie de altura al muestrear, y añadir un enlace al repositorio en el pie que abra el navegador.

**Completada:** 2026-10-29
**Resultado:** La cabecera conserva solo el selector de idioma y el reinicio. La tarjeta reserva siempre la línea de coordenadas y los códigos mantienen una fila fija, por lo que muestrear o elegir un color no desplaza el panel. El separador entre el historial y el cuentagotas tiene 10 px de margen por cada lado. El pie enlaza `github.com/hunkstalker/hunk-converter` y lo abre en el navegador. Verificado con capturas comparadas y un navegador simulado.

## t-31 — Refactorización de arquitectura y bloqueos persistentes

**Descripción:** Sustituir el `app.rs` monolítico por un monolito modular con dominio puro, acciones explícitas de UI, errores tipados y persistencia verificada de los colores bloqueados en `config.ini`.

**Completada:** 2026-10-29
**Resultado:** `app.rs` pasa de más de 2.300 líneas a un shell de ~340 líneas y componentes en `app/ui/`. La lógica de formatos, color, historial, nombres de exportación y errores vive en `domain/`. Los códecs se separan en `codec/` y la configuración en `settings/`. La UI devuelve `AppAction` para workers, disco, portapapeles y configuración. Los bloqueos del historial se guardan con `colorN=#RRGGBBAA|1` y cuentan con pruebas de ida y vuelta. Documentado en `docs/arquitectura.md`.

## t-32 — CI multiplataforma y release 1.0.0

**Descripción:** Preparar GitHub Actions para compilar Linux Debian x86_64, Windows 10/11 x86_64 y macOS universal, publicar los artefactos con checksums al empujar un tag y dejar la versión 1.0.0 lista para su release.

**Completada:** 2026-10-29
**Resultado:** `release.yml` ejecuta pruebas, compila las tres plataformas, genera `SHA256SUMS.txt` y crea la GitHub Release en tags `*.*.*` o `v*.*.*`. `build_dist.sh` incorpora el preset macOS universal. Versión, documentación y capturas actualizadas a 1.0.0. Tag local `v1.0.0` preparado sobre el commit final.

## t-33 — Iconos nativos de plataforma

**Descripción:** Integrar el icono de HUNK Converter en el ejecutable de Windows, en el paquete `.app` de macOS y en el lanzador de Linux, además del icono de ventana existente.

**Completada:** 2026-10-29
**Resultado:** `winresource` incrusta `icon.ico` en el `.exe` MSVC. `make_icons` genera `icon_256.png`, `icon.ico` e `icon.icns`; `build_dist.sh macos` empaqueta `HUNK Converter.app` con `Info.plist` e icono. La build Linux incluye `hunk-converter.desktop` y `hunk-converter.png`. El icono de ventana sigue cargándose en tiempo de ejecución.

## t-10 — Respetar la orientación EXIF

**Descripción:** Leer la etiqueta `Orientation` de las imágenes JPEG/WebP que la incluyan y aplicar la rotación o el volteo correspondiente antes de mostrar, comparar, muestrear y exportar. Añadir pruebas con imágenes de orientación 1, 3, 6 y 8 para evitar fotos de móvil giradas.

**Completada:** 2026-10-29
**Resultado:** `codec::decode_raster` lee la orientación con `ImageDecoder::orientation` y la aplica con `DynamicImage::apply_orientation` antes de generar el RGBA. Las dimensiones se validan después del giro, por lo que los formatos de 90° y 270° intercambian ancho y alto correctamente. Pruebas de integración con JPEG y EXIF para orientaciones 1, 3, 6 y 8. La salida sigue sin conservar metadatos, pero los píxeles ya salen orientados.

## t-13 — CI y publicación de la release

**Descripción:** Extender la integración continua para ejecutar la suite, compilar las builds portables y publicar artefactos con checksums al crear una etiqueta de versión. Mantener la build sin instalador y la carpeta `export/` junto al ejecutable.

**Completada:** 2026-10-29
**Resultado:** Workflow único `.github/workflows/ci.yml` con pruebas, builds Linux/Windows/macOS, checksums y GitHub Release en tags. NASM instalado en el job de pruebas y acciones actualizadas a Node 24. La release [v1.0.0](https://github.com/hunkstalker/hunk-converter/releases/tag/v1.0.0) se publicó correctamente con los tres paquetes y `SHA256SUMS.txt`.

## t-23 — Validar la build Windows 10/11 de la release

**Descripción:** Ejecutar el `.zip` portable de la release en Windows 10 y 11 y comprobar arranque, high DPI, carga por arrastre, diálogo nativo, exportación a `export/`, cierre limpio e imports del sistema. Incluye la verificación visual del icono en Explorer, barra de tareas y accesos directos.

**Completada:** 2026-10-29
**Resultado:** Release validada en Windows 10/11: arranque, DPI, arrastre, diálogo nativo, exportación y cierre correctos. El ejecutable muestra el icono de HUNK Converter en Explorer y barra de tareas. La comprobación absorbe la antigua `t-24`.

## Tareas descartadas o absorbidas

### t-17 — Firma y notarización de macOS (descartada)

**Motivo:** no hay presupuesto para certificado y notarización. La build universal `arm64 + x86_64` se distribuye como `HUNK Converter.app` sin firmar, y así queda documentado.

### t-24 — Verificación visual del icono Windows (absorbida por t-23)

**Motivo:** el icono ya se incrusta en la build MSVC y la comprobación visual forma parte de la validación completa de Windows de `t-23`. No se mantiene como tarea independiente.

### t-11 — Redimensionado y encuadre (descartada)

**Motivo:** fuera del alcance del producto. HUNK Converter está orientado a conversión, compresión y comparación de una imagen, no a edición geométrica.

### t-12 — Conversión por lotes (descartada)

**Motivo:** incompatible con el flujo principal de comparación y control de calidad por imagen. Procesar lotes con una calidad común ignora la inspección individual que define la aplicación.

### t-15 — Entrada HEIC y códecs opcionales (descartada)

**Motivo:** HEIC no es un objetivo del producto, orientado a desarrollo web. Añadir un decodificador HEVC implicaría dependencias pesadas, problemas de portabilidad y riesgo de patentes, sin aportar valor al flujo actual.

## t-16 — Navegación avanzada del lienzo

**Descripción:** Completar la navegación con un modo 1:1, zoom mediante rueda, pinza y atajos de encuadre (ajustar, restablecer). El divisor, su círculo y el cuentagotas ya funcionan a cualquier zoom; el encuadre debe conservarse al cambiar de formato.

**Completada:** 2026-10-29
**Resultado:** Navegación con rueda, pinza, paneo con clic derecho y atajos (`0`/`F` ajustar, `1` tamaño real, `+`/`-` pasos de zoom), sin botones en la cabecera. La imagen se abre al 100 %. El divisor llega a los bordes superior e inferior y el porcentaje de zoom queda centrado con “Vista previa”. La posición y el tamaño de la ventana se persisten en `config.ini`. Suite ampliada a 31 pruebas y capturas de documentación regeneradas.

## t-18 — Exportación segura a `export/`

**Descripción:** Mantener `export/` junto al ejecutable como destino fijo, sin selector de ubicación. Evitar sobrescrituras mediante creación atómica de archivos nuevos con sufijos `-2`, `-3`, etc., y avisar si la carpeta no se puede crear o escribir.

**Completada:** 2026-10-29
**Resultado:** Exportación fija a `export/`, sin selector de carpeta. El archivo se crea con `OpenOptions::create_new(true)`, de modo que comprobación y creación son atómicas; ante una colisión se prueban `nombre-2.ext`, `nombre-3.ext`, etc. Si la escritura falla, se elimina el archivo parcial. Validado con una exportación real que dejó intacto un `demo.webp` preexistente y creó `demo-2.webp`.

## t-22 — Accesibilidad y navegación completa por teclado

**Descripción:** Incorporar soporte de accesibilidad nativa de egui/AccessKit, orden de foco explícito, nombres y descripciones para lector de pantalla, navegación completa sin ratón y pruebas de contraste y escalado de fuente. El cuentagotas y el comparador deben poder usarse con teclado.

**Completada:** 2026-10-30
**Resultado:** AccessKit activo; roles, nombres, estados y valores en controles estándar y personalizados; avisos como alertas. Foco visible, formatos con navegación por flechas, historial con tab stop único, Enter y L, comparador como slider con flechas/Inicio/Fin y paneo, y cuentagotas por teclado con flechas/Shift/Enter/Esc. Contraste WCAG AA con prueba automática. Suite ampliada a 34 pruebas y capturas regeneradas.

## t-34 — Optimizar el CI para 1.1.0

**Descripción:** Limitar los builds de Linux, Windows y macOS a tags y ejecución manual, dejando `main` y las pull requests solo con pruebas. Añadir caché de Cargo (`Swatinem/rust-cache`) y evaluar `sccache` para las dependencias C/C++. Mantener el workflow único y la release automática en tags.

**Completada:** 2026-10-30
**Resultado:** Builds limitados a tags y `workflow_dispatch`; `main` y PR solo ejecutan formato, clippy y pruebas. `rust-cache` añadido a pruebas y builds. `sccache` evaluado y descartado por solapamiento con la caché de `target` y por no cubrir NASM. Workflow único y release automática en tags.

## t-35 — Selector de salida con modo automático

**Descripción:** Sustituir la rejilla de tarjetas por un desplegable compacto, añadir `Automático` para resolver el formato de salida desde la entrada y usar WebP como fallback, mantener la selección fija entre imágenes, avisar de la recompresión en JPG/WebP y persistir la elección en `config.ini`.

**Completada:** 2026-10-31
**Resultado:** Desplegable compacto con `Automático · formato` y selección fija; PNG/JPG/WebP resuelven al mismo formato y BMP/SVG usan WebP. La elección se guarda en `[output] format`, el reinicio vuelve a `Auto` y el aviso de recompresión aparece con entradas JPG/WebP. El selector permanece dentro del scroll, el menú muestra las siete opciones completas y el panel recupera la geometría de `HEAD` (x=30..389, hasta el footer). Suite de 36 pruebas, capturas regeneradas y builds Linux/Windows GNU actualizadas.

## t-36 — Entrada TIFF

**Descripción:** Abrir `.tif` y `.tiff`, limitados a la primera página y a RGB/RGBA/gris de 8 bits, con compresiones habituales, orientación y límites compartidos. La salida TIFF queda fuera del alcance web de 1.1.

**Completada:** 2026-10-31
**Resultado:** Entrada TIFF con detección por magic bytes y extensión, primera página, RGB/RGBA/gris/gris con alfa de 8 bits, LZW/Deflate/PackBits/sin compresión, orientación y rechazo controlado de 16 bits y CMYK. `Automático` usa WebP como fallback. Se añadieron pruebas unitarias de compresiones, predictor, alfa, orientación, multipágina y rechazo de 16 bits; matriz de integración de entradas PNG/JPG/WebP/BMP/TGA/TIFF/SVG; límites y archivos corruptos; y el CLI `check_tiff` con pack de validación. Suite de 47 pruebas.

## t-37 — Entrada AVIF

**Descripción:** Abrir AVIF con `avif-decode`, detectar por cabecera y extensión, cargar la primera imagen, conservar alfa y respetar los límites.

**Completada:** 2026-10-31
**Resultado:** AVIF se carga desde archivo, se valida su tamaño con `avif-parse` antes de decodificar y se convierte a RGBA8 conservando alfa. Se añadió al filtro y a la matriz de integración; `Automático` resuelve AVIF → AVIF. JPEG XL queda fuera del alcance web.

## t-39 — Paridad JPEG y PNG

**Descripción:** Sustituir los encoders JPEG/PNG por MozJPEG y OxiPNG, mantener alfa y límites, y verificar la paridad de tamaño y velocidad frente a Squoosh.

**Completada:** 2026-10-31
**Resultado:** JPG usa MozJPEG-rs con progresivo y submuestreo automático (4:2:0 <80, 4:2:2 80–89, 4:4:4 ≥90). PNG usa OxiPNG nivel 2. El benchmark queda en paridad práctica: JPEG +0,4 % mediana a q75 y +0,3 % a q90, con 2,5× más velocidad; PNG −0,8 % mediana y 3,2× más velocidad. Suite de 49 pruebas con progresivo, submuestreo y píxeles PNG exactos.
