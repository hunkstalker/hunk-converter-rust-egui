# Tareas pendientes

> Convención: cada tarea usa un identificador `t-N`, un título y una descripción.
> Al completarse se retira de este archivo y se registra en `t-completadas.md`.
> Las tareas se listan en orden ascendente y los identificadores no se reutilizan.

## t-18 — Selector avanzado de carpeta de salida

**Descripción:** Mantener `export/` junto al ejecutable como destino predeterminado y añadir una opción avanzada para elegir otra carpeta de forma puntual o recordada. Debe seguir evitando sobrescrituras accidentales y avisar si el destino no es escribible.

## t-22 — Accesibilidad y navegación completa por teclado

**Descripción:** Incorporar soporte de accesibilidad nativa de egui/AccessKit, orden de foco explícito, nombres y descripciones para lector de pantalla, navegación completa sin ratón y pruebas de contraste y escalado de fuente. El cuentagotas y el comparador deben poder usarse con teclado.

## t-34 — Optimizar el CI para 1.1.0

**Descripción:** Limitar los builds de Linux, Windows y macOS a tags y ejecución manual, dejando `main` y las pull requests solo con pruebas. Añadir caché de Cargo (`Swatinem/rust-cache`) y evaluar `sccache` para las dependencias C/C++. Mantener el workflow único y la release automática en tags.
