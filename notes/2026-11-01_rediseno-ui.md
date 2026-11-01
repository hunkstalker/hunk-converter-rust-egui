# Rediseño de UI según el mockup

Fecha: 2026-11-01 · Referencia: `temp/HUNK converter.png`

## Decisiones

- El panel de ajustes pasa del lateral izquierdo al **lateral derecho**.
- La vista previa ocupa el resto de la ventana, sin caja ni esquinas
  redondeadas alrededor del lienzo; el selector de idioma y `Reiniciar` se
  mueven arriba a la derecha, en la franja libre sobre el panel.
- El panel derecho empieza por debajo de esos controles; no llega hasta el
  borde superior.
- El formato de salida deja de ser un desplegable y pasa a una **cuadrícula de
  tarjetas** de tres columnas. JPEG XL sigue oculto; su hueco lo ocupa `Auto`
  para conservar el modo automático.
- El historial y el cuentagotas dejan el panel y pasan a una **tarjeta flotante**
  abajo a la izquierda del lienzo, con sombra. La tarjeta y los botones rápidos
  quedan por encima del footer, sin taparlo, y con margen respecto al borde
  izquierdo del visor.
- Se retira el anillo de foco del canvas: activar el cuentagotas ya no dibuja
  una caja con borde lima alrededor de la vista previa.
- Se añaden los botones rápidos **Invertir colores** y **Girar 90°** junto a la
  tarjeta de color. Invertir afecta al resultado procesado y se puede deshacer
  volviendo a pulsar; girar usa la rotación existente.
- Se elimina la fila de botones de rotación del bloque Preprocesado: el botón
  rápido `Girar 90°` es el único control de giro.
- La línea de calidad se mantiene tal como estaba (deslizador fino con relleno
  verde), no la variante del mockup.
- Preprocesado y Avanzado siguen en el mismo lugar relativo del panel, debajo de
  la calidad.
- Los pesos original/resultado y el botón de exportar quedan fijos al pie del
  panel derecho. El texto de ayuda de exportación conserva un margen inferior
  visible antes del pie global.
- El pie global se conserva.

## Persistencia

- `config.ini` ya no escribe `[advanced]` ni claves de preprocesado. Las claves
  antiguas se ignoran y esos bloques arrancan con sus valores por defecto.
- El formato de salida, idioma, historial, bloqueos y geometría de ventana sí
  siguen persistiendo.

## Ajustes posteriores

- El porcentaje de zoom solo se pinta en lima al 100 %; en cualquier otra escala
  usa el color atenuado. La rueda y la pinza funcionan en cualquier punto del
  visor, no solo sobre la imagen.
- Los elementos seleccionados de los desplegables (`Nivel`, submuestreo) usan
  texto oscuro sobre el acento para que se lean.
- La exportación abre el explorador nativo para elegir carpeta y nombre, con la
  extensión del formato actual. Ya no fuerza la carpeta `export/`; si existe, se
  propone como destino inicial.
- En formatos sin pérdida el deslizador de calidad permanece visible, atenuado y
  bloqueado.
- Las etiquetas de sección (`IMAGEN`, `FORMATO DE SALIDA`, `CALIDAD`,
  `HISTORIAL`, `CUENTAGOTAS`) suben a 12 px. También pasan a 12 px los badges
  del canvas, toda la línea de pesos y el footer. El tamaño vive en
  `T::SMALL_TEXT_SIZE`.
- `Cambiar` pasa a ser un botón secundario debajo de la información de la
  imagen, con hover y sin estado pulsado, justo encima del separador.
- El botón de exportar incorpora `Ctrl+E` y desaparece el texto de ayuda
  inferior.
- `Vista previa` y el porcentaje de zoom se agrandan.
- La tarjeta de color gana aire: códigos mayores, sin iconos de copia y con
  filas OKLCH/OKLAB antes de HEX. Los valores quedan alineados en una columna.
- La columna flotante ordena borrar, cuentagotas, invertir y girar; el
  cuentagotas usa un icono de pipeta dibujado a mano y mide 38 px.
- Se elimina el texto de ayuda del cuentagotas inactivo. Al hacer clic en un
  código se copia el valor; con Ctrl, HEX copia `HEX#...` y el resto mantiene su
  función CSS completa.
- El aviso de recompresión con pérdida sube a 12 px.
- `Auto` pasa a la primera tarjeta del selector de formato.
- Se elimina la etiqueta de muestra/coordenadas del cuentagotas; queda un swatch
  compacto junto al título y los códigos siguen copiándose al hacer clic.
- El nombre del archivo se limita a dos líneas. Si es extremadamente largo, se
  elide por el centro conservando la cola y la extensión; el nombre completo
  queda en el tooltip.

## Verificación

- Capturas Xvfb a 1280×1000 y 1280×800 con el panel derecho, la cuadrícula de
  formato y la tarjeta flotante.
- Comprobado el cuentagotas sobre el lienzo con la tarjeta flotante encima.
- Comprobados `Invertir colores` y `Girar 90°`.
- Estado vacío sin cabecera, con selector de idioma arriba a la derecha.
- Suite de **62 pruebas**, `cargo fmt --check` y clippy limpios.
