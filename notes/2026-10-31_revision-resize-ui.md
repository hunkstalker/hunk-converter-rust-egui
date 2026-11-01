# Revisión de RESIZE y UI — primera pasada

Fecha: 2026-10-31 · Estado: implementada; pendiente de revisión con el usuario

## 1. Proporción no sincroniza los dígitos

**Reproducción:** con `Mantener proporción` activo, cambiar el Ancho. La imagen
se redimensiona correctamente, pero el valor visible del Alto no cambia hasta
que se recalcula o se vuelve a editar. Lo mismo ocurre al revés.

**Causa probable:** `keep_aspect` solo se aplica en `apply_geometry`; los campos
`width` y `height` de la UI siguen siendo independientes.

**Propuesta:**

- Al editar Ancho con proporción activa, recalcular y escribir el Alto en el
  mismo frame, usando el aspect ratio de la imagen original.
- Al editar Alto, recalcular el Ancho.
- Redondear de forma consistente y evitar cadenas de recálculo.
- Test de UI/estado que compruebe ambos campos, no solo las dimensiones del
  resultado.

## 1.1. Referencia del tamaño original en el canvas

**Decisión:** no añadir más texto a la barra lateral. La referencia será un
marco visual en el canvas que represente el tamaño original frente al tamaño de
trabajo actual.

**Propuesta:**

- Dibujar un marco fantasma con la extensión del original.
- Centrarlo respecto al encuadre actual y recortarlo contra el canvas.
- Sin etiqueta de texto; el marco debe explicarse por sí mismo.
- Si no hay resize, no dibujar el marco.
- Mantener el marco visible al hacer zoom y pan.

## 1.2. Arrastre numérico vertical

**Reproducción:** el campo Ancho está cerca del borde izquierdo. Al arrastrar
para reducirlo, el cursor se queda sin recorrido antes de completar el cambio.

**Decisión:** sustituir el arrastre horizontal por una caja numérica con
arrastre vertical, sin barras de slider.

**Propuesta:**

- Clic mantenido y arrastre vertical: arriba aumenta, abajo reduce.
- El gesto no depende del borde del panel.
- Mantener edición directa por teclado.
- Evitar sliders y evitar botones `−`/`+` como control principal.

### 1.2.1. Entrada de expresiones estilo Blender

**Decisión:** la caja numérica acepta operaciones sencillas antes de confirmar.

Ejemplos:

```text
1600x2
1600/4
1600*2
1600-200
```

- Al pulsar `Enter`, se evalúa y se aplica el resultado.
- `Esc` cancela y restaura el valor anterior.
- Si la proporción está activa, el otro campo se recalcula en el mismo frame.
- Resultado entero, limitado a 1–16 384 px; si la expresión no es válida, se
  mantiene el valor anterior y se muestra un estado de error discreto.
- No se aceptan expresiones arbitrarias ni código; solo operadores básicos.

## 1.3. Rendimiento del resize

**Observación:** el resize se ejecuta en el hilo de UI al cambiar el control
(`refresh_preprocessed_source`), con Lanczos3 y subida de textura. En imágenes
grandes puede notarse.

**Propuesta:**

- Debounce para cambios de geometría, como ya existe para la codificación.
- Durante el arrastre, usar un filtro rápido o una preview reducida; aplicar
  Lanczos3 al soltar.
- Mover el resize de resolución completa a un worker si la latencia sigue siendo
  perceptible.
- Evitar clonar la imagen original en cada frame.

## 1.4. Inconsistencia visual Preprocesado/Avanzado

**Observación:** `Preprocesado` y `Avanzado` no comparten estilo. `Avanzado`
mezcla `·` y resúmenes como `Máxima · —`, que resultan confusos.

**Propuesta:**

- Un único componente de sección colapsable para ambos.
- Misma tipografía, padding, chevron, color y altura.
- Título simple: `Preprocesado` / `Avanzado`.
- Resumen como badge o línea secundaria alineada a la derecha, sin cadenas de
  `·` duplicadas.
- Reset con el mismo patrón en ambos bloques.
- Revisar contraste y foco de teclado al unificar.

## Implementación (2026-10-31)

- Ancho y alto se pintan después de procesar la interacción, así que ambos
  valores enlazados cambian en el mismo frame.
- El canvas dibuja un marco de referencia sólido y uniforme con el tamaño
  original; se sustituyó el trazo discontinuo porque la línea inferior quedaba
  más tenue por el antialiasing. Con resize activo, encuadra el original para
  que el marco sea visible.
- Los campos numéricos usan arrastre vertical con delta total y expresiones
  `1600x2`, `1600/4`, `1600*2`, `1600-200`; `Enter` aplica y `Esc` cancela.
- El resize completo corre en un worker con debounce de 120 ms; la vista previa
  no se bloquea.
- `Preprocesado` y `Avanzado` comparten cabecera, cuerpo, reset y scroll. Se
  eliminaron los resúmenes con `·`.
- El panel de historial/cuentagotas vuelve a quedar fijo sobre Exportar; no se
  desplaza con el scroll de las secciones.
- El cuerpo de las secciones reserva ancho para el scrollbar y deja margen
  derecho, evitando que el marco se recorte contra el borde del panel.
- La fila de rotación compacta sus botones y puede envolver si no cabe: era la
  que ensanchaba el cuerpo y empujaba el panel entero hasta recortarlo por la
  derecha.
- Al abrir una sección se desplaza su cabecera a la vista para que el cuerpo no
  aparezca cortado. El desplazamiento se mantiene durante la animación de
  apertura y se retira después, sin bloquear el scroll manual.
- Los atajos globales (`0`, `1`, `F`, etc.) se ignoran mientras un campo de texto
  tiene el foco.
- Suite de **59 pruebas** y validación visual bajo Xvfb: enlace en ambos
  sentidos, arrastre vertical, expresión, rotación, marco original y secciones.

## Orden de trabajo

1. `t-44` — cerrada; revisar AVIF con el usuario.
2. `t-45` — revisar las correcciones implementadas de resize/UI.
3. Revisar el resto de features una por una.
4. `t-43` — benchmark final, capturas y release.
