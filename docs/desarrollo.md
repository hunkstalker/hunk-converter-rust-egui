# Desarrollo

Guía para clonar, compilar y probar HUNK Converter. Para usar la aplicación no
hace falta este entorno: descarga una build portable de la release.

## 1. Dos formas de trabajar

| Vía | Cuándo usarla | Requisitos |
| --- | --- | --- |
| Local | Desarrollo normal, GUI y diálogos nativos | Rust, CMake, NASM y librerías del sistema |
| Contenedor | Reproducir el entorno, CI local, forks o equipos sin dependencias | Docker + Docker Compose |

Rust está fijado en **1.99.1** por `rust-toolchain.toml`; `rustup` lo instala al
entrar en el proyecto. Las dependencias de Cargo están fijadas en `Cargo.lock`.

## 2. Instalación rápida

### Linux y macOS

```bash
./scripts/setup_dev.sh
./scripts/setup_dev.sh --install
```

El primer comando solo muestra los comandos de instalación; el segundo instala
las dependencias del sistema y el toolchain de Rust. En Linux detecta
Debian/Ubuntu, Fedora y Arch. En macOS comprueba Xcode Command Line Tools y
Homebrew.

### Windows

```powershell
.\scripts\setup_dev.ps1
.\scripts\setup_dev.ps1 -Install
```

Sin `-Install` comprueba Rust, CMake, NASM y las Visual Studio Build Tools. Con
`-Install` usa `winget` para instalar lo que falte. Después de instalar Rust o
las Build Tools, abre una terminal nueva para refrescar `PATH`.

### Comprobar el entorno

```bash
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo run --release
```

## 3. Linux manual

### Debian, Ubuntu y derivadas

```bash
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  build-essential cmake nasm pkg-config git curl ca-certificates \
  libx11-dev libxcursor-dev libxrandr-dev libxi-dev libxinerama-dev \
  libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev wayland-protocols \
  libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libgl1-mesa-dev libegl1-mesa-dev libgles2-mesa-dev \
  libfontconfig1-dev libfreetype-dev fonts-dejavu-core \
  xvfb xauth libgl1-mesa-dri mesa-utils dbus-x11
```

### Fedora

```bash
sudo dnf install -y \
  gcc gcc-c++ make cmake nasm pkgconf-pkg-config git curl \
  libX11-devel libXcursor-devel libXrandr-devel libXi-devel libXinerama-devel \
  libxkbcommon-devel libxkbcommon-x11-devel wayland-devel wayland-protocols-devel \
  mesa-libGL-devel mesa-libEGL-devel fontconfig-devel freetype-devel dejavu-sans-fonts \
  xorg-x11-server-Xvfb xauth mesa-dri-drivers
```

### Arch

```bash
sudo pacman -S --needed \
  base-devel cmake nasm pkgconf git curl \
  libx11 libxcursor libxrandr libxi libxinerama \
  libxkbcommon libxkbcommon-x11 wayland wayland-protocols mesa \
  fontconfig freetype2 ttf-dejavu xorg-server-xvfb xauth
```

Si usas Wayland, las mismas dependencias permiten compilar; el backend gráfico
se elige en tiempo de ejecución. Para probar sin pantalla se usa Xvfb.

## 4. macOS manual

