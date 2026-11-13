# Pruebas y automatización

> Estado: 2026-11-02 · `t-45` y `t-46` completadas; pendiente el cierre `t-43`

## 1. Capas

| Capa | Qué cubre | Comando |
| --- | --- | --- |
| Unitaria | Dominio, formatos, configuración, códecs, ICO, exportación, tema y accesibilidad | `cargo test` |
| Integración | Matriz de entradas, alfa, límites y errores controlados | `cargo test` |
| Pack TIFF | Compresiones, gris, alfa, orientación, multipágina y rechazo de 16 bits | `cargo run --release --bin check_tiff -- dist/pruebas-tiff` |
| CI | Formato, clippy y pruebas en `main` y PR | GitHub Actions |

## 2. Matriz de integración

`tests/format_matrix.rs` comprueba:

- carga desde archivo de PNG, JPG, WebP, AVIF, BMP, TGA, TIFF y SVG;
- dimensiones y etiqueta de formato;
- alfa en PNG, WebP y TIFF;
- codificación AVIF con AOM, cuantizador directo y alfa;
- preprocesado: rotación, resize con proporción, paleta con alfa e inversión de color;
- recorte antes de codificar: dimensiones del resultado y píxeles exactos tras decodificar;
- presets de proporción `1:1`, `4:3` y `16:9`, conversión a píxeles y redondeo de bordes;
- colocación de las herramientas y del flyout de recorte según el espacio;
- reserva estable del contenido del canvas vacío según el lado del panel;
- expresiones de tamaño `1600x2`, `1600/4`, `1600*2`, `1600-200`, límites y errores;
- proporción enlazada y dimensiones rotadas;
- perfiles de color `dark`/`light`, búsqueda por identificador y alternancia;
- opciones avanzadas y preprocesado sin persistencia; `config.ini` guarda idioma, perfil de color, historial, formato, panel y ventana;
- rechazo de imágenes de más de 16 384 px por lado;
- rechazo de archivos de más de 200 MB antes de leerlos;
- error controlado con archivos corruptos.

## 3. Pack TIFF

`src/bin/check_tiff.rs` valida el pack de `dist/pruebas-tiff`:

- `foto-rgb-sin-compresion.tiff`;
- `foto-rgb-lzw.tiff`;
- `foto-rgb-deflate.tiff`;
- `foto-rgb-packbits.tiff`;
- `gris-packbits.tiff`;
- `icono-rgba-deflate.tiff`;
- `multipagina.tiff` (solo primera página);
- `orientacion-6.tiff`;
- `rgba-16bits.tiff` (debe rechazarse).

El pack se regenera con el generador local de fixtures; no se versiona para no
añadir binarios de prueba al repositorio.

## 4. Qué sigue siendo manual

- Aspecto visual, espaciado y contraste fuera de las pruebas de tema.
- Capturas de documentación.
- Comportamiento de la ventana nativa en Windows y macOS.
- Codecs que dependen de builds del sistema o de hardware concreto.

La regla es que **ninguna funcionalidad de carga, codificación, configuración o
exportación** dependa de una validación manual. Lo manual queda para lo visual y
lo específico de plataforma.

Estado actual: **73 pruebas**, incluidas las expresiones de tamaño, la
proporción enlazada, la conversión OKLab/OKLCH, el recorte antes de codificar,
los presets de proporción y el copiado completo con Ctrl. La interacción de
arrastre, el marco del canvas y los tiradores del recorte se validan visualmente.

## 5. Comandos

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check

cargo run --release --bin check_tiff -- dist/pruebas-tiff
```

## 6. CI

El workflow ejecuta formato, clippy y pruebas en `main` y pull requests. Las
builds de Linux, Windows y macOS se reservan para tags y ejecución manual.
