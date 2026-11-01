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
- El pie global se conserva con la identidad del producto y la versión:
  `HUNK Converter · v1.1.0`.

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
- El botón de exportar incorpora `Ctrl+S` (con `Ctrl+E` como alias) y desaparece el texto de ayuda
  inferior.
- `Vista previa` y el porcentaje de zoom se agrandan.
- La tarjeta de color gana aire: códigos mayores, sin iconos de copia y con
  filas OKLCH/OKLAB antes de HEX. Los valores quedan alineados en una columna.
- La columna flotante ordena borrar, recortar, cuentagotas, invertir y girar; el
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

## Recorte no destructivo

- El estado vive como `Option<CropRect>` en coordenadas normalizadas; la imagen
  de origen nunca se recorta ni se reemplaza. La codificación y la exportación
  recortan los píxeles justo antes de aplicar la paleta y el encoder.
- Edición: se muestra la imagen completa, la máscara con ocho tiradores, la
  cuadrícula de tercios y el exterior atenuado. El resultado y el divisor se
  ocultan mientras se edita.
- Aplicado: el comparador se limita a la zona recortada. Dentro se pinta el
  resultado real a la izquierda del divisor y el original a la derecha; fuera se
  mantiene la imagen completa atenuada. El divisor y sus etiquetas solo ocupan
  el recorte.
- Presets en un desplegable a la derecha del botón: `Libre`, `Original`, `1:1`,
  `4:3` y `16:9`. Los presets tienen en cuenta la proporción de la fuente;
  arrastrar un tirador cambia el preset a `Libre`. Arrastrar dentro del marco lo
  mueve sin redimensionarlo.
- `Aplicar`/`Enter` confirma; `Cancelar`/`Esc` descarta. Tras aplicar,
  `Quitar recorte` vuelve a la imagen completa sin tocar el original.
- El recorte se limpia al cargar otra imagen, reiniciar o girar 90°. No se
  persiste en `config.ini`.
- En ICO el botón se desactiva y un borrador abierto se cancela, pero un recorte
  ya aplicado se conserva; se puede quitar al volver a otro formato.
- Con recorte aplicado, `0`/`F` encuadra el recorte y `1` lo centra a escala
  real. La rueda, el paneo y el porcentaje de zoom usan el recorte como
  contenido cuando no se está editando. Al entrar o salir de la edición se
  conserva la escala y el centro del recorte si la vista estaba ampliada; en
  modo ajuste se reencuadra el nuevo contenido.

## Verificación

- Capturas Xvfb a 1280×1000 y 1280×800 con el panel derecho, la cuadrícula de
  formato y la tarjeta flotante.
- Comprobado el cuentagotas sobre el lienzo con la tarjeta flotante encima.
- Comprobados `Invertir colores` y `Girar 90°`.
- Recorte comprobado con presets `1:1`, `4:3` y `16:9`, arrastre de tiradores y
  movimiento del marco, `Aplicar`, `Quitar recorte`, `Enter`, `Esc`, el botón
  `Cancelar` y el cambio a ICO durante la edición.
- Comprobados `0`/`F` y `1` con recorte aplicado, el reencuadre al aplicar y al
  entrar en edición, el zoom con rueda sobre el recorte y el tooltip único de
  `Recortar`.
- Estado vacío sin cabecera, con selector de idioma arriba a la derecha.
- Suite de **68 pruebas**, `cargo fmt --check` y clippy limpios.
- Validación visual e interactiva completada; los cambios se incluyen en el
  commit de la revisión de UI y recorte. La release sigue sin etiquetar.
