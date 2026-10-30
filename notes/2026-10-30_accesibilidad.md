# Accesibilidad y navegación por teclado

Fecha: 2026-10-30

## Accesibilidad nativa

- `eframe` compila con la feature `accesskit`.
- Los controles personalizados registran rol, nombre, estado y valor:
  - formatos como grupo de radios;
  - muestras del historial con color, bloqueo y selección;
  - cuentagotas como interruptor;
  - lienzo como slider del divisor (0–100 %), con incremento, decremento y
    valor para lectores de pantalla;
  - avisos como alertas.
- El selector de idioma y el slider de calidad tienen nombre accesible.

## Teclado y foco

- Anillo de foco de acento en controles estándar y personalizados.
- Formatos: flechas para recorrer y seleccionar el grupo.
- Historial: un único tab stop; flechas para navegar, `Enter` para aplicar y
  `L` para bloquear o desbloquear.
- Lienzo enfocado: `←`/`→` mueven el divisor, `Inicio`/`Fin` lo llevan a los
  extremos, `↑`/`↓` hacen paneo vertical y `Shift+←`/`→` paneo horizontal.
- Cuentagotas: se activa con `Tab` + `Enter`; las flechas mueven la muestra
  1 px (`Shift`: 10 px), `Enter` confirma y `Esc` cancela sin perder el foco.
- Los atajos globales existentes se mantienen.

## Contraste

- `TEXT_FAINT` sube a un color que cumple 4,5:1 sobre todos los fondos.
- Se añade `TEXT_DISABLED` para texto deshabilitado.
- Una prueba automática comprueba texto informativo, texto sobre acento y
  anillo de foco.

## Verificación

- `cargo test`: 34 pruebas.
- `cargo clippy --all-targets -- -D warnings` y `cargo fmt --all -- --check`
  limpios.
- Bajo Xvfb se comprobó con teclado: foco visible, selección de formato por
  flechas, bloqueo y selección de historial, activación del cuentagotas,
  muestreo y confirmación de color, y movimiento del divisor con `Inicio`.
- Bajo Xvfb se comprobó también el escalado al 125 % con la ventana mínima.
- Las capturas de documentación ocultan los anillos de foco mediante
  `HUNK_SCREENSHOT`; en la aplicación real se muestran.
**Validación:** usuario, 2026-10-30. La comprobación con lectores concretos
queda recomendada en la validación de la release 1.1.0.
