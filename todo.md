# Tareas pendientes

> Convención: cada tarea usa un identificador `t-N`, un título y una descripción.
> Al completarse se retira de este archivo y se registra en `t-completadas.md`.
> Las tareas se listan en orden ascendente y los identificadores no se reutilizan.

## t-11 — Redimensionado y encuadre opcionales

**Descripción:** Añadir controles para cambiar el tamaño de salida por píxeles o porcentaje, con opción de mantener proporción y de elegir un método de interpolación. Para ICO, permitir alternar entre ajuste centrado y recorte cuadrado. La exportación sin tocar estos ajustes debe conservar el comportamiento actual.

## t-12 — Conversión por lotes

**Descripción:** Permitir arrastrar varias imágenes y procesarlas con un mismo formato y calidad, mostrando progreso y resultados individuales. Los archivos deben guardarse en `export/` sin sobrescribir y con nombres únicos, igual que en el flujo de una sola imagen.

## t-15 — Entrada HEIC y códecs opcionales

**Descripción:** Evaluar HEIC como formato de entrada manteniendo la portabilidad y sin introducir llamadas de red. AVIF y JPEG XL ya están integrados como salida (`t-26`); queda valorar si los códecs pesados deben poder excluirse mediante features de Cargo y documentar el impacto en tamaño y compilación.

## t-16 — Navegación avanzada del lienzo

**Descripción:** Completar la navegación con un modo 1:1, zoom mediante botones o gestos además de la rueda y atajos de encuadre (ajustar, restablecer). El divisor, su círculo y el cuentagotas ya funcionan a cualquier zoom; el encuadre debe conservarse al cambiar de formato.

## t-18 — Selector avanzado de carpeta de salida

**Descripción:** Mantener `export/` junto al ejecutable como destino predeterminado y añadir una opción avanzada para elegir otra carpeta de forma puntual o recordada. Debe seguir evitando sobrescrituras accidentales y avisar si el destino no es escribible.

## t-22 — Accesibilidad y navegación completa por teclado

**Descripción:** Incorporar soporte de accesibilidad nativa de egui/AccessKit, orden de foco explícito, nombres y descripciones para lector de pantalla, navegación completa sin ratón y pruebas de contraste y escalado de fuente. El cuentagotas y el comparador deben poder usarse con teclado.

## t-34 — Optimizar el CI para 1.1.0

**Descripción:** Limitar los builds de Linux, Windows y macOS a tags y ejecución manual, dejando `main` y las pull requests solo con pruebas. Añadir caché de Cargo (`Swatinem/rust-cache`) y evaluar `sccache` para las dependencias C/C++. Mantener el workflow único y la release automática en tags.
