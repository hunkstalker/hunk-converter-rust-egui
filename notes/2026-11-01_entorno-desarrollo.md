# Entorno de desarrollo local y en contenedor

Fecha: 2026-11-01

## Decisión

El repositorio incluye un camino reproducible para contribuidores sin convertir
Docker en un requisito:

- instalación local con scripts para Linux, macOS y Windows;
- guía detallada en `docs/desarrollo.md`;
- imagen `docker/dev.Dockerfile` y servicio `dev` en `compose.yaml`;
- `.devcontainer/devcontainer.json` para VS Code/Codespaces.

El `Dockerfile` genérico de OpenCode que se usó para preparar el entorno del
agente no forma parte de la imagen de desarrollo del proyecto.

## Instalación local

`scripts/setup_dev.sh` detecta Debian/Ubuntu, Fedora, Arch y macOS. Sin
`--install` solo muestra los comandos; con `--install` instala paquetes y el
toolchain de Rust. `scripts/setup_dev.ps1` hace la comprobación equivalente en
Windows y usa `winget` con `-Install`.

La guía recoge las dependencias exactas de X11/GL/Wayland, CMake, NASM,
fontconfig y Xvfb, además de la ejecución de la GUI y las pruebas sin pantalla.

## Contenedor

La imagen parte de `rust:1.99.1-bookworm`, instala las dependencias nativas y
crea un usuario `dev` con UID/GID configurables. Compose monta el repositorio en
`/workspace` y conserva el registro de Cargo en volúmenes. El devcontainer usa
el mismo servicio.

Limitaciones documentadas: la GUI necesita X11 y el diálogo nativo de guardado
depende de `xdg-portal`; para probar ese flujo se recomienda ejecutar la app en
el host.

## Verificación

- `bash -n scripts/setup_dev.sh` correcto.
- `setup_dev.sh` comprobado en modo inspección sobre el Linux del entorno.
- `cargo fmt --check`, clippy y las 68 pruebas siguen pasando.
- El build de la imagen no se pudo ejecutar dentro de este contenedor porque no
  tiene Docker instalado. La validación final de `docker compose build` queda
  para una máquina con Docker.
