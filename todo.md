# Tareas pendientes

> Convención: cada tarea usa un identificador `t-N`, un título y una descripción.
> Al completarse se retira de este archivo y se registra en `t-completadas.md`.
> Las tareas se listan en orden ascendente y los identificadores no se reutilizan.

## t-43 — Cierre 1.1: benchmark, documentación y release

**Descripción:** Repetir el benchmark final, regenerar capturas, actualizar documentación, validar builds y preparar la release 1.1.

**Estado:** bloqueada por AVIF y por la revisión incremental de features. No etiquetar `v1.1.0` hasta cerrar `t-44`, aplicar `t-45` y revisar cada feature con el usuario; después se repetirá el benchmark final.

## t-44 — Cerrar la brecha de compresión AVIF

**Descripción:** Igualar o superar a Squoosh en AVIF a igual calidad perceptual. Repetir la curva con más casos de UI/texto y probar variantes de ravif (speed 2–8, profundidad 8/10 bits, modelo YCbCr/RGB y alpha quality). Si no basta, evaluar libavif/AOM por tamaño, velocidad, build y portabilidad. No se acepta una brecha del 6,5 %.

**Aceptación:**

- Mediana de tamaño ≤ 1,00× frente a Squoosh a igual SSIMULACRA2.
- Sin caso de UI claramente peor.
- Velocidad ≥ 1,2× o justificación medida si se cambia a AOM.
- Pruebas de alfa, decodificación y benchmark reproducible.

## t-45 — Correcciones de resize y UI

**Descripción:** Aplicar la primera pasada de revisión de la UI de preprocesado/avanzado: sincronizar ancho y alto con la proporción, mostrar el tamaño original como referencia, resolver el arrastre del ancho contra el borde del panel y unificar el diseño de los desplegables. Debe hacerse después de `t-44`.

**Aceptación:**

- Al cambiar el ancho con proporción activa, el alto visible se actualiza en el mismo frame.
- El usuario ve el tamaño original junto al de destino.
- El control de ancho se puede reducir arrastrando sin chocar con el borde izquierdo.
- Resize y preprocesado no bloquean la UI de forma perceptible.
- `Preprocesado` y `Avanzado` comparten tipografía, padding, chevron y resumen; sin cadenas de `·` duplicadas.
