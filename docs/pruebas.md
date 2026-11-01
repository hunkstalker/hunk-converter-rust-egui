# Pruebas y automatización

> Estado: 2026-10-31 · `t-45` implementada, pendiente de revisión con el usuario

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
- preprocesado: rotación, resize con proporción y paleta con alfa;
- expresiones de tamaño `1600x2`, `1600/4`, `1600*2`, `1600-200`, límites y errores;
- proporción enlazada y dimensiones rotadas;
- opciones avanzadas y preprocesado sin persistencia; `config.ini` solo guarda idioma, historial, formato y ventana;
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

Estado actual: **59 pruebas**, incluidas las expresiones de tamaño y la
proporción enlazada. La interacción de arrastre y el marco del canvas se validan
visualmente.

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
