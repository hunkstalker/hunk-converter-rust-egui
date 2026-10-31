# Revisión de RESIZE y UI — primera pasada

Fecha: 2026-10-31 · Estado: pendiente, se aplicará después de `t-44` (AVIF)

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

## 1.1. Falta referencia del tamaño original

**Propuesta:** mostrar junto a los campos algo como:

```text
Original 1600 × 1000  →  800 × 500
```

Debe verse sin tener que recordar la ficha de la imagen. Si no hay resize, se
puede mostrar solo `Original 1600 × 1000`.

## 1.2. Arrastrar el Ancho hacia la izquierda choca con el panel

**Reproducción:** el campo Ancho está cerca del borde izquierdo. Al arrastrar
para reducirlo, el cursor se queda sin recorrido antes de completar el cambio.

**Propuesta:**

- No usar un `DragValue` puro como único control.
- Ofrecer entrada numérica + botones `−`/`+` y, opcionalmente, un slider en un
  popover o modal de resize.
- Si se mantiene el arrastre, capturar el puntero y permitir movimiento fuera
  del panel sin perder el gesto.
- Mantener edición por teclado.

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

## Orden de trabajo

1. `t-44` — cerrar la brecha AVIF.
2. `t-45` — aplicar estas correcciones.
3. `t-43` — benchmark final, capturas y release.
