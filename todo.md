# Tareas pendientes

> Convención: cada tarea usa un identificador `t-N`, un título y una descripción.
> Al completarse se retira de este archivo y se registra en `t-completadas.md`.
> Las tareas se listan en orden ascendente y los identificadores no se reutilizan.

## t-43 — Cierre 1.1: benchmark, documentación y release

**Descripción:** Repetir el benchmark final, regenerar capturas, actualizar documentación, validar builds y preparar la release 1.1.

**Estado:** AVIF superado. Bloqueada por la revisión incremental de features y por `t-45`. No etiquetar `v1.1.0` hasta revisar AVIF, aplicar `t-45`, revisar resize y el resto de features con el usuario; después se repetirá el benchmark final.

## t-45 — Correcciones de resize y UI

**Descripción:** Aplicar la primera pasada de revisión de la UI de preprocesado/avanzado: sincronizar ancho y alto con la proporción, sustituir la referencia textual por un marco del tamaño original en el canvas, usar cajas numéricas con arrastre vertical y expresiones estilo Blender, y unificar el diseño de los desplegables. Debe hacerse después de la revisión de AVIF.

**Aceptación:**

- Al cambiar el ancho con proporción activa, el alto visible se actualiza en el mismo frame.
- La referencia del tamaño original es un marco en el canvas, no texto adicional en la barra lateral.
- Las cajas numéricas aceptan `1600x2`, `1600/4`, `1600*2` y `1600-200`; `Enter` aplica y `Esc` cancela.
- El arrastre vertical cambia el valor sin chocar con el borde del panel; no hay barras de slider.
- Resize y preprocesado no bloquean la UI de forma perceptible.
- `Preprocesado` y `Avanzado` comparten tipografía, padding, chevron y resumen; sin cadenas de `·` duplicadas.
