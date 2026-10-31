# WebP y AVIF avanzados

Fecha: 2026-10-31 · Tarea: `t-40`

> Actualización: ravif se sustituyó por libavif/AOM en `t-44`; esta nota
> conserva la decisión y el benchmark de la pasada anterior.

## Cambios

- WebP usa `WebPConfig` avanzado:
  - lossless;
  - method 0–6;
  - alpha quality 0–100.
- AVIF usa ravif directo:
  - speed 1–10;
  - alpha quality;
  - salida de 8 bits para mantener el pipeline RGBA8.
- `EncodeOptions` incorpora estos campos con valores por defecto seguros.
- Los defaults conservan el benchmark anterior: WebP produce los mismos bytes y
  AVIF mantiene su curva.

## Decisión de backend AVIF

Se mantiene ravif en lugar de integrar libavif/AOM:

- es nativo y no añade otra dependencia C;
- la velocidad medida es competitiva;
- la brecha de tamaño está concentrada en UI y calidad alta;
- el ajuste de speed/alfa ya está disponible para la UI.

Queda pendiente una curva AVIF específica de UI antes de valorar AOM.

## Verificación

- `cargo test`: **51 pruebas**.
- `cargo fmt --all -- --check` y `cargo clippy --all-targets -- -D warnings`
  limpios.
- Nuevas pruebas:
  - WebP lossless conserva todos los píxeles;
  - AVIF con speed 10 y alpha quality 100 produce salida válida y conserva alfa.
- WebP por defecto sigue generando bytes idénticos a Squoosh.

## Pendiente

- Exponer las opciones en la UI y persistirlas (`t-42`).
- Curva AVIF específica de UI.
- Validación visual de la build de Windows en el cierre de release.