```bash
xcode-select --install
brew install cmake nasm pkg-config
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

`rust-toolchain.toml` instala Rust 1.99.1 la primera vez que ejecutes `cargo`.
No hacen falta librerías X11: eframe usa Cocoa/Metal en macOS.

Si `brew install nasm` instala NASM 3.x, `libaom-sys` puede fallar con
`Unsupported nasm: multipass optimization not supported`. El CI compila NASM
2.16.01 desde fuente por ese motivo; para desarrollo local sirve cualquier NASM
2.16.x anterior a la serie 3.

## 5. Windows manual

1. Instala **Visual Studio Build Tools** con el workload
   **Desktop development with C++** (MSVC, Windows SDK y CMake opcional):
   <https://visualstudio.microsoft.com/visual-cpp-build-tools/>.
2. Instala Rust con <https://rustup.rs>. El proyecto usa el toolchain MSVC.
3. Instala CMake y NASM:

```powershell
winget install --id Kitware.CMake -e
winget install --id NASM.NASM -e
```

4. Abre una terminal nueva y comprueba:

```powershell
rustc --version
cmake --version
nasm -v
cargo test
```

Para compilar la build Windows GNU desde Linux/macOS, consulta
`scripts/build_dist.sh windows` y define `CC_x86_64_pc_windows_gnu`,
`CXX_x86_64_pc_windows_gnu` y `AR_x86_64_pc_windows_gnu`. Para desarrollo local
en Windows, MSVC es el camino recomendado.

## 6. Contenedor

### VS Code / Codespaces

Abre la carpeta del repositorio y elige **Reopen in Container**. El
`.devcontainer/devcontainer.json` usa `compose.yaml`, monta el repositorio en
`/workspace` y ejecuta `cargo fetch`.

### Docker Compose

```bash
HOST_UID=$(id -u) HOST_GID=$(id -g) docker compose build
docker compose run --rm dev
docker compose run --rm dev cargo test
```

El servicio `dev` monta el repositorio, conserva el registro de Cargo en volúmenes
y abre una shell. Para ejecutar un comando directo:

```bash
docker compose run --rm dev cargo fmt --all -- --check
docker compose run --rm dev cargo clippy --all-targets -- -D warnings
```

Si tu usuario no es 1000, pasa `HOST_UID` y `HOST_GID` al construir para que los
archivos creados en el bind mount pertenezcan a tu usuario.

### Docker sin Compose

```bash
docker build -f docker/dev.Dockerfile -t hunk-dev .
docker run --rm -it -v "$PWD:/workspace" -w /workspace hunk-dev
```

### GUI en Linux

La GUI necesita un servidor X11. Con un escritorio Linux:

```bash
xhost +local:docker
docker compose run --rm \
  -e DISPLAY \
  -v /tmp/.X11-unix:/tmp/.X11-unix \
  dev cargo run --release
xhost -local:docker
```

En macOS y Windows, Docker no muestra ventanas nativas sin un servidor X
adicional. Para desarrollo gráfico es más práctico ejecutar la app en el host;
el contenedor sigue siendo útil para compilar y probar.

### Pruebas sin pantalla

Xvfb y Mesa están incluidos en la imagen:

```bash
docker compose run --rm \
  -e LIBGL_ALWAYS_SOFTWARE=1 \
  -e HUNK_DEMO=1 \
  -e HUNK_SCREENSHOT=/workspace/captura.png \
  -e HUNK_SCREENSHOT_DELAY_MS=1800 \
  dev xvfb-run -a cargo run --release
```

### Limitaciones del contenedor

- El diálogo nativo de guardado usa `xdg-portal`; en un contenedor sin portal ni
  DBus puede no abrirse. Para probar la exportación con diálogo, ejecuta la app
  en el host.
- La GUI dentro del contenedor usa X11; en macOS/Windows requiere un servidor X
  externo.
- El renderizado con `LIBGL_ALWAYS_SOFTWARE=1` es más lento; es solo para
  pruebas y capturas.
- La imagen de desarrollo usa Debian bookworm. Las builds Linux de release se
  generan en Ubuntu 22.04 desde el CI para conservar compatibilidad de glibc.

## 7. Estructura del entorno

```text
docker/dev.Dockerfile       Imagen de desarrollo
compose.yaml                Servicio dev y volúmenes de Cargo
.devcontainer/              Integración con VS Code/Codespaces
scripts/setup_dev.sh        Dependencias en Linux/macOS
scripts/setup_dev.ps1       Dependencias en Windows
```

## 8. Problemas frecuentes

**`cargo: command not found` después de instalar Rust**

```bash
source "$HOME/.cargo/env"
```

En Windows, cierra y abre la terminal.

**Falta CMake o NASM**

Vuelve a ejecutar el script de instalación o instala los paquetes indicados en
la sección de tu sistema. Ambos son necesarios para AOM/AVIF.

**Error al crear una ventana GL**

Comprueba que tienes un servidor X11/Wayland activo. En un entorno sin GPU:

```bash
LIBGL_ALWAYS_SOFTWARE=1 cargo run --release
```

**Archivos con propietario incorrecto en el contenedor**

Reconstruye con tu UID/GID:

```bash
HOST_UID=$(id -u) HOST_GID=$(id -g) docker compose build
```

En SELinux puede hacer falta añadir `:z` al volumen del repositorio o usar
`--security-opt label=disable`.

**El diálogo de guardado no aparece en el contenedor**

Es la limitación de `xdg-portal` descrita arriba. Ejecuta la app en el host para
probar el flujo completo de exportación.

## 9. Más información

- [README](../README.md) — uso y características.
- [docs/arquitectura.md](arquitectura.md) — organización del código.
- [docs/pruebas.md](pruebas.md) — capas de prueba.
- [docs/ci.md](ci.md) — builds y release.
